advent_of_code::solution!(2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

//Parse string to Color
impl Color {
    pub fn from_str(s: &str) -> Option<Color> {
        let split_comma = s.split(" ").collect::<Vec<&str>>();
        let color = split_comma[1].trim();
        let value = split_comma[0].trim().parse().unwrap();

        match color {
            "red" => Some(Color::Red(value)),
            "green" => Some(Color::Green(value)),
            "blue" => Some(Color::Blue(value)),
            _ => None,
        }
    }

    pub fn is_above_limit(&self) -> bool {
        match self {
            Color::Red(value) => value > &12,
            Color::Green(value) => value > &13,
            Color::Blue(value) => value > &14,
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            Color::Red(value) => value.clone(),
            Color::Green(value) => value.clone(),
            Color::Blue(value) => value.clone(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;

    for line in input.lines()  {
        let split_2_dot = line.split(":").collect::<Vec<&str>>();
        let game_id: u32 = split_2_dot[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
   
        let games = split_2_dot[1].split(";").collect::<Vec<&str>>();
        let mut is_possible = true;

        for game in games {
            let colors = game.split(",").collect::<Vec<&str>>();

            for col in colors {
                let color = Color::from_str(col.trim());

                if color.is_none() {
                    continue;
                }

                if color.unwrap().is_above_limit() {
                    is_possible = false;
                    break;
                }      
            }    
        }

        if is_possible {
            result += game_id;
        }        
    }
    
    Some(result)
}

struct  MaxColor {
    red: Color,
    green: Color,
    blue: Color,
}

impl MaxColor {
    pub fn new() -> MaxColor {
        MaxColor {
            red: Color::Red(0),
            green: Color::Green(0),
            blue: Color::Blue(0),
        }
    }

    pub fn update(&mut self, color: Color) {
        match color {
            Color::Red(value) => {
                if value > self.red.value() {
                    self.red = color;
                }
            },
            Color::Green(value) => {
                if value > self.green.value() {
                    self.green = color;
                }
            },
            Color::Blue(value) => {
                if value > self.blue.value() {
                    self.blue = color;
                }
            },
        }
    }

    pub fn get_max(&self) -> u32 {
        self.red.value() * self.green.value() * self.blue.value()
    }
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;

    for line in input.lines()  {
        let split_2_dot: Vec<&str> = line.split(":").collect::<Vec<&str>>();
   
        let games = split_2_dot[1].split(";").collect::<Vec<&str>>();
        let mut max_color = MaxColor::new();

        for game in games {
            let colors = game.split(",").collect::<Vec<&str>>();

            for col in colors {
                let color = Color::from_str(col.trim());

                if color.is_none() {
                    continue;
                }

                max_color.update(color.unwrap());    
            }    
        }

        result += max_color.get_max();       
    }

    Some(result)
    
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
        assert_eq!(result, Some(2286));
    }
}
