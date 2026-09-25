#!/usr/bin/env python3
from dataclasses import dataclass
from collections import deque

@dataclass(frozen=True)
class State:
    applied_mask: int = 0
    checkpoint: int = 0
    apply_count: int = 0


def apply_mutation(s: State, mid: int) -> State:
    bit = 1 << mid
    if s.applied_mask & bit:
        return s
    return State(s.applied_mask | bit, s.checkpoint, s.apply_count + 1)


def next_states(s: State):
    out = [apply_mutation(s, 0), apply_mutation(s, 1)]
    max_checkpoint = s.applied_mask.bit_count()
    if s.checkpoint < max_checkpoint:
        out.append(State(s.applied_mask, s.checkpoint + 1, s.apply_count))
    return out


def check_transition(a: State, b: State):
    assert b.checkpoint >= a.checkpoint, "checkpoint regressed"
    assert b.apply_count >= a.apply_count, "applied mutation count regressed"
    assert b.apply_count == b.applied_mask.bit_count(), "duplicate mutation counted twice"
    assert b.checkpoint <= b.apply_count, "checkpoint advanced beyond applied mutations"


def main():
    start = State(); q = deque([start]); seen = {start}; edges = 0
    while q:
        s = q.popleft()
        assert s.apply_count == s.applied_mask.bit_count()
        assert s.checkpoint <= s.apply_count
        for n in next_states(s):
            edges += 1; check_transition(s, n)
            if n not in seen:
                seen.add(n); q.append(n)
    assert State(3, 2, 2) in seen
    print(f"mutation/checkpoint model: {len(seen)} states, {edges} transitions")

if __name__ == "__main__":
    main()
