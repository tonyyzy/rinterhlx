pub mod data;

use data::Helices;
use data::Vec3;
use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
    pub res1: String,
    pub res2: String,
    pub res3: String,
    pub res4: String,
}

enum Location {
    Head,
    Tail,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 6 {
            return Err(Box::from("not enough arguments"));
        }
        let config = Config {
            filename: args[1].clone(),
            res1: args[2].clone(),
            res2: args[3].clone(),
            res3: args[4].clone(),
            res4: args[5].clone(),
        };
        // validate that helices are long enough to construct the vectors
        if (config.res2.parse::<i32>()? - config.res1.parse::<i32>()?).abs() < 6
            || (config.res4.parse::<i32>()? - config.res3.parse::<i32>()?).abs()
                < 6
        {
            Err(Box::from("Helix not long enough"))
        } else {
            Ok(config)
        }
    }
}

// Parse a pdb file, find the coordinates
// TODO: Switch to regex?
pub fn parse_pdb(config: &Config) -> Result<Helices, Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename.clone())?;

    let v1_tail = find_position(&config.res1, 1, &contents);
    let v1_head = find_position(&config.res2, 2, &contents);
    let v2_tail = find_position(&config.res3, 3, &contents);
    let v2_head = find_position(&config.res4, 4, &contents);

    Ok(Helices::new(v1_tail - v1_head, v2_tail - v2_head))
}

fn parse_coord(coord: &str) -> Vec3 {
    let mut parsed = coord
        .split_ascii_whitespace()
        .map(|x| x.parse::<f64>().expect("Parse Coord"));
    Vec3::new(
        parsed.next().expect("Parse Coord"),
        parsed.next().expect("Parse Coord"),
        parsed.next().expect("Parse Coord"),
    )
}

fn residues_list(residue: &str, location: &Location) -> [String; 4] {
    let res = residue.parse::<i32>().unwrap();
    match location {
        Location::Head => [
            format!("{:4}", res),
            format!("{:4}", res + 1),
            format!("{:4}", res + 2),
            format!("{:4}", res + 3),
        ],
        Location::Tail => [
            format!("{:4}", res - 3),
            format!("{:4}", res - 2),
            format!("{:4}", res - 1),
            format!("{:4}", res),
        ],
    }
}

fn find_position(residue: &str, index: i32, contents: &str) -> Vec3 {
    use Location::*;
    let backbone = [" N  ", " CA ", " C  "];
    let loc = if index % 2 == 0 { Tail } else { Head };
    let mut points = vec![];
    let residues = residues_list(residue, &loc);
    for line in contents.lines() {
        if line[0..4] == *"ATOM"
            && backbone.contains(&&line[12..16])
            && (residues.contains(&line[22..26].to_string()))
        {
            points.push(parse_coord(&line[30..54]))
        }
    }
    match loc {
        Head => points.into_iter().take(10).sum::<Vec3>() / 10.0,
        Tail => points.into_iter().rev().take(10).sum::<Vec3>() / 10.0,
    }
}
