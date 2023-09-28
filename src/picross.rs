use serde::Deserialize;

use std::fs::File;
use std::io::BufReader;

pub mod solve;

#[derive(Deserialize)]
pub struct AllKeys {
    row: Vec<Vec<usize>>,
    column: Vec<Vec<usize>>,
}

impl AllKeys {
    fn new(name: &str) -> Self {
        let file = File::open(format!("keys/{}.json", name))
            .expect(format!("Cannot open keys/{}.json", name).as_str());

        let reader = BufReader::new(file);

        serde_json::from_reader(reader).expect("The JSON file has an unexpected structure")
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
enum CellState {
    Gray,
    Black(usize),
    White(usize),
}

struct Grid {
    row_num: usize,
    column_num: usize,
    cells: Vec<Vec<CellState>>,
}

impl Grid {
    fn new(row_num: usize, column_num: usize) -> Self {
        let cells;
        if row_num == 0 || column_num == 0 {
            panic!("Cannot generate a grid");
        } else {
            cells = vec![vec![CellState::Gray; column_num]; row_num];
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
                        CellState::Gray => "\x1b[100m  ",
                        CellState::Black(_) => "\x1b[40m  ",
                        CellState::White(_) => "\x1b[107m  ",
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
