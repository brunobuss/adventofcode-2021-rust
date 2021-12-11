use std::env;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
pub enum ChallengeParts {
    PartOne,
    PartTwo,
    Both,
}

impl FromStr for ChallengeParts {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<ChallengeParts, Self::Err> {
        match input.to_lowercase().as_str() {
            "one" | "1" => Ok(ChallengeParts::PartOne),
            "two" | "2" => Ok(ChallengeParts::PartTwo),
            "both" => Ok(ChallengeParts::Both),
            _ => Err("Invalid challenge part. Accepted values: one, 1, two, 2, both."),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub day: u8,
    pub part: ChallengeParts,
    pub input_file: PathBuf,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // Discard program name.
        args.next();

        let day: u8 = match args.next() {
            Some(d) => match d.parse() {
                Ok(v) => v,
                Err(_) => return Err("Unable to parse day parameter."),
            },
            None => return Err("Missing day parameter."),
        };

        let part: ChallengeParts = match args.next() {
            Some(p) => match ChallengeParts::from_str(&p) {
                Ok(v) => v,
                Err(e) => return Err(e),
            },
            None => return Err("Missing part parameter."),
        };

        let input_file: PathBuf = match args.next() {
            Some(p) => PathBuf::from(p),
            None => PathBuf::from("input.txt"),
        };

        Ok(Config {
            day,
            part,
            input_file,
        })
    }
}
