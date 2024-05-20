# Letter Combinations

This project generates all possible combinations of uppercase and lowercase letters for a given input string. It also displays progress bars for both the combination generation and file writing processes.

## Features

- Generates all possible letter combinations for an input string
- Displays progress bars for the combination generation and file writing processes
- Writes the generated combinations to a specified file or defaults to `combinations.txt`

## Dependencies

- `rayon` for parallel computation
- `progress_bar` for displaying progress bars

## Usage

### Building the Project

To build the project in release mode:

```sh
cargo build --release
```

### Running the Project

To run the project with an input string and an optional output file path:

```sh
./target/release/letter_combinations "YourInputString" [output_file]   # Unix
.\target\release\letter_combinations.exe "YourInputString" [output_file]   # Windows
```

If no output file is specified, the program defaults to `combinations.txt`.

### Example

```sh
cargo run --release -- "Ab" "output.txt"
```

## Development

To build and run the project in development mode:

```sh
cargo run -- "YourInputString" [output_file]
```

## License

This project is licensed under the MIT License.