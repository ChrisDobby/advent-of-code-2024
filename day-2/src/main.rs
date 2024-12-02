use std::fs;

fn is_report_safe(report: &str, bad_level_allowed: bool) -> bool {
    let mut direction = "unknown";
    let mut prev_level = -1;
    let mut number_of_bad_levels = 0;
    for level in report.split_whitespace() {
        let level = level.parse::<i32>().unwrap();
        if prev_level == -1 {
            prev_level = level;
            continue;
        }

        if direction == "unknown" {
            direction = if prev_level < level { "up" } else { "down" };
        }

        if direction == "up" && prev_level > level {
            number_of_bad_levels += 1;
        } else if direction == "down" && prev_level < level {
            number_of_bad_levels += 1;
        }

        if prev_level == level || (prev_level - level).abs() > 3 {
            number_of_bad_levels += 1;
        }

        if number_of_bad_levels > 1 || (!bad_level_allowed && number_of_bad_levels > 0) {
            return false;
        }

        prev_level = level;
    }
    true
}

fn main() {
    let content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let count_of_safe_reports = content
        .lines()
        .filter(|s| !s.is_empty())
        .filter(|report| is_report_safe(report, false))
        .count();

    println!("Number of safe reports: {}", count_of_safe_reports);

    let count_of_safe_reports_with_dampener = content
        .lines()
        .filter(|s| !s.is_empty())
        .filter(|report| is_report_safe(report, true))
        .count();

    println!(
        "Number of safe reports: {}",
        count_of_safe_reports_with_dampener
    );
}
