use chrono::Utc;
use clap::{Parser, Subcommand};
use color_eyre::Result;
use rusqlite::Connection;

mod model;

use crate::model::{
    NewExpense, NewPayment, add_expense, add_payment, create_tables, get_data_path, get_entries,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
}

fn main() -> Result<()> {
    let conn = Connection::open(get_data_path())?;
    _ = create_tables(&conn);

    _ = add_expense(
        &conn,
        &NewExpense {
            created_at: Utc::now(),
            name: "Test".to_string(),
            periodicity: model::Periodicity::Monthly,
            due_date_reference: Utc::now(),
        },
    );
    _ = add_payment(
        &conn,
        &NewPayment {
            created_at: Utc::now(),
            expense_name: "Test".to_string(),
            paid_at: Utc::now(),
        },
    );
    _ = add_payment(
        &conn,
        &NewPayment {
            created_at: Utc::now(),
            expense_name: "Test".to_string(),
            paid_at: Utc::now(),
        },
    );

    let cli = Cli::parse();
    match &cli.command {
        Commands::List => {
            for expense in get_entries(&conn).unwrap() {
                println!("expense: {expense:?}");
            }
        }
    }

    Ok(())
}
