#[macro_use]
extern crate clap;

extern crate csv;
extern crate rand;

use std::path::PathBuf;
use clap::App;
use rand::Rng;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut curr_num = String::from("");
    let mut pass = String::from("");
    let start = csv::Position::new();
    let mut gen = rand::os::OsRng::new().unwrap();

    let len;
    if matches.is_present("length") {
        len = matches.value_of("length").unwrap().parse::<i8>().unwrap();
    } else {
        len = 6;
    }
    let mut path = PathBuf::new();
    if matches.is_present("wordlist") {
        path = PathBuf::from(matches.value_of("wordlist").unwrap());
    } else {
        path.push(std::env::current_exe().unwrap());
        path.set_file_name("eff_large_wordlist.tsv");
    }

    let sep;
    if matches.is_present("delimiter") {
        sep = matches.value_of("delimiter").unwrap();
    } else {
        sep = "";
    }

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(path.as_path())
        .unwrap();

    for i in 1..(len + 1) {
        for _ in 1..6 {
            curr_num.push_str(((gen.next_u32() % 6) + 1).to_string().as_str());
        }

        for result in reader.records() {
            let record = result.unwrap();
            if record.get(0).unwrap() == &curr_num {
                pass.push_str(&record.get(1).unwrap());
                pass.push_str(&sep);
                println!("Word {}: {}", i, &record.get(1).unwrap());
            }
        }
        curr_num = String::from("");
        let _ = reader.seek(start.clone());
    }
    pass = pass.trim_right_matches(&sep).to_string();
    println!("Passphrase: {}", pass);
}
