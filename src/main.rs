mod calculation;
mod cli;
mod macros;
mod parse;

use clap::Parser;
use itertools::Itertools;
use parse::{parse, Parsed};
use std::{env, fs::read_to_string, path::PathBuf};

fn main() {
    let cli::Cli {
        unit_set,
        value,
        destination: end_unit,
    } = cli::Cli::parse();
    let start_unit = value.unit;
    let value = value.value;

    let config_dir_file = dirs::config_dir().map(|dir| dir.join("manada").join(&unit_set));
    let etc_file = env::var("MANADA_CONFIG")
        .map_or(PathBuf::from("/etc/manada"), PathBuf::from)
        .join(&unit_set);

    let file_path = match (config_dir_file, etc_file) {
        (Some(file), _) if file.exists() => file,
        (_, file) if file.exists() => file,
        (Some(f1), f2) => exit!(1, "Neither {} nor {} exist", f1.display(), f2.display()),
        (None, file) => exit!(1, "{} doesn't exist", file.display()),
    };

    let content = match read_to_string(&file_path) {
        Ok(str) => str,
        Err(e) => exit!(1, "Can't read file {} ({})", file_path.display(), e.kind()),
    };

    // TODO: Show an real error message with an explaination (maybe even location?)
    let Some(Parsed { graph, nodes }) = parse(&content) else {
        exit!(1, "Can't parse the conversion file {}", file_path.display());
    };

    let Some(&start) = nodes.get(start_unit.as_str()) else {
        exit!(1, "There is no {start_unit} in {unit_set}");
    };

    // TODO: a* is a bit too much, it works but is way more intensive then needed
    let shortest_path =
        petgraph::algo::astar(&graph, start, |n| graph[n] == end_unit, |_| 1, |_| 1);
    let converted = match shortest_path {
        Some((_, nodes)) => nodes
            .into_iter()
            .tuple_windows()
            .filter_map(|(n1, n2)| {
                let edge = graph.find_edge(n1, n2)?;
                graph.edge_weight(edge)
            })
            .try_fold(value, |converted, calc| calc.evaluate(converted)),
        None => exit!(1, "There is no {end_unit} in {unit_set}"),
    };
    match converted {
        Some(conv) => {
            println!("{conv}{end_unit}");
        }
        // TODO: Show the entire calculation => add a substitute method for the calculations
        None => {
            exit!(1, "The Calculation failed");
        }
    }
}
