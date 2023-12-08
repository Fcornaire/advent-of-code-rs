advent_of_code::solution!(3);

#[derive(Debug)]
struct Number {
    value: u32,
    position: (u32, u32),
}

#[derive(Debug)]
struct Symbol {
    _value: char,
    position: (u32, u32),
}

impl Symbol {
    fn new(value: char, position: (u32, u32)) -> Self {
        Self {
            _value: value,
            position,
        }
    }

    fn is_adjacent_to_two_different_numbers(&self, numbers: &[Number]) -> (bool, Number) {
        let mut adjacent_numbers = Vec::new();
        let (x, y) = self.position;

        for number in numbers {
            let (nx, ny) = number.position;
            let num_digits = (number.value as f32).log10() as u32 + 1;

            for i in 0..num_digits {
                let xi = nx - i;
                if (xi == x && (ny == y + 1 || ny == y - 1))
                    || (ny == y && (xi == x + 1 || xi == x - 1))
                    || (xi == x + 1 && ny == y + 1)
                    || (xi == x - 1 && ny == y - 1)
                    || (xi == x + 1 && ny == y - 1)
                    || (xi == x - 1 && ny == y + 1)
                {
                    adjacent_numbers.push(number);
                    break;
                }
            }
        }

        //multiply all adjacent numbers
        let mut result = 1;
        for number in adjacent_numbers.iter() {
            result *= number.value;
        }

        (adjacent_numbers.len() == 2, Number::new(result, (x, y)))
    }
}

impl Number {
    fn new(value: u32, position: (u32, u32)) -> Self {
        Self { value, position }
    }

    fn is_adjacent_to_symbol(&self, symbol: &Symbol) -> bool {
        let (x, y) = self.position;
        let (sx, sy) = symbol.position;
        let num_digits = (self.value as f32).log10() as u32 + 1;

        for i in 0..num_digits {
            let xi = x - i;
            if (xi == sx && (y == sy + 1 || y == sy - 1))
                || (y == sy && (xi == sx + 1 || xi == sx - 1))
                || (xi == sx + 1 && y == sy + 1)
                || (xi == sx - 1 && y == sy - 1)
                || (xi == sx + 1 && y == sy - 1)
                || (xi == sx - 1 && y == sy + 1)
            {
                return true;
            }
        }

        false
    }
}

fn is_a_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

fn is_a_gear(c: char) -> bool {
    c == '*'
}

pub fn part_one(input: &str) -> Option<u32> {
    let symbols: Vec<Symbol> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| is_a_symbol(*c))
                .map(move |(x, c)| Symbol::new(c, (x as u32, y as u32)))
        })
        .collect();

    let numbers: Vec<Number> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c.is_numeric())
                .map(move |(x, c)| Number::new(c.to_digit(10).unwrap(), (x as u32, y as u32)))
        })
        .collect();

    let mut combined_numbers = Vec::new();
    let mut current_number = numbers[0].value;
    let mut current_position = numbers[0].position;

    for number in numbers.iter().skip(1) {
        if number.position.0 == current_position.0 + 1 && number.position.1 == current_position.1 {
            current_number = current_number * 10 + number.value;
            current_position.0 += 1;
        } else {
            combined_numbers.push(Number {
                value: current_number,
                position: current_position,
            });
            current_number = number.value;
            current_position = number.position;
        }
    }

    combined_numbers.push(Number {
        value: current_number,
        position: current_position,
    });

    let mut result = 0;
    for number in combined_numbers {
        for symbol in symbols.iter() {
            if number.is_adjacent_to_symbol(symbol) {
                result += number.value;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let gears: Vec<Symbol> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| is_a_gear(*c))
                .map(move |(x, c)| Symbol::new(c, (x as u32, y as u32)))
        })
        .collect();

    let numbers: Vec<Number> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c.is_numeric())
                .map(move |(x, c)| Number::new(c.to_digit(10).unwrap(), (x as u32, y as u32)))
        })
        .collect();

    let mut combined_numbers = Vec::new();
    let mut current_number = numbers[0].value;
    let mut current_position = numbers[0].position;

    for number in numbers.iter().skip(1) {
        if number.position.0 == current_position.0 + 1 && number.position.1 == current_position.1 {
            current_number = current_number * 10 + number.value;
            current_position.0 += 1;
        } else {
            combined_numbers.push(Number {
                value: current_number,
                position: current_position,
            });
            current_number = number.value;
            current_position = number.position;
        }
    }

    combined_numbers.push(Number {
        value: current_number,
        position: current_position,
    });

    let mut result = 0;
    for gear in gears {
        let (is_adjacent, number) = gear.is_adjacent_to_two_different_numbers(&combined_numbers);
        if is_adjacent {
            result += number.value;
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
