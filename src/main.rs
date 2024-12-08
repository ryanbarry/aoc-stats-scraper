use std::{fmt, io::Write};

mod parser;

fn main() {
    println!("year,day01both,day01firstonly,day02both,day02firstonly,day03both,day03firstonly,day04both,day04firstonly,day05both,day05firstonly,day06both,day06firstonly,day07both,day07firstonly,day08both,day08firstonly,day09both,day09firstonly,day10both,day10firstonly,day11both,day11firstonly,day12both,day12firstonly,day13both,day13firstonly,day14both,day14firstonly,day15both,day15firstonly,day16both,day16firstonly,day17both,day17firstonly,day18both,day18firstonly,day19both,day19firstonly,day20both,day20firstonly,day21both,day21firstonly,day22both,day22firstonly,day23both,day23firstonly,day24both,day24firstonly,day25both,day25firstonly");
    for year in [2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023, 2024] {
        let stats = get_stats(year);
        let stats_csv_line = stats.map(|el| el.to_string()).join(",");
        println!("{year},{}", stats_csv_line);
        std::io::stdout()
            .flush()
            .expect("failed to flush after writing a line");
    }
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

    eprintln!("got a response with code/status: {}", res.status().as_u16(),);

    match res.text() {
        Err(err) => {
            panic!("{err}");
        }
        Ok(text) => text,
    }
}
