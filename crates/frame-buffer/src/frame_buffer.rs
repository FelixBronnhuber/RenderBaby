use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::thread;
use std::thread::JoinHandle;
use crate::frame_iterator::*;

pub enum BufferCommand {
    Provide(Box<dyn FrameIterator>),
    StopCurrentProvider,
}

pub struct Provider {
    iterator: Option<Box<dyn FrameIterator>>,
    has_provider_arc: Arc<AtomicBool>,
}

impl Provider {
    pub fn new(has_next: Arc<AtomicBool>) -> Self {
        Self {
            iterator: None,
            has_provider_arc: has_next,
        }
    }

    pub fn set(&mut self, iterator: Box<dyn FrameIterator>) {
        if !iterator.has_next() {
            self.has_provider_arc
                .store(false, std::sync::atomic::Ordering::SeqCst);
            self.iterator = None;
        } else {
            self.has_provider_arc
                .store(true, std::sync::atomic::Ordering::SeqCst);
            self.iterator = Some(iterator);
        }
    }

    pub fn reset(&mut self) {
        self.has_provider_arc
            .store(false, std::sync::atomic::Ordering::SeqCst);
        self.iterator = None;
    }

    pub fn is_none(&self) -> bool {
        self.iterator.is_none()
    }

    pub fn is_some(&self) -> bool {
        self.iterator.is_some()
    }
}

impl FrameIterator for Provider {
    fn has_next(&self) -> bool {
        self.iterator
            .as_ref()
            .expect("Tried has_next() on None")
            .has_next()
    }

    fn next(&mut self) -> anyhow::Result<Frame> {
        self.iterator.as_mut().expect("Tried next() on None").next()
    }

    fn destroy(&mut self) {
        if let Some(mut iterator) = self.iterator.take() {
            iterator.destroy()
        }
    }
}

pub struct FrameBuffer {
    latest_frame_rx: Arc<Mutex<Receiver<anyhow::Result<Frame>>>>,
    command_tx: Sender<BufferCommand>,
    join_handle: JoinHandle<()>,
    clone_last: bool,
    last_frame: Arc<Mutex<Option<Frame>>>,
    has_provider: Arc<AtomicBool>,
}

impl FrameBuffer {
    pub fn new(clone_last: bool) -> Self {
        let (latest_frame_tx, latest_frame_rx) = std::sync::mpsc::channel();
        let (command_tx, command_rx) = std::sync::mpsc::channel();

        let last_frame = Arc::new(Mutex::new(None));
        let last_frame_clone = if clone_last {
            Some(last_frame.clone())
        } else {
            None
        };
        let has_provider = Arc::new(AtomicBool::new(false));
        let has_provider_clone = has_provider.clone();
        let join_handle = thread::spawn(move || {
            FrameBuffer::frame_iter_loop(
                command_rx,
                latest_frame_tx,
                last_frame_clone,
                has_provider_clone,
            );
        });

        Self {
            latest_frame_rx: Arc::new(Mutex::new(latest_frame_rx)),
            command_tx,
            join_handle,
            clone_last,
            last_frame,
            has_provider,
        }
    }

    fn frame_iter_loop(
        command_rx: Receiver<BufferCommand>,
        frame_tx: Sender<anyhow::Result<Frame>>,
        last_frame: Option<Arc<Mutex<Option<Frame>>>>,
        has_provider: Arc<AtomicBool>,
    ) {
        let mut provider: Provider = Provider::new(has_provider);

        let set_last_frame = |frame: &anyhow::Result<Frame>| {
            if let Some(last_frame) = last_frame.as_ref()
                && let Ok(frame) = frame.as_ref()
            {
                *last_frame.lock().unwrap() = Some(frame.clone());
            }
        };

        let catch_command = |command: BufferCommand, provider: &mut Provider| -> bool {
            match command {
                BufferCommand::Provide(p) => {
                    provider.set(p);
                    true
                }
                BufferCommand::StopCurrentProvider => {
                    provider.destroy();
                    provider.reset();
                    true
                }
            }
        };

        loop {
            if provider.is_none() {
                match command_rx.recv() {
                    Ok(BufferCommand::Provide(p)) => {
                        provider.set(p);
                    }
                    Ok(BufferCommand::StopCurrentProvider) => continue,
                    Err(_) => {
                        break;
                    }
                }
            } else {
                #[allow(clippy::collapsible_if)]
                if let Ok(command) = command_rx.try_recv() {
                    if catch_command(command, &mut provider) {
                        continue;
                    }
                }

                let frame = provider.next();
                let is_last = !provider.has_next();

                if is_last {
                    set_last_frame(&frame);
                    provider.reset();
                } else if let Ok(command) = command_rx.try_recv() {
                    set_last_frame(&frame);
                    if catch_command(command, &mut provider) {
                        continue;
                    }
                }

                frame_tx.send(frame).unwrap();
            }
        }
    }

    pub fn provide(&self, iterator: Box<dyn FrameIterator>) {
        self.command_tx
            .send(BufferCommand::Provide(iterator))
            .unwrap();
    }

    pub fn stop_current_provider(&self) {
        self.command_tx
            .send(BufferCommand::StopCurrentProvider)
            .unwrap();
    }

    pub fn has_provider(&self) -> bool {
        self.has_provider.load(std::sync::atomic::Ordering::SeqCst) && self.thread_running()
    }

    pub fn try_recv(&self) -> Option<anyhow::Result<Frame>> {
        self.latest_frame_rx.lock().unwrap().try_recv().ok()
    }

    pub fn get_last_frame(&self) -> anyhow::Result<Option<Frame>> {
        if self.clone_last {
            Ok(self.last_frame.lock().unwrap().clone())
        } else {
            Err(anyhow::anyhow!(
                "Frame buffer is not configured to separately store last frame. Enable separate_last flag in FrameBuffer constructor to enable this feature."
            ))
        }
    }

    pub fn thread_running(&self) -> bool {
        !self.join_handle.is_finished()
    }

    pub fn spawn_new(&mut self) -> anyhow::Result<()> {
        if self.thread_running() {
            Err(anyhow::anyhow!("Frame buffer is already running"))
        } else {
            let (latest_frame_tx, latest_frame_rx) = std::sync::mpsc::channel();
            let (command_tx, command_rx) = std::sync::mpsc::channel();

            let last_frame_clone = if self.clone_last {
                Some(self.last_frame.clone())
            } else {
                None
            };

            let has_provider = self.has_provider.clone();
            self.join_handle = thread::spawn(move || {
                FrameBuffer::frame_iter_loop(
                    command_rx,
                    latest_frame_tx,
                    last_frame_clone,
                    has_provider,
                );
            });

            self.latest_frame_rx = Arc::new(Mutex::new(latest_frame_rx));
            self.command_tx = command_tx;
            Ok(())
        }
    }
}
