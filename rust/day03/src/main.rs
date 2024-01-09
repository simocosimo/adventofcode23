use regex::Regex;
use std::{fs::File, io::{BufReader, BufRead}, error::Error, collections::HashMap};

#[derive(Debug, Clone)]
struct JustMyError;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Number {
    n: u64,
    row: usize,
    start_pos: usize,
    end_pos: usize,
}

#[derive(Debug, Clone)]
struct Symbol {
    s: char,
    row: usize,
    pos: usize,
}

impl Number {
    pub fn new(n: u64, row: usize, start_pos: usize, end_pos: usize) -> Self {
        Self { n, row, start_pos, end_pos}
    }
}

impl Symbol {
    pub fn new(s: char, row: usize, pos: usize) -> Self {
        Self { s, row, pos}
    }
}

impl From<(usize, usize)> for Symbol {
    fn from(value: (usize, usize)) -> Self {
        Symbol {
            s: ' ',
            row: value.0,
            pos: value.1,
        }
    }
}

impl PartialEq<Symbol> for Number {
    fn eq(&self, other: &Symbol) -> bool {
        (self.start_pos <= other.pos && self.end_pos > other.pos) && self.row == other.row
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.row == other.row
    }
}

fn box_coords(row: usize, start: usize, end: usize) -> Vec<(usize, usize)> {
    (row.saturating_sub(1)..row + 2)
        .flat_map(|i| {
            (start.saturating_sub(1)..end + 1)
                .map(|j| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", std::env::current_dir().unwrap().display());
    let mut file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let re = Regex::new(r"\d+")?;

    let mut numbers = Vec::<Number>::new();
    let mut symbols = Vec::<Symbol>::new();

    // this loop is for the integer parsing
    for (row, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                for mat in re.find_iter(&line) {
                    let num: u64 = mat.as_str().parse()?;
                    let n = Number::new(num, row, mat.start(), mat.end());
                    numbers.push(n);
                }
            },
            Err(err) => return Err(Box::new(err)),
        }
    }

    // This loop is for the symbols parsing
    file = File::open("input.txt")?;
    reader = BufReader::new(file);
    for (row, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                for (pos, mat) in line.chars().enumerate() {
                    if mat != '.' && !mat.is_numeric() {
                        let s = Symbol::new(mat, row, pos);
                        symbols.push(s);
                    }
                }
            },
            Err(err) => return Err(Box::new(err)),
        }
    }

    // let res: u64 = numbers.iter().filter_map(|n| {
    //     let coords = box_coords(n.row, n.start_pos, n.end_pos);

    //     match coords.iter().any(
    //         |c| {
    //             let local_s = Symbol::from(*c);
    //             symbols.contains(&local_s)
    //         }
    //     ) {
    //         true => Some(n.n),
    //         false => None
    //     }
    // }).sum();

    // println!("Part 1: {}", res);

    println!("symbols are {:?}", symbols);

    let mut hs = HashMap::<(usize, usize), Vec<Number>>::new();
    symbols.iter().filter(|s| {
        s.s == '*'
    }).for_each(|s| {
        let coords = box_coords(s.row, s.pos, s.pos + 1);
        println!("Gear with coords {:?} has box coords of {:?}", (s.row,s.pos), coords);
        coords.iter().for_each(|c| {
            let local_s = Symbol::from(*c);
            numbers.iter().for_each(|n| {
                if n.to_owned() == local_s {
                    let entry = hs.entry((s.row, s.pos)).or_insert_with(Vec::new);
                    if !entry.contains(n) {
                        entry.push(n.clone());
                    }
                }
            })
        });
    });

    println!("hs: {:?}", hs);
    
    let temp: Vec<u64> = hs.iter().filter(|h| h.1.len() >= 2).map(|e| {
        e.1.iter().fold(1, |acc, n| acc * n.n)
    }).collect();

    println!("hs: {:?}", temp);

    let res2: u64 = temp.iter().sum();

    println!("Part 2: {}", res2);

    Ok(())
}
