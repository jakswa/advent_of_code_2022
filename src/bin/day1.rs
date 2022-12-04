use advent_of_code_2022::input;

fn main() {
    let inp = input("1");
    let mut elves: Vec<u64> = vec![0];
    inp.lines().for_each(|line| match line.parse::<u64>() {
        Err(_) => elves.push(0),
        Ok(calories) => {
            let len = elves.len();
            elves[len - 1] += calories;
        }
    });
    elves.sort();
    let last_elf = elves.pop().unwrap();
    println!("part1: {}", last_elf);
    println!(
        "part2: {}",
        last_elf + elves.pop().unwrap() + elves.pop().unwrap()
    );
}
