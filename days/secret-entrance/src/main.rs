use anyhow::Result;
use cli_app;

mod password;

fn main() -> Result<()> { cli_app::run(App {}) }

#[derive(Clone, Copy)]
struct App {}

impl cli_app::App for App {
  type Input = Box<dyn Iterator<Item = password::Rotation>>;
  type Output = usize;

  fn parse_input(self, buf: std::io::BufReader<std::fs::File>) -> Result<Self::Input> {
    Ok(Box::new(password::parse_rotations(buf)?))
  }

  fn solve_part_one(self, input: Self::Input) -> Result<Self::Output> { Ok(password::count_zeroes(input)) }

  fn solve_part_two(self, input: Self::Input) -> Result<Self::Output> { Ok(password::count_zeroes_2(input)) }
}
