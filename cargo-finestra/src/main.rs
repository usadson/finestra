// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

fn main() {
    println!("{} environment variable(s) set:", std::env::vars().count());
    for (name, value) in std::env::vars() {
        println!("    - {name}: \"{value}\"");
    }

    println!("{} argument(s) set:", std::env::args().count());
    for arg in std::env::args() {
        println!("    - \"{arg}\"");
    }
}
