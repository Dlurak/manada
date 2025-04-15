mod calculation;
mod cli;
mod config;
mod macros;
mod parse;

use clap::Parser;
use config::NewConfigError;
use manada::file_path;
use parse::{ConversionError, Parsed};
use std::fs::read_to_string;

fn main() {
    let cli::Cli {
        unit_set,
        value: cli::Value {
            unit: start_unit,
            value,
        },
        destination: end_unit,
    } = cli::Cli::parse();

    let conversions_file_path = file_path(&unit_set).unwrap_or_else(|e| exit!(1, "{e}"));
    let file_content = read_to_string(&conversions_file_path).unwrap_or_else(|err| {
        exit!(
            1,
            "Can't read file {} ({})",
            conversions_file_path.display(),
            err.kind()
        );
    });

    let parsed = Parsed::try_new(&file_content).unwrap_or_else(|err| {
        err.print(conversions_file_path, &file_content);
        std::process::exit(1);
    });

    let config = config::Config::try_new(&unit_set).map_or_else(
        |err| match err {
            NewConfigError::NoConfig => None,
            NewConfigError::FileRead { path, error } => {
                exit!(1, "Can't read file {} ({})", path.display(), error.kind())
            }
            NewConfigError::ParseError { path, error } => {
                exit!(1, "Can't parse {}: {}", path.display(), error)
            }
        },
        Some,
    );

    let (start_unit, end_unit) = match config {
        Some(config) => {
            let config2 = config.clone();
            let start = config2.get_full_unit(&start_unit).unwrap_or(start_unit);
            let end = config.get_full_unit(&end_unit).unwrap_or(end_unit);
            (start, end)
        }
        None => (start_unit, end_unit),
    };

    let Some(&start) = parsed.get_node_by_name(&start_unit) else {
        exit!(1, "There is no {start_unit} in {unit_set}");
    };

    let converted = parsed.convert(start, &end_unit, value);
    match converted {
        Ok(conv) => println!("{}{end_unit}", conv.normalize()),
        Err(ConversionError::EndDoesntExist { end }) => {
            exit!(1, "There is no {end} in \"{unit_set}\"")
        }
        Err(ConversionError::NoPathFound) => {
            exit!(
                1,
                "A conversion from {start_unit} to {end_unit} isn't possible."
            )
        }
        // TODO: Show the entire calculation => add a substitute method for the calculations
        Err(ConversionError::CalculationFailed) => exit!(1, "The Calculation failed"),
    }
}
