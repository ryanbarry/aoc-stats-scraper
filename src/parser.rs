use crate::{AocStats, DailyStats};

pub fn parse_body(text: &str) -> AocStats {
    let mut result: AocStats = [DailyStats {
        both: 0,
        firstonly: 0,
    }; 25];
    let dom = match tl::parse(text, tl::ParserOptions::new().track_classes()) {
        Err(err) => panic!("got error parsing body: {err}"),
        Ok(dom) => dom,
    };
    let parser = dom.parser();
    let stats_both = dom
        .query_selector(".stats-both")
        .expect("failed to use query selector for .stats-both");
    let stats_both: Vec<usize> = stats_both
        .filter_map(|n| n.get(parser))
        .map(|el| el.inner_html(parser).trim_start().to_string())
        .filter_map(|t| t.parse().ok())
        .collect();
    let stats_firstonly = dom
        .query_selector(".stats-firstonly")
        .expect("failed to use query selector for .stats-firstonly");
    let stats_firstonly: Vec<usize> = stats_firstonly
        .filter_map(|n| n.get(parser))
        .map(|el| el.inner_html(parser).trim_start().to_string())
        .filter_map(|t| t.parse().ok())
        .collect();
    assert_eq!(stats_both.len(), 25, "wrong number of elements found matching both querysel");
    assert_eq!(stats_firstonly.len(), 25, "wrong number of elements found matching firstonly querysel");
    for (idx, (statboth, statfirstonly)) in stats_both.iter().rev().zip(stats_firstonly.iter().rev()).enumerate() {
        result[idx].both = *statboth;
        result[idx].firstonly = *statfirstonly;
    }
    result
}

#[test]
fn test_parse_body() {
    let expected: AocStats = [
        DailyStats {
            both: 196525,
            firstonly: 14001,
        },
        DailyStats {
            both: 137350,
            firstonly: 34012,
        },
        DailyStats {
            both: 122690,
            firstonly: 13028,
        },
        DailyStats {
            both: 95806,
            firstonly: 8211,
        },
        DailyStats {
            both: 75905,
            firstonly: 10097,
        },
        DailyStats {
            both: 51098,
            firstonly: 20308,
        },
        DailyStats {
            both: 42453,
            firstonly: 3166,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
        DailyStats {
            both: 0,
            firstonly: 0,
        },
    ];
    let example_body = include_str!("stats_body.in");
    let stats = parse_body(example_body);
    assert_eq!(
        stats.len(),
        expected.len(),
        "got a different number of DailyStats than expected"
    );
    for i in 0..stats.len() {
        print!("checking day {i}: ");
        assert_eq!(
            stats[i], expected[i],
            "results on day {i} differ (left->got, right->expected)"
        );
        println!("good");
    }
}
