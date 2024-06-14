use todo::*;
use clap::*;

fn main() {
    let cli = Cli::parse();
    parse_command(cli);
}
