use advent_of_code_2022::input;
use std::collections::HashSet;

fn main() {
    let inp = input("12");
    println!("part1: {}", run(&inp));
    //println!("part2: {}", run2(&inp));
}

fn run(s: &str) -> usize {
    let grid = build_grid(s);
    let x = grid
        .iter()
        .enumerate()
        .find(|(_ind, uvec)| uvec.contains(&('S' as u8)))
        .unwrap()
        .0;
    let y = grid[x]
        .iter()
        .enumerate()
        .find(|i| *i.1 == 'S' as u8)
        .unwrap()
        .0;
    let mut hist = HashSet::new();
    hist.insert((x as i32, y as i32));
    dig(
        (x as i32, y as i32),
        grid.len() * grid[0].len(),
        &hist,
        &grid,
    )
}

fn dig(pos: (i32, i32), bench: usize, hist: &HashSet<(i32, i32)>, grid: &Vec<Vec<u8>>) -> usize {
    let mut best = bench;
    let mut height = grid[pos.0 as usize][pos.1 as usize];
    if height == 'S' as u8 {
        height = 'a' as u8;
    }
    if height == 'E' as u8 {
        return hist.len();
    }
    if hist.len() + 1 >= best {
        return best;
    }
    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .for_each(|(dx, dy)| {
            let dpos = (pos.0 + dx, pos.1 + dy);
            if dpos.0 < 0
                || dpos.0 as usize >= grid.len()
                || dpos.1 < 0
                || dpos.1 as usize >= grid[0].len()
            {
                return;
            }
            if hist.contains(&dpos) {
                return;
            }
            let mut dh = grid[dpos.0 as usize][dpos.1 as usize];
            if dh == 'E' as u8 {
                dh = 'z' as u8;
            }
            if dh == height || dh - 1 == height {
                let mut dhist = hist.clone();
                dhist.insert(dpos);
                let res = dig(dpos, best, &dhist, grid);
                if res < best {
                    best = res;
                }
            }
        });
    best
}

fn build_grid(s: &str) -> Vec<Vec<u8>> {
    s.lines()
        .map(|l| l.chars().map(|c| c as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>()
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
