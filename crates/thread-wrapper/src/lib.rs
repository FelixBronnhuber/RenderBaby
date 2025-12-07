use std::thread;

pub struct Threaded<F, T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    task: Option<F>,
    _phantom: std::marker::PhantomData<T>,
}

pub fn threaded<F, T>(task: F) -> Threaded<F, T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    Threaded {
        task: Some(task),
        _phantom: std::marker::PhantomData,
    }
}

impl<F, T> Threaded<F, T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    pub fn after<C>(mut self, callback: C) -> ()
    where
        C: FnOnce(T) + Send + 'static,
    {
        let task = self.task.take().expect("Task already taken");
        thread::spawn(move || {
            let result = task();
            callback(result);
        });
    }

    pub fn call(self) {
        let task = self.task.expect("Task already taken");
        thread::spawn(move || {
            task();
        });
    }
}
