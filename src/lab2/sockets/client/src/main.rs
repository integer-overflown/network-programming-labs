use std::error::Error;

use clap::Parser;

use server_utils::cli;

mod app;

fn run() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let args = app::Args::parse();
    let mut app = app::App::new(args)?;

    app.start_loop()?;

    Ok(())
}

fn main() {
    cli::exec(run);
}
