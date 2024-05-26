# Beckhoff PLC Connection and Data Handling

This project enables connection to Beckhoff TwinCAT PLCs using the ADS (Automation Device Specification) protocol. By integrating the `pyads` library in Python and the `ads-rs` library in Rust, it enables establishing a connection with Beckhoff TwinCAT PLCs.

**Note:** This project only works on the **Linux** operating system.

## Installation

### Python Installation

Download and install Python 3.x version from [Python.org](https://www.python.org/).

### pyads Installation

Install the `pyads` library by running the following command in your terminal or command prompt:

```shell
pip install pyads
```

### ads-rs Installation

After installing the Rust programming language from the [Rust official website](https://www.rust-lang.org/), add the `ads` library to your Rust project's `Cargo.toml` file. Then, run the following command to compile:

```shell
cargo build
```

