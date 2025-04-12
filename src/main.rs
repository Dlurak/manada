mod calculation;
mod macros;
mod parse;

use itertools::Itertools;
use parse::{parse, Parsed};
use std::{env, fs::read_to_string, path::PathBuf};

const UNIT_SET: &str = "distance";
const START_UNIT: &str = "m";
const END_UNIT: &str = "km";
const VALUE: f64 = 1500.0;

fn main() {
    let config_dir_file = dirs::config_dir().map(|dir| dir.join("manada").join(UNIT_SET));
    let etc_file = env::var("MANADA_CONFIG")
        .map_or(PathBuf::from("/etc/manada"), PathBuf::from)
        .join(UNIT_SET);

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

    let Some(&start) = nodes.get(START_UNIT) else {
        exit!(1, "There is no {START_UNIT} in {UNIT_SET}");
    };

    // TODO: a* is a bit too much, it works but is way more intensive then needed
    let shortest_path =
        petgraph::algo::astar(&graph, start, |n| graph[n] == END_UNIT, |_| 1, |_| 1);
    let converted = match shortest_path {
        Some((_, nodes)) => nodes
            .into_iter()
            .tuple_windows()
            .filter_map(|(n1, n2)| {
                let edge = graph.find_edge(n1, n2)?;
                graph.edge_weight(edge)
            })
            .try_fold(VALUE, |converted, calc| calc.evaluate(converted)),
        None => exit!(1, "There is no {END_UNIT} in {UNIT_SET}"),
    };
    match converted {
        Some(conv) => {
            println!("{conv}{END_UNIT}");
        }
        // TODO: Show the entire calculation => add a substitute method for the calculations
        None => {
            exit!(1, "The Calculation failed");
        }
    }
}
