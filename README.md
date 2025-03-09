# my-cpu

A utility to detect the current CPU and select a matching binary to run from the result.

## Overview

`my-cpu` is a tool that helps you run CPU-specific binaries by detecting the host CPU architecture and matching it against provided patterns. This is useful for running optimized binaries based on the CPU capabilities of the host system.

## Features

- Automatic CPU detection using LLVM
- Regular expression based matching for CPU selection
- Fallback binary support
- Command passthrough to selected binary

## Installation

### From Source

```bash
cargo build --release
```

### Using Docker

```bash
docker build -t my-cpu .
```

## Usage

```bash
my-cpu --fallback /path/to/fallback/binary -t /path/to/binary:regex [command args...]
```

### Arguments

- `--fallback <PATH>`: Path to the fallback binary to be used if the CPU cannot be detected or there are no matches
- `-t, --target <PATH:REGEX>`: One or more target pairs specified as path to binary and a regex pattern to match against the CPU name
- `[command...]`: The command arguments to pass to the selected binary

### Example

```bash
my-cpu --fallback /usr/bin/app-generic \
       -t /usr/bin/app-x64-v4:x86-64-v4 \
       -t /usr/bin/app-avx512:".*avx512.*" \
       -- --some-arg value
```

This will:
1. Detect the current CPU
2. Try to match it against the provided patterns
3. Run the matching binary (or fallback) with the provided arguments

## Building

### Requirements

- Rust 1.85.0 or later
- LLVM 19 or later
- C++ compiler
