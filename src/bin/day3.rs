use advent_of_code_2022::input;

fn main() {
    let inp = run(input("3"));
    println!("part1: {}", inp);
}

fn run2(inp: String) -> i64 {
    panic!("finish");
}

fn run(inp: String) -> i64 {
    let priorities = inp.lines().map(|line| {
        line.chars()
            .map(|c| match c as u8 >= 'a' as u8 {
                true => c as i64 - 'a' as i64 + 1,
                false => c as i64 - 'A' as i64 + 27,
            })
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
    println!("derp: {:?}", dupes);
    dupes.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_works() {
        assert_eq!(
            run("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
                .to_string()),
            157
        );
    }
}
