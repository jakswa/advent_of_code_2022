use advent_of_code_2022::input;

fn main() {
    let inp = input("6");
    println!("part1: {}", run(&inp));
    //println!("part2: {}", run2(&inp));
}

fn run(s: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part0_works() {
        assert_eq!(
            run("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            0
        );
    }

}
