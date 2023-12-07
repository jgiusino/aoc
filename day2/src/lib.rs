#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}

pub struct Bag {
    red_cubes: i32,
    green_cubes: i32,
    blue_cubes: i32,
    red_pulled: i32,
    green_pulled: i32,
    blue_pulled: i32,
    pub ok: bool,
}

impl Bag {
    pub fn reset(&mut self) {
        self.red_pulled = 0;
        self.green_pulled = 0;
        self.blue_pulled = 0;
        self.ok = true;
    }

    pub fn update(&mut self, pull: &BagPull) -> Result<(),&str> {
        match pull.color {
            Color::Red => {
                self.red_pulled += pull.number;
                if self.red_pulled > self.red_cubes {
                    self.ok = false;
                    return Err("too many red pulled")
                }
            },
            Color::Green => {
                self.green_pulled += pull.number;
                if self.green_pulled > self.green_cubes {
                    self.ok = false;
                    return Err("too many green pulled")
                }
            },
            Color::Blue => {
                self.blue_pulled += pull.number;
                if self.blue_pulled > self.blue_cubes {
                    self.ok = false;
                    return Err("too many blue pulled")
                }
            }
        }

        Ok(())
    }

    pub fn new(red: i32, green: i32, blue: i32) -> Bag {
        Bag {
            red_cubes: red,
            green_cubes: green,
            blue_cubes: blue,
            red_pulled: 0,
            green_pulled: 0,
            blue_pulled: 0,
            ok: true
        }
    }
}

#[derive(Debug)]
pub struct BagPull {
    pub color: Color,
    pub number: i32
}

pub struct Game {
    pub number: i32,
    pub rounds: Vec<Vec<BagPull>>
}

impl Game {
    pub fn new(index: i32, s: &str) -> Game {
        let token: &str = s.split(": ")
            .collect::<Vec<&str>>()
            .last().unwrap();

        let mut rounds: Vec<Vec<BagPull>> = vec![];
        for round in token.split("; ") {
            let pulls: Vec<&str> = round.split(", ").collect();
            let mut v: Vec<BagPull> = vec![];
            for pull in pulls {
                let info: Vec<&str> = pull.split_ascii_whitespace().collect();
                let color = match info.last().unwrap().to_string().as_str() {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "blue" => Color::Blue,
                    _ => Color::Red
                };
                let number: i32 = info.first().unwrap().parse().unwrap();

                v.push(BagPull { color, number });
            }
            rounds.push(v);
        }

        Game { number: index, rounds: rounds }
    }
}
