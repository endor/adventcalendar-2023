struct Set {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn parse(lines: std::str::Lines) -> Vec<Game> {
    lines.map(|line| {
        let parts = line.split(": ").collect::<Vec<&str>>();
        let game_id: u32 = parts[0].split(" ").last().map(|x| x.parse::<u32>()).unwrap().unwrap();
        let sets = parts[1].split("; ");

        Game {
            id: game_id,
            sets: sets.map(|set| {
                let set_parts = set.split(", ");

                let mut red = None;
                let mut green = None;
                let mut blue = None;

                for part in set_parts {
                    let mut part_parts = part.split(" ");
                    let value = part_parts.next().unwrap();
                    let color = part_parts.next().unwrap();
                    let color_value = value.parse::<u32>().unwrap();

                    if color == "red" {
                        red = Some(color_value);
                    } else if color == "green" {
                        green = Some(color_value);
                    } else if color == "blue" {
                        blue = Some(color_value);
                    }
                }

                Set {
                    red,
                    green,
                    blue,
                }
            }).collect::<Vec<Set>>(),
        }
    }).collect::<Vec<Game>>()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    let games = parse(contents.lines());

    // part 1
    let part1: u32 = games.iter()
        .filter_map(|game| {
            let invalid_set = game.sets.iter().find(|set| {
                let invalid = set.red.is_some() && set.red.unwrap() > red_max ||
                    set.green.is_some() && set.green.unwrap() > green_max ||
                    set.blue.is_some() && set.blue.unwrap() > blue_max;

                invalid
            });

            match invalid_set {
                Some(_) => None,
                None => Some(game.id),
            }
        })
        .sum();

    println!("part 1: {}", part1);


    // part 2
    let part2: u32 = games.iter()
        .map(|game| {
            let mut greens = vec![];
            let mut blues = vec![];
            let mut reds = vec![];

            game.sets.iter().for_each(|set| {
                if set.red.is_some() {
                    reds.push(set.red.unwrap());
                }
                if set.green.is_some() {
                    greens.push(set.green.unwrap());
                }
                if set.blue.is_some() {
                    blues.push(set.blue.unwrap());
                }
            });

            greens.iter().max().unwrap() *
            blues.iter().max().unwrap() *
            reds.iter().max().unwrap()
        })
        .sum();

    println!("part 2: {}", part2);
}
