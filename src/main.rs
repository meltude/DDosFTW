mod hack_sequence;
mod logo;
mod style;

use std::io::{self, Write};

/// Reads one line from stdin. Returns None on EOF or read failure.
fn read_line() -> Option<String> {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(line),
    }
}

/// Prompts until a non-empty value is entered.
/// Returns None when the user cancels with 'q'/'quit' or input ends.
fn read_input(prompt: &str) -> Option<String> {
    loop {
        print!("{prompt}");
        io::stdout().flush().ok();

        let value = read_line()?;
        let value = value.trim();

        if value.eq_ignore_ascii_case("q") || value.eq_ignore_ascii_case("quit") {
            return None;
        }
        if !value.is_empty() {
            return Some(value.to_string());
        }

        println!("Err: This field cannot be empty.");
    }
}

fn main() {
    logo::print_logo();

    loop {
        print!(
            "\nChoose mode: [1] domain hack  [2] account hack  [3] de-anonymise [q] quit: "
        );
        io::stdout().flush().ok();

        let Some(mode) = read_line() else {
            println!("exit");
            return;
        };
        let mode = mode.trim().to_lowercase();

        if mode == "q" || mode == "quit" {
            println!("exit");
            return;
        }

        if mode == "3" {
            let Some(alias) = read_input("Username/alias to de-anonymise (or 'q' to cancel): ")
            else {
                continue;
            };
            hack_sequence::fake_deanon_sequence(&alias);
            println!(
                "\n{}",
                style::green(&format!(
                    "{alias} de-anonymisation complete (fictional)"
                ))
            );
        } else if mode == "2" {
            let Some(service) = read_input("Service name (or 'q' to cancel): ") else {
                continue;
            };
            let Some(account) =
                read_input("Username/email for the hacking account (or 'q' to cancel): ")
            else {
                continue;
            };
            hack_sequence::fake_account_demo(&service, &account);
        } else {
            let Some(target) = read_input("Write domain here (or 'q' to quit): ") else {
                println!("exit");
                return;
            };
            hack_sequence::fake_hack_sequence(&target);
            println!(
                "\n{}",
                style::green(&format!("{target} has been successfully hacked"))
            );
        }

        print!("\nYou have another target? (press Enter to continue, or 'q' to quit): ");
        io::stdout().flush().ok();

        let Some(again) = read_line() else {
            println!("exit");
            return;
        };
        let again = again.trim();
        if again.eq_ignore_ascii_case("q") || again.eq_ignore_ascii_case("quit") {
            println!("exit");
            return;
        }
    }
}
