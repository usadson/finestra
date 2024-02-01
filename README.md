# Finestra
[![CI](https://github.com/usadson/finestra/actions/workflows/ci.yml/badge.svg)](https://github.com/usadson/finestra/actions/workflows/ci.yml) ![Crates.io Version](https://img.shields.io/crates/v/finestra) ![GitHub License](https://img.shields.io/github/license/usadson/finestra)

Proof of Concept for a cross-platform UI library. Written in Rust.

## Example
```rs
use finestra::*;

#[derive(Default)]
struct Application;

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
    App::with_state(Application::default(), AppState::default())
        .run();
}
```

## Further Reading
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
