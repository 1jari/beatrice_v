use colored::Colorize;
use serde_json::Value;
use std::env;
use std::fs;
use std::str::FromStr;

mod beatrice_v;

fn main() {
    println!(
        "{}",
        "============================================================".blue()
    );
    println!(
        " ✨ {} - {}",
        " Beatrice V".purple(),
        "https://github.com/Lunoversis/beatrice-v"
            .green()
            .underline()
    );
    println!(
        "{}",
        "============================================================".blue()
    );
    let arg = match env::args().nth(1) {
        Some(a) => a,
        None => {
            eprintln!(
                "{} 🚨 {}: {} {}",
                "==>".red().bold(),
                "Usage".red().bold().underline(),
                "beatrice_v".green().bold(),
                "<project/directory>".underline()
            );
            return;
        }
    };

    let beatrice = beatrice_v::Session::spawn(arg);
}
