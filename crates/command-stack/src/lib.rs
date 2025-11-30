mod builder;

use std::sync::{Arc, Mutex, MutexGuard};

pub struct Command<T> {
    pub execute: Box<dyn FnMut() + Send + 'static>,
    pub rollback: Option<Box<dyn FnMut() + Send + 'static>>,
    pub command_type: T,
    pub rollback_is_last_execute: bool,
}

pub struct CommandStack<T> {
    execute_stack: Arc<Mutex<Vec<Command<T>>>>,
    rollback_stack: Arc<Mutex<Vec<Command<T>>>>,
}

impl<T> CommandStack<T>
where
    T: Clone + Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            execute_stack: Arc::new(Mutex::new(Vec::new())),
            rollback_stack: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn execute(&self, mut command: Command<T>) {
        (command.execute)();
        self.execute_stack.lock().unwrap().push(command);
        self.clear_rollback_stack();
    }

    pub fn rollback(&self) {
        if let Some(mut command) = self.execute_stack.lock().unwrap().pop() {
            /*
            Sometimes, the rollback of one command is just the execution method of the last command of the same type.
            The attribute rollback_is_last_execute is used to identify this case.
            */
            if command.rollback_is_last_execute {
                if let Some(mut guard) = self.get_executes_guard(&command.command_type) {
                    if let Some(prev_command_of_type) = guard
                        .iter_mut()
                        .rev()
                        .find(|c| &c.command_type == &command.command_type)
                    {
                        (prev_command_of_type.execute)();
                    }
                }
            } else if let Some(ref mut rollback_fn) = command.rollback {
                rollback_fn();
            }
            self.rollback_stack.lock().unwrap().push(command);
        }
    }

    pub fn reexecute(&self) {
        if let Some(mut command) = self.rollback_stack.lock().unwrap().pop() {
            (command.execute)();
            self.execute_stack.lock().unwrap().push(command);
        }
    }

    fn clear_rollback_stack(&self) {
        self.rollback_stack.lock().unwrap().clear();
    }

    fn get_executes_guard(&'_ self, command_type: &T) -> Option<MutexGuard<'_, Vec<Command<T>>>> {
        let guard = self.execute_stack.lock().unwrap();
        if guard.iter().any(|c| &c.command_type == command_type) {
            Some(guard)
        } else {
            None
        }
    }
}
