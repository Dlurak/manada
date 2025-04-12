mod calculation;
mod parse;

use itertools::Itertools;
use parse::parse;

const START_UNIT: &str = "m";
const END_UNIT: &str = "dm";

fn main() {
    let Some((graph, hashmap)) = parse(include_str!("../test")) else {
        std::process::exit(1);
    };

    let value = 5.0;
    let start = *hashmap.get(START_UNIT).unwrap();
    let dest = *hashmap.get(END_UNIT).unwrap();

    let path_result = petgraph::algo::astar(&graph, start, |n| n == dest, |_| 1, |_| 1);
    if let Some((_, nodes)) = path_result {
        let converted = nodes
            .into_iter()
            .tuple_windows()
            .filter_map(|(n1, n2)| {
                let edge = graph.find_edge(n1, n2)?;
                graph.edge_weight(edge)
            })
            .fold(Some(value), |converted, v| {
                converted.and_then(|x| v.evaluate(x))
            });

        if let Some(converted) = converted {
            println!("{value}{START_UNIT} -> {converted}{END_UNIT}");
        }
    }
}
