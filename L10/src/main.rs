use iced::alignment::Horizontal;
use iced::widget::{button, column, container, row, text};
use iced::{Application, Command, Element, Length, Settings, Theme};

pub fn main() -> iced::Result {
    Calc::run(Settings::default())
}

struct Calc {
    display: String,         // shows the current entry/result, starts at "0"
    first: Option<f64>,      // LHS stored when an operator is chosen
    op: Option<Op>,          // pending operation (+ - * /)
    entering_new: bool,      // true when the next digit should start a new number
}

#[derive(Debug, Clone, Copy)]
enum Op { Add, Sub, Mul, Div }

#[derive(Debug, Clone)]
enum Message {
    Digit(u8),    // 0..=9
    SetOp(Op),    // + - * /
    Equals,       // =
    Clear,        // C
    Exit,         // X
}

impl Default for Calc {
    fn default() -> Self {
        Self {
            display: "0".into(),
            first: None,
            op: None,
            entering_new: false,
        }
    }
}

impl Application for Calc {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String { "Iced — Calculator (Starter+)".into() }

    fn update(&mut self, msg: Self::Message) -> Command<Self::Message> {
        match msg {
            Message::Digit(n) => {
                // HINT IMPLEMENTED: number entry
                // If we're starting a fresh number or currently "0", replace.
                // Otherwise append.
                if self.entering_new || self.display == "0" {
                    self.display = n.to_string();
                    self.entering_new = false;
                } else {
                    self.display.push(char::from(b'0' + n));
                }
            }

            Message::SetOp(new_op) => {
                let current = parse_f64(&self.display);
                match (self.first, self.op) {
                    (None, _) => {
                        // First op after typing a number: store LHS
                        self.first = Some(current);
                    }
                    (Some(_lhs), Some(_op)) if !self.entering_new => {
                        // TODO (student): CHAINING
                    }
                    _ => {
                        // Pressed operator twice? You choose a policy:
                        // - Replace pending op
                        // - Or ignore second press
                    }
                }
                // Set/replace the pending operator and start new entry
                self.op = Some(new_op);
                self.entering_new = true;
            }

            Message::Equals => {
                // TODO (student): EQUALS
            }

            Message::Clear => {
                // Reset to initial state
                *self = Self::default();
                self.display = "0".into();
            }

            Message::Exit => {
                // Simple close for the lab
                std::process::exit(0);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        // Layout constants (avoid stretching to full window)
        const BTN: f32 = 64.0;
        const GAP: f32 = 8.0;
        const GRID_W: f32 = BTN * 4.0 + GAP * 3.0;

        // Uniform button helpers
        let btn = |label: &str, msg: Message| {
            button(text(label))
                .on_press(msg)
                .width(Length::Fixed(BTN))
                .height(Length::Fixed(BTN))
        };
        let digit = |n: u8| btn(&n.to_string(), Message::Digit(n));
        let op = |sym: &str, o: Op| btn(sym, Message::SetOp(o));

        // Display: fixed width, right aligned
        let display = container(text(&self.display).size(36))
            .width(Length::Fixed(GRID_W))
            .padding([8, 12])
            .align_x(Horizontal::Right);

        // Rows
        let r1 = row![ digit(7), digit(8), digit(9), op("/", Op::Div) ]
            .spacing(GAP).width(Length::Fixed(GRID_W));
        // Do your code 

        // Exit matches grid width (not Fill)
        let exit = button(text("Exit (X)"))
            .on_press(Message::Exit)
            .width(Length::Fixed(GRID_W))
            .height(Length::Fixed(48.0));

        column![display, r1, r2, r3, r4, exit]
            .spacing(GAP)
            .padding(12)
            .into()
    }
}

/* ──────────────── Small utilities ──────────────── */

fn parse_f64(s: &str) -> f64 {
    // HINT: safe parse; extend later for decimals
    s.parse::<f64>().unwrap_or(0.0)
}

// You will write this:
// fn apply(op: Op, a: f64, b: f64) -> f64 {
//     match op {
//         Op::Add => /* ... */,
//         Op::Sub => /* ... */,
//         Op::Mul => /* ... */,
//         Op::Div => /* choose divide-by-zero policy */,
//     }
// }