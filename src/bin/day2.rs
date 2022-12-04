use advent_of_code_2022::input;

fn main() {
    let inp = run(input("2"));
    println!("part1: {}", inp.0);
    println!("part2: {}", inp.1);
}
fn run(inp: String) -> (i64, i64) {
    let mut res = (0, 0);
    let mut my_score = 0;
    inp.lines().for_each(|line| {
        let mut chars = line.chars();
        let l = chars.next().unwrap() as u8 - ('A' as u8) + 1;
        chars.next().unwrap();
        let r = chars.next().unwrap() as u8 - ('W' as u8);
        let ll = l as i64;
        let rr = r as i64;
        if ll == rr {
            my_score += rr + 3;
        } else if rr - ll == -1 || rr - ll == 2 {
            my_score += rr;
        } else {
            my_score += rr + 6;
        }
    });
    res.0 = my_score;

    let mut my_score: i64 = 0;
    inp.lines().for_each(|line| {
        let mut chars = line.chars();
        let l = chars.next().unwrap() as u8 - ('A' as u8) + 1;
        chars.next().unwrap();
        let r = chars.next().unwrap();
        match r {
            'Z' => match l {
                1 => my_score += 8,
                2 => my_score += 9,
                3 => my_score += 7,
                _ => panic!("X-l is fucked"),
            },
            'X' => match l {
                1 => my_score += 3,
                2 => my_score += 1,
                3 => my_score += 2,
                _ => panic!("Y-l is fucked"),
            },
            'Y' => my_score += l as i64 + 3,
            _ => panic!("unexpect char {}", r),
        };
    });
    res.1 = my_score;
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_works() {
        assert_eq!(run("A Y\nB X\nC Z".to_string()), (15, 12));
    }
}
