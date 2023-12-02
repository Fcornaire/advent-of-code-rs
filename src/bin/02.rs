advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    println!("{}", input);

    let red = 12;
    let green = 13;
    let blue = 14;

    for line in input.lines()  {
        let split_2_dot = line.split(":").collect::<Vec<&str>>();
        let game_id: u32 = split_2_dot[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
   
        let games = split_2_dot[1].split(";").collect::<Vec<&str>>();

        for game in games {
            let split_comma = game.split(",").collect::<Vec<&str>>();
            
        }
        
    }
    
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
