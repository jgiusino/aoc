use std::{path::Path, fs::File, io::Read};

use day2::{Game, Bag};



fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("can't read file {} : {}", display,why),
        Ok(file) => file
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => println!("can't read file {} : {}", display, why),
        Ok(_) => println!("opening file {}", display)
    };

    let mut bag = Bag::new(12, 13, 14);
    let mut total = 0;
    let mut total_power = 0;

    for (index, line) in s.lines().collect::<Vec<&str>>().into_iter().enumerate() {
        println!("{line}");
        let game = Game::new((index + 1) as i32, line);
        for (round_idx,round) in game.rounds.into_iter().enumerate() {
            for pull in round {
                // dbg!(&pull);
                match bag.update(&pull) {
                    Ok(()) => continue,
                    Err(why) => {
                        println!("round #{} bag pull failed, {}", round_idx + 1, why);
                    },
                }
            }

            bag.reset();
        }

        if bag.ok {
            println!("adding {} to {}", game.number, total);
            total += game.number;
        } else {
            println!("skipping")
        }

        println!("adding {} to power {}\n", bag.power(), total_power);
        total_power += bag.power();

        // restart bag for next game
        bag.restart();
    }

    println!("{total}");
    println!("power: {}", total_power);
}
