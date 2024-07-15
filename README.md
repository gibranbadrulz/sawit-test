# Sawit Test

SRE Technical Test from Sawit Pro, built with Rust as CLI tool. This tool is used to collect log data from source and ship into Elasticsearch Data Lake.

## Usage

```
cargo build --release
./target/release/sawit-log --log-file <filename>
```

## Architecture

Here's a simple diagram about the architecture of the tool which describes what are the main components and how they interact together.

![components](./assets/images/diagram.png)
