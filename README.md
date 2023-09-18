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

```json
{
    "verbose": True,
    "count": 3,
    "name": "John"
}
```

It can also handle lists of things in brackets, with breaks on commas- like so:

```python
from rust_arg_parser import parse_command

cmd = "--words [I love to eat cake, lemon cakes, peanut butter] "\
      "--prompt I love to eat pancakes! also- I like to eat waffles! "\
      "--nums [1,2,3,4,5,6,7, 8,9,10]" # can handle spaces in lists

result = parse_command(cmd,set())
```
This would give the result:
```json
{
    "prompt": "I love to eat pancakes! also- I like to eat waffles!",   
    "words": ["I love to eat cake", "lemon cakes", "peanut butter"],    
    "nums": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
}
```

If the list items were floats:
```py
cmd = "--nums [1.1, 2.2, 3.3, 4.3, 5.1, 6.1, 7.1, 8.6, 9, 10]"

result = parse_command(cmd,set())
```
It would give you:
```json
{
    "nums": [1.1, 2.2, 3.3, 4.3, 5.1, 6.1, 7.1, 8.6, 9.0, 10.0]
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