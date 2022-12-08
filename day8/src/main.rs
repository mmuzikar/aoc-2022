type Grid = Vec<Vec<u8>>;

fn create_grid(s: &str) -> Grid {
    s.lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).expect("Incorrect number for a tree") as u8)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn find_visible_trees(grid: &Grid) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut visible = width * 2 + height * 2 - 4;
    for y in 1..height - 1 {
        let row = &grid[y];
        for x in 1..width - 1 {
            let column = grid.iter().map(|r| r[x]).collect::<Vec<_>>();
            let tree = row[x];
            let (top, mut bottom) = column.split_at(y);
            bottom = &bottom[1..];
            let (left, mut right) = row.split_at(x);
            right = &right[1..];

            let directions = [left, right, top, bottom]
                .iter()
                .map(|&dir| dir.iter().find(|&&i| i >= tree))
                .map(|o| o.is_some())
                .collect::<Vec<_>>();
            if directions.contains(&false) {
                visible += 1;
            }
        }
    }

    visible
}

fn calc_scenic_score((x, y): (usize, usize), grid: &Grid) -> u32 {
    let row = &grid[y];
    let column = grid.iter().map(|r| r[x]).collect::<Vec<_>>();

    let (top, mut bottom) = column.split_at(y);
    bottom = &bottom[1..];
    let (left, mut right) = row.split_at(x);
    right = &right[1..];

    let tree = row[x];

    let sides: [&mut dyn Iterator<Item = &u8>; 4] = [
        &mut top.iter().rev(),
        &mut left.iter().rev(),
        &mut bottom.iter(),
        &mut right.iter(),
    ];

    let mut scores = vec![];

    for side in sides {
        let mut score = 0;
        for i in side {
            score += 1;
            if *i >= tree {
                break;
            }
        }
        scores.push(score);
    }

    scores.iter().fold(1, |acc, x| acc * x)
}

fn find_highest_scenic_score(grid: &Grid) -> u32 {
    let height = grid.len();
    let width = grid[0].len();

    let mut max = 0;

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let score = calc_scenic_score((x, y), grid);
            if score > max {
                max = score;
            }
        }
    }

    max
}

fn main() {
    let grid = create_grid(include_str!("input"));
    println!("Visible trees: {}", find_visible_trees(&grid));

    println!("Highest scenic score: {}", find_highest_scenic_score(&grid));
}
