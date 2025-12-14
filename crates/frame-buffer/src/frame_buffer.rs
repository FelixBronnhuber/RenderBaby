use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use crate::frame_provider::*;

pub enum BufferCommand {
    Provide(Box<dyn FrameProvider>),
    StopCurrentProvider,
}

pub struct FrameBuffer {
    latest_frame_rx: Receiver<anyhow::Result<Frame>>,
    command_tx: Sender<BufferCommand>,
    join_handle: JoinHandle<()>,
}

impl FrameBuffer {
    pub fn new() -> Self {
        let (latest_frame_tx, latest_frame_rx) = std::sync::mpsc::channel();
        let (command_tx, command_rx) = std::sync::mpsc::channel();

        let join_handle = thread::spawn(move || {
            FrameBuffer::frame_iter_loop(command_rx, latest_frame_tx);
        });

        Self {
            latest_frame_rx,
            command_tx,
            join_handle,
        }
    }

    fn frame_iter_loop(
        command_rx: Receiver<BufferCommand>,
        frame_tx: Sender<anyhow::Result<Frame>>,
    ) {
        let mut provider: Option<Box<dyn FrameProvider>> = None;
        loop {
            if provider.is_none() {
                match command_rx.recv() {
                    Ok(BufferCommand::Provide(p)) => {
                        provider = Some(p);
                    }
                    Ok(BufferCommand::StopCurrentProvider) => continue,
                    Err(_) => {
                        break;
                    }
                }
            }

            if let Some(p) = provider.as_mut() {
                if !p.has_next() {
                    provider = None;
                    continue;
                }

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
                frame_tx.send(frame).unwrap();
            }
        }
    }

    pub fn provide(&self, provider: Box<dyn FrameProvider>) {
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

    pub fn spawn_new(&mut self) -> anyhow::Result<()> {
        if self.is_running() {
            Err(anyhow::anyhow!("Frame buffer is already running"))
        } else {
            let (latest_frame_tx, latest_frame_rx) = std::sync::mpsc::channel();
            let (command_tx, command_rx) = std::sync::mpsc::channel();

            self.join_handle = thread::spawn(move || {
                FrameBuffer::frame_iter_loop(command_rx, latest_frame_tx);
            });
            self.latest_frame_rx = latest_frame_rx;
            self.command_tx = command_tx;
            Ok(())
        }
    }
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self::new()
    }
}
