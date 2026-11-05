use std::io::{self, IsTerminal};

pub fn colors_enabled() -> bool {
    if std::env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty()) {
        return false;
    }
    if std::env::var("TERM").is_ok_and(|term| term == "dumb") {
        return false;
    }
    io::stdout().is_terminal()
}

fn paint(code: &str, text: &str) -> String {
    if colors_enabled() {
        format!("\x1b[{code}m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}

pub fn red(text: &str) -> String {
    paint("31", text)
}

pub fn green(text: &str) -> String {
    paint("32", text)
}

pub fn dim(text: &str) -> String {
    paint("2", text)
}
