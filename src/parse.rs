use crate::calculation::{
    Value,
    parser::{CalculationParseError, Parser},
    token::{TokenizeError, token_list},
};
use ansi_term::{Colour, Style};
use derive_more::{Display, From};
use petgraph::{Directed, Graph, stable_graph::NodeIndex};
use std::{collections::HashMap, path::PathBuf};

pub struct Parsed<'a> {
    pub graph: Graph<&'a str, Value>,
    pub nodes: HashMap<&'a str, NodeIndex>,
}

#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub error_kind: ParseErrorKind,
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

pub fn parse(unparsed: &str) -> Result<Parsed, ParseError> {
    let mut graph: Graph<&str, Value, Directed> = Graph::new();
    let mut nodes = HashMap::new();

    for (i, line) in unparsed.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with('#') {
            continue;
        }

        let (origin, rest) = line.split_once(" -> ").ok_or(ParseError {
            line: i,
            error_kind: ParseErrorKind::MissingArrow,
        })?;
        let (dest, conv) = rest.split_once(": ").ok_or(ParseError {
            line: i,
            error_kind: ParseErrorKind::MissingColon,
        })?;

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
