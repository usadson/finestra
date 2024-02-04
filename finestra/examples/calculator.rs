// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use finestra::*;

#[derive(Default)]
struct Application;

impl AppDelegate<AppState> for Application {
    fn configure_main_window(&mut self) -> WindowConfiguration {
        WindowConfiguration::default()
            .with_title("Finestra Calculator")
    }

    fn make_content_view(&mut self, state: &mut AppState, _: Window) -> impl finestra::View<Self, AppState>  {
        state.label.set("0");
        state.clear_text.set("AC");

        Stack::vertical()
            .with({
                Stack::horizontal()
                    .with(Button::new(state.clear_text.clone())
                            .with_on_click(|state: &mut AppState, _| state.clear())
                            .with_background_color(SystemColor::Orange))
                    .with(Label::new(state.label.clone()))
            })
            .with({
                Stack::horizontal()
                    .with(Button::new("7")
                        .with_on_click(|state: &mut AppState, _| state.digit(7)))
                    .with(Button::new("8")
                        .with_on_click(|state: &mut AppState, _| state.digit(8)))
                    .with(Button::new("9")
                        .with_on_click(|state: &mut AppState, _| state.digit(9)))
                    .with(Button::new("+")
                        .with_on_click(|state: &mut AppState, _| state.operator(Operator::Add)))
            })
            .with({
                Stack::horizontal()
                    .with(Button::new("4")
                        .with_on_click(|state: &mut AppState, _| state.digit(4)))
                    .with(Button::new("5")
                        .with_on_click(|state: &mut AppState, _| state.digit(5)))
                    .with(Button::new("6")
                        .with_on_click(|state: &mut AppState, _| state.digit(6)))
                    .with(Button::new("-")
                        .with_on_click(|state: &mut AppState, _| state.operator(Operator::Subtract)))
            })
            .with({
                Stack::horizontal()
                    .with(Button::new("1")
                        .with_on_click(|state: &mut AppState, _| state.digit(1)))
                    .with(Button::new("2")
                        .with_on_click(|state: &mut AppState, _| state.digit(2)))
                    .with(Button::new("3")
                        .with_on_click(|state: &mut AppState, _| state.digit(3)))
                    .with(Button::new("*")
                        .with_on_click(|state: &mut AppState, _| state.operator(Operator::Multiply)))
            })
            .with({
                Stack::horizontal()
                    .with(Button::new("0")
                        .with_on_click(|state: &mut AppState, _| state.digit(0)))
                    .with(Button::new("=")
                        .with_on_click(|state: &mut AppState, _| state.evaluate()))
                    .with(Button::new(",")
                        .with_on_click(|state: &mut AppState, _| state.decimal()))
                    .with(Button::new("/")
                        .with_on_click(|state: &mut AppState, _| state.operator(Operator::Divide)))
            })
    }
}

#[derive(Debug, Default)]
struct AppState {
    clear_text: TextValue,

    label: TextValue,

    decimal: bool,
    value: f64,
    operator: Operator,
    rhs: Option<f64>,
}

impl AppState {
    fn clear(&mut self) {
        if self.rhs.is_none() {
            self.reset();
            return;
        }

        self.rhs = None;
        self.label.set("0");
        self.clear_text.set("AC");
    }

    fn decimal(&mut self) {
        self.decimal = true;
    }

    fn digit(&mut self, digit: u8) {
        let mut rhs = self.rhs.unwrap_or_default();

        if self.decimal {
            let abs = rhs as isize as f64;
            let mut decimal = rhs - abs;
            decimal += digit as f64;
            decimal /= 10.0;
            rhs = abs + decimal;
        } else {
            rhs *= 10.0;
            rhs += digit as f64;
        }

        self.rhs = Some(rhs);
        self.label.set(format!("{}", rhs));

        self.clear_text.set("C");
    }

    fn evaluate(&mut self) {
        self.decimal = false;

        let Some(rhs) = self.rhs else {
            return;
        };

        match self.operator {
            Operator::Initial => {
                self.value = rhs;
            }

            Operator::Add => {
                self.value += rhs;
            }

            Operator::Subtract => {
                self.value -= rhs;
            }

            Operator::Divide => {
                self.value /= rhs;
            }

            Operator::Multiply => {
                self.value *= rhs;
            }
        }

        self.label.set(format!("{}", self.value));
        self.rhs = None;
        self.operator = Operator::Initial;
    }

    fn operator(&mut self, operator: Operator) {
        self.evaluate();

        self.operator = operator;
    }

    fn reset(&mut self) {
        self.clear_text.set("AC");
        self.label.set("0");
        self.decimal = false;
        self.value = 0.0;
        self.operator = Operator::default();
        self.rhs = None;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Operator {
    #[default]
    Initial,
    Add,
    Subtract,
    Divide,
    Multiply,
}

fn main() {
    App::with_state(Application::default(), AppState::default())
        .run();
}
