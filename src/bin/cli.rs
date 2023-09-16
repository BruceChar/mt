use clap::{
    Arg, Command,
    Parser,
    Subcommand,
};
use mt::{
    gcd, gcd_iter,
};

fn main() {
    let args =
        Cli::parse(
        );
    println!(
        "{:?}",
        args
    );
    let re = gcd_iter(args.nums);
    println!("Hello, world! {}", re);
}

#[derive(
    Parser, Debug,
)]
#[command(
    about, version
)]
pub struct Cli {
    pub(crate) nums:
        Vec<u64>,

    #[command(
        subcommand
    )]
    command: Option<
        Commands,
    >,
}

#[derive(
    Subcommand,
    Debug,
)]
pub enum Commands {
    gcd,
}
