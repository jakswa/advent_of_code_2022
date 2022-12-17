use advent_of_code_2022::input;
use std::collections::VecDeque;

fn main() {
    let inp = input("12");
    println!("part1: {}", run(&inp));
    //println!("part2: {}", run2(&inp));
}

fn run(s: &str) -> i32 {
    let mut grid = build_grid(s);
    let x = grid
        .iter()
        .enumerate()
        .find(|(_ind, uvec)| uvec.contains(&(E, i32::MAX)))
        .unwrap()
        .0;
    let y = grid[x].iter().enumerate().find(|i| i.1 .0 == E).unwrap().0;
    let mut q = VecDeque::new();
    q.push_back((x as i32, y as i32, 1));
    grid[x][y].1 = 0;
    while !q.is_empty() {
        if let Some(dist) = dig(&mut q, &mut grid) {
            return dist;
        }
    }
    0
}

const S: u8 = 'S' as u8;
const E: u8 = 'E' as u8;
const A: u8 = 'a' as u8;

fn dig(queue: &mut VecDeque<(i32, i32, i32)>, grid: &mut Vec<Vec<(u8, i32)>>) -> Option<i32> {
    if queue.len() == 0 {
        return None;
    }
    let curr = queue.pop_front().unwrap();
    let hd = grid[curr.0 as usize][curr.1 as usize];
    let hdh = match hd.0 {
        S => 'a' as u8,
        E => 'z' as u8,
        _ => hd.0,
    };
    let mut found_dist = None;
    [(-1, 0), (0, -1), (1, 0), (0, 1)]
        .iter()
        .for_each(|(dx, dy)| {
            let dpos = (curr.0 + dx, curr.1 + dy);
            if dpos.0 < 0
                || dpos.0 >= grid.len() as i32
                || dpos.1 < 0
                || dpos.1 >= grid[0].len() as i32
            {
                return;
            }
            let dh = grid[dpos.0 as usize][dpos.1 as usize];
            let dhh = match dh.0 {
                S => 'a' as u8,
                E => 'z' as u8,
                _ => dh.0,
            };
            if dhh + 1 < hdh || dh.1 != i32::MAX {
                return;
            }
            grid[dpos.0 as usize][dpos.1 as usize].1 = hd.1 + 1;
            queue.push_back((dpos.0, dpos.1, hd.1 + 1));
            if dh.0 == A {
                found_dist = Some(hd.1 + 1);
            }
        });
    found_dist
}

fn build_grid(s: &str) -> Vec<Vec<(u8, i32)>> {
    s.lines()
        .map(|l| {
            l.chars()
                .map(|c| (c as u8, i32::MAX))
                .collect::<Vec<(u8, i32)>>()
        })
        .collect::<Vec<Vec<(u8, i32)>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part0_works() {
        assert_eq!(run(TEST_INP), 32);
    }

    const TEST_INP: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
}
