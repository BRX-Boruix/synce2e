# synce2e

An **end-to-end acceptance program** for BORUIX, verifying blocking wait and wake-up round trips on sync words.

[简体中文](README.md)

## What it tests

This is the **waiting side** of a round-trip test. It calls the sync wait interface and **genuinely blocks** (registering as a waiter in the kernel, with a real process switch), until the coordinating side wakes it and passes in a value. The program asserts the wake-up value is exactly `42`, failing honestly otherwise.

The critical path it verifies is **a real process context switch** — blocking and waking require the process to be genuinely suspended and resumed, which the in-kernel self-test cannot cover.

## Usage

Spawned as a child by the shell builtin `synce2e`, with the argument `waiter:<id>`, where `<id>` is the sync word number.

```
synce2e[waiter]: blocking on sync_wait
synce2e[waiter]: woke with value 42 OK
```

The coordinating side (in the shell) performs the wake and collects this process's exit code with `waitpid` as the verdict.

## Exit codes

| Exit code | Meaning |
| --- | --- |
| `0` | Woke with value 42; the round trip succeeded |
| `2` | Malformed argument (not `waiter:<id>`) |
| `3` | The wait call itself returned an error |
| `4` | Woken, but the value received was not 42 |

## Building

```bash
cargo build --release
```

The artifact is deployed as `/programs/synce2e.elf`.

## Layout

```
synce2e/
├── Cargo.toml    # package definition
├── build.rs      # injects the linker script
├── linker.ld     # user-space section layout
└── src/
    └── main.rs   # waiting-side logic and verdict
```

## Related projects

- [`libsys`](https://github.com/BRX-Boruix/libsys) — provides the sync wait interface
- [`shell`](https://github.com/BRX-Boruix/shell) — the coordinating side, spawning and waking it

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
