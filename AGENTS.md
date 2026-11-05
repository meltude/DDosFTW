# DDosFTW

Fake "hacking" CLI for trolling: animated, entirely fictional output. No network access, no real lookups, no data collection.

## Commands (repo root)

- `make build` — `cargo build --release`, then copies the binary to `bin/main`
- `make run` — `cargo run` (debug build)
- `make clean` — remove `bin/` and `target/`
- `nix develop` — flake dev shell (rustc/cargo/rust-analyzer/make) that runs `make build` on entry
- `nix build` — flake package build (`packages.default`), result at `./result/bin/DDosFTW`
- If `cargo` is not on PATH: `nix develop --command cargo build --release`

## Layout

- Single binary crate `DDosFTW` (Rust 2021), **no dependencies** by design.
- `flake.nix` uses flake-parts with `perSystem` for x86_64/aarch64 Linux and aarch64 Darwin; inputs are pinned in `flake.lock`. `packages.default` gets its name/version from `Cargo.toml` and the lock from `Cargo.lock`.
- `src/main.rs` — interactive mode loop: `1` domain, `2` account, `3` fake de-anonymisation.
- `src/hack_sequence.rs` — shared `Step`; steps/delays in `hack_sequence/sequence.rs`; fake data + xorshift PRNG in `hack_sequence/random.rs`.
- `src/logo.rs`, `src/style.rs` — ASCII art and ANSI colors (auto-disabled for non-TTY, `NO_COLOR`, or `TERM=dumb`).

## Verification

- No tests, CI, lint, or formatter config exists; do not run unconfigured test/lint commands.
- Always pipe input when smoke-testing: the program is an interactive loop and each step sleeps ~0.3–2.5 s (a full run takes 10–20 s):
  - `printf '2\nGitHub\nuser@example.com\nq\n' | make run` (account mode)
  - `printf '3\nsomealias\nq\n' | make run` (de-anonymisation mode)
  - `printf 'q\n' | make run` (immediate quit)
- Any mode other than `2` or `3` falls through to domain mode, including mistyped values.

## Gotchas

- `bin/` (Makefile output) and `target/` (Cargo) are gitignored; `AGENTS.md` is also gitignored in this repo.
- The dev shell sets `NIX_ENFORCE_PURITY=0`; without it the shellHook's `make build` fails to link into `./target` (`impure path ... used in link`).
- Flake inputs are pinned by `flake.lock`; refresh with `nix flake update`.
- Nix flakes only see Git-tracked files: `git add -N <file>` new files used by the flake before building, or `nix build` fails with missing paths.
