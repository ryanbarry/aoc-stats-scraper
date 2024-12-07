fn main() {
    println!("Hello, world!");

    let res = get_stats(2024);
}

#[derive(Copy, Clone)]
struct DailyStats {
    both: usize,
    firstonly: usize,
}

type AocStats = [DailyStats; 25];

fn get_stats(year: i32) -> AocStats {
    let body = fetch_stats(year);
    parse_body(&body)
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

fn parse_body(text: &str) -> AocStats {
    let mut result: AocStats = [DailyStats {
        both: 0,
        firstonly: 0,
    }; 25];
    // for i in 0..result.len() {
    //     result[i].both = 0;
    //     result[i].firstonly = 0;
    // }
    result
}
