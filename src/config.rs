// Parse the CLI args

pub struct Config {
    pub day: u8,
    pub part: Option<u8>,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // First item is name of module

        let day = match args.next() {
            Some(arg) => parse_day(&arg)?,
            None => return Err("not enough arguments"),
        };

        let part = match args.next() {
            Some(arg) => Some(parse_part(&arg)?),
            None => None,
        };

        Ok(Config { day, part })
    }
}

fn parse_day(day: &str) -> Result<u8, &'static str> {
    let day_err = "day must be an integer between 1 and 25";
    let day: u8 = match day.parse() {
        Ok(x) => x,
        Err(_) => return Err(day_err),
    };

    if !(1..=25).contains(&day) {
        return Err(day_err);
    }

    Ok(day)
}

fn parse_part(part: &str) -> Result<u8, &'static str> {
    let part_err = "part must be either 1 or 2";

    let part: u8 = match part.parse() {
        Ok(x) => x,
        Err(_) => return Err(part_err),
    };

    if !(1..=2).contains(&part) {
        return Err(part_err);
    }

    Ok(part)
}
