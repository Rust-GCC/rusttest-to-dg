# rusttest-to-dg

Converts `rustc` testcases into dejagnu testcases for `gccrs`

# Usage

This tool was invoked by the script [run](./run.sh). Furthermore, we need the rustc and gccrs source code to be downloaded on the system. And add their paths to the environment variables `GCCRS_PATH`, `RUST_PATH` and `RUSTTEST_TO_DG_PATH`

```bash
export GCCRS_PATH=/path/to/gccrs
export RUST_PATH=/path/to/rust
export RUSTTEST_TO_DG_PATH=/path/to/rusttest-to-dg
```

Then, simply run the script [run](./run.sh)

```bash
bash run.sh
```
