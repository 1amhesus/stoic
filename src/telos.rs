// Telos module: 
// a tiny state machine for an "automata-agentic-workflow" mode.
//
// Purpose:
// This module defines a simple cyclic state machine that represents
// a user's cognitive/behavioral flow while working toward a goal (telos).
//
// Instead of randomness (like quote selection), this system provides
// structured, interval-based guidance inspired by Stoic discipline
// and Bitcoin-like 'Telos intervals'.
//
// Core Idea:
// Each "telos_height" acts like a block height.
// At each step, the system:
// 1. Shows the current state (reflection / guidance)
// 2. Transitions deterministically to the next state
//
// This is NOT random. It is a deterministic loop designed to:
// - re-orient attention
// - maintain focus
// - detect drift
// - recover from distraction
// - close cycles with completion
//
// // Telos State Machine (v0.1)
//
// Summary:
// This module implements a minimal deterministic state machine for the "telos" mode.
// Instead of random quote selection, it models a structured workflow cycle:
//
// Orient → Focus → Drift → Recover → Complete → (repeat)
//
// Each transition represents a conceptual "telos interval",
// inspired by Bitcoin's block time (~10 minutes),
// and actual intervals are synced with bitcoin full node
//
// The current implementation:
// - Uses `telos_height` as a simple counter (like block height)
// - Cycles through predefined states
// - Prints reflective guidance for each state
//
// Conceptual Direction:
// The goal is to evolve this into a system where each "telos_height"
// represents a real unit of time (Bitcoin-like interval),
// and each interval captures a meaningful self-defined state.
//
// TODO (Next Steps):
//
// 1. Redefine `telos_height`
//    → Treat it as a true "block interval index" (time-based, not just counter)
//
// 2. Introduce state recording
//    → Create a struct like `TelosRecord`
//    → Store (height, state, source)
//
// 3. Support two modes of state definition:
//    - Active: user explicitly defines their state
//    - Passive: system infers state based on behavior or rules
//
// 4. Replace simple cycling logic
//    → Move from fixed transitions to input-driven or rule-based transitions
//
// 5. Add interval timing
//    → Implement real block-like timing (e.g., 10 min loop)
//
// 6. Persist logs
//    → Store telos records (file or memory)
//    → Enable reflection over past intervals
//
// Long-term Vision:
// A CLI-based agentic stoic meditation workflow tool where:
// - time is structured into intervals
// - each interval has intentional meaning
// - the system helps maintain alignment with a chosen telos

#[derive(Debug)]
pub enum TelosState {
    Orient,
    Focus,
    Drift,
    Recover,
    Complete,
}

pub struct TelosMachine {
    pub state: TelosState,
    pub telos_height: u64,
}

impl TelosMachine {
    pub fn new() -> Self {
        Self {
            state: TelosState::Orient,
            telos_height: 0,
        }
    }

    pub fn show_status(&self) {
        println!("[Telos {}]", self.telos_height);

        match self.state {
            TelosState::Orient => {
                println!("State: Orient");
                println!("Ask: what is the essential task right now?");
            }
            TelosState::Focus => {
                println!("State: Focus");
                println!("Stay with the tiny meaningful unit of work.");
            }
            TelosState::Drift => {
                println!("State: Drift");
                println!("You are drifting. Notice it without judgment.");
            }
            TelosState::Recover => {
                println!("State: Recover");
                println!("Return by doing one concrete action now.");
            }
            TelosState::Complete => {
                println!("State: Complete");
                println!("Close this interval with a visible result.");
            }
        }
    }

    pub fn next(&mut self) {
        self.telos_height += 1;

        self.state = match self.state {
            TelosState::Orient => TelosState::Focus,
            TelosState::Focus => TelosState::Drift,
            TelosState::Drift => TelosState::Recover,
            TelosState::Recover => TelosState::Complete,
            TelosState::Complete => TelosState::Orient,
        };
    }
}


