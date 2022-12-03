use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Item(char);

impl Item {
    fn priority(&self) -> u32 {
        if self.0.is_lowercase() {
            self.0 as u32 - 'a' as u32 + 1
        } else {
            26 + self.0 as u32 - 'A' as u32 + 1
        }
    }
}

enum Rucksack<'a> {
    Single(&'a str),
    Group((&'a str, &'a str, &'a str)),
}

impl<'a> From<&'a str> for Rucksack<'a> {
    fn from(s: &'a str) -> Self {
        Self::Single(s)
    }
}

impl<'a> From<(&'a str, &'a str, &'a str)> for Rucksack<'a> {
    fn from(s: (&'a str, &'a str, &'a str)) -> Self {
        Self::Group(s)
    }
}

impl <'a> Rucksack<'a> {
    
}

fn get_item_priority(items: &Vec<Item>) -> u32 {
    items.iter().map(Item::priority).sum()
}

fn validation(rucksack: &Rucksack, s: char) -> bool {
    match rucksack {
        Rucksack::Single(x) => x.split_at(x.len() / 2).1.contains(s),
        Rucksack::Group((_, b, c)) => b.contains(s) && c.contains(s) 
    }
}

fn get_duplicate_items(rucksack: Rucksack) -> Vec<Item> {
    let x = match rucksack {
        Rucksack::Single(s) => s.split_at(s.len() / 2).0,
        Rucksack::Group((a, _, _)) => a
    };
    let x = x.chars();
    x.into_iter()
        .filter(|it| validation(&rucksack, *it))
        .map(|it| Item(it))
        .unique_by(|it| it.0)
        .collect::<Vec<Item>>()
}

fn main() {
    let data = include_str!("input").lines().collect::<Vec<&str>>();
    let total_priority = data
        .iter()
        .map(|it| get_duplicate_items(Rucksack::from(*it)))
        .map(|it| get_item_priority(&it))
        .sum::<u32>();
    println!("Total priority: {}", total_priority);

    let total_priority = data
        .iter()
        .tuple_windows()
        .step_by(3)
        .map(|(a, b, c)| get_duplicate_items(Rucksack::from((*a, *b, *c))))
        .map(|it| get_item_priority(&it))
        .sum::<u32>();

    println!("Total priority: {}", total_priority);
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn correct_priority() {
        assert_eq!(Item('p').priority(), 16);
        assert_eq!(Item('P').priority(), 42);
    }

    #[test]
    fn duplicate_items() {
        let rucksack: Vec<Item> = get_duplicate_items(Rucksack::Single("vJrwpWtwJgWrhcsFMMfFFhFp"));
        assert_eq!(rucksack.len(), 1);
        assert_eq!(rucksack[0], Item('p'));
    }
}
