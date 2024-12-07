use crate::{AocStats, DailyStats};

pub fn parse_body(text: &str) -> AocStats {
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
    let stats = parse_body(include_str!("stats_body.in"));
    assert_eq!(
        stats.len(),
        expected.len(),
        "got a different number of DailyStats than expected"
    );
    for i in 0..stats.len() {
        print!("checking day {i}: ");
        assert_eq!(stats[i], expected[i], "results on day {i} differ (left->got, right->expected)");
        println!("good");
    }
}
