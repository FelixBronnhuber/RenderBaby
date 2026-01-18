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

/// FrameIterator wrapper that allows to switch between multiple providers.
pub struct Provider {
    /// The current [`FrameIterator`].
    iterator: Option<Box<dyn FrameIterator>>,
    /// Pointer to the [`AtomicBool`] that indicates whether the [`FrameBuffer`] and thereby [`Provider`] has an [`FrameIterator`].
    has_provider_arc: Arc<AtomicBool>,
}

impl Provider {
    /// Create a new [`Provider`] with the given [`AtomicBool`] to indicate back whether the [`Provider`] has a [`FrameIterator`].
    pub fn new(has_next: Arc<AtomicBool>) -> Self {
        Self {
            iterator: None,
            has_provider_arc: has_next,
        }
    }

    /// Set a new [`FrameIterator`] and automatically update the `has_provider_arc` [`AtomicBool`].
    /// Immediately dismisses [`FrameIterator`] if [`FrameIterator::has_next`] returns `false`.
    pub fn set(&mut self, iterator: Box<dyn FrameIterator>) {
        if !iterator.has_next() {
            self.set_has_provider(false);
            self.iterator = None;
        } else {
            self.set_has_provider(true);
            self.iterator = Some(iterator);
        }
    }

    /// Destroys the current [`FrameIterator`] and resets the `has_provider_arc` [`AtomicBool`] indicator to `false`.
    pub fn reset(&mut self) {
        self.set_has_provider(false);
        self.iterator = None;
    }

    /// Returns whether the current [`FrameIterator`] is `None`.
    pub fn is_none(&self) -> bool {
        self.iterator.is_none()
    }

    /// Returns whether the current [`FrameIterator`] is `Some`.
    pub fn is_some(&self) -> bool {
        self.iterator.is_some()
    }

    /// Helper function to easily set the `has_provider_arc` [`AtomicBool`] indicator.
    fn set_has_provider(&mut self, has_provider: bool) {
        self.has_provider_arc
            .store(has_provider, std::sync::atomic::Ordering::SeqCst);
    }
}

impl FrameIterator for Provider {
    /// Returns `true` if there are more frames to be rendered.
    /// Delegates to the current [`FrameIterator`].
    /// Panics, if the function is called while the current [`FrameIterator`] is `None`:
    /// This is because any logic involving this [`Provider`] has to consistently account for this case.
    fn has_next(&self) -> bool {
        self.iterator
            .as_ref()
            .expect("Tried has_next() on None")
            .has_next()
    }

    /// Returns the next [`Frame`] in the sequence.
    /// Delegates to the current [`FrameIterator`].
    /// Panics, if the function is called while the current [`FrameIterator`] is `None`:
    /// This is because any logic involving this [`Provider`] has to consistently account for this case.
    fn next(&mut self) -> anyhow::Result<Frame> {
        self.iterator.as_mut().expect("Tried next() on None").next()
    }

    /// Destroys/deletes the iterator.
    /// Delegates to the current [`FrameIterator`].
    /// If the current [`FrameIterator`] is `None`, this function does nothing.
    fn destroy(&mut self) {
        if let Some(mut iterator) = self.iterator.take() {
            iterator.destroy()
        }
    }
}

/// A thread-safe buffer for [`Frame`]s.
///
/// Iterates over [`FrameIterator`] threaded so that any delay coming from the source of iteration does not delay the [`Frame`] generation.
/// # Examples
/// ```ignore
/// let iterator: FrameIterator = ...;
/// let frame_buffer = FrameBuffer::new(...);
/// frame_buffer.provide(Box::new(iterator));
/// // already starts iterating through the FrameIterator in a separate thread
/// loop {
///     frame_buffer.try_recv().unwrap().unwrap(); // frame generation is not started just here!
///     delay_for_ten_seconds();
///     // -> does not halt the generation of Frames,
///     // -> the next try_recv() call will (probably) return a Frame immediately!
/// }
pub struct FrameBuffer {
    // Receiver for the latest frame, used to avoid blocking the thread when calling try_recv()
    latest_frame_rx: Receiver<anyhow::Result<Frame>>,
    // Sender for commands to the thread, used to switch between iterators or stop them
    command_tx: Sender<BufferCommand>,
    // Thread handle to thread
    join_handle: JoinHandle<()>,
    // Whether to clone the last frame or not. If false, get_last_frame() will return an error.
    clone_last: bool,
    // Arc to the last frame, used to avoid blocking the thread when calling get_last_frame()
    last_frame: Arc<Mutex<Option<Frame>>>,
    // Indicator whether the FrameBuffer has an iterator or not
    has_provider: Arc<AtomicBool>,
}

impl FrameBuffer {
    /// Creates a new [`FrameBuffer`] with the given [`FrameIterator`] as the initial provider.
    /// If `clone_last` is `true`, the last frame will be cloned and stored in the buffer.
    /// Otherwise, get_last_frame() will return an error.
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

    /// Helper function to start the main loop of the iterator thread that constantly runs.
    /// Receives commands from the `command_tx` channel and handles them accordingly.
    /// Sends the next [`Frame`] to the `latest_frame_tx` channel.
    /// Idle if there is no iterator.
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
                let frame = provider.next();
                let is_last = !provider.has_next();

                if is_last {
                    set_last_frame(&frame);
                    provider.reset();
                }

                if let Ok(command) = command_rx.try_recv() {
                    if !is_last {
                        set_last_frame(&frame);
                    }

                    match command {
                        BufferCommand::Provide(p) => {
                            provider.set(p);
                            continue;
                        }
                        BufferCommand::StopCurrentProvider => {
                            provider.destroy();
                            provider.reset();
                            continue;
                        }
                    }
                }

                frame_tx.send(frame).unwrap();
            }
        }
    }

    /// Provides a new [`FrameIterator`] to the [`FrameBuffer`].
    /// This stops the currently running iterator if there is any.
    pub fn provide(&self, iterator: Box<dyn FrameIterator>) {
        self.command_tx
            .send(BufferCommand::Provide(iterator))
            .unwrap();
    }

    /// Stops the currently running [`FrameIterator`] if there is any.
    pub fn stop_current_provider(&self) {
        self.command_tx
            .send(BufferCommand::StopCurrentProvider)
            .unwrap();
    }

    /// Returns whether the [`FrameBuffer`] has a running [`FrameIterator`].
    pub fn has_provider(&self) -> bool {
        self.has_provider.load(std::sync::atomic::Ordering::SeqCst) && self.thread_running()
    }

    /// Returns the next [`Frame`] from the [`FrameBuffer`].
    /// Does not block but returns `None` if there is no frame available.
    pub fn try_recv(&self) -> Option<anyhow::Result<Frame>> {
        self.latest_frame_rx.try_recv().ok()
    }

    /// Returns the last [`Frame`] that was generated by the [`FrameBuffer`].
    /// Returns an error if the [`FrameBuffer`] was not configured to separately store the last frame (via [`clone_last`] flag).
    pub fn get_last_frame(&self) -> anyhow::Result<Option<Frame>> {
        if self.clone_last {
            Ok(self.last_frame.lock().unwrap().clone())
        } else {
            Err(anyhow::anyhow!(
                "Frame buffer is not configured to separately store last frame. Enable separate_last flag in FrameBuffer constructor to enable this feature."
            ))
        }
    }

    /// Returns whether the [`FrameBuffer`] main loop thread is still running.
    pub fn thread_running(&self) -> bool {
        !self.join_handle.is_finished()
    }

    /// Spawns a new thread for the [`FrameBuffer`] main loop.
    /// Returns an error if the [`FrameBuffer`] is already running.
    /// This is important if there was somehow an error in the main loop that is expected or to be ignored.
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
