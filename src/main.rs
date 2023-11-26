use std::io;

use clap::Parser;
use console::Term;
use dialoguer::{Input, Select};
use dialoguer::theme::ColorfulTheme;

use crate::args::Args;
use crate::constants::VERSION;
use crate::parser::parse_timestamp_from_str;
use crate::re::{DATE_TIME_REGEX, OFFSET_REGEX, TIME_REGEX};
use crate::term::TermExt;
use crate::utils::{generate_date_time_formats, generate_discord_timestamp};

mod term;
mod args;
mod parser;
mod re;
mod constants;
mod utils;


fn main() -> io::Result<()> {
    let args = Args::parse();

    args.date.map(|date| {
        default_mode(date, args.style.unwrap_or("".to_string()))
    }).unwrap_or_else(|| {
        interactive_mode()
    })?;

    Ok(())
}


fn default_mode(date: String, format: String) -> io::Result<()> {
    let term = Term::stdout();

    let timestamp = match parse_timestamp_from_str(date.as_str()) {
        Some(v) => v,
        None => {
            term.print_error("Invalid date format!")?;
            return Ok(());
        }
    };

    let generated = match generate_discord_timestamp(timestamp, format.as_str()) {
        Some(v) => v,
        None => {
            term.print_error("Failed to generate timestamp!")?;
            return Ok(());
        }
    };

    term.print_success("Generated timestamp", generated.as_str())?;

    Ok(())
}

fn interactive_mode() -> io::Result<()> {
    let term = Term::stdout();

    term.set_title("Discord Timestamp Generator");
    term.write_line(&format!("Discord Timestamp Generator v{} by Mantelis\n", VERSION))?;

    let selections = &[
        "Generate Discord Timestamp",
        "Exit",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&selections[..])
        .default(0)
        .interact()
        .unwrap();

    if selection == 1 {
        term.write_line("Goodbye!")?;
        return Ok(());
    }

    loop {
        let date: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter the date")
            .validate_with(|input: &String| -> Result<(), &str> {
                // if parse_timestamp_from_str(input).is_none() {
                //     return Err("Invalid date format!");
                // }

                if !(DATE_TIME_REGEX.is_match(input.as_str()) ||
                    TIME_REGEX.is_match(input.as_str()) ||
                    OFFSET_REGEX.is_match(input.as_str())) {
                    return Err("Invalid date format!");
                }

                Ok(())
            })
            .interact_text()
            .unwrap();

        let timestamp = match parse_timestamp_from_str(date.as_str()) {
            Some(v) => v,
            None => {
                term.print_error("Invalid date format!")?;
                return Ok(());
            }
        };

        let formats = generate_date_time_formats(timestamp, None);


        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select output style")
            .items(&formats.keys().map(|k| k).collect::<Vec<_>>()[..])
            .default(0)
            .interact()
            .unwrap();

        let format = formats.get_key_value(formats.keys().collect::<Vec<_>>()[selection]).unwrap().1;
        let generated = match generate_discord_timestamp(timestamp, format) {
            Some(v) => v,
            None => {
                term.print_error("Failed to generate timestamp!")?;
                return Ok(());
            }
        };

        term.print_success("Generated timestamp", generated.as_str())?;

        let selections = [
            "Yes",
            "No",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Generate another timestamp?")
            .items(&selections[..])
            .default(0)
            .interact()
            .unwrap();

        if selection != 0 {
            break;
        }

        term.write_line("")?;
    }

    Ok(())
}