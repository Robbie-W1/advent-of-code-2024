// use itertools::Itertools;

use std::collections::HashMap;

fn task_one(file_path: &str) -> i32 {
    let input = std::fs::read_to_string(file_path).unwrap();

    // Parse the input
    let nums = input
        .lines()
        .map(|x| {
            let split: Vec<_> = x.split_whitespace().collect();
            (
                split[0].parse::<i32>().unwrap(),
                split[1].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    // Convert to two separate lists
    let (mut list1, mut list2): (Vec<_>, Vec<_>) = nums.iter().map(|&(x, y)| (x, y)).unzip();

    list1.sort();
    list2.sort();

    let result = list1
        .into_iter()
        .zip(list2)
        .map(|(a, b)| (a - b).abs())
        .sum();

    result
}

fn task_two(file_path: &str) -> i32 {
    let input = std::fs::read_to_string(file_path).unwrap();

    // Parse the input
    let nums = input
        .lines()
        .map(|x| {
            let split: Vec<_> = x.split_whitespace().collect();
            (
                split[0].parse::<i32>().unwrap(),
                split[1].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let (list1, list2): (Vec<_>, Vec<_>) = nums.iter().map(|&(x, y)| (x, y)).unzip();

    // let mut counts: HashMap<i32, i32> = HashMap::new();
    let mut counts = 0;
    for item in list1 {
        counts += list2.iter().filter(|&&x| x == item).count() as i32 * item;
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_one() {
        assert_eq!(task_one("test-input/day-1.txt"), 11);
    }

    #[test]
    fn test_task_two() {
        assert_eq!(task_two("test-input/day-1.txt"), 31);
    }
}

fn main() {
    let task_one = task_one("input/day-1.txt");
    println!("Task 1: {}", task_one);

    let task_two = task_two("input/day-1.txt");
    println!("Task 1: {}", task_two);
}
