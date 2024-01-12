use iced::{
    widget::{container, text_input},
    Element, Length, Sandbox, Settings,
};

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
        String::from("buggy_text_selection")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ContentChanged(text) => {
                self.content = text;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        container(
            text_input("placeholder", &self.content)
                .on_input(Message::ContentChanged)
                .width(Length::Fixed(200.0))
                .size(25),
        )
        .width(Length::Fill)
        .center_x()
        .padding(50)
        .into()
    }
}
