#!/usr/bin/env rust

use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let stoic_quotes = vec![
        "The mind is everything; what you think, you become. - Buddha",
        "Simplicity is the ultimate sophistication. - Leonardo da Vinci",
        "Be present above all else. - Naval Ravikant",
        "The obstacle is the way. - Marcus Aurelius",
        "Know yourself. - Socrates",
        "It's not what happens to you, but how you react to it that matters. - Epictetus",
        "The only true wisdom is in knowing you know nothing. - Socrates",
        "The mind that is anxious about future events is miserable. - Seneca",
    ];

    let mut rng = thread_rng();
    let quote = stoic_quotes.choose(&mut rng).unwrap();

    println!("{}", quote);
}
