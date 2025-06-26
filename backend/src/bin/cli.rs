use std::{io::{self, Write}, process::Command};

use hll::database;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() {
    let db = database::init_database().await
        .expect("Database connecton could not be made");

    let _ = database::search::update_spellfix_table(&db).await;
    
    loop {
        println!("
            ----- HLL interactive menu -----
            1) Launch Sqlite REPL
            2) Test Main Features
            3) Quit");

        print!("-> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            match input.trim() {
                "1" => launch_sqlite_repl(),
                "2" => test_main_features(&db).await,
                "3" => break,
                _ => println!("Please enter a valid option")
            };
        }
    }

}

async fn test_main_features(pool: &SqlitePool) {
    loop {
        println!("
            ----- Test Main Features -----
            1) Search for books
            2) Correct spelling with spellfix
            3) Back");

        print!("-> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            match input.trim() {
                "1" => println!("Search books"),
                "2" => match correct_spelling(pool).await {
                    Ok(()) => {},
                    Err(_) => println!("Something went wrong")
                },
                "3" => break,
                _ => println!("Please enter a valid option")
            };
        }
        
    }
}

async fn correct_spelling(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("
        ----- Correct spelling -----
            Quit with empty input");
    loop {
        print!("-> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            if input == "\n" {
                return Ok(());
            }
            let Some(candidates) = database::search::get_spelling_candidates(pool, &input, 5).await? else {
                println!("Not enough good results");
                continue;
            };
            println!("Spellfix =>");
            for (i, c) in candidates.enumerate() {
                println!("{}. {c}", i+1);
            }
        }
    }
}

fn launch_sqlite_repl() {
    let result = match Command::new("sqlite3")
        .arg("db/db.sqlite").arg("-cmd").arg(".load ./spellfix1")
        .status() {
            Ok(status) => match status.code() {
                Some(code) => format!("Exited with status code: {code}"),
                None => format!("REPL terminated by signal")
            },
            Err(_) => String::from("Failed to launch REPL")
        };
    println!("{result}");
}
