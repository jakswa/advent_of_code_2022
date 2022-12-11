use advent_of_code_2022::input;

fn main() {
    let inp = input("5");
    println!("part1: {}", run(&inp));
    println!("part2: {}", run2(&inp));
}

fn run2(s: &str) -> String {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    (0..9).for_each(|_i| stacks.push(vec![]));
    let mut lines = s.lines();
    loop {
        let line = lines.next().unwrap();
        if line == "" { break; }
        line.chars().enumerate().filter(|(_i, c)| {
            c.is_ascii_alphabetic()
        }).for_each(|(i, c)| {
            stacks[i/4].push(c);
        });
    }

    stacks.iter_mut().for_each(|i| i.reverse());

    lines.map(|i| {
        i.split(' ').flat_map(|i| i.parse::<usize>()).collect::<Vec<usize>>()
    }).for_each(|cmd| {
            let mut tmp = vec![];
            (0..cmd[0]).for_each(|_i| {
                let from = stacks[cmd[1]-1].pop().unwrap();
                tmp.push(from);
            });
            while !tmp.is_empty() { stacks[cmd[2]-1].push(tmp.pop().unwrap()); }
        });

    stacks.iter().filter_map(|s| s.last()).collect::<String>()
}

fn run(s: &str) -> String {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    (0..9).for_each(|_i| stacks.push(vec![]));
    let mut lines = s.lines();
    loop {
        let line = lines.next().unwrap();
        if line == "" { break; }
        line.chars().enumerate().filter(|(_i, c)| {
            c.is_ascii_alphabetic()
        }).for_each(|(i, c)| {
            stacks[i/4].push(c);
        });
    }

    stacks.iter_mut().for_each(|i| i.reverse());

    lines.map(|i| {
        i.split(' ').flat_map(|i| i.parse::<usize>()).collect::<Vec<usize>>()
    }).for_each(|cmd| {
            (0..cmd[0]).for_each(|_i| {
                let from = stacks[cmd[1]-1].pop().unwrap();
                stacks[cmd[2]-1].push(from);
            });
        });

    stacks.iter().filter_map(|s| s.last()).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_part1_works() {
        assert_eq!(
            run("    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"),
            "CMZ"
        );
    }

    #[test]
    fn day5_part2_works() {
        assert_eq!(
            run2("    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"),
            "MCD"
        );
    }
}
