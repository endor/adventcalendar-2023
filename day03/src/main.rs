#[derive(Debug)]
enum Position {
    Empty,
    Number(char),
    Symbol(char),
}

impl Position {
    fn is_number(&self) -> bool {
        if let Position::Number(_) = self {
            true
        } else {
            false
        }
    }

    fn is_symbol(&self) -> bool {
        if let Position::Symbol(_) = self {
            true
        } else {
            false
        }
    }
}

fn is_adjacent_to_symbol(field: &Vec<Vec<Position>>, x: usize, y: usize) -> bool {
    let mut adjacent = false;
    if x > 0 {
        if field[y][x - 1].is_symbol() {
            adjacent = true;
        }
    }
    if x < field[y].len() - 1 {
        if field[y][x + 1].is_symbol() {
            adjacent = true;
        }
    }
    if y > 0 {
        if field[y - 1][x].is_symbol() {
            adjacent = true;
        }
    }
    if y < field.len() - 1 {
        if field[y + 1][x].is_symbol() {
            adjacent = true;
        }
    }
    if x > 0 && y > 0 {
        if field[y - 1][x - 1].is_symbol() {
            adjacent = true;
        }
    }
    if x < field[y].len() - 1 && y > 0 {
        if field[y - 1][x + 1].is_symbol() {
            adjacent = true;
        }
    }
    if x > 0 && y < field.len() - 1 {
        if field[y + 1][x - 1].is_symbol() {
            adjacent = true;
        }
    }
    if x < field[y].len() - 1 && y < field.len() - 1 {
        if field[y + 1][x + 1].is_symbol() {
            adjacent = true;
        }
    }
    adjacent
}

fn adjacent_numbers(field: &Vec<Vec<Position>>, x: usize, y: usize) -> Vec<i32> {
    let mut numbers = vec![];
    if x > 0 {
        if field[y][x - 1].is_number() {
            numbers.push(collect_number(field, x - 1, y));
        }
    }
    if x < field[y].len() - 1 {
        if field[y][x + 1].is_number() {
            numbers.push(collect_number(field, x + 1, y));
        }
    }
    if y > 0 {
        if field[y - 1][x].is_number() {
            numbers.push(collect_number(field, x, y - 1));
        }
    }
    if y < field.len() - 1 {
        if field[y + 1][x].is_number() {
            numbers.push(collect_number(field, x, y + 1));
        }
    }
    if x > 0 && y > 0 {
        if field[y - 1][x - 1].is_number() {
            numbers.push(collect_number(field, x - 1, y - 1));
        }
    }
    if x < field[y].len() - 1 && y > 0 {
        if field[y - 1][x + 1].is_number() {
            numbers.push(collect_number(field, x + 1, y - 1));
        }
    }
    if x > 0 && y < field.len() - 1 {
        if field[y + 1][x - 1].is_number() {
            numbers.push(collect_number(field, x - 1, y + 1));
        }
    }
    if x < field[y].len() - 1 && y < field.len() - 1 {
        if field[y + 1][x + 1].is_number() {
            numbers.push(collect_number(field, x + 1, y + 1));
        }
    }

    numbers.sort();
    numbers.dedup();
    numbers
}

fn collect_number(field: &Vec<Vec<Position>>, orig_x: usize, y: usize) -> i32 {
    let mut number = vec![];
    let mut x = orig_x;

    loop {
        if x > 0 {
            if let Position::Number(n) = field[y][x - 1] {
                number.push(n);
                x -= 1;
                continue;
            }
        }

        break;
    }

    number.reverse();
    if let Position::Number(n) = field[y][orig_x] {
        number.push(n);
    }
    x = orig_x;

    loop {
        if x < field[y].len() - 1 {
            if let Position::Number(n) = field[y][x + 1] {
                number.push(n);
                x += 1;
                continue;
            }
        }

        break;
    }

    number.iter().cloned().collect::<String>().parse::<i32>().unwrap()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let field = contents.lines().map(|line| {
        line.chars().map(|c| {
            if c == '.' {
                Position::Empty
            } else if c.is_digit(10) {
                Position::Number(c)
            } else {
                Position::Symbol(c)
            }
        }).collect::<Vec<Position>>()
    }).collect::<Vec<Vec<Position>>>();

    let mut part_numbers = vec![];
    let mut gear_ratios = vec![];
    field.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, p)| {
            if p.is_number() {
                if is_adjacent_to_symbol(&field, x, y) {
                    part_numbers.push(collect_number(&field, x, y));
                }
            }
            if let Position::Symbol('*') = p {
                let numbers = adjacent_numbers(&field, x, y);
                if numbers.len() == 2 {
                    gear_ratios.push(numbers[0] * numbers[1]);
                }
            }
        });
    });
    part_numbers.dedup();

    println!("part 1 {:?}", part_numbers.into_iter().sum::<i32>());
    println!("part 2 {:?}", gear_ratios.into_iter().sum::<i32>());
}
