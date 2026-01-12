use std::sync::{Arc, Mutex};
use egui::{Context, Window};
use log::info;

#[derive(Clone)]
pub struct Message {
    title: String,
    message: String,
}

impl Message {
    pub fn new(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
        }
    }

    pub fn from_error(error: anyhow::Error) -> Self {
        Self {
            title: "Error".to_string(),
            message: format!("{:?}", error),
        }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.title, self.message)
    }
}

#[derive(Clone)]
pub struct MessagePopupPipe {
    message_pipe: Arc<Mutex<Vec<Message>>>,
}

impl MessagePopupPipe {
    pub fn new() -> Self {
        Self {
            message_pipe: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn create_popup(title: String) -> Window<'static> {
        Window::new(title)
            .auto_sized()
            .collapsible(false)
            .resizable(false)
            .title_bar(true)
    }

    pub fn show_message_popup(
        ctx: &Context,
        title: String,
        message: String,
        on_ok: Option<Box<dyn FnOnce() + 'static>>,
    ) {
        Self::create_popup(title)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                ui.label(message);
                if ui.button("OK").clicked()
                    && let Some(on_ok) = on_ok
                {
                    on_ok();
                }
            });
    }

    pub fn show_last(&mut self, ctx: &Context) {
        let last_message = self.message_pipe.lock().unwrap().last().cloned();
        if let Some(last_message) = last_message {
            let pipe = Arc::clone(&self.message_pipe);
            let function = Some(Box::new(move || {
                pipe.lock().unwrap().pop();
            }) as Box<dyn FnOnce()>);
            Self::show_message_popup(ctx, last_message.title, last_message.message, function)
        }
    }

    pub fn push_message(&self, message: Message) {
        info!("Message pushed: {}", message);
        self.message_pipe.lock().unwrap().push(message);
    }

    pub fn default_handle<T>(&self, result: anyhow::Result<T>) {
        match result {
            Ok(_) => {}
            Err(e) => self.push_message(Message::from_error(e)),
        }
    }
}

impl Default for MessagePopupPipe {
    fn default() -> Self {
        Self::new()
    }
}
