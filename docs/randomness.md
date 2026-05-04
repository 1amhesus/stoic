/* Shaking Strategy vs. Bitcoin Entropy Use – Comparative Summary */

/*

1. Bit‑Shift + XOR (bit shifts + XOR mixing)

   → Strategy: Mix different bit ranges of a timestamp (e.g. `now`) using right-shift and XOR to avoid repeated output when only the lower bits change.

   - Bitcoin Similarity:

     · In Bitcoin Core’s RNG design discussion (#14623), the “fast seed” phase suggests mixing fast-moving entropy like
       stack pointers and high-resolution timestamps. These values are often manipulated via shifts/XORs.
       Source: https://github.com/bitcoin/bitcoin/issues/14623

        -> Need to Fact-Check : [x] "fast" seeding is invoked by GetRandBytes and mixes in stack pointer + high-precision timestamp (merged #14955, src/random.h)

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
