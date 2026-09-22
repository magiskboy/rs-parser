use std::{any::Any, fs::File, io::Read, str::FromStr};

use crate::error::AppError;
use rs_parser::json::lexer::Lexer;
use structopt::StructOpt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum InputFormat {
    JSON,
    AUTO,
}

impl FromStr for InputFormat {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(InputFormat::JSON),
            "auto" => Ok(InputFormat::AUTO),
            _ => Err(AppError::InvalidFormat),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "rs-parser-cli", about = "CLI for rs-parser")]
struct Opts {
    #[structopt(short, long, default_value = "-")]
    input: String,

    #[structopt(short, long, default_value = "auto")]
    format: InputFormat,
}

impl Opts {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.input != String::from("-") && self.format == InputFormat::AUTO {
            return Err(AppError::InvalidFormat);
        }
        Ok(())
    }
}

fn load_input(input: &String) -> Result<Box<dyn Read>, AppError> {
    match input.as_str() {
        "-" => Ok(Box::new(std::io::stdin())),
        _ => Ok(Box::new(
            File::open(&input).map_err(|_| AppError::InvalidInput)?,
        )),
    }
}

fn process(mut reader: Box<dyn Read>, opts: &Opts) -> Result<(), AppError> {
    let mut source = String::new();
    reader
        .read_to_string(&mut source)
        .map_err(|_| AppError::InvalidInput)?;

    println!("{}", source);

    let mut lexer = Lexer::new();
    let tokens = lexer.parse(&source).map_err(|e| {
        eprintln!("{:?}", e);
        AppError::InvalidInput
    })?;
    for token in tokens {
        Lexer::print_token(token, source.as_str());
    }
    Ok(())
}

pub fn execute() -> Result<(), AppError> {
    let opts = Opts::from_args();

    opts.validate()?;

    let reader = load_input(&opts.input)?;
    process(reader, &opts)?;

    Ok(())
}
