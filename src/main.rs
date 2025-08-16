#!/usr/bin/env rust

use std::time::{SystemTime, UNIX_EPOCH};

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
        "Don't explain your philosophy. Embody it. - Epictetus",
        "Attention(prosochê) is a continuous vigilance and presence of mind, self-consciousness which never sleeps. — Paraphrased from Epictetus’ Discourses, esp. 4.12, via modern Stoic interpretations.",
        "You have power over your mind — not outside events. Realize this, and you will find strength.— Marcus Aurelius, Meditations",
        "Difficulties strengthen the mind, as labor does the body. - Seneca",
        "He who is brave is free. - Seneca",
        "You've endured countless troubles - all from not letting your ruling reason so the work it was made for - enough already!- Marcus Aurelius, Meditations, 9.26 ",
        "Waste no more time arguing what a good man should be. Be one.” – Marcus Aurelius",
        "Man conquers the world by conquering himself. — Zeno of Citium",
        "Better to trip with the feet than with the tongue. - Zeno",
        "Know well what leads you forward and what holds you back, and choose the path that leads you to wisdom. - Buddha",
        "행복은 사물 그 자체가 아니라, 우리가 그것을 어떻게 받아들이느냐에 달려 있다. — 에픽테토스, 담화록",
        "Focus your energy on what is within your control. — Epictetus",
        "Direct your energy toward what you can control. — Epictetus",
    ];

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();
    let index = (now % stoic_quotes.len() as u128) as usize;
    let quote = stoic_quotes[index];

    println!("{}", quote);
}
