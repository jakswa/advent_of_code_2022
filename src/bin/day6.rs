use advent_of_code_2022::input;
use std::collections::HashSet;

fn main() {
    let inp = input("6");
    println!("part1: {}", run(&inp));
    println!("part2: {}", run2(&inp));
}

fn run(s: &str) -> usize {
    s.as_bytes().windows(4).enumerate().find(|(_i,w)| {
        let set = w.iter().map(|i| *i).collect::<HashSet<u8>>();
        set.len() == 4
    }).unwrap().0 + 4
}

fn run2(s: &str) -> usize {
    s.as_bytes().windows(14).enumerate().find(|(_i,w)| {
        let set = w.iter().map(|i| *i).collect::<HashSet<u8>>();
        set.len() == 14
    }).unwrap().0 + 14
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_part1_works() {
        assert_eq!(
            run("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            5
        );
        assert_eq!(
            run("nppdvjthqldpwncqszvftbrmjlhg"),
            6
        );

        assert_eq!(
            run2("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            23
        );
        assert_eq!(
            run2("nppdvjthqldpwncqszvftbrmjlhg"),
            23
        );
    }

}
