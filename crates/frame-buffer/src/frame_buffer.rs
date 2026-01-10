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

pub struct FrameBuffer {
    latest_frame_rx: Receiver<anyhow::Result<Frame>>,
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
            latest_frame_rx,
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
        let mut provider: Option<Box<dyn FrameIterator>> = None;
        has_provider.store(false, std::sync::atomic::Ordering::SeqCst);

        loop {
            if provider.is_none() {
                match command_rx.recv() {
                    Ok(BufferCommand::Provide(p)) => {
                        // initial check just in case
                        if p.has_next() {
                            provider = Some(p);
                            has_provider.store(true, std::sync::atomic::Ordering::SeqCst);
                        }
                    }
                    Ok(BufferCommand::StopCurrentProvider) => continue,
                    Err(_) => {
                        break;
                    }
                }
            }

            if let Some(p) = provider.as_mut() {
                if let Ok(command) = command_rx.try_recv() {
                    match command {
                        BufferCommand::Provide(p) => {
                            if p.has_next() {
                                provider = Some(p);
                                has_provider.store(true, std::sync::atomic::Ordering::SeqCst);
                            }
                            continue;
                        }
                        BufferCommand::StopCurrentProvider => {
                            p.destroy();
                            provider = None;
                            has_provider.store(false, std::sync::atomic::Ordering::SeqCst);
                            continue;
                        }
                    }
                }

                let frame = p.next();
                if !p.has_next() {
                    provider = None;
                    has_provider.store(false, std::sync::atomic::Ordering::SeqCst);
                    if let Some(last_frame) = last_frame.as_ref()
                        && let Ok(frame) = frame.as_ref()
                    {
                        *last_frame.lock().unwrap() = Some(frame.clone());
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
        self.latest_frame_rx.try_recv().ok()
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

            self.latest_frame_rx = latest_frame_rx;
            self.command_tx = command_tx;
            Ok(())
        }
    }
}
