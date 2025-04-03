# STEVE: Search Technical Evidence Very Easy

STEVE is a command-line tool specifically designed to help teams quickly verify that major architectural decisions are well-documented. It provides three main functionalities:

- **Search**: Looks up documentation related to a query within a directory specified in your local `.env` file.
- **Audit**: Uses `git diff` to compare the current branch against `main` and checks that important architectural decisions have been documented.
- **Roast**: Also leverages `git diff` but provides a lighthearted, tongue-in-cheek code review.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/)
- A configured `.env` file

## Setup

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd steve
   ```

2. **Configure your environment:**
   Create a `.env` file (or copy an existing example) and set the directory where your documentation resides:
   ```dotenv
   OPENAI_API_KEY=<API_TOKEN>
   STEVE_DOCS_PATH=/workspaces/Steve/docs
   ```

## Installation

Install the project with Cargo:
```bash
cargo install steve
```

## Usage

Run STEVE using one of its subcommands:

### Search

Search in the documentation directory for evidence matching your query. If no query is given, STEVE reads from STDIN.
```bash
steve search "your query here"
```

### Audit

Analyze the current branch against `main` to ensure that major architectural decisions are properly documented.
```bash
steve audit
```

### Roast

Enjoy a fun, no-holds-barred code review by comparing current changes with documentation.
```bash
steve roast
```

## Contributing

Contributions are welcome! Please review the guidelines in [CONTRIBUTING.md](CONTRIBUTING.md) if you’d like to help improve STEVE.

## License

This project is licensed under the MIT License.