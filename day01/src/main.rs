fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let result: u32 = contents.lines()
        .map(|line| {
            let digits: Vec<u32> = line
                .chars()
                .enumerate()
                .into_iter()
                .map(|(i, c)| {
                    if c.is_digit(10) {
                        return Some(c.to_digit(10).unwrap());
                    }
                    let chars = line.chars().skip(i).collect::<String>();

                    if chars.starts_with("one") {
                        Some(1)
                    } else if chars.starts_with("two") {
                        Some(2)
                    } else if chars.starts_with("three") {
                        Some(3)
                    } else if chars.starts_with("four") {
                        Some(4)
                    } else if chars.starts_with("five") {
                        Some(5)
                    } else if chars.starts_with("six") {
                        Some(6)
                    } else if chars.starts_with("seven") {
                        Some(7)
                    } else if chars.starts_with("eight") {
                        Some(8)
                    } else if chars.starts_with("nine") {
                        Some(9)
                    } else {
                        None
                    }
                })
                .filter(|o| o.is_some())
                .map(|val| val.unwrap())
                .collect::<Vec<u32>>();

            let first = digits.first().unwrap();
            let last = digits.last().unwrap();
            let s: String = format!("{}{}", first, last);
            s.parse::<u32>().unwrap()
        })
        .sum();

    println!("{}", result);
}
