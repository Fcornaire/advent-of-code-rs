advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut res = 0;

    //Part 1
    for line in lines.clone() {
        let fst_char = line.chars().into_iter().find(|c| c.is_digit(10));
        if fst_char.is_none() {
            continue;
        }

        let last_char = line.chars().rev().into_iter().find(|c| c.is_digit(10));

        let mut digit1 = 0;
        if let Some(char1) = fst_char {
            digit1 = char1.to_digit(10).unwrap();
        }

        let mut digit2 = 0;
        if let Some(char2) = last_char {
            digit2 = char2.to_digit(10).unwrap();
        }

        let value = digit1 * 10 + digit2;
        res += value;
    }

    Some(res)
}

fn get_first(line: &str, numbers: Vec<&str>) -> Option<u32> {
    let fst_digit = (
        line.chars().into_iter().find(|c| c.is_digit(10)),
        line.chars().into_iter().position(|c| c.is_digit(10)),
    );

    let indices: Vec<(&str, usize)> = numbers
        .iter()
        .filter_map(|&s| line.find(s).map(|index| (s, index)))
        .collect();

    let fst_number = indices.into_iter().min_by_key(|&(_, index)| index);

    if fst_digit.1.is_none() && fst_number.is_none() {
        return None;
    }

    let mut fst: u32 = 0;

    if let Some(fst_ind) = fst_digit.1 {
        if let Some((numb, fst_num_ind)) = fst_number {
            if fst_ind < fst_num_ind {
                fst = fst_digit.0.unwrap().to_digit(10).unwrap();
            } else {
                fst = word_to_number(numb);
            }
        } else {
            fst = fst_digit.0.unwrap().to_digit(10).unwrap();
        }
    } else {
        fst = word_to_number(fst_number.unwrap().0);
    }

    if fst == 0 {
        return None;
    }

    return Some(fst);
}

fn word_to_number(word: &str) -> u32 {
    match word.to_lowercase().as_str() {
        "one" | "eno" => 1,
        "two" | "owt" => 2,
        "three" | "eerht" => 3,
        "four" | "ruof" => 4,
        "five" | "evif" => 5,
        "six" | "xis" => 6,
        "seven" | "neves" => 7,
        "eight" | "thgie" => 8,
        "nine" | "enin" => 9,
        _ => 0,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split("\n").collect();

    let mut res2 = 0;
    let mut ind = 1;
    for line in lines {
        let numbers: Vec<&str> = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let fst = get_first(line, numbers.clone());

        let rev: String = line.chars().rev().collect();

        let reversed_numbers: Vec<String> = numbers
            .clone()
            .iter()
            .map(|&s| s.chars().rev().collect())
            .collect();
        let str_refs: Vec<&str> = reversed_numbers.iter().map(|s| s.as_str()).collect();

        let second = get_first(&rev, str_refs);

        let mut digit1 = 0;
        if let Some(dig1) = fst {
            digit1 = dig1
        }

        let mut digit2 = 0;
        if let Some(dig2) = second {
            digit2 = dig2
        }

        let value = digit1 * 10 + digit2;

        println!("ind {ind} : value : {value}");

        ind += 1;
        res2 += value;
    }

    Some(res2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
