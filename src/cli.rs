use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub unit_set: String,
    #[arg(value_parser = value_parser)]
    pub value: Value,
    pub destination: String,
}

#[derive(Clone)]
pub struct Value {
    pub value: f64,
    pub unit: String,
}

fn value_parser(s: &str) -> Result<Value, String> {
    let mut unit = String::new();
    for char in s.chars().rev() {
        if char.is_ascii_digit() {
            break;
        }
        unit = char.to_string() + &unit;
    }

    let num_str = &s[..s.len() - unit.len()];
    let num = num_str.parse();
    let num = num.map_err(|_| format!("{num_str} isn't a number"))?;

    Ok(Value { unit, value: num })
}
