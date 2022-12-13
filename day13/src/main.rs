use std::{collections::VecDeque, cmp::Ordering, vec};

#[derive(Debug, PartialEq, Clone)]
enum Payload {
    Number(i32),
    List(Vec<Payload>),
}

#[derive(Debug, PartialEq)]
enum Action {
    Fail,
    Success,
    Continue,
}


impl Payload {
    fn order_fine(&self, other: &Payload) -> Action {
        use Action::*;
        use Payload::*;

        match (self, other) {
            (Number(a), Number(b)) => {
                if a == b {
                    Continue
                } else if a > b {
                    Fail
                } else {
                    Success
                }
            }
            (List(a), List(b)) => {
                for i in 0..a.len().max(b.len()) {
                    let action = match (a.get(i), b.get(i)) {
                        (Some(a), Some(b)) => a.order_fine(b),
                        (Some(_), None) => Fail,
                        (None, Some(_)) => Success,
                        (None, None) => panic!("Should never happen"),
                    };

                    if action != Continue {
                        return action;
                    }
                }

                return Continue;
            }
            (List(_), Number(b)) => self.order_fine(&List(vec![Number(*b)])),
            (Number(a), List(_)) => List(vec![Number(*a)]).order_fine(other),
        }
    }

    fn start_list(s: &mut VecDeque<char>) -> Self {
        let mut list: Vec<Payload> = vec![];

        while let Some(ch) = s.front() {
            match ch {
                ']' => {
                    s.pop_front();
                    break;
                }
                _ => {
                    list.push(Payload::consume(s));
                }
            }
        }

        return Payload::List(list);
    }

    fn consume<'a>(s: &mut VecDeque<char>) -> Self {
        use Payload::*;

        while let Some(ch) = s.pop_front() {
            match ch {
                '[' => return Payload::start_list(s),
                '0'..='9' => {
                    let mut number_buffer = String::new();
                    number_buffer.push(ch);
                    while let Some(c) = s.pop_front() {
                        if c.is_numeric() {
                            number_buffer.push(c);
                        } else {
                            s.push_front(c);
                            break;
                        }
                    }
                    return Number(number_buffer.parse().unwrap());
                }
                ',' => continue,
                _ => panic!(),
            }
        }
        panic!()
    }
}

fn main() {
    let mut packets = include_str!("input")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| Payload::consume(&mut s.chars().collect::<VecDeque<_>>()))
        .collect::<Vec<_>>();

    let mut sum = 0;

    for (i, chunk) in packets.chunks(2).enumerate() {
        println!("{:?}\n{:?}", chunk[0], chunk[1]);
        let action = chunk[0].order_fine(&chunk[1]);
        println!("{} = {:?}", i, action);

        if action == Action::Success {
            sum += i + 1;
        }
    }

    println!("Answer: {}", sum);

    let divider_packets = {
        use Payload::*;
        [List(vec![List(vec![Number(2)])]), List(vec![List(vec![Number(6)])])]
    };

    for divider in &divider_packets {
        packets.push(divider.clone());
    }

    packets.sort_by(|a, b| {
        let action = a.order_fine(&b);

        if action == Action::Continue {
            panic!();
        }

        if action == Action::Fail {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });


    println!("Sorted packets:");
    let mut decoder_key = 1;
    for (i, p) in packets.iter().enumerate() {
        if divider_packets.contains(p) {
            decoder_key *= i + 1;
        }
        println!("[{}] = {:?}", i+1, p);
    }

    println!("Decoder key: {}", decoder_key);
}
