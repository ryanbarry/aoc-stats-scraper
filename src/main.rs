use std::fmt;

mod parser;

fn main() {
    println!("Hello, world!");

    let res = get_stats(2024);
    println!("{res:?}");
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct DailyStats {
    both: usize,
    firstonly: usize,
}

impl fmt::Display for DailyStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.both, self.firstonly)
    }
}

type AocStats = [DailyStats; 25];

fn get_stats(year: i32) -> AocStats {
    let body = fetch_stats(year);
    parser::parse_body(&body)
}

fn fetch_stats(year: i32) -> String {
    let res = match reqwest::blocking::get(format!("https://adventofcode.com/{year}/stats")) {
        Err(err) => {
            panic!("{err}");
        }
        Ok(res) => res,
    };

    println!(
        "got a response with code/status: {}/{}",
        res.status().as_u16(),
        res.status().as_str()
    );

    match res.text() {
        Err(err) => {
            panic!("{err}");
        }
        Ok(text) => text,
    }
}

