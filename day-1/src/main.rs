use std::{fs, vec};

fn main() {
    let content =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let (mut list1, mut list2) =
        content
            .lines()
            .filter(|s| !s.is_empty())
            .fold((vec![], vec![]), |mut acc, str| {
                let mut nums = str.split("   ");
                acc.0.push(nums.next().unwrap().parse::<i32>().unwrap());
                acc.1.push(nums.next().unwrap().parse::<i32>().unwrap());
                acc
            });

    list1.sort();
    list2.sort();

    let total_distance = list1
        .iter()
        .zip(list2.iter())
        .fold(0, |acc, (a, b)| acc + (a - b).abs());

    println!("{}", total_distance);

    let similarity = list1.iter().fold(0, |acc, a| {
        acc + (*a * list2.iter().filter(|b| *a == **b).count() as i32)
    });

    println!("{}", similarity);
}
