extern crate csv;
extern crate rand;

use csv::StringRecord;
use std::path::PathBuf;
use std::fs::OpenOptions;

fn main() {
    let mut currNum = String::from("");
    let mut pass = String::from("");
    let start = csv::Position::new();

    // Set up the wordlist reader...
    // Will be able to change the wordlist eventually.
    let mut path = PathBuf::new();
    path.push(std::env::current_exe().unwrap());
    path.set_file_name("eff_large_wordlist.tsv");
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(path.as_path())
        .unwrap();

    for i in 1..7 {
        for _ in 1..6 {
            currNum.push_str(((rand::random::<u8>() % 6) + 1).to_string().as_str());
        }

        for result in reader.records() {
            let record = result.unwrap();
            if record.get(0).unwrap() == &currNum {
                pass.push_str(&record.get(1).unwrap());
                println!("Word {}: {}", i, &record.get(1).unwrap());
            }
        }
        currNum = String::from("");
        reader.seek(start.clone());
    }
    println!("Passphrase: {}", pass);
}
