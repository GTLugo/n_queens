use std::{collections::HashMap, ops::Range};

mod test;

// MAIN //
fn main() {
  println!("{}", solve_n_queens(8, (3, 2)).unwrap_or("Unsolveable".to_string()));
}
// //// //

#[derive(Debug)]
struct Board {
  size: usize,
  data: HashMap<Position, Status>,
}

impl Board {
  fn new(size: usize) -> Self {
    Board {
      size,
      data: Default::default(),
    }
  }

  fn is_valid(&self, pos: Position) -> bool {
    // vertical
    for y in 0..self.size {
      if self.data.contains_key(&Position { x: pos.x, y }) {
        return false;
      }
    }

    // horizontal
    for x in 0..self.size {
      if self.data.contains_key(&Position { x, y: pos.y }) {
        return false;
      }
    }

    // diagonals
    macro_rules! check {
      ($pos:ident, $x_func:ident, $y_func:ident) => {{
        let mut dpos = $pos;
        while dpos.is_in_bounds(0..self.size) {
          if self.data.contains_key(&dpos) {
            return false;
          }

          match dpos.x.$x_func(1) {
            Some(r) => dpos.x = r,
            None => break,
          }

          match dpos.y.$y_func(1) {
            Some(r) => dpos.y = r,
            None => break,
          }
        }
      }};
    }
    check!(pos, checked_add, checked_add);
    check!(pos, checked_add, checked_sub);
    check!(pos, checked_sub, checked_add);
    check!(pos, checked_sub, checked_sub);

    true
  }

  // check contested cells before and after
  // place queen
  // set contested cells
  fn update_square(&mut self, pos: Position, new_status: Status) -> Result<(), ()> {
    if !pos.is_in_bounds(0..self.size) || !self.is_valid(pos) {
      return Err(());
    }

    match self.data.get_mut(&pos) {
      Some(current_status) => {
        if let (Status::Queen, Status::Queen)
        | (Status::Queen, Status::Contested)
        | (Status::Contested, Status::Queen) = (current_status, new_status)
        {
          return Err(());
        }
      }
      None => {
        self.data.insert(pos, new_status);
      }
    }

    Ok(())
  }

  fn evaluate(&mut self) -> Option<String> {
    for _ in 0..self.size {}
    Some(self.to_string())
  }
}

impl std::fmt::Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = positions(self.size)
      .map(|p| match self.data.get(&p) {
        Some(Status::Queen) => "Q",
        _ => ".",
      })
      .enumerate()
      .fold(String::new(), |s, (i, c)| format!("{s}{}{c}", if i > 0 && i % self.size == 0 { "\n" } else { "" }));
    write!(f, "{s}")
  }
}

fn positions(size: usize) -> impl Iterator<Item = Position> {
  (0..size).flat_map(move |y| (0..size).map(move |x| Position::from((x, y))))
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
  x: usize,
  y: usize,
}

impl Position {
  fn is_in_bounds(&self, bounds: Range<usize>) -> bool {
    bounds.contains(&self.x) && bounds.contains(&self.y)
  }
}

impl From<(usize, usize)> for Position {
  fn from((x, y): (usize, usize)) -> Self {
    Self { x, y }
  }
}

#[allow(unused)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status {
  Queen,
  Contested,
}

pub fn solve_n_queens(n: usize, mandatory_coords: (usize, usize)) -> Option<String> {
  let mut board = Board::new(n);
  board.update_square(mandatory_coords.into(), Status::Queen).ok()?;
  board.evaluate()
}
