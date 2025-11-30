use crate::Command;

/*
let cmd = CommandBuilder::new()
    .execute(|| set_fov(40))
    .rollback(|| set_fov(60))
    .command_type(MyCommandType::Something)
    .build();
*/

pub struct CommandBuilder<T> {
    execute: Option<Box<dyn FnMut() + Send + 'static>>,
    rollback: Option<Box<dyn FnMut() + Send + 'static>>,
    command_type: Option<T>,
    rollback_is_last_execute: bool,
}

impl<T> CommandBuilder<T> {
    pub fn new() -> Self {
        Self {
            execute: None,
            rollback: None,
            command_type: None,
            rollback_is_last_execute: false,
        }
    }

    pub fn execute<F>(mut self, f: F) -> Self
    where
        F: FnMut() + Send + 'static,
    {
        self.execute = Some(Box::new(f));
        self
    }

    pub fn rollback<F>(mut self, f: F) -> Self
    where
        F: FnMut() + Send + 'static,
    {
        self.rollback = Some(Box::new(f));
        self
    }

    pub fn command_type(mut self, command_type: T) -> Self {
        self.command_type = Some(command_type);
        self
    }

    pub fn rollback_is_last_execute(mut self) -> Self {
        self.rollback_is_last_execute = true;
        self
    }

    pub fn build(self) -> Command<T> {
        Command {
            execute: self.execute.unwrap(),
            rollback: self.rollback,
            command_type: self.command_type.unwrap(),
            rollback_is_last_execute: self.rollback_is_last_execute,
        }
    }
}

/*
Another approach to build command easier, so that:
let cmd = Command {
    execute: as_cmd(move || set_fov(40)),
    rollback: Some(as_cmd(move || set_fov(60))),
    command_type: MyCommandType::Something,
};
*/

fn as_cmd<F>(f: F) -> Box<dyn FnMut() + Send + 'static>
where
    F: FnMut() + Send + 'static,
{
    Box::new(f)
}
