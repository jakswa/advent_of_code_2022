use advent_of_code_2022::input;
use std::collections::HashSet;

fn main() {
    let inp = input("9");
    println!("part1: {}", run(&inp));
    println!("part2: {}", run2(&inp));
}

fn run2(s: &str) -> usize {
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut rope: [(i64, i64); 10] = [(0, 0); 10];
    visited.insert(rope[0]);

    s.lines().for_each(|line| {
        let mut parts = line.split(" ");
        let dir = parts.next().unwrap();
        let dist = parts.next().unwrap().parse::<i64>().unwrap();
        (0..dist).for_each(|_| {
            let mut head = rope[9];
            match dir {
                "R" => head.1 += 1,
                "U" => head.0 -= 1,
                "D" => head.0 += 1,
                "L" => head.1 -= 1,
                _ => panic!("what char?"),
            }
            rope[9] = head;
            if !shrink(&mut rope) {
                visited.insert(rope[0]);
            }
        });
    });

    visited.len()
}

fn shrink(r: &mut [(i64, i64); 10]) -> bool {
    let prev_tail = r[0].clone();
    (0..9).for_each(|i| {
        let head = r[9 - i];
        let mut tail = r[9 - i - 1];
        if (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1 {
            return;
        }

        if head.0 < tail.0 {
            tail.0 -= 1;
        } else if head.0 > tail.0 {
            tail.0 += 1;
        }
        if head.1 < tail.1 {
            tail.1 -= 1;
        } else if head.1 > tail.1 {
            tail.1 += 1;
        }
        r[9 - i - 1] = tail;
    });
    prev_tail == r[0]
}

fn run(s: &str) -> usize {
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut tail: (i64, i64) = (0, 0);
    let mut head: (i64, i64) = (0, 0);
    visited.insert(tail);

    s.lines().for_each(|line| {
        let mut parts = line.split(" ");
        let dir = parts.next().unwrap();
        let dist = parts.next().unwrap().parse::<i64>().unwrap();
        (0..dist).for_each(|_| {
            let prev_head = head.clone();
            match dir {
                "R" => head.1 += 1,
                "U" => head.0 -= 1,
                "D" => head.0 += 1,
                "L" => head.1 -= 1,
                _ => panic!("what char?"),
            }
            if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                tail = prev_head;
                visited.insert(tail);
            }
        });
    });
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_part2_works() {
        assert_eq!(
            run2(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            ),
            36
        );
    }

    #[test]
    fn day9_part1_works() {
        assert_eq!(
            run("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"),
            13
        );
    }
}
