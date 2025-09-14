use iced::widget::{button, column, row, text};
use iced::{Alignment, Element, Sandbox, Settings, Theme};
// button, column, row, text → common UI widgets.
// Alignment → controls layout alignment.
// Element → the generic container type for any widget tree.
// Sandbox → the simple synchronous app trait.
// Settings, Theme → window setup and light/dark style.

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

#[derive(Default)] // #[derive(Default)] lets us create a Counter with value = 0.
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)] // Clone + Copy so messages are lightweight and easy to reuse.
enum Message {
    Increment,
    Decrement,
}

impl Sandbox for Counter {  
    type Message = Message; // Tells Iced this app will use Message as its event type.

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Cool Counter")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Self::Message) {      // Runs whenever a Message is triggered.
        match message {                                 // Changes the app’s state: increases or decreases the count.
            Message::Increment => self.value += 1,      // After updating, Iced automatically re-renders the view.
            Message::Decrement => self.value -= 1,
        }
    }
    
    fn view(&self) -> Element<Self::Message> {
        let dec = button(text("-")).on_press(Message::Decrement);
        let inc = button(text("+")).on_press(Message::Increment);

        column![                                        // A column containing a title and a row.
            text("Counter").size(32),                   // The row has:
            row![dec, text(self.value).size(28), inc]   // A decrement button "-".
                .spacing(12)                            // he current value (self.value).
                .align_items(Alignment::Center),        // An increment button "+".
        ]
        .padding(24)
        .spacing(16)
        .into()
        
        // column![
        //     text("Operations").size(32),
        //     row![inc],
        // ]
        // .padding(24)
        // .spacing(16)
        // .into()
    }
}