mod calculation;
mod cli;
mod macros;
mod parse;

use clap::Parser;
use derive_more::Display;
use parse::{ConversionError, Parsed};
use std::{
    env,
    fs::read_to_string,
    path::{Path, PathBuf},
};

fn file_path<P: AsRef<Path>>(file_name: P) -> Result<PathBuf, NoFilePathError> {
    let config_dir_file = dirs::config_dir().map(|dir| dir.join("manada").join(&file_name));
    let etc_file = env::var("MANADA_CONFIG")
        .map_or(PathBuf::from("/etc/manada"), PathBuf::from)
        .join(&file_name);

    match (config_dir_file, etc_file) {
        (Some(file), _) if file.exists() => Ok(file),
        (_, file) if file.exists() => Ok(file),
        (Some(home_file), etc_file) => Err(NoFilePathError::NeitherHomeNorEtc(home_file, etc_file)),
        // Very unlikly but it can happen, if there is no home config dir
        (None, etc_file) => Err(NoFilePathError::NoEtc(etc_file)),
    }
}

#[derive(Display)]
enum NoFilePathError {
    #[display("Neither {} nor {} exist", _0.display(), _1.display())]
    NeitherHomeNorEtc(PathBuf, PathBuf),
    #[display("{} doesn't exist", _0.display())]
    NoEtc(PathBuf),
}

fn main() {
    let cli::Cli {
        unit_set,
        value: cli::Value {
            unit: start_unit,
            value,
        },
        destination: end_unit,
    } = cli::Cli::parse();

    let file_path = file_path(&unit_set).unwrap_or_else(|e| exit!(1, "{e}"));
    let file_content = read_to_string(&file_path).unwrap_or_else(|err| {
        exit!(
            1,
            "Can't read file {} ({})",
            file_path.display(),
            err.kind()
        );
    });

    let parsed = Parsed::try_new(&file_content).unwrap_or_else(|err| {
        err.print(file_path, &file_content);
        std::process::exit(1);
    });

    let Some(&start) = parsed.get_node_by_name(&start_unit) else {
        exit!(1, "There is no {start_unit} in {unit_set}");
    };

    let converted = parsed.convert(start, &end_unit, value);
    match converted {
        Ok(conv) => println!("{conv}{end_unit}"),
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
