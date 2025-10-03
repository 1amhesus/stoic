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
        "Some things are up to us, and some are not. Up to us are our opinions, desires, and aversions—in short, whatever is our own doing. Not up to us are the body, property, reputation, and office—in short, whatever is not our own doing. — Epictetus, Enchiridion 1",
        "우리는 짧은 시간이 아니라, 많은 시간을 낭비하고 있는 것이다. 인생은 충분히 길며, 올바르게 사용한다면 가장 위대한 업적을 이룰 수 있을 만큼 넉넉하다. — 세네카, 인생의 짧음에 대하여 (De Brevitate Vitae)",
        "You have power over your mind – not outside events. Realize this, and you will find strength. - Marcus Aurelius, Meditations",
        "The present moment is all that belongs to us; the rest is not ours. — Marcus Aurelius, Meditations",
        "Don’t hope that events will turn out the way you want, welcome events in whichever way they happen: this is the path to peace. — Epictetus",
        "First say to yourself what you would be;and then do what you have to do. — Epictetus, Discourses II.23.1",
        "Freedom is practiced not in great things but in small ones. - Epictetus",
        "Wealth consists not in having great possessions, but in having few wants. - Epictetus",
        "He suffers more than necessary, who suffers before it is necessary. — Seneca",
        "Never let the future disturb you. You will meet it, if you have to, with the same weapons of reason which today arm you against the present.— Marcus Aurelius, Meditations",
        ];

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();
    let index = (now % stoic_quotes.len() as u128) as usize;
    let quote = stoic_quotes[index];

    println!("{}", quote);
}

/* Shaking Strategy vs. Bitcoin Entropy Use – Comparative Summary */

/*

1. Bit‑Shift + XOR (bit shifts + XOR mixing)

   → Strategy: Mix different bit ranges of a timestamp (e.g. `now`) using right-shift and XOR to avoid repeated output when only the lower bits change.

   - Bitcoin Similarity:

     · In Bitcoin Core’s RNG design discussion (#14623), the “fast seed” phase suggests mixing fast-moving entropy like
       stack pointers and high-resolution timestamps. These values are often manipulated via shifts/XORs.
       Source: https://github.com/bitcoin/bitcoin/issues/14623

        -> Need to Fact-Check : [ ]

     · The answer on “How Bitcoin Core generates entropy” also mentions mixing multiple fast inputs
       (cycle counters, stack positions, etc.) in a way similar to bit-mixing.
       Source: https://bitcoin.stackexchange.com/questions/112218

*/

/*

2. LCG-Style Math (Linear Congruential Generator)

   → Strategy: Multiply and add constants to shake a numeric seed (e.g., `now`), using modulo to create controlled variability.

    - Bitcoin Relevance:

     · No explicit LCG-style math is found in Bitcoin Core as part of randomness generation.
       Instead, hash-based mixing (e.g., SHA512) is preferred for high-entropy use cases.

     · The RNG design issue mentions mixing entropy from OS sources and applying cryptographic hashes.
       This gives better unpredictability than LCGs.

*/

/*

3. Rotate + XOR (bit rotations + XOR)

   → Strategy: Rotate bits left and right before applying XOR to generate more diverse output than just shifting alone.

   - Bitcoin Relevance:

     · While not explicitly showing rotate operations in code, discussions on entropy collection (see #14623)
       and randomness generation imply that advanced bit manipulation is used at low levels.

        -> Need to Fact-Check : [ ]

     · Especially when mixing time, stack, and process-based entropy, rotating + XOR patterns are conceptually aligned.

*/

/*
4. Combining System Sources (time, PID, OS info, memory, etc.)

   → Strategy: Mix multiple entropy sources like timestamps, process IDs, memory info, etc., instead of relying on one value.

   - Bitcoin Similarity:

     · The Bitcoin Core HD wallet seed generation uses a combination of OS entropy (getrandom/urandom),
       CPU instructions (RDRAND/RDSEED), and environmental noise (heap/stack addresses, sleep durations).
       These sources are periodically mixed together.
        
        -> Need To Fact-Check : [ ]

     · The RNG design #14623 mentions “fast”, “medium”, and “slow” seeding layers – each adding entropy
       from different sources including system performance counters and application events.

        -> Need To Fact-Check : [ ]

*/

/*
== Summary Analysis =================================================================================================================

- Bitcoin prefers robust entropy sources and cryptographic mixing (e.g., SHA512)rather than basic numeric transformations like LCGs.?

- For a CLI use case (`stoic-cli` tool), strategies 1 (Bit‑Shift + XOR) or 4 (System Sources) provide a good balance of simplicity 

  and perceptual randomness without heavy dependencies.

- If cryptographic unpredictability or adversary safety is required, follow Bitcoin’s thing:

  using multiple entropy sources and secure hashing or OS-provided RNG.

=========================================================================================================================
*/

