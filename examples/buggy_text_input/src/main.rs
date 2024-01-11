use iced::{widget::text_input, Element, Sandbox, Settings};

fn main() -> iced::Result {
    App::run(Settings::default())
}

pub struct App {
    content: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ContentChanged(String),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            content: "The first five digits of Pi are".into(),
        }
    }

    fn title(&self) -> String {
        String::from("buggy_text_input")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ContentChanged(text) => {
                self.content = text;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        text_input("placeholder", &self.content)
            .on_input(Message::ContentChanged)
            .size(50)
            .into()
    }
}
