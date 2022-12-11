use std::cell::RefCell;

use iter_tools::Itertools;

struct Monkey {
    id: usize,
    items: Vec<i64>,
    inspections: i64,
    operation: Operation,
    test: i64,
    test_pass: usize,
    test_fail: usize,
}

struct Operation {
    op: Box<dyn Fn(i64, i64) -> i64>,
    arg: Option<i64>,
}

impl Operation {
    fn new(func: impl Fn(i64, i64) -> i64 + 'static, arg: Option<i64>) -> Self {
        Self {
            op: Box::new(func),
            arg,
        }
    }

    fn call(&mut self, old: i64) -> i64 {
        (self.op)(old.clone(), self.arg.clone().unwrap_or(old))
    }
}

fn parse_expression(s: &str) -> Operation {
    let parts = s.split(" ").collect::<Vec<_>>();

    assert_eq!(parts.len(), 5);
    assert_eq!(parts[0], "new");
    assert_eq!(parts[1], "=");
    assert_eq!(parts[2], "old");

    let arg = parts[4].parse::<i64>().map_or(None, |i| Some(i));

    match parts[3] {
        "+" => Operation::new(|old, arg| old + arg, arg),
        "*" => Operation::new(|old, arg| old * arg, arg),
        "-" => Operation::new(|old, arg| old - arg, arg),
        _ => panic!("Wrong operation HELP"),
    }
}

fn parse_monkey(lines: &[&str]) -> Monkey {
    let id = lines[0]
        .replace("Monkey ", "")
        .replace(":", "")
        .parse::<usize>()
        .expect("Invalid id format");
    let items = lines[1]
        .trim()
        .replace("Starting items: ", "")
        .split(", ")
        .flat_map(&str::parse::<i64>)
        .collect::<Vec<_>>();
    let operation = parse_expression(lines[2].trim().replace("Operation: ", "").as_str());
    let test = lines[3]
        .trim()
        .replace("Test: divisible by ", "")
        .parse::<i64>()
        .expect("Unexpected test format");
    let test_pass = lines[4]
        .trim()
        .replace("If true: throw to monkey ", "")
        .parse::<usize>()
        .expect("Wrong monkey throw format");
    let test_fail = lines[5]
        .trim()
        .replace("If false: throw to monkey ", "")
        .parse::<usize>()
        .expect("Wrong monkey throw format");

    Monkey {
        id,
        items,
        operation,
        test,
        test_pass,
        test_fail,
        inspections: 0,
    }
}

fn simulate_round(monkeys: &mut Vec<RefCell<Monkey>>, simulate_worry: bool) {
    let modulo = monkeys.iter().fold(1, |acc, x| acc * x.borrow().test);
    for i in 0..monkeys.len() {
        simulate_monkey(monkeys, i, simulate_worry, modulo);
    }
}

fn simulate_monkey(monkeys: &mut Vec<RefCell<Monkey>>, monkey: usize, simulate_worry: bool, modulo: i64) {    
    let mut current_monkey = monkeys[monkey].borrow_mut();

    let items = current_monkey.items.clone();
    current_monkey.items.clear();

    for mut item in items {
        item = current_monkey.operation.call(item);
        if simulate_worry {
            item = item / 3;
        }

        item = item % modulo;

        current_monkey.inspections += 1;

        if &item % current_monkey.test == i64::from(0) {
            monkeys[current_monkey.test_pass]
                .borrow_mut()
                .items
                .push(item.clone());
        } else {
            monkeys[current_monkey.test_fail]
                .borrow_mut()
                .items
                .push(item.clone());
        }
    }
}

fn main() {
    let mut monkeys = include_str!("input")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .chunks(6)
        .into_iter()
        .map(|chunk| {
            let lines = chunk.collect_vec().into_boxed_slice();

            RefCell::new(parse_monkey(&lines))
        })
        .collect_vec();
    for _ in 0..10000 {
        simulate_round(&mut monkeys, false);
    }
    let monkey_business = monkeys
        .iter()
        .map(|m| m.borrow().inspections)
        .sorted()
        .rev()
        .take(2)
        .fold(1i64, |acc, x| acc * x);
    println!("Monkey business: {}", monkey_business);
}
