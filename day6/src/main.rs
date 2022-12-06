use itertools::Itertools;

fn find_start(s: &str, window_size: usize) -> Option<usize> {
    for (index, window) in s.chars().collect::<Vec<_>>().windows(window_size).enumerate() {
        if window.iter().unique().count() == window_size {
            return Some(index + window_size);
        }
    };
    return None;
}

fn solve(s: &str, window_size: usize) -> () {
    if let Some(index) = find_start(s, window_size) {
        println!("{}: {:?}", s, index);
    } else {
        println!("No solution found for {}", s);
    }
}

fn main() {
    solve(include_str!("input"), 4);
    solve(include_str!("input"), 14);
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_1() {
        assert_eq!(find_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
        assert_eq!(find_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), Some(10));
    }

    #[test]
    fn test_2() {
        assert_eq!(find_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), Some(19));
        assert_eq!(find_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), Some(29));
    }
}