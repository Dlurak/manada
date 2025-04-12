// km -> m: x / 1.000

use petgraph::stable_graph::NodeIndex;
use petgraph::{Directed, Graph};
use std::collections::HashMap;

use crate::calculation::parser::Parser;
use crate::calculation::token::token_list;
use crate::calculation::Value;

pub struct Parsed<'a> {
    pub graph: Graph<&'a str, Value>,
    pub nodes: HashMap<&'a str, NodeIndex>,
}

pub fn parse(unparsed: &str) -> Option<Parsed> {
    let mut graph: Graph<&str, Value, Directed> = Graph::new();
    let mut nodes = HashMap::new();

    for line in unparsed.lines() {
        let (origin, rest) = line.split_once(" -> ")?;
        let (dest, conv) = rest.split_once(": ")?;

        let origin = nodes
            .entry(origin)
            .or_insert_with(|| graph.add_node(origin));
        let origin = *origin;
        let dest = nodes.entry(dest).or_insert_with(|| graph.add_node(dest));

        let tokens = token_list(conv).ok()?;
        let mut parser = Parser::new(tokens);
        let value = parser.parse_expression().ok()?;

        graph.add_edge(origin, *dest, value);
    }

    Some(Parsed { graph, nodes })
}
