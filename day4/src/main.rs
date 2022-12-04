use std::ops::Range as StdRange;


type Range = StdRange<u32>;

fn fits((a, b): (&Range, &Range)) -> bool {
    a.start >= b.start && a.end <= b.end
}

fn overlaps((a, b): (&Range, &Range)) -> bool {
    a.start >= b.start && a.start <= b.end
}

fn parse_range(start: &str, end: &str) -> Range {
    Range {
        start: start.parse().expect("Failed to parse start assignment"),
        end: end.parse().expect("Failed to parse end assignment")
    }
}

fn parse_assignment(s: &str) -> Range {
    let (start, end) = s.split_once("-").expect("Failed to split into start and end");
    parse_range(start, end)
}

fn parse_assignments(s: &str) -> (Range, Range) {
    let (first, second) = s.split_once(",").expect("Failed to split assignment into 2 parts");

    (parse_assignment(first), parse_assignment(second))
}

fn call_on_both<T, F>(tuple: (&T, &T), f: F) -> bool 
where F : Fn((&T, &T)) -> bool
{
    f(tuple) || f((tuple.1, tuple.0))
}

fn main() {
    let contained = include_str!("input")
        .lines()
        .map(parse_assignments)
        .filter(|(first, second)| call_on_both((first, second), overlaps))
        .collect::<Vec<_>>();

    println!("Amount overlapped: {}", contained.len());
    println!("Amount fits: {}", contained.iter().filter(|(first, second)| call_on_both((first, second), fits)).count());

}
