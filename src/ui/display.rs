use owo_colors::OwoColorize;

pub struct Display;

impl Display {
    pub fn show_success(message: &str) {
        println!("{} {}", "✓".green(), message);
    }

    pub fn show_error(message: &str) {
        eprintln!("{} {}", "✗".red(), message);
    }

    pub fn show_info(message: &str) {
        println!("{} {}", "ℹ".blue(), message);
    }
}
