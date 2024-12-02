use std::fs;

fn is_safe(report: &[i32]) -> bool {
    if report.is_empty() {
        return false;
    }
    return report
        .windows(2)
        .all(|w| w[0] < w[1] && (w[0] - w[1]).abs() <= 3)
        || report
            .windows(2)
            .all(|w| w[0] > w[1] && (w[0] - w[1]).abs() <= 3);
}

fn is_safe_dampened(report: &[i32]) -> bool {
    if report.is_empty() {
        return false;
    }
    return is_safe(report)
        || report.iter().enumerate().any(|(i, _)| {
            let mut cloned = report.to_owned();
            cloned.remove(i);
            is_safe(&cloned)
        });
}

fn main() {
    let mut safe = 0;
    let mut safe_dampened = 0;
    fs::read_to_string("./input/02")
        .unwrap()
        .trim()
        .lines()
        .for_each(|s| {
            let report: Vec<i32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
            if is_safe(&report) {
                safe += 1;
            }
            if is_safe_dampened(&report) {
                safe_dampened += 1;
            }
        });

    println!("Safe reports: {}", safe);
    println!("Safe reports with dampening: {}", safe_dampened);
}
