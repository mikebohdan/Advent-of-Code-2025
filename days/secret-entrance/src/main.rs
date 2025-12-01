use anyhow::Result;
use clap::Parser;

mod cli;
mod password;

fn main() -> Result<()> {
  let args = cli::Args::parse();

  match args.part {
    cli::Part::First => println!(
      "Password: {:?}",
      password::find_password(password::count_zeroes, args.file)?
    ),
    cli::Part::Second => println!(
      "Password: {:?}",
      password::find_password(password::count_zeroes_2, args.file)?
    ),
  }

  Ok(())
}
