use std::collections::HashSet;

use lending_iterator::{lending_iterator::constructors::windows_mut, LendingIterator};

const WIDTH: usize = 40;
const HEIGHT: usize = 40;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction {
    dir: Direction,
    step: u8,
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Point {
    x: i16,
    y: i16,
}

impl From<(i16, i16)> for Point {
    fn from((x, y): (i16, i16)) -> Self {
        Self { x, y }
    }
}

impl Point {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        use Direction::*;
        let (dir, step) = s.split_once(" ").expect("Wrong instruction format");
        let dir = match dir {
            "R" => Right,
            "U" => Up,
            "L" => Left,
            "D" => Down,
            _ => panic!("Invalid direction"),
        };

        Self {
            dir,
            step: step.parse().expect("Wrong number format"),
        }
    }
}

fn visualize(segments: &Vec<Point>) {
    let mut board = vec![vec![".".to_string(); WIDTH]; HEIGHT];

    let mid = WIDTH as i16 / 2;

    board[mid as usize][mid as usize] = "s".to_string();

    for (i, segment) in segments.iter().enumerate() {
        board[(mid + segment.y) as usize][(mid + segment.x) as usize] = i.to_string();
    }

    board.iter().rev().for_each(|row| {
        let row = row.iter().fold(String::new(), |acc, x| acc + x);
        println!("{}", row);
    })
}

fn visualize_path(path: &HashSet<Point>) {
    let mut board = vec![vec![".".to_string(); WIDTH]; HEIGHT];

    let mid = WIDTH as i16 / 2;

    board[mid as usize][mid as usize] = "s".to_string();

    for point in path {
        board[(mid + point.y) as usize][(mid + point.x) as usize] = "#".to_string();
    }

    board.iter().rev().for_each(|row| {
        let row = row.iter().fold(String::new(), |acc, x| acc + x);
        println!("{}", row);
    })
}

fn main() {
    let instructions = include_str!("input")
        .lines()
        .map(Instruction::from)
        .collect::<Vec<_>>();

    // let mut head = Point::new(0, 0);
    // let mut tail = Point::new(0, 0);

    let mut segments = vec![Point::new(0, 0); 10];

    let mut visited_points = HashSet::<Point>::new();

    visited_points.insert(segments.last().unwrap().clone());

    for Instruction { dir, step } in instructions {
        for _ in 0..step {
            move_in_dir(&dir, segments.first_mut().unwrap());

            segments.windows_mut::<2>().for_each(|window| {
                let (head, child) = window.split_first_mut().unwrap();

                let mut child = &mut child[0];

                let diff_x = head.x - child.x;
                let diff_y = head.y - child.y;

                if diff_x.abs() <= 1 && diff_y.abs() <= 1 {
                    return;
                }

                let x_dir = if head.x == child.x {
                    0
                } else if head.x > child.x {
                    1
                } else {
                    -1
                };
                let y_dir = if head.y == child.y {
                    0
                } else if head.y > child.y {
                    1
                } else {
                    -1
                };

                if child.x == head.x || child.y == head.y {
                    let dir = match (x_dir, y_dir) {
                        (1, _) => Direction::Right,
                        (-1, _) => Direction::Left,
                        (_, 1) => Direction::Up,
                        (_, -1) => Direction::Down,
                        _ => unreachable!(),
                    };

                    move_in_dir(&dir, &mut child)
                } else {
                    child.x += x_dir;
                    child.y += y_dir;
                }
            });

            visited_points.insert(segments.last().unwrap().clone());
        }
        // println!("{:?} {}", dir, step);
        // visualize(&segments);
    }

    // visualize_path(&visited_points);

    println!("Visited points: {}", visited_points.len());
}

fn move_in_dir(dir: &Direction, point: &mut Point) {
    match *dir {
        Direction::Down => point.y -= 1,
        Direction::Up => point.y += 1,
        Direction::Left => point.x -= 1,
        Direction::Right => point.x += 1,
    }
}
