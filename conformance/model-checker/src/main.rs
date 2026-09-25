use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    applied_mask: u8,
    checkpoint: u8,
    apply_count: u8,
}

#[derive(Clone, Copy, Debug)]
enum Behavior {
    Correct,
    DuplicateCountsTwice,
    CheckpointLeapsAhead,
}

#[derive(Debug)]
struct Stats {
    states: usize,
    transitions: usize,
}

fn apply_mutation(state: State, mutation_id: u8, behavior: Behavior) -> State {
    let bit = 1_u8 << mutation_id;
    if state.applied_mask & bit != 0 {
        return match behavior {
            Behavior::DuplicateCountsTwice => State {
                apply_count: state.apply_count + 1,
                ..state
            },
            _ => state,
        };
    }
    State {
        applied_mask: state.applied_mask | bit,
        apply_count: state.apply_count + 1,
        ..state
    }
}

fn next_states(state: State, behavior: Behavior) -> Vec<State> {
    let mut out = vec![
        apply_mutation(state, 0, behavior),
        apply_mutation(state, 1, behavior),
    ];
    let applied = state.applied_mask.count_ones() as u8;
    if state.checkpoint < applied {
        out.push(State {
            checkpoint: state.checkpoint + 1,
            ..state
        });
    }
    if matches!(behavior, Behavior::CheckpointLeapsAhead) && state.checkpoint <= applied {
        out.push(State {
            checkpoint: applied + 1,
            ..state
        });
    }
    out
}

fn validate_transition(previous: State, next: State) -> Result<(), String> {
    if next.checkpoint < previous.checkpoint {
        return Err("checkpoint regressed".into());
    }
    if next.apply_count < previous.apply_count {
        return Err("applied mutation count regressed".into());
    }
    if next.apply_count != next.applied_mask.count_ones() as u8 {
        return Err("duplicate mutation counted twice".into());
    }
    if next.checkpoint > next.apply_count {
        return Err("checkpoint advanced beyond applied mutations".into());
    }
    Ok(())
}

fn check_model(behavior: Behavior) -> Result<Stats, String> {
    let start = State {
        applied_mask: 0,
        checkpoint: 0,
        apply_count: 0,
    };
    let mut queue = VecDeque::from([start]);
    let mut seen = HashSet::from([start]);
    let mut transitions = 0;

    while let Some(state) = queue.pop_front() {
        if state.apply_count != state.applied_mask.count_ones() as u8 {
            return Err("state counted duplicate application".into());
        }
        if state.checkpoint > state.apply_count {
            return Err("state checkpoint exceeds applied count".into());
        }
        for next in next_states(state, behavior) {
            transitions += 1;
            validate_transition(state, next)?;
            if seen.insert(next) {
                queue.push_back(next);
            }
        }
    }

    let terminal = State {
        applied_mask: 0b11,
        checkpoint: 2,
        apply_count: 2,
    };
    if !seen.contains(&terminal) {
        return Err("full two-mutation terminal state was not explored".into());
    }

    Ok(Stats {
        states: seen.len(),
        transitions,
    })
}

fn require_rejected(label: &str, behavior: Behavior) -> Result<(), String> {
    match check_model(behavior) {
        Ok(stats) => Err(format!(
            "negative control {label} unexpectedly passed ({} states, {} transitions)",
            stats.states, stats.transitions
        )),
        Err(_) => Ok(()),
    }
}

fn main() -> Result<(), String> {
    let stats = check_model(Behavior::Correct)?;
    require_rejected("duplicate-counts-twice", Behavior::DuplicateCountsTwice)?;
    require_rejected("checkpoint-leaps-ahead", Behavior::CheckpointLeapsAhead)?;
    println!(
        "mutation/checkpoint model: {} states, {} transitions; negative controls rejected",
        stats.states, stats.transitions
    );
    Ok(())
}
