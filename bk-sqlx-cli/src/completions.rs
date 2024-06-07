use std::io;

use clap::CommandFactory;
use clap_complete::{generate, Shell};

use crate::opt::Command;

pub fn run(shell: Shell) {
    generate(shell, &mut Command::command(), "bk_sqlx", &mut io::stdout())
}
