use advent_of_code_2022::input;

fn main() {
    let inp = input("11");
    println!("part1: {}", run(&inp));
    //println!("part2: {}", run2(&inp));
}

#[derive(Debug)]
enum Mop {
    Multiply,
    Add,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    divide: i64,
    op: (Mop, i64),
    targets: (i64, i64),
    inspects: i64,
}

fn run(s: &str) -> usize {
    let mut monkeys: Vec<Monkey> = s
        .split("\n\n")
        .map(|monkey| {
            let mut lines = monkey.lines().skip(1);
            let items = lines
                .next()
                .unwrap()
                .split([',', ' '])
                .filter_map(|s| s.parse::<i64>().ok())
                .collect::<Vec<i64>>();
            let op_line = lines.next().unwrap();
            let op = match op_line.contains("+") {
                true => Mop::Add,
                false => Mop::Multiply,
            };
            let op_right = op_line
                .split(' ')
                .filter_map(|i| i.parse::<i64>().ok())
                .next()
                .unwrap_or(0);
            let divide = lines
                .next()
                .unwrap()
                .split(' ')
                .flat_map(|s| s.parse::<i64>().ok())
                .next()
                .unwrap();
            let targ1 = lines
                .next()
                .unwrap()
                .split(' ')
                .flat_map(|s| s.parse::<i64>().ok())
                .next()
                .unwrap();
            let targ2 = lines
                .next()
                .unwrap()
                .split(' ')
                .flat_map(|s| s.parse::<i64>().ok())
                .next()
                .unwrap();

            Monkey {
                items,
                op: (op, op_right),
                divide,
                targets: (targ1, targ2),
                inspects: 0,
            }
        })
        .collect::<Vec<Monkey>>();

    for _i in 0..20 {
        let mut deliveries: Vec<(i64, i64)> = vec![];
        monkeys.iter_mut().enumerate().for_each(|(ind, monkey)| {
            deliveries = deliveries
                .drain(..)
                .filter(|d| {
                    if d.0 == ind as i64 {
                        monkey.items.push(d.1);
                        false
                    } else {
                        true
                    }
                })
                .collect::<Vec<(i64, i64)>>();

            new_deliveries(monkey)
                .drain(..)
                .for_each(|d| deliveries.push(d));
        });
        deliveries
            .drain(..)
            .for_each(|d| monkeys[d.0 as usize].items.push(d.1));
    }
    monkeys.sort_by_key(|m| m.inspects);

    monkeys[monkeys.len() - 1].inspects as usize * monkeys[monkeys.len() - 2].inspects as usize
}

fn new_deliveries(monkey: &mut Monkey) -> Vec<(i64, i64)> {
    monkey
        .items
        .drain(..)
        .map(|item| {
            let op1 = match monkey.op.1 {
                0 => item,
                a => a,
            };
            let new_item = match monkey.op.0 {
                Mop::Multiply => item * op1 / 3,
                Mop::Add => (item + op1) / 3,
            };
            let target = match new_item % monkey.divide == 0 {
                true => monkey.targets.0,
                false => monkey.targets.1,
            };
            monkey.inspects += 1;
            (target, new_item)
        })
        .collect::<Vec<(i64, i64)>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part0_works() {
        assert_eq!(run(TEST_INP), 2713310158);
    }

    const TEST_INP: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
}
