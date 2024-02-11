# Finestra
[![CI](https://github.com/usadson/finestra/actions/workflows/ci.yml/badge.svg)](https://github.com/usadson/finestra/actions/workflows/ci.yml) [![Crates.io Version](https://img.shields.io/crates/v/finestra)](https://crates.io/crates/finestra) [![GitHub License](https://img.shields.io/github/license/usadson/finestra)](https://github.com/usadson/finestra/blob/main/COPYING)

Finestra is a simple and practical desktop UI framework for Rust. It maintains the authentic look and feel of each platform by integrating with their native UI backends. With Finestra, you can write an application that targets both Windows and macOS.

## Installation
Finestra [provides a crate](https://crates.io/crates/finestra) which contains all the tools you need to start developing desktop applications:
```toml
[dependencies]
finestra = "0.1.0"
```

## Example
The following example demonstrates the basic usage of Finestra, by providing a button that get its text updated each time it is clicked.

```rs
use finestra::*;

struct MyApplication;

impl AppDelegate<AppState> for Application {
    fn make_content_view(&mut self, state: &mut AppState, _: Window) -> impl finestra::View<Self, AppState>  {
        state.label.set("Count: 0");

        Button::new(&state.label)
            .with_on_click(|state: &mut AppState, window: Window| {
                state.count += 1;
                state.label.set(format!("Count: {}", state.count));

                if state.count % 10 == 0 {
                    window.create_dialog(format!("You clicked {} times!", state.count))
                        .show();
                }
            })
    }
}

#[derive(Debug, Default)]
struct AppState {
    count: usize,
    label: TextValue,
}

fn main() {
    App::with_state(MyApplication, AppState::default())
        .run();
}
```

## Usage
The crate provides a single entrypoint, the [`App`](https://docs.rs/finestra/latest/finestra/struct.App.html) structure.
```rs
App::new(MyApplication::default()).run()
```

To react to common events (such as [launching](https://docs.rs/finestra/latest/finestra/trait.AppDelegate.html#method.did_launch)) and to [`configure`](https://docs.rs/finestra/latest/finestra/struct.WindowConfiguration.html) and [`populate`](https://docs.rs/finestra/latest/finestra/trait.AppDelegate.html#method.make_content_view) the window, you must provide an [`AppDelegate`](https://docs.rs/finestra/latest/finestra/trait.AppDelegate.html) implementation.

```rs
struct MyApplication;

impl AppDelegate for MyApplication {
    fn did_launch(&mut self, _: &mut ()) {
        println!("Taking Off 🚀");
    }

    fn configure_main_window(&mut self, _: &mut ()) -> WindowConfiguration {
        WindowConfiguration::new()
            .with_title("Exciting Window Title 🤩")
    }

    fn will_show_window(&mut self, _: Window, _: &mut ()) {
        println!("About to show the window, be prepared! 👀");
    }

    fn make_content_view(&mut self, _: &mut (), _: Window) -> impl View<Self> {
        Label::new("Welcome to Finestra!")
    }
}
```

### State
A powerful tool for application development is the [`State<T>`](https://docs.rs/finestra/latest/finestra/struct.State.html) object, which is a shared and subscribed object that you can use to write once, update everywhere. It is akin to WPF's [Binding](https://learn.microsoft.com/en-us/dotnet/api/system.windows.data.binding) and SwiftUI's [Binding](https://developer.apple.com/documentation/swiftui/binding). It is also deeply integrated in the library, for example by allowing you to pass a `State<String>` everywhere you pass a `String`/`&str`.

In the following example, the user can modify the title of the window by changing the contents of a [`TextField`](https://docs.rs/finestra/latest/finestra/struct.TextField.html):
```rs
struct Application;

impl AppDelegate<AppState> for Application {
    fn configure_main_window(&mut self, state: &mut AppState) -> WindowConfiguration {
        state.title.set("My Application");

        WindowConfiguration::new()
            .with_title(state.title.clone())
    }

    fn make_content_view(&mut self, state: &mut AppState, _: Window) -> impl finestra::View<Self, AppState>  {
        Stack::horizontal()
            .with(Label::new("Choose a title:"))
            .with(TextField::new(state.title.clone()))
    }
}

#[derive(Debug, Default)]
struct AppState {
    title: TextValue,
}

fn main() {
    App::with_state(Application, AppState::default())
        .run();
}
```

[Click here](finestra/examples/text_field_as_window_title.rs) for the full example.

### Stacking
To place multiple items in the same row or column, use the [`Stack`](https://docs.rs/finestra/latest/finestra/struct.Stack.html) view. This can be horizontal (row-like) or vertical (column-like).

```rs
fn make_content_view(&mut self, _: &mut (), _: Window) -> impl finestra::View<Self, ()> {
    Stack::horizontal()
        .with(Label::new("Hello, world!"))
        .with(Button::new("Click Me"))
        .with(Button::new("Goodbye, world!"))
}
```

To see how these can be combined to create a powerful interface, see the [Calculator App](./finestra/examples/calculator.rs) example.

### Dialogs
When a specific event occurs that requires attention from the user, you can use [dialog boxes](https://docs.rs/finestra/latest/finestra/struct.Window.html#method.create_dialog).

```rs
use rand::seq::SliceRandom;

const FRUITS: &[&str] = &[
    "Apple", "Banana",
    "Strawberry", "Orange",
    "Kiwi", "Pear",
    "Berries", "Lemon",
    // "Tomato",
];

Button::new("Random Fruit")
    .with_on_click(|_, window| {
        window.create_dialog(FRUITS.choose(&mut rand::thread_rng()))
                .title("Fruit")
                .kind(DialogKind::Informational)
                .show();
    })
```

For further information, consult the documentation of [DialogBuilder](https://docs.rs/finestra/latest/finestra/struct.DialogBuilder.html) and [`DialogKind`](https://docs.rs/finestra/latest/finestra/enum.DialogKind.html).

### Colors
To use colors that are consistent with every platform Finestra supports, you can use the [`SystemColor`](https://docs.rs/finestra/latest/finestra/enum.SystemColor.html) enumeration:
```rs
Label::new("BlueExampleSoftware")
    .with_color(Color::system(SystemColor::Blue))
```

These will ensure that you always use the correct color, harmonizing with the system applications.

If you need to use a specific color, you can of use the [`Color::rgb()`](https://docs.rs/finestra/latest/finestra/struct.Color.html#method.rgb) and [`Color::rgba()`](https://docs.rs/finestra/latest/finestra/struct.Color.html#method.rgba) functions:

```rs
Label::new("Maroon Balloon 🎈")
    .with_color(Color::rgb(155, 68, 68))
```

You can naturally use the [`State<Color>`](https://docs.rs/finestra/latest/finestra/struct.State.html) pattern for changing colors dynamically. See the [Disco Button](./finestra/examples/disco_button.rs) example to see how it's implemented.

### Component Overview
The following components are supported by Finestra at the moment:
- [`Button`](https://docs.rs/finestra/latest/finestra/struct.Button.html) can be used to invoke a specific action.
- [`ImageView`](https://docs.rs/finestra/latest/finestra/struct.ImageView.html) can display images.
- [`Label`](https://docs.rs/finestra/latest/finestra/struct.Label.html) contains a single line of text.
- [`Stack`](https://docs.rs/finestra/latest/finestra/struct.Stack.html) places items  horizontally or vertically.
- [`TextBlock`](https://docs.rs/finestra/latest/finestra/struct.TextBlock.html) can contain multiple lines of text, and allows for specific alignment.
- [`TextField`](https://docs.rs/finestra/latest/finestra/struct.TextField.html) can be used to request a specific string from the user.

## Rationale
Operating Systems often specify their own design language, e.g. [Apple's Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/) and Microsoft's [Windows 11 Design Principles](https://learn.microsoft.com/en-us/windows/apps/design/signature-experiences/design-principles). These guidelines are provided to let users experience a consistent and familiar user interface, and honoring them is almost always appreciated by the users of your applications, just like Arc for Windows [was praised on X/Twitter](https://x.com/zacbowden/status/1752720555302666273).

<sup>_TODO: Update with blogpost_</sup>

## Further Reading
* [Documentation](https://docs.rs/finestra/latest/finestra)
* [Examples](./finestra/examples/)
* [Roadmap](./ROADMAP.md)
* [License](./COPYING)

## Copyright
<sub>Copyright (C) 2024 Tristan Gerritsen</sub>

<sub>Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at <http://www.apache.org/licenses/LICENSE-2.0>.</sub>

<sub>Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.</sub>

<sub>Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.</sub>
