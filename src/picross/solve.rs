use super::*;

enum SolveResult {
    NoAnswer,
    NoFixedAnswer,
    OnlyOneAnswer,
}

struct Position {
    row_index: usize,
    column_index: usize,
}

impl Position {
    fn new(row_index: usize, column_index: usize) -> Self {
        Self {
            row_index,
            column_index,
        }
    }
}

struct Layer {
    branch_position: Position,
    held_grid: Option<Grid>,
}

impl Layer {
    fn new(branch_position: Position) -> Self {
        Self {
            branch_position,
            held_grid: None,
        }
    }

    fn hold(&mut self, grid: Grid) {
        self.held_grid = Some(grid);
    }
}

struct LayerStack {
    vector: Vec<Layer>,
}

impl LayerStack {
    fn new() -> Self {
        Self { vector: Vec::new() }
    }

    fn get_num(&self) -> usize {
        self.vector.len()
    }
}

struct Solver<'a> {
    board_grid: &'a mut Grid,
    solver_grid: Grid,
    layer_stack: LayerStack,
}

impl<'a> Solver<'a> {
    pub fn new(board: &'a mut Board) -> Self {
        let solver_grid = Grid::new(board.grid.row_num, board.grid.column_num);
        let layer_stack = LayerStack::new();

        Self {
            board_grid: &mut board.grid,
            solver_grid,
            layer_stack,
        }
    }

    fn init(&mut self) {
        self.solver_grid = Grid::new(self.board_grid.row_num, self.board_grid.column_num);
    }

    fn is_base_layer(&self) -> bool {
        if self.layer_stack.get_num() == 0 {
            true
        } else {
            false
        }
    }

    fn is_changed(&self) -> bool {
        true
    }

    fn is_full_grid(&self) -> bool {
        true
    }

    fn is_full_grid_held(&self) -> bool {
        true
    }

    fn solve_direction(&mut self) -> bool {
        true
    }

    fn direction_switch(&mut self) {}

    fn peel_off(&mut self) {}

    fn remove_stacked_layers(&mut self) {}

    fn hold(&mut self) {}

    fn stack(&mut self) {}

    pub fn solve(&mut self) -> SolveResult {
        self.init();

        loop {
            if self.is_changed() {
                if self.solve_direction() {
                    self.direction_switch();
                } else {
                    if self.is_base_layer() {
                        self.init();
                        break SolveResult::NoAnswer;
                    } else {
                        self.peel_off();
                    }
                }
            } else {
                if self.is_full_grid() {
                    if self.is_base_layer() {
                        break SolveResult::OnlyOneAnswer;
                    } else {
                        if self.is_full_grid_held() {
                            self.remove_stacked_layers();
                            break SolveResult::NoFixedAnswer;
                        } else {
                            self.hold();
                        }
                    }
                } else {
                    self.stack();
                }
            }
        }
    }
}
