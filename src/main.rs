use clap::Parser;
use color_eyre::Result;
use rusqlite::Connection;

mod cli;
mod model;
mod queries;

fn main() -> Result<()> {
    let conn = Connection::open(queries::get_data_path())?;
    conn.execute("PRAGMA foreign_keys = ON;", ())?;

    _ = queries::create_tables(&conn);

    let cli = cli::Cli::parse();
    cli.run(&conn)?;

    Ok(())
}
