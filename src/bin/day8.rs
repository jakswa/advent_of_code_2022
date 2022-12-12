use advent_of_code_2022::input;

fn main() {
    let inp = input("8");
    println!("part1: {}", run(&inp));
    println!("part2: {}", run2(&inp));
}

fn grid_for(s: &str) -> Vec<Vec<u8>> {
    s.lines()
        .map(|line| {
            line.chars()
                .map(|i| i as u8 - '0' as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}

fn run2(s: &str) -> usize {
    let grid = grid_for(s);
    let h = grid.len();
    let w = grid[0].len();

    (1..h - 1)
        .map(|x| (1..w - 1).map(|y| vis_trees(x, y, &grid)).max().unwrap())
        .max()
        .unwrap()
}

fn run(s: &str) -> usize {
    let grid = grid_for(s);
    let h = grid.len();
    let w = grid[0].len();

    let mut visibles = h * 2 + w * 2 - 4;

    (1..h - 1).for_each(|x| {
        (1..w - 1).for_each(|y| {
            if is_visible(x, y, &grid) {
                visibles += 1;
            }
        });
    });

    visibles
}

fn is_visible(xx: usize, yy: usize, grid: &Vec<Vec<u8>>) -> bool {
    let tree = grid[xx][yy];
    (0..xx).all(|x| grid[x][yy] < tree)
        || (xx + 1..grid.len()).all(|x| grid[x][yy] < tree)
        || (0..yy).all(|y| grid[xx][y] < tree)
        || (yy + 1..grid[0].len()).all(|y| grid[xx][y] < tree)
}

fn vis_trees(xx: usize, yy: usize, grid: &Vec<Vec<u8>>) -> usize {
    let tree = grid[xx][yy];
    let mut vis: Vec<usize> = vec![];
    let mut visible = 0;
    (0..xx).find(|x| {
        visible += 1;
        grid[xx - x - 1][yy] >= tree
    });
    vis.push(visible);
    visible = 0;
    (xx + 1..grid.len()).find(|x| {
        visible += 1;
        grid[*x][yy] >= tree
    });
    vis.push(visible);
    visible = 0;
    (0..yy).find(|y| {
        visible += 1;
        grid[xx][yy - y - 1] >= tree
    });
    vis.push(visible);
    visible = 0;
    (yy + 1..grid[0].len()).find(|y| {
        visible += 1;
        grid[xx][*y] >= tree
    });
    vis.push(visible);
    vis.into_iter().reduce(|a, b| a * b).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INP: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn day8_part1_works() {
        assert_eq!(run(TEST_INP), 21);
    }

    #[test]
    fn day8_part2_works() {
        assert_eq!(run2(TEST_INP), 8);
    }
}
