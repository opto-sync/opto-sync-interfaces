# Opto Sync interfaces instructions

- Keep this repository declaration-only. Merge algorithms, validators,
  transports, persistence, schedulers, and telemetry exporters belong in their
  implementation repositories.
- Preserve `provenance.json` and the byte-identical canonical schema. Contract
  changes must record an immutable source revision and classify compatibility.
- Keep Rust, TypeScript, Dart, Kotlin, Swift, Java, and C-facing declarations
  synchronized before changing a public wire name or enum value.
- Never include credentials, record payload fixtures from real users, database
  URLs, or production topology.
- Run `python3 scripts/verify_contracts.py`, Rust formatting/tests/Clippy, and
  every locally available language syntax check before pushing.

## Repository-local Git worktrees

- Create or use a Git worktree only when the human operator explicitly authorizes it for the current task. Concurrency or a dirty checkout is not permission by itself.
- Put every authorized worktree at `<repository-root>/tmp/worktrees/<name>`; from the repository root, use `./tmp/worktrees/<name>`. Never place worktrees beside repositories or organization directories.
- Keep `tmp`, `temp`, `tmp/worktrees`, and `temp/worktrees` ignored in the repository-root `.gitignore`. Do not commit files from those directories.
- Relocate or remove a worktree only when the operator explicitly requests it. Before removal, preserve and publish intended changes, verify its commit is represented on the target branch, and confirm there are no tracked, untracked, ignored-sensitive, or in-use files that must survive. Remove it with `git worktree remove <path>` without `--force`; never delete a worktree directory with `rm`.
