use super::*;

enum SolveResult {
    NoAnswer,
    NoFixedAnswer,
    OnlyOneAnswer,
}

enum Direction {
    Row,
    Column,
}

struct LineID {
    direction: Direction,
    index: usize,
}

struct Solver<'a> {
    board_grid: &'a mut Grid,
}

impl<'a> Solver<'a> {
    pub fn new(board: &'a mut Board) -> Self {
        Self {
            board_grid: &mut board.grid,
        }
    }
}

impl Solver<'_> {
    fn init(&mut self) {}

    fn is_base_layer(&self) -> bool {
        true
    }

    fn has_full_grid(&self) -> bool {
        true
    }

    fn apply_line(&mut self, line_id: LineID) -> bool {
        true
    }

    fn stack_layer(&mut self) -> bool {
        true
    }

    fn priority_pop(&mut self) -> Option<LineID> {
        Some(LineID {
            direction: Direction::Row,
            index: 0,
        })
    }

    fn apply_opposite_layer(&mut self) {}

    fn remove_stacked_layers(&mut self) {}

    fn set_full_grid(&mut self) {}

    fn switch_layer(&mut self) {}

    pub fn solve(&mut self) -> SolveResult {
        self.init();
        loop {
            if let Some(line_id) = self.priority_pop() {
                if !self.apply_line(line_id) {
                    if self.is_base_layer() {
                        self.init();
                        break SolveResult::NoAnswer;
                    } else {
                        self.apply_opposite_layer();
                    }
                }
            } else {
                if !self.stack_layer() {
                    if self.is_base_layer() {
                        break SolveResult::OnlyOneAnswer;
                    } else {
                        if self.has_full_grid() {
                            self.remove_stacked_layers();
                            break SolveResult::NoFixedAnswer;
                        } else {
                            self.set_full_grid();
                            self.switch_layer();
                        }
                    }
                }
            }
        }
    }
}
