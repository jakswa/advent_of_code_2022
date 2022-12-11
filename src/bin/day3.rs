use advent_of_code_2022::input;
use std::collections::HashSet;

fn main() {
    let inp = input("3");
    println!("part1: {}", run(&inp));
    println!("part2: {}", run2(&inp));
}

fn run2(inp: &str) -> i64 {
    let char_groups = inp.lines().map(|i| i.chars().collect::<HashSet<char>>()).collect::<Vec<HashSet<char>>>();
    char_groups.chunks(3).map(|chunk| {
        let sect1 = chunk[0].intersection(&chunk[1]).map(|i| *i).collect::<HashSet<char>>();
        *sect1.intersection(&chunk[2]).next().unwrap()
    }).map(priority).sum()
}

fn priority(c: char) -> i64 {
    match c as u8 >= 'a' as u8 {
        true => c as i64 - 'a' as i64 + 1,
        false => c as i64 - 'A' as i64 + 27,
    }
}

fn run(inp: &str) -> i64 {
    let priorities = inp.lines().map(|line| {
        line.chars()
            .map(priority)
            .collect::<Vec<i64>>()
    });
    let dupes = priorities
        .map(|line| {
            let mut chunks = line.chunks(line.len() / 2);
            let chunk1 = chunks.next().unwrap();
            let chunk2 = chunks.next().unwrap();
            chunk1
                .iter()
                .find(|i| chunk2.iter().any(|j| &j == i))
                .unwrap()
                .clone()
        })
        .collect::<Vec<i64>>();
    dupes.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part1_works() {
        assert_eq!(
            run("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
                ),
            157
        );
    }

    #[test]
    fn day3_part2_works() {
        assert_eq!(
            run2("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
                ),
            70
        );
    }
}
