use clap::Parser;
use color_eyre::Result;
use rusqlite::Connection;

mod cli;
mod model;

fn main() -> Result<()> {
    let conn = Connection::open(model::get_data_path())?;
    conn.execute("PRAGMA foreign_keys = ON;", ())?;

    _ = model::create_tables(&conn);

    let cli = cli::Cli::parse();
    cli.run(&conn)?;

    Ok(())
}
