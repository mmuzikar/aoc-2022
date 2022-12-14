use std::{cell, collections::HashMap, thread::sleep, time::Duration};

type Point = (i32, i32);

#[derive(Clone, PartialEq)]
#[derive(Debug)]
enum Cell {
    Air,
    Sand,
    Rock,
    Pour,
    Invalid,
}

struct Cave {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
    overflow: Option<HashMap<Point, Cell>>,
}

const print: bool = false;

impl Cave {
    fn parse(s: &str) -> Self {
        let mut width = usize::MAX..=0;
        let mut height = 0..=0;

        let mut walls = vec![];

        for line in s.lines() {
            let points = line
                .split("->")
                .map(str::trim)
                .map(|s| {
                    let parts = s.split(",").collect::<Vec<_>>();
                    let point = (
                        parts.get(0).unwrap().parse::<usize>().unwrap(),
                        parts.get(1).unwrap().parse::<usize>().unwrap(),
                    );
                    width = *width.start().min(&point.0)..=*width.end().max(&point.0);
                    height = *height.start().min(&point.1)..=*height.end().max(&point.1);
                    point
                })
                .collect::<Vec<_>>();
            walls.push(points);
        }
        let map_start = width.start().clone();

        walls.iter_mut().for_each(|wall| {
            wall.iter_mut().for_each(|(x, _)| {
                *x = *x - map_start;
            })
        });

        let width = width.count();
        let height = height.count();
        let mut cells = vec![Cell::Air; width * height];

        cells[500 - map_start] = Cell::Pour;

        for wall in &walls {
            for i in 1..wall.len() {
                let (a, b) = (wall[i - 1], wall[i]);
                if a.0 != b.0 {
                    for x in a.0.min(b.0)..=a.0.max(b.0) {
                        cells[a.1 * width + x] = Cell::Rock;
                    }
                } else {
                    for y in a.1.min(b.1)..=a.1.max(b.1) {
                        cells[y * width + a.0] = Cell::Rock;
                    }
                }
            }
        }

        Self {
            width: width as i32,
            height: height as i32,
            cells,
            overflow: None,
        }
    }

    fn get_point(&mut self, (x, y): (i32, i32)) -> Cell {
        if let Some(overflow) = &mut self.overflow {
            if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
                self.cells
                    .get((y * self.width + x) as usize)
                    .unwrap()
                    .clone()
            } else {
                overflow
                    .entry((x, y))
                    .or_insert(if y == self.height + 1 {
                        Cell::Rock
                    } else {
                        Cell::Air
                    })
                    .clone()
            }
        } else {
            if x < 0 {
                return Cell::Invalid;
            }
            if let Some(cell) = self.cells.get((y * self.width + x) as usize) {
                cell.clone()
            } else {
                Cell::Invalid
            }
        }
    }

    fn simulate(&mut self) -> usize {
        use Cell::*;
        let mut current_sand: (i32, i32) = (0, 0);
        let mut spawn_new = true;
        let pour_pos = self.cells.iter().position(|c| c.eq(&Pour)).unwrap() as i32;
        loop {
            if spawn_new {
                current_sand = (pour_pos, 0);
                spawn_new = false;
                if print {
                    println!("{}", self.to_string());
                }
            }
            let prev_pos = current_sand.clone();

            let down_block = self.get_point((current_sand.0, current_sand.1 + 1));
            if down_block.eq(&Invalid) {
                self.put_cell(current_sand, Air);
                break;
            }
            if down_block.eq(&Air) {
                current_sand.1 += 1;
            } else if [Rock, Sand].contains(&down_block) {
                let left_diag = self.get_point((current_sand.0 - 1, current_sand.1 + 1));
                let right_diag = self.get_point((current_sand.0 + 1, current_sand.1 + 1));
                if left_diag.eq(&Invalid) {
                    self.put_cell(current_sand, Air);
                    break;
                }
                if left_diag.eq(&Air) {
                    current_sand.0 -= 1;
                    current_sand.1 += 1;
                } else if right_diag.eq(&Air) {
                    current_sand.0 += 1;
                    current_sand.1 += 1;
                } else {
                    spawn_new = true;
                }
            }

            self.put_cell(prev_pos, Air);
            self.put_cell(current_sand, Sand);


            if current_sand.0 == pour_pos && current_sand.1 == 0 {
                break;
            }
        }

        let cell_sand = self.cells.iter().filter(|&cell| cell.eq(&Sand)).count();
        if let Some(overflow) = &mut self.overflow {
            cell_sand + overflow.values().filter(|&cell| cell.eq(&Sand)).count()
        } else {
            cell_sand
        }
    }

    fn add_floor(&mut self) {
        self.overflow = Some(HashMap::new());
    }

    fn put_cell(&mut self, (x, y): (i32, i32), cell: Cell) {
        if let Some(overflow) = &mut self.overflow {
            if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
                self.cells[(y * self.width + x) as usize] = cell;
            } else {
                overflow.insert((x, y), cell);
            }
        } else {
            self.cells[(y * self.width + x) as usize] = cell;
        }
    }

    fn to_string(&mut self) -> String {
        let to_symbol = |cell: &Cell| match cell {
            Cell::Air => '.',
            Cell::Rock => '#',
            Cell::Sand => 'o',
            Cell::Pour => '+',
            _ => '!',
        };

        if let Some(overflow) = &mut self.overflow {
            let mut x_range = i32::MAX..=0;
            let mut y_range = 0..=0;
            for (x, y) in overflow.keys() {
                x_range = *x_range.start().min(x)..=*x_range.end().max(x);
                y_range = *y_range.start().min(y)..=*y_range.end().max(y);
            }
            if overflow.keys().count() == 0 {
                x_range = -1..=self.width;
                y_range = -1..=self.height + 1;
            }
            let mut s =
                String::with_capacity((x_range.clone().count() * y_range.clone().count()) as usize);

            for y in y_range {
                for x in x_range.clone() {
                    if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
                        let ch = to_symbol(self.cells.get((y * self.width + x) as usize).unwrap());
                        s.push(ch);
                    } else {
                        let ch = self.get_point((x, y));
                        s.push(to_symbol(&ch));
                    }
                }
                s.push('\n');
            }
            s
        } else {
            let mut s = String::with_capacity((self.width * self.height) as usize);
            for (i, cell) in self.cells.iter().enumerate() {
                let ch = to_symbol(cell);
                if i % (self.width as usize) == 0 {
                    s.push('\n');
                }
                s.push(ch);
            }

            s
        }
    }
}

fn main() {
    let mut cave = Cave::parse(include_str!("input"));
    cave.add_floor();
    let iter = cave.simulate();
    println!("{}", iter);
}
