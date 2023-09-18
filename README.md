# Rust Arg Parser (PyO3)

A high-performance, customizable command parser written in Rust, for Python, using the PyO3 library. This library aims to provide fast and efficient command parsing capabilities for your Python projects.

## Features

- Type inference for command arguments
  - Supports basic types like integers, floats, and strings
  - Supports list types with square bracket notation (`[...]`)
- Boolean flag parsing
- Easily extendable and customizable
- High performance thanks to Rust

## Installation

### Prerequisites

- Rust's Package Manager `cargo`
- Python 3.x

### Build and Install

Clone the repository, then run:

```bash
# build the library
cargo build --release

# Install the Python package
pip install .
```

## Usage

Import the module and use the `parse_command` function:

```python
from rust_arg_parser import parse_command

# Define boolean flags
boolean_flags = {"verbose", "dry_run"}

# Parse a command
result = parse_command("--verbose --count 3 --name John", boolean_flags)

print(result)
```

This will return a Python dictionary containing the parsed arguments and their types, like so:

```python
{
    'verbose': True,
    'count': 3,
    'name': 'John'
}
```

## API Documentation

### `parse_command(command: str, boolean_flags: set) -> dict`

Parses a given command and returns a dictionary of arguments.

- `command`: The command string to be parsed.
- `boolean_flags`: A set containing the names of boolean flags.

## Contributing

Contributions are welcome! Please read the contributing guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

---

Feel free to add, remove, or modify sections as you see fit!