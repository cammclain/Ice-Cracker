# ice-cracker

![License](https://img.shields.io/badge/license-MIT-green)
![Rust Version](https://img.shields.io/badge/rust-stable-brightgreen.svg)
![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)

**ice-cracker** is an open-source command-line tool designed for performing dictionary attacks on password hashes using massive datasets. It utilizes the powerful Polars library for high-performance data manipulation, enabling efficient handling of data that far exceeds system RAM limits.

## 🌟 Features

- **High Efficiency**: Leverages Polars for memory mapping and lazy data processing, handling large datasets without overwhelming system RAM.
- **Customizable Input**: Supports command-line arguments for specifying paths to custom hash and dictionary files.
- **Fast Processing**: Uses Rust's concurrency features to enhance the speed of hashing and comparison operations.

## 📋 Prerequisites

To install and run **ice-cracker**, you must have:

- [Rust Programming Language](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/) (Rust's package manager)

## 🚀 Installation

Clone the repository and compile the project:

```bash
git clone https://github.com/yourusername/ice-cracker.git
cd ice-cracker
cargo build --release
```

The executable will be located in `./target/release`.

## 🔍 Usage

Run **ice-cracker** by specifying the paths to your hash and dictionary files:

```bash
./target/release/ice-cracker -h /path/to/hashes.txt -d /path/to/dictionary.txt
```

### Command-line Arguments

- `-h`, `--hashes` FILE: Specify the path to the password hashes file.
- `-d`, `--dictionary` FILE: Specify the path to the dictionary file.



## 📄 License

**ice-cracker** is made available under the MIT License. For more details, see the [LICENSE.md](LICENSE) file.

## ⚠️ Disclaimer

**ice-cracker** is intended for educational and ethical testing purposes only. Neither the author nor any contributors are responsible for misuse or damages caused by this software.

