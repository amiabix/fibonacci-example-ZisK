# ZisK Fibonacci Example

A Fibonacci sequence calculator running inside the [ZisK](https://github.com/0xPolygonHermez/zisk) zkVM. The guest computes Fibonacci(n) and the host generates a zero-knowledge proof of that computation.

## What It Does

1. Sends n=10 as input to the guest
2. Guest computes Fibonacci(10) = 55 inside the ZisK VM
3. Generates a VADCOP STARK proof of the computation
4. Verifies the proof

## Project Structure

```
fibonacci/
├── Cargo.toml              # Workspace
├── guest/
│   ├── Cargo.toml
│   └── src/main.rs         # Runs inside zkVM: computes Fibonacci(n)
├── host/
│   ├── Cargo.toml
│   ├── build.rs            # Cross-compiles guest to RISC-V ELF
│   └── src/main.rs         # Sends input, proves, verifies
└── lib/
    ├── Cargo.toml
    └── src/lib.rs           # Shared types (FibResult)
```

## Prerequisites

- [ZisK v0.16.0](https://github.com/0xPolygonHermez/zisk) installed via `ziskup`
- Linux x86_64 (required for proof generation)
- 16 GB RAM minimum, 64 GB recommended

## Run

```bash
cargo run --release
```

```
[1/3] Executing...
      Fibonacci(10) = 55
      Cycles: 10645
[2/3] Generating proof...
[3/3] Verifying...
      Proven: Fibonacci(10) = 55
      Proof verified!
```

## How It Works

### Guest (`guest/src/main.rs`)

Runs inside the ZisK VM:

1. Reads n from the host via `io::read()`
2. Computes Fibonacci(n) iteratively
3. Commits the result via `io::commit()` (becomes public output in the proof)

### Host (`host/src/main.rs`)

Runs natively on your machine:

1. Sends n=10 as input to the guest
2. **Executes** the guest in the VM (fast sanity check, no proof)
3. **Proves** the execution (generates a VADCOP STARK proof)
4. **Verifies** the proof against the verification key

### Shared Types (`lib/src/lib.rs`)

- `FibResult` - n and the computed value (committed as public output)

## Build Flow

`cargo run --release` does everything:

1. `host/build.rs` cross-compiles the guest to a RISC-V ELF
2. `include_elf!("guest")` embeds the ELF into the host binary
3. The host binary runs: execute, prove, verify

## License

MIT
