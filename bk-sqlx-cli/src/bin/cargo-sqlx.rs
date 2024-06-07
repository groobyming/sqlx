use clap::Parser;
use console::style;
use bk_sqlx_cli::Opt;
use std::process;

// cargo invokes this binary as `cargo-bk_sqlx bk_sqlx <args>`
// so the parser below is defined with that in mind
#[derive(Parser, Debug)]
#[clap(bin_name = "cargo")]
enum Cli {
    Sqlx(Opt),
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let Cli::Sqlx(opt) = Cli::parse();

    if let Err(error) = bk_sqlx_cli::run(opt).await {
        println!("{} {}", style("error:").bold().red(), error);
        process::exit(1);
    }
}
