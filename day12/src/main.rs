use std::{collections::VecDeque, ops::Add, slice::Iter, thread::{Thread, self}, sync::{Arc, mpsc}};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: i16,
    y: i16,
}

impl From<(i16, i16)> for Point {
    fn from((x, y): (i16, i16)) -> Self {
        Self { x, y }
    }
}

impl Dir {
    fn into_point(&self) -> Point {
        match self {
            Dir::Left => Point::from((-1, 0)),
            Dir::Right => Point::from((1, 0)),
            Dir::Up => Point::from((0, 1)),
            Dir::Down => Point::from((0, -1)),
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from((self.x + rhs.x, self.y + rhs.y))
    }
}

impl Dir {
    pub fn all_values() -> Iter<'static, Dir> {
        use Dir::*;
        [Left, Right, Up, Down].iter()
    }
}

type Path = Vec<Point>;

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

struct HeightMap {
    map: Vec<Vec<char>>,
    height: usize,
    width: usize,
    start_pos: Point,
    end_pos: Point,
}

impl From<&str> for HeightMap {
    fn from(s: &str) -> Self {
        let mut start_pos: Option<Point> = None;
        let mut end_pos: Option<Point> = None;

        let map = s
            .lines()
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let height = map.len();
        let width = map[0].len();

        for y in 0..height {
            for x in 0..width {
                if map[y][x] == 'S' {
                    start_pos = Some(Point::from((x as i16, y as i16)));
                } else if map[y][x] == 'E' {
                    end_pos = Some(Point::from((x as i16, y as i16)));
                }
            }
        }

        Self {
            map,
            width,
            height,
            start_pos: start_pos.expect("Couldn't find start pos"),
            end_pos: end_pos.expect("Couldn't find end pos"),
        }
    }
}

impl HeightMap {
    fn find_available_points(&self, pos: &Point, visited_points: &Vec<Point>) -> Vec<Point> {
        let mut points = vec![];
        for dir in Dir::all_values() {
            let point = pos.clone().add(dir.into_point());

            if visited_points.contains(&point) {
                continue;
            }

            if self.point_in_bounds(&point) {
                let mut value = self.get_point(&point);
                let mut curr_value = self.get_point(pos);

                if curr_value == 'S' {
                    curr_value = 'a';
                }

                if value == 'E' {
                    value = 'z';
                }

                if value as u8 - 1 == curr_value as u8 || value <= curr_value {
                    points.push(point);
                }
            }
        }

        points
    }

    fn get_point(&self, point: &Point) -> char {
        self.map[point.y as usize][point.x as usize]
    }

    fn point_in_bounds(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.width as i16 && point.y >= 0 && point.y < self.height as i16
    }

    fn find_shortest_path(&self, start_post: Point) -> Option<usize> {
        let pos = start_post;
        let mut point_queue = VecDeque::<(Point, usize)>::with_capacity(self.width * self.height);
        let mut visited = Vec::with_capacity(self.width * self.height);

        visited.push(pos);

        point_queue.push_back((pos, 0));

        while let Some((point, level)) = point_queue.pop_front() {
            let value = self.get_point(&point);

            if value == 'E' {
                return Some(level);
            }

            let availabe_points = self.find_available_points(&point, &visited);
            for p in availabe_points {
                visited.push(p);
                point_queue.push_back((p, level + 1));
            }
        }

        return None;
    }
}

fn main() {
    let heightmap = HeightMap::from(include_str!("input"));

    println!("Shortest path (S -> E): {:?}", heightmap.find_shortest_path(heightmap.start_pos));

    let mut starting_positions = vec![];

    for y in 0..heightmap.height {
        for x in 0..heightmap.width {
            let point = Point::from((x as i16, y as i16));
            let value = heightmap.get_point(&point);

            if value == 'a' {
                starting_positions.push(point);
            }
        }
    }


    let length = starting_positions.par_iter().flat_map(|&s| heightmap.find_shortest_path(s)).min();
    println!("Shortest possible path: {:?}", length);
}
