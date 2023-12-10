use std::collections::{HashSet, HashMap};

fn parse_numbers(s: &str) -> HashSet<i32> {
    s
        .trim()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut part1 = 0;
    let mut scratchcards = HashMap::new();

    contents.lines().enumerate().for_each(|(idx, line)| {
        let line = line.split(": ").last().unwrap();
        let parts = line.split(" | ").collect::<Vec<_>>();
        let winning = parse_numbers(parts[0]);
        let mine = parse_numbers(parts[1]);
        let intersection = winning.intersection(&mine).count();

        let current_card = scratchcards.entry(idx + 1).or_insert(1);

        for _ in 1..=*current_card {
            for j in 1..=intersection {
                let entry = scratchcards.entry(idx + 1 + j).or_insert(1);
                *entry += 1;
            }
        }

        part1 += if intersection == 0 {
            0
        } else if intersection == 1 {
            1
        } else {
            2_usize.pow((intersection - 1).try_into().unwrap())
        };
    });

    println!("part 1 {:?}", part1);
    println!("part 2 {:?}", scratchcards.values().sum::<usize>());
}
