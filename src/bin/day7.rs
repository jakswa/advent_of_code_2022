use advent_of_code_2022::input;
use std::collections::HashMap;

fn main() {
    let inp = input("7");
    println!("part1: {}", run(&inp));
    println!("part2: {}", run2(&inp));
}

fn build_tree(s: &str) -> HashMap<String, Vec<&str>> {
    let mut cwd: Vec<&str> = vec![];
    let mut tree: HashMap<String, Vec<&str>> = HashMap::new();

    s.lines().for_each(|line| {
        if line.starts_with("$ cd ") {
            match line.split(" ").collect::<Vec<&str>>().pop().unwrap() {
                ".." => {
                    cwd.pop();
                }
                "/" => {
                    cwd = vec![];
                }
                newdir => {
                    cwd.push(newdir);
                }
            }
        } else if !line.starts_with("$ ls") {
            let dirlist = tree.entry(cwd.join("/")).or_default();
            (*dirlist).push(line);
        }
    });
    tree
}

fn run(s: &str) -> usize {
    let tree = build_tree(s);
    tree.keys()
        .filter(|k| *k != "")
        .map(|k| size(k, &tree))
        .filter(|i| *i < 100000)
        .sum()
}

fn run2(s: &str) -> usize {
    let disk = 70000000;
    let tree = build_tree(s);
    let unused = disk - size("", &tree);

    let mut candidates = tree
        .keys()
        .filter(|i| unused + size(i, &tree) >= 30000000)
        .collect::<Vec<&String>>();

    candidates.sort_by_cached_key(|k| size(k, &tree));
    size(candidates[0], &tree)
}

fn size(dir: &str, tree: &HashMap<String, Vec<&str>>) -> usize {
    let mut sum = 0;
    for entry in tree.get(dir).unwrap().iter() {
        let mut pieces = entry.split(" ");
        match pieces.next().unwrap() {
            "dir" => {
                let digdir = match dir {
                    "" => pieces.next().unwrap().to_string(),
                    nonroot => format!("{}/{}", nonroot, pieces.next().unwrap()),
                };
                sum += size(&digdir, tree);
            }
            number => sum += number.parse::<usize>().unwrap(),
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INP: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn day7_part1_works() {
        assert_eq!(run(TEST_INP), 95437);
    }

    #[test]
    fn day7_part2_works() {
        assert_eq!(
            run2(TEST_INP),
            24933642
        );
    }
}
