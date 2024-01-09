use std::{error::Error, io::{BufReader, BufRead}, fs::File};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", std::env::current_dir().unwrap().display());
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"\d+")?;

    let res: u64 = reader.lines().map(|l| {
        match l {
            Ok(l) => {
                let mut winnings = Vec::<u64>::new();
                let mut scratched = Vec::<u64>::new();
                let mut card_score = 0;

                let pos = l.find(':').unwrap();
                let mut l_cloned = l.clone();
                l_cloned.drain(..=pos);

                for mat in re.find_iter(&l_cloned) {
                    if winnings.len() < 10 { winnings.push(mat.as_str().parse().unwrap()); }
                    else { scratched.push(mat.as_str().parse().unwrap()); }
                }

                println!("winnings: {:?}", winnings);
                println!("scratched: {:?}", scratched);

                winnings.iter().for_each(|w| {
                    if scratched.contains(w) {
                        if card_score == 0 { card_score = 1; }
                        else { card_score *= 2; }
                    }
                });
                card_score
            },
            Err(_) => 0,
        }
    }).sum();

    println!("{}", res);
    Ok(())
}
