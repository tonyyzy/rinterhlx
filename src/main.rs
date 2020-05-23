use std::env;
use std::process;

use interhlx::parse_pdb;
use interhlx::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // test that each helix has enough backbone atoms to define the vector
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });
    let atoms = parse_pdb(&config);
    println!("{:.3}", atoms.unwrap().dot());
}
