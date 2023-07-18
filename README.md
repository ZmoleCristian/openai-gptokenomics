# OpenAI-GPTokenomics

OpenAI-GPTokenomics is a simple command-line tool to calculate the price for requests sent to the OpenAI models.

## Features

- Calculate request price based on model and number of tokens.
- Calculate answer price based on model and number of tokens.
- Support for multiple OpenAI models: GPT-4, GPT-3.5 Turbo, DaVinci, Curie, Babbage, and Ada.
- Read data from input files or directly from command-line arguments.
- Write output to a file or print it to the console.

## Requirements

- Rust (tested with Edition 2021)
- Dependencies:
  - tiktoken-rs v0.3.3
  - structopt v0.3

## Installation

- Clone the repository:

```
git clone https://github.com/tragDate/openai-gptokenomics.git
```

- Change into the project directory:

```
cd openai-gptokenomics
```

- Build the project:

```
cargo build --release
```

- The binary will be available at `./target/release/openai-gptokenomics`.

## Usage

```
openai-gptokenomics [FLAGS] [OPTIONS]
```

### Flags:

- `-r`, `--request_file`: A file with the request data (conflicts with `--request_data`).
- `-d`, `--request_data`: A string with the request data (conflicts with `--request_file`).
- `-a`, `--answer_file`: A file with the answer data (conflicts with `--answer_data`).
- `--answer_data`: A string with the answer data (conflicts with `--answer_file`).
- `--answer`: Calculate answer price from stdin.
- `-o`, `--output_file`: Save the output into a file, instead of printing it to the console.

### Options:

- `-m`, `--model` (default: gpt-3.5-turbo): The OpenAI model.
- `--output_file`: The path to save the output.

### Example:

```
openai-gptokenomics -m "gpt-4" -d "What is the capital of France?" --answer_data "The capital of France is Paris."
```

## Author

[@tragDate](https://github.com/tragDate)

## License

GPL-3.0
