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

    for (index, line) in s.lines().collect::<Vec<&str>>().into_iter().enumerate() {
        println!("{line}");
        let game = Game::new((index + 1) as i32, line);
        for (round_idx,round) in game.rounds.into_iter().enumerate() {
            for pull in round {
                // dbg!(&pull);
                match bag.update(&pull) {
                    Ok(()) => continue,
                    Err(why) => {
                        println!("round #{} bag pull failed, {}\n", round_idx + 1, why);
                        break
                    },
                }
            }

            match bag.ok {
                true => bag.reset(),
                false => break,
            }
        }

        if bag.ok {
            println!("adding {} to {}\n", game.number, total);
            total += game.number;
        }
        bag.reset();
    }

    println!("{total}");
}
