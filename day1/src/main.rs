fn main() {
    let lines = include_str!("input").lines();

    let mut elves : Vec<i32> = Vec::new();

    let mut sum = 0;
    for line in lines {
        if line.is_empty() {
            elves.push(sum);
            sum = 0;
            continue;
        } else {
            let num : i32 = line.parse().expect("Failed to parse number");
            sum += num;
        }
    }

    elves.sort();
    elves.reverse();

    println!("Elves: {:?}", elves);
    println!("Max: {:?}", elves.first());
    println!("Max 3: {:?}", elves.iter().take(3).sum::<i32>());
}
