use std::{cell::RefCell, rc::Rc};

use super::*;
use fnv::FnvHashMap;

enum SolveResult {
    NoAnswer,
    NoFixedAnswer,
    OnlyOneAnswer,
}

enum Direction {
    Row,
    Column,
}

struct LineId {
    direction: Direction,
    index: usize,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct CellId {
    row_index: usize,
    column_index: usize,
}

struct Layer {
    is_full_grid: bool,
    grid: FnvHashMap<CellId, Color>,
    parent: Option<Rc<RefCell<Layer>>>,
    opposite: Option<Rc<RefCell<Layer>>>,
    main_child: Option<Rc<RefCell<Layer>>>,
    sub_child: Option<Rc<RefCell<Layer>>>,
}

impl Layer {
    fn new(parent: Option<Rc<RefCell<Layer>>>) -> Self {
        let grid = FnvHashMap::default();

        Self {
            is_full_grid: false,
            grid,
            parent,
            opposite: None,
            main_child: None,
            sub_child: None,
        }
    }

    fn insert(&mut self, cell_id: CellId, color: Color) {
        self.grid.insert(cell_id, color);
    }
}

struct LayeredGrid {
    root_layer: Rc<RefCell<Layer>>,
    current_layer: Rc<RefCell<Layer>>,
}

impl LayeredGrid {
    fn new() -> Self {
        let root_layer = Rc::new(RefCell::new(Layer::new(None)));
        let current_layer = Rc::clone(&root_layer);

        Self {
            root_layer,
            current_layer,
        }
    }

    fn get_main_cell(&self, cell_id: CellId) -> Option<Color> {
        let mut tmp_layer = Rc::clone(&self.root_layer);

        loop {
            let next_layer;

            match tmp_layer.borrow().grid.get(&cell_id) {
                Some(color) => break Some(*color),
                None => match &tmp_layer.borrow().main_child {
                    Some(layer) => next_layer = Rc::clone(layer),
                    None => break None,
                },
            }

            tmp_layer = next_layer;
        }
    }

    fn coloring(&mut self, cell_id: CellId, color: Color) {
        self.current_layer.borrow_mut().insert(cell_id, color);
    }

    fn stack_layer(&mut self, cell_id: CellId, color: Color) {
        let mut main_layer = Layer::new(Some(Rc::clone(&self.current_layer)));
        main_layer.insert(cell_id, color);
        let main_layer = Rc::new(RefCell::new(main_layer));

        let mut sub_layer = Layer::new(Some(Rc::clone(&self.current_layer)));
        sub_layer.insert(cell_id, color.oposite());
        let sub_layer = Rc::new(RefCell::new(sub_layer));

        main_layer.borrow_mut().opposite = Some(Rc::clone(&sub_layer));
        sub_layer.borrow_mut().opposite = Some(Rc::clone(&main_layer));

        self.current_layer.borrow_mut().main_child = Some(Rc::clone(&main_layer));
        self.current_layer.borrow_mut().sub_child = Some(Rc::clone(&sub_layer));

        self.current_layer = Rc::clone(&main_layer);
    }

    fn switch_layer(&mut self) {
        let opposite_layer;

        match &self.current_layer.borrow().opposite {
            Some(layer) => opposite_layer = Rc::clone(layer),
            None => panic!("Solver::switch_layer(): currnet_layer doesn't have opposite_layer"),
        }

        self.current_layer.swap(&opposite_layer);
    }

    fn apply_layer(&mut self) {
        let parent_layer;

        match &self.current_layer.borrow().parent {
            Some(layer) => parent_layer = Rc::clone(layer),
            None => panic!("Solver::apply_layer(): current_layer doesn't have opposite_layer"),
        }

        parent_layer
            .borrow_mut()
            .grid
            .extend(self.current_layer.borrow().grid.iter());

        parent_layer.borrow_mut().main_child = match &self.current_layer.borrow().main_child {
            Some(layer) => Some(Rc::clone(layer)),
            None => None,
        };

        parent_layer.borrow_mut().sub_child = match &self.current_layer.borrow().sub_child {
            Some(layer) => Some(Rc::clone(layer)),
            None => None,
        };

        parent_layer.borrow_mut().is_full_grid = self.current_layer.borrow().is_full_grid;

        self.current_layer = Rc::clone(&parent_layer);
    }
}

struct Solver<'a> {
    board_grid: &'a mut Grid,
    layered_grid: LayeredGrid,
}

impl<'a> Solver<'a> {
    pub fn new(board: &'a mut Board) -> Self {
        let layered_grid = LayeredGrid::new();

        Self {
            board_grid: &mut board.grid,
            layered_grid,
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

    fn apply_line(&mut self, line_id: LineId) -> bool {
        true
    }

    fn stack_layer(&mut self) -> bool {
        true
    }

    fn priority_pop(&mut self) -> Option<LineId> {
        Some(LineId {
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
