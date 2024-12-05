use std::collections::HashMap;
use std::fs;

fn main() {
    let (sort_order, lines) = read_file("./input.txt");

    let sorted: Vec<_> = lines
        .iter()
        .map(|line| (line, sort_line(line, &sort_order)))
        .map(|(line, sorted)| {
            (
                sorted.clone(),
                sorted
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .eq(line),
            )
        })
        .collect();

    let valid_sum = sorted
        .iter()
        .filter(|(_, is_valid)| *is_valid)
        .map(|(sorted, _)| sorted[sorted.len() / 2])
        .sum::<usize>();

    let invalid_sum = sorted
        .iter()
        .filter(|(_, is_valid)| !*is_valid)
        .map(|(sorted, _)| sorted[sorted.len() / 2])
        .sum::<usize>();

    println!(
        "Valid lines sum: {} Invalid lines sum: {}",
        valid_sum, invalid_sum
    );
}

fn sort_line(line: &str, sort_order: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let page_numbers = line.split(",");
    let mut sorted = page_numbers
        .map(|page_number| page_number.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    sorted.sort_by(|a, b| {
        let a_sort_order = sort_order.get(a);
        if a_sort_order == None {
            return std::cmp::Ordering::Equal;
        }
        let a_before_b = a_sort_order.unwrap().contains(b);
        if a_before_b {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    sorted
}

fn read_file(file_path: &str) -> (HashMap<usize, Vec<usize>>, Vec<String>) {
    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let sort_order = content
        .lines()
        .filter(|s| !s.is_empty() && s.contains("|"))
        .fold(
            HashMap::<usize, Vec<usize>>::new(),
            |mut sort_order, line| {
                let mut parts = line.split("|");
                let key = parts.next().unwrap().parse::<usize>().unwrap();
                if sort_order.contains_key(&key) {
                    sort_order
                        .get_mut(&key)
                        .unwrap()
                        .push(parts.next().unwrap().parse::<usize>().unwrap());
                } else {
                    sort_order.insert(key, vec![parts.next().unwrap().parse::<usize>().unwrap()]);
                }
                sort_order
            },
        );

    let lines: Vec<String> = content
        .lines()
        .filter(|s| !s.is_empty() && !s.contains("|"))
        .map(|s| s.to_string())
        .collect();

    (sort_order, lines)
}
