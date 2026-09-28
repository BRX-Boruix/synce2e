# synce2e

A BORUIX end-to-end acceptance test for the blocking wait and wake round-trip on sync words.

[简体中文](README.md)

## What it tests

This is the waiting side of a round-trip test: it calls the sync wait interface and **blocks for
real** — the process is suspended and switched away — until the coordinator wakes it with a value.
The program asserts that the value is `42`.

What it covers is a real context switch: blocking and waking require the process to be genuinely
suspended and resumed, a path kernel-space self-tests cannot reach.

## Usage

Not run standalone. The `synce2e` builtin of the shell spawns it as a child with the command line
`waiter:<id>`:

```
synce2e[waiter]: blocking on sync_wait
synce2e[waiter]: woke with value 42 OK
```

The coordinator creates the sync word, spawns this program, performs the wake, and collects this
process's exit code as the verdict.

## Exit codes

- `0` — woke with 42, round-trip succeeded
- `2` — malformed command line (not `waiter:<number>`)
- `3` — the wait call itself returned an error
- `4` — woken, but the value was not 42

## Building

```bash
cargo build --release
```

The binary deploys as `/programs/synce2e.elf`.

## Repository layout

```
synce2e/
├── Cargo.toml    # package manifest
├── build.rs      # injects the linker script
├── linker.ld     # user-space segment layout
└── src/
    └── main.rs   # argument parsing, blocking wait, verdict
```

## Related projects

- [`libsys`](https://github.com/BRX-Boruix/libsys) — the sync wait interface
- [`shell`](https://github.com/BRX-Boruix/shell) — coordinator: creates, spawns, wakes, collects

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
