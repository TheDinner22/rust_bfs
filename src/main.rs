// this is for testing only making sure remote wo&rks
// use rust_bfs::*;

#[derive(Default)]
struct Maze {
    squares: Grid,
}

struct Grid {
    cells: Vec<Square>,
    width: usize,
    height: usize,
}

// 6 by three grid where only middle three squares are active
// 0  1  2  3  4  5
// 6  7  8  9  10 11
// 12 13 14 15 16 17
//
impl Default for Grid {
    fn default() -> Self {
        let cells = (0..18)
            .into_iter()
            .map(|num| {
                Square::new(num, !(7..=10).contains(&num))
            })
            .collect();

        Grid { cells, width: 6, height: 3 }
    }
}

impl Grid {
    fn area(&self) -> usize { self.width * self.height }

    fn move_as_delta_index(&self, moov: &Move) -> isize {
        match moov {
            Move::Up => self.width as isize,
            Move::Down => -(self.width as isize),
            Move::Left => -1,
            Move::Right => 1,
        }
    }

    fn attempt_move_from_cell(&self, cell_start_index: usize, moov: &Move) -> Option<usize> {
        if self.cells[cell_start_index].is_wall() { return None; }

        match moov { // todo make this not move to bad cells
            Move::Up => {
                if cell_start_index >= self.width {
                    // get new cells index
                    let new_index = (cell_start_index as isize + self.move_as_delta_index(moov)) as usize;
                    // make sure it is not a wall
                    if !self.cells[new_index].is_wall() {
                        Some(new_index)
                    }
                    else {
                        None
                    }
                }
                else {
                    None
                }
            },
            Move::Down => {
                if (self.area() - self.width) < cell_start_index {
                    // get new cells index
                    let new_index = (cell_start_index as isize + self.move_as_delta_index(moov)) as usize;
                    // make sure it is not a wall
                    if !self.cells[new_index].is_wall() {
                        Some(new_index)
                    }
                    else {
                        None
                    }
                }
                else {
                    None
                }
            },
            Move::Left => {
                if cell_start_index % self.width != 0 {
                    // get new cells index
                    let new_index = (cell_start_index as isize + self.move_as_delta_index(moov)) as usize;
                    // make sure it is not a wall
                    if !self.cells[new_index].is_wall() {
                        Some(new_index)
                    }
                    else {
                        None
                    }
                }
                else {
                    None
                }
            },
            Move::Right => {
                if (cell_start_index + 1) % self.width != 0 {
                    // get new cells index
                    let new_index = (cell_start_index as isize + self.move_as_delta_index(moov)) as usize;
                    // make sure it is not a wall
                    if !self.cells[new_index].is_wall() {
                        Some(new_index)
                    }
                    else {
                        None
                    }
                }
                else {
                    None
                }
            },
        }
    }
}

struct Square{
    id: i32,
    is_wall: bool,
}

impl Square {
    fn new(id:i32, is_wall: bool) -> Self {
        Square {
            id,
            is_wall,
        }
    }

    fn is_wall(&self) -> bool {
        self.is_wall
    }
}

enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn main(){
    let ez_maze = Maze::default();

    assert_eq!(ez_maze.squares.area(), 18);

    // failing moves
    assert_eq!(ez_maze.squares.attempt_move_from_cell(0, &Move::Up), None);
    assert_eq!(ez_maze.squares.attempt_move_from_cell(0, &Move::Down), None);
    assert_eq!(ez_maze.squares.attempt_move_from_cell(0, &Move::Right), None);
    assert_eq!(ez_maze.squares.attempt_move_from_cell(0, &Move::Left), None);
    assert_eq!(ez_maze.squares.attempt_move_from_cell(13, &Move::Up), None);
    assert_eq!(ez_maze.squares.attempt_move_from_cell(4, &Move::Down), None);
    assert_eq!(ez_maze.squares.attempt_move_from_cell(2, &Move::Left), None);
    assert_eq!(ez_maze.squares.attempt_move_from_cell(7, &Move::Left), None);
    assert_eq!(ez_maze.squares.attempt_move_from_cell(7, &Move::Up), None);
    assert_eq!(ez_maze.squares.attempt_move_from_cell(10, &Move::Right), None);

    // passing moves
    assert_eq!(ez_maze.squares.attempt_move_from_cell(7, &Move::Right), Some(8));
    assert_eq!(ez_maze.squares.attempt_move_from_cell(8, &Move::Left), Some(7));
    assert_eq!(ez_maze.squares.attempt_move_from_cell(8, &Move::Left), Some(7));
    assert_eq!(ez_maze.squares.attempt_move_from_cell(9, &Move::Right), Some(10));
    assert_eq!(ez_maze.squares.attempt_move_from_cell(9, &Move::Left), Some(8));
    assert_eq!(ez_maze.squares.attempt_move_from_cell(10, &Move::Left), Some(9));

    // using the id
    assert_eq!(0, ez_maze.squares.cells[0].id);
    assert_eq!(1, ez_maze.squares.cells[1].id);
    assert_eq!(2, ez_maze.squares.cells[2].id);
    assert_eq!(3, ez_maze.squares.cells[3].id);
    assert_eq!(4, ez_maze.squares.cells[4].id);
    assert_eq!(5, ez_maze.squares.cells[5].id);
    assert_eq!(6, ez_maze.squares.cells[6].id);
}
