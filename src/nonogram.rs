use serde::Deserialize;

use std::fs::File;
use std::io::BufReader;

pub mod solve;

#[derive(Deserialize)]
struct AllKeys {
    row: Vec<Vec<usize>>,
    column: Vec<Vec<usize>>,
}

impl AllKeys {
    fn new(name: &str) -> Self {
        let file = File::open(format!("keys/{}.json", name))
            .expect(format!("Cannot open keys/{}.json", name).as_str());

        let reader = BufReader::new(file);

        let mut all_keys: AllKeys =
            serde_json::from_reader(reader).expect("The JSON file has an unexpected structure");

        all_keys.remove0();

        all_keys
    }

    fn remove0(&mut self) {
        // check_keys関数に統合する
        for index in 0..self.row.len() {
            self.row[index].retain(|&x| x != 0);
        }
        for index in 0..self.column.len() {
            self.column[index].retain(|&x| x != 0);
        }
    }

    fn show(&self) {
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

#[derive(Clone, Copy)]
enum Color {
    Black,
    White,
}

impl Color {
    fn oposite(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

struct Grid {
    row_num: usize,
    column_num: usize,
    cells: Vec<Vec<Option<Color>>>,
}

impl Grid {
    fn new(row_num: usize, column_num: usize) -> Self {
        let cells;
        if row_num == 0 || column_num == 0 {
            panic!("Grid::new(): cannot generate a grid");
        } else {
            cells = vec![vec![Option::None; column_num]; row_num];
        }

        Self {
            row_num,
            column_num,
            cells,
        }
    }

    fn show(&self) {
        print!(" ");
        for _ in 0..self.column_num {
            print!("__");
        }
        println!("");

        for row_line in self.cells.iter() {
            print!("|");
            for cell in row_line.iter() {
                print!(
                    "{}",
                    match cell {
                        Option::None => "\x1b[100m  ",
                        Option::Some(color) => {
                            match color {
                                Color::Black => "\x1b[40m  ",
                                Color::White => "\x1b[107m  ",
                            }
                        }
                    }
                )
            }
            println!("\x1b[0m|");
        }

        print!(" ");
        for _ in 0..self.column_num {
            print!("‾‾");
        }
        println!("");
    }
}

pub struct Board {
    name: String,
    all_keys: AllKeys,
    grid: Grid,
}

impl Board {
    pub fn new(name: &str) -> Self {
        let all_keys = AllKeys::new(name);
        let grid = Grid::new(all_keys.row.len(), all_keys.column.len());
        let name = name.to_string();

        Self {
            name,
            all_keys,
            grid,
        }
    }

    pub fn show_all_keys(&self) {
        self.all_keys.show();
    }

    pub fn show_grid(&self) {
        self.grid.show();
    }
}
