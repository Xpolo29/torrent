/*
use iced::{Element, Settings, Text, Clipboard, Application, Command, Subscription};

#[derive(Debug)]
pub struct File {
    name: String,
    seeding: bool,
    leaching: bool,
    percentage: f32,
}

#[derive(Debug)]
pub enum Message {
    FileUpdated(File),
}

pub struct P2PApp {
    files: Vec<File>,
}

impl Application for P2PApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (P2PApp, Command<Message>) {
        (
            P2PApp {
                files: vec![],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("P2P Application")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::FileUpdated(file) => {
                // Update the file status here
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let content = self.files.iter().map(|file| {
            Element::new(Text::new(format!("{} - Seeding: {} - Leaching: {} - Percentage: {}%", file.name, file.seeding, file.leaching, file.percentage)))
        }).collect::<Vec<_>>();

        iced::Column::with_children(content).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

fn main() {
    let _ = P2PApp::run(Settings::default());
}
*/