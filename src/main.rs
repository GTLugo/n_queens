use self::queens::solve_n_queens;

mod queens;
mod test;

fn main() {
  println!("{}", solve_n_queens(8, (3, 2)).unwrap_or("Unsolveable".to_string()));
}
