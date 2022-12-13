use advent_of_code_2022::input;

fn main() {
    let inp = input("10");
    println!("part1: {}", run(&inp));
    //println!("part2: {}", run2(&inp));
}

fn run(s: &str) -> i64 {
    let mut x = 1;
    let mut tick = 1;
    let mut sum = 0;
    let ticks = [20, 60, 100, 140, 180, 220];

    s.lines().for_each(|line| {
        draw_pixel(tick, x);
        if line == "noop" {
            tick += 1;
            if let Some(t) = ticks.iter().find(|&i| tick - i == 0) {
                sum += x * t;
            }
        } else {
            tick += 1;
            if let Some(t) = ticks.iter().find(|&i| tick - i == 0) {
                sum += x * t;
            }
            draw_pixel(tick, x);

            tick += 1;
            x += line
                .split(" ")
                .skip(1)
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            if let Some(t) = ticks.iter().find(|&i| tick - i == 0) {
                sum += x * t;
            }
        }
    });

    sum
}

fn draw_pixel(big_tick: i64, x: i64) {
    let tick = big_tick % 40;
    if tick - x >= 0 && tick - x <= 2 {
        print!("#");
    } else {
        print!(".");
    }
    if tick % 40 == 0 {
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_part1_works() {
        assert_eq!(run(TEST_INP), 13140);
    }

    const TEST_INP: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
