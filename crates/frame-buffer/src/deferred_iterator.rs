use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

pub trait TemporaryLow: Send + Sync {
    // Use interior mutability in implementors; allow calling through Arc
    fn callback_low(&self);
    fn callback_normal(&self);
    fn is_active(&self) -> bool;
}

#[derive(Clone)]
pub struct DeferredIterator {
    temporary_low: Arc<dyn TemporaryLow>,
    delay: Duration,
    schedule_gen: Arc<AtomicUsize>,
}

impl DeferredIterator {
    pub fn new(temporary_low: Arc<dyn TemporaryLow>, delay_ms: u64) -> Self {
        let delay = Duration::from_millis(delay_ms);
        Self {
            temporary_low,
            delay,
            schedule_gen: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn force(&mut self) {
        self.schedule_gen.fetch_add(1, Ordering::SeqCst);
        self.temporary_low.callback_normal();
    }

    pub fn schedule(&mut self) {
        let schedule_gen_c = self.schedule_gen.fetch_add(1, Ordering::SeqCst) + 1;
        self.temporary_low.callback_low();

        let self_clone = self.clone();
        std::thread::spawn(move || {
            std::thread::sleep(self_clone.delay);
            if self_clone.schedule_gen.load(Ordering::SeqCst) != schedule_gen_c {
                return;
            }
            self_clone.temporary_low.callback_normal();
        });
    }

    pub fn is_active(&self) -> bool {
        self.schedule_gen.load(Ordering::SeqCst) != 0 || self.temporary_low.is_active()
    }
}
