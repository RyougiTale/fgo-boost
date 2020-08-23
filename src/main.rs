#[macro_use]
extern crate log;
extern crate pretty_env_logger;
use pretty_env_logger::formatted_builder;
use std::env;
use iced::{button, executor, Application, Button, Column, Command, Element, Settings, Text};

enum Gui{
    Loading,
    Loaded(State),
}


fn main() {
    // formatted_builder(). .filter(&env::var("MY_APP_LOG").unwrap_or_default()).init();
    pretty_env_logger::init();
    // log::error!("this is an error {}", "message");
    Gui::run(Settings::default());
}

#[derive(Debug, Default)]
struct State {
    connect_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ConnectButtonPressed,
}

impl Application for Gui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Gui, Command<Self::Message>) {
        (Gui::Loading, Command::none())
    }

    fn title(&self) -> String {
        info!("[fgo boost] click connect button");

        String::from("fgo boost")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("hello world").into()
    }
}



// impl Gui {
//     pub fn view(&mut self) -> Column<Message> {
//         Column::new().push(
//             Button::new(&mut self.connect_button, Text::new("connect"))
//                 .on_press(Message::ConnectButtonPressed),
//         )
//     }

//     pub fn update(&mut self, message: Message) {
//         match message {
//             Message::ConnectButtonPressed => {
//                 info!("click connect button");
//             }
//         }
//     }
// }
