use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
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
        let join_handle = thread::spawn(move || {
            FrameBuffer::frame_iter_loop(command_rx, latest_frame_tx, last_frame_clone);
        });

        Self {
            latest_frame_rx,
            command_tx,
            join_handle,
            clone_last,
            last_frame,
        }
    }

    fn frame_iter_loop(
        command_rx: Receiver<BufferCommand>,
        frame_tx: Sender<anyhow::Result<Frame>>,
        last_frame: Option<Arc<Mutex<Option<Frame>>>>,
    ) {
        let mut provider: Option<Box<dyn FrameIterator>> = None;
        loop {
            if provider.is_none() {
                match command_rx.recv() {
                    Ok(BufferCommand::Provide(p)) => {
                        // initial check just in case
                        if p.has_next() {
                            provider = Some(p);
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
                        BufferCommand::Provide(_) => {
                            provider = None;
                            continue;
                        }
                        BufferCommand::StopCurrentProvider => {
                            p.destroy();
                            provider = None;
                            continue;
                        }
                    }
                }

                let frame = p.next();
                if !p.has_next() {
                    provider = None;
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

    pub fn provide(&self, provider: Box<dyn FrameIterator>) {
        self.command_tx
            .send(BufferCommand::Provide(provider))
            .unwrap();
    }

    pub fn stop_current_provider(&self) {
        self.command_tx
            .send(BufferCommand::StopCurrentProvider)
            .unwrap();
    }

    pub fn is_running(&self) -> bool {
        !self.join_handle.is_finished()
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

    pub fn spawn_new(&mut self) -> anyhow::Result<()> {
        if self.is_running() {
            Err(anyhow::anyhow!("Frame buffer is already running"))
        } else {
            let (latest_frame_tx, latest_frame_rx) = std::sync::mpsc::channel();
            let (command_tx, command_rx) = std::sync::mpsc::channel();

            let last_frame_clone = if self.clone_last {
                Some(self.last_frame.clone())
            } else {
                None
            };
            self.join_handle = thread::spawn(move || {
                FrameBuffer::frame_iter_loop(command_rx, latest_frame_tx, last_frame_clone);
            });
            self.latest_frame_rx = latest_frame_rx;
            self.command_tx = command_tx;
            Ok(())
        }
    }
}
