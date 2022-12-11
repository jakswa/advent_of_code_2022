use advent_of_code_2022::input;

fn main() {
    let inp = input("4");
    println!("part1: {}", run(&inp));
    println!("part2: {}", run2(&inp));
}

fn run(s: &str) -> usize {
    s.lines().map(|i| i.split(&['-', ',']).map(|i| i.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .filter(|i| (i[0] <= i[2] && i[1] >= i[3]) || (i[0] >= i[2] && i[1] <= i[3]))
        .count()
}

fn run2(s: &str) -> usize {
    s.lines().map(|i| i.split(&['-', ',']).map(|i| i.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .filter(|i| !(i[2] > i[1] || i[3] < i[0]))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part1_works() {
        assert_eq!(
            run("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"),
            2
        );
    }

    #[test]
    fn day3_part2_works() {
        assert_eq!(
            run2("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"),
            4
        );
    }
}
