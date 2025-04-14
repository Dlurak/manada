use crate::calculation::{
    parser::{CalculationParseError, Parser},
    token::{token_list, TokenizeError},
    Value,
};
use ansi_term::{Colour, Style};
use derive_more::{Display, From};
use itertools::Itertools;
use petgraph::{algo::astar, stable_graph::NodeIndex, Directed, Graph};
use std::{collections::HashMap, path::PathBuf};

pub struct Parsed<'a> {
    graph: Graph<&'a str, Value>,
    nodes: HashMap<&'a str, NodeIndex>,
}

impl Parsed<'_> {
    pub fn try_new(unparsed: &str) -> Result<Parsed<'_>, ParseError> {
        let mut graph: Graph<&str, Value, Directed> = Graph::new();
        let mut nodes = HashMap::new();

        let lines = unparsed.lines().enumerate().filter_map(|(i, line)| {
            let line = line.trim();
            let line = line
                .find('#')
                .map_or(line, |comment_start| &line[0..comment_start]);

            (!line.is_empty()).then_some((i, line))
        });
        for (i, line) in lines {
            let (origin, rest) = line.split_once(" -> ").ok_or(ParseError {
                line: i,
                error_kind: ParseErrorKind::MissingArrow,
            })?;
            let (dest, conv) = rest.split_once(": ").ok_or(ParseError {
                line: i,
                error_kind: ParseErrorKind::MissingColon,
            })?;
            let origin = origin.trim();
            let dest = dest.trim();
            let conv = conv.trim();

            let origin = *nodes
                .entry(origin)
                .or_insert_with(|| graph.add_node(origin));
            let dest = nodes.entry(dest).or_insert_with(|| graph.add_node(dest));

            let tokens = token_list(conv).map_err(|e| ParseError {
                line: i,
                error_kind: e.into(),
            })?;
            let mut parser = Parser::new(tokens);
            let value = parser.parse_expression().map_err(|e| ParseError {
                line: i,
                error_kind: e.into(),
            })?;

            graph.add_edge(origin, *dest, value);
        }

        Ok(Parsed { graph, nodes })
    }

    pub fn get_node_by_name(&self, name: &str) -> Option<&NodeIndex> {
        self.nodes.get(name)
    }

    pub fn convert<'a>(
        &self,
        start: NodeIndex,
        end: &'a str,
        x: f64,
    ) -> Result<f64, ConversionError<'a>> {
        // TODO: a* is a bit too much, it works but is way more intensive then needed
        let (_, nodes) = astar(&self.graph, start, |n| self.graph[n] == end, |_| 1, |_| 1)
            .ok_or_else(|| {
                if self.nodes.contains_key(end) {
                    ConversionError::NoPathFound
                } else {
                    ConversionError::EndDoesntExist { end }
                }
            })?;
        nodes
            .into_iter()
            .tuple_windows()
            .filter_map(|(n1, n2)| {
                let edge = self.graph.find_edge(n1, n2)?;
                self.graph.edge_weight(edge)
            })
            .try_fold(x, |converted, calc| calc.evaluate(converted))
            .ok_or(ConversionError::CalculationFailed)
    }
}

pub enum ConversionError<'a> {
    NoPathFound,
    CalculationFailed,
    EndDoesntExist { end: &'a str },
}

#[derive(Debug)]
pub struct ParseError {
    line: usize,
    error_kind: ParseErrorKind,
}

#[derive(Debug, From, Display)]
pub enum ParseErrorKind {
    #[display("Missing arrow \" -> \" between units")]
    MissingArrow,
    #[display("Missing arrow \": \" between second unit and conversion")]
    MissingColon,
    #[from]
    #[display("{_0}")]
    Tokenizer(TokenizeError),
    #[from]
    #[display("{_0}")]
    CalculationParseError(CalculationParseError),
}

impl ParseError {
    pub fn print(self, file_path: PathBuf, file_content: &str) {
        let Self {
            ref error_kind,
            line,
        } = self;
        eprintln!("{}{error_kind}", Colour::Red.paint("error: "));
        let line_number_width = (line as f64).log10() as usize;
        eprintln!(
            "{:line_number_width$} {} {}:{line}",
            "",
            Colour::Blue.paint("-->"),
            file_path.display(),
        );
        if let Some(line_content) = file_content.lines().nth(line) {
            eprintln!(
                "{} {}",
                Colour::Blue.paint(format!("{line} |")),
                line_content
            );
        }

        if let Some(note) = self.note() {
            eprintln!(
                "{:line_number_width$} {} {} {note}",
                " ",
                Colour::Blue.paint("="),
                Style::new().bold().paint("note:"),
            );
        }
    }

    fn note(self) -> Option<String> {
        if let ParseErrorKind::Tokenizer(TokenizeError::InvalidChar {
            serialized_string,
            char,
            position,
        }) = self.error_kind
        {
            let before_faulty = &serialized_string[..position];
            let after_faulty = &serialized_string[position + 1..];
            let serialized = Style::new().italic().paint(format!(
                "{before_faulty}{}{after_faulty}",
                Style::new().underline().paint(char.to_string())
            ));

            Some(format!(
                "Unexpected charachter {char} in serialized calculation \"{serialized}\""
            ))
        } else {
            None
        }
    }
}
