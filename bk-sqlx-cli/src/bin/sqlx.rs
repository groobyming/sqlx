use clap::Parser;
use console::style;
use bk_sqlx_cli::Opt;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    // no special handling here
    if let Err(error) = bk_sqlx_cli::run(Opt::parse()).await {
        println!("{} {}", style("error:").bold().red(), error);
        std::process::exit(1);
    }
}
