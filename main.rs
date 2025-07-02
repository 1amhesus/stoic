#!/usr/bin/env rust

use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::OpenOptions;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--refection".to_string()) {
        println!("How are you feeling today?");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("refection.txt")
            .expect("Unable to open file");
        if let Err(e) = writeln!(file, "{}", answer.trim()) {
            eprintln!("Could not write to file: {}", e);
        } else {
            println!("Reflection saved. Here's a quote for you:");
        }
    }

    let stoic_quotes = vec![
        "The mind is everything; what you think, you become. - Buddha",
        "Simplicity is the ultimate sophistication. - Leonardo da Vinci",
        "Be present above all else. - Naval Ravikant",
        "The obstacle is the way. - Marcus Aurelius",
        "Know yourself. - Socrates",
        "It's not what happens to you, but how you react to it that matters. - Epictetus",
        "The only true wisdom is in knowing you know nothing. - Socrates",
        "The mind that is anxious about future events is miserable. - Seneca",
        "Don't explain your philosophy. Embody it. - Epictetus",
        "He who is brave is free. - Seneca",
    ];

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();
    let index = (now % stoic_quotes.len() as u128) as usize;
    let quote = stoic_quotes[index];

    println!("{}", quote);
}
