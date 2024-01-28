# Finestra
[![CI](https://github.com/usadson/finestra/actions/workflows/ci.yml/badge.svg)](https://github.com/usadson/finestra/actions/workflows/ci.yml) ![Crates.io Version](https://img.shields.io/crates/v/finestra) ![GitHub License](https://img.shields.io/github/license/usadson/finestra)

Proof of Concept for a cross-platform UI library. Written in Rust.

## Example
```rs
use finestra::{App, AppDelegate, Label};

#[derive(Default)]
struct Application;

impl AppDelegate for Application {
    fn make_content_view(&mut self) -> impl finestra::View {
        Label::new("Hello, world!")
    }
}

fn main() {
    let app = App::new(Application::default());
    app.run();
}
```
