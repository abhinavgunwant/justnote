# justnote

Justnote is a secure, free and open source notes editing and management tool.

Notes are stored and organized inside "Vaults".

Each "Vault" is secured by it's own unique password.

**Note:** This project is a work-in-progress.

## Building and running

### Pre-requisites
- You need to have the [rust toolchain](https://rustup.rs/) installed.
- You need to have the `flatc` binary from the [flatbuffer GitHub releases](https://github.com/google/flatbuffers/releases). Make sure it's added to your `PATH` environment.
  This is required to build the flatbuffer generated rust code in this repo.

Dev build:

```bash
cargo build
```

Release build:

```bash
cargo build --release
```

### Build the flatbuffer generated code

`cd` into the "fb" directory in this repo and run:

```bash
flatc --rust -o ./src/generated/ ./fbs/<fbs-file>
```

Here, replace `<fbs-file>` with any `.fbs` file from the `fb/fbs/` directory
that you want.

