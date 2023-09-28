use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::BufReader;

pub mod solve;

#[derive(Serialize, Deserialize)]
pub struct AllKeys {
    row: Vec<Vec<usize>>,
    column: Vec<Vec<usize>>,
}

impl AllKeys {
    pub fn new(name: &str) -> Self {
        let file = File::open(format!("keys/{}.json", name)).unwrap();
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).unwrap()
    }

    pub fn show(&self) {
        println!("row");
        for keys in self.row.iter() {
            for key in keys.iter() {
                print!("{} ", key);
            }
            println!("");
        }
        println!("");

        println!("column");
        for keys in self.column.iter() {
            for key in keys.iter() {
                print!("{} ", key);
            }
            println!("");
        }
    }
}
