use std::io;

use console::{style, Term};

pub trait TermExt {
    fn print_success(&self, label: &str, value: &str) -> io::Result<()>;
    fn print_error(&self, error: &str) -> io::Result<()>;
    fn print_info(&self, label: &str, value: &str) -> io::Result<()>;
    fn print_warning(&self, warning: &str) -> io::Result<()>;
}

impl TermExt for Term {
     fn print_success(&self, label: &str, value: &str) -> io::Result<()> {
        self.write_line(&format_success(label, value))
    }

    fn print_error(&self, error: &str) -> io::Result<()> {
        self.write_line(&format_error(error))
    }

    fn print_info(&self, label: &str, value: &str) -> io::Result<()> {
        self.write_line(&format_info(label, value))
    }

    fn print_warning(&self, warning: &str) -> io::Result<()> {
        self.write_line(&format_warning(warning))
    }
}

fn format_error(error: &str) -> String {
    format!("{} {}", style("✖".to_string()).for_stderr().red(), style(error).bold())
}

fn format_success(label: &str, value: &str) -> String {
    format!("{} {} · {}", style("✔".to_string()).for_stderr().green(), style(label).bold(), style(value).green())
}

fn format_info(label: &str, value: &str) -> String {
    format!("{} {} · {}", style("ℹ".to_string()).for_stderr().blue(), style(label).bold(), style(value).blue())
}

fn format_warning(warning: &str) -> String {
    format!("{} {}", style("⚠".to_string()).for_stderr().yellow(), style(warning).bold())
}