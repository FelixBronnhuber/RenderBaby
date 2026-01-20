use std::sync::{Arc, Mutex};
use egui::{Context, Window};
use log::info;

/// Maximum length of a message to clip long messages.
const MAX_MESSAGE_LENGTH: usize = 150;

/// Struct that holds a title and a message.
#[derive(Clone)]
pub struct Message {
    /// Title of the message.
    title: String,
    /// Message of the message.
    message: String,
}

impl Message {
    /// Create a new [`Message`] with the given title and message.
    pub fn new(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: Self::cut_length(message.to_string()),
        }
    }

    /// Create a new [`Message`] from an [`anyhow::Error`].
    ///
    /// Sets the title to `"Error"` and the message to the message of the [`anyhow::Error`].
    pub fn from_error(error: anyhow::Error) -> Self {
        Self {
            title: "Error".to_string(),
            message: Self::cut_length(format!("{:?}", error)),
        }
    }

    /// Automatically cuts long messages to a maximum length ([`MAX_MESSAGE_LENGTH`]).
    fn cut_length(message: String) -> String {
        if message.chars().count() <= MAX_MESSAGE_LENGTH {
            message
        } else {
            message.chars().take(MAX_MESSAGE_LENGTH).collect::<String>() + "..."
        }
    }
}

impl std::fmt::Display for Message {
    /// Formats the [`Message`] as `"title: message"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.title, self.message)
    }
}

/// Struct that holds a thread-safe pipe of [`Message`]s.
#[derive(Clone)]
pub struct MessagePopupPipe {
    message_pipe: Arc<Mutex<Vec<Message>>>,
}

impl MessagePopupPipe {
    /// Create a new [`MessagePopupPipe`] with an empty pipe.
    pub fn new() -> Self {
        Self {
            message_pipe: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Helper function to create a [`Window`] for a message popup.
    pub fn create_popup(title: String) -> Window<'static> {
        Window::new(title)
            .auto_sized()
            .collapsible(false)
            .resizable(false)
            .title_bar(true)
    }

    /// Helper function that generally shows a message popup.
    ///
    /// # Function Parameters
    /// - `ctx`: The [`Context`] to show the popup in.
    /// - `title`: The title of the popup.
    /// - `message`: The message to show in the popup.
    /// - `on_ok`: Optional function that is called when the "OK" button is pressed
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

    /// Shows the last message in the pipe.
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

    /// Pushes a message to the pipe.
    pub fn push_message(&self, message: Message) {
        info!("Message pushed: {}", message);
        self.message_pipe.lock().unwrap().push(message);
    }

    /// Function that handles the result of an operation and pushes a message to the pipe if it failed.
    /// Warning: does not return the unwrapped result!
    pub fn default_handle<T>(&self, result: anyhow::Result<T>) {
        match result {
            Ok(_) => {}
            Err(e) => self.push_message(Message::from_error(e)),
        }
    }
}

impl Default for MessagePopupPipe {
    /// Create a new [`MessagePopupPipe`] with an empty pipe.
    fn default() -> Self {
        Self::new()
    }
}
