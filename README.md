# bookman

A simple browser-independent CLI Bookmark Manager.

## Features

- **CLI Interface:** Access and manage bookmarks without leaving the terminal.
- **Clipboard Support:** Quickly add bookmarks from the clipboard.
- **Import/Export:** Import bookmarks from `.html` files (Firefox/Chrome migration).
- **Fuzzy Search:** Supports a fuzzy search in the database.
- **Notifications:** OS-specfic notifications are supported (e.g., `notify-send` on Linux).
- **Secure:** Stored bookmarks are secured with AES 256-bit encryption.

## Installation

### Prerequisites

- Rust (latest stable version)
- SQLite (optional, if not using the bundled library)

### Clone and Build

```sh
git clone https://github.com/pleskunov/bookman.git
cd bookman
cargo build --release
```

## Usage

```sh
bookman <COMMAND> [OPTIONS]
```

### Commands

| Command         | Description                                   |
| --------------- | --------------------------------------------- |
| `add`           | Add a new bookmark (supports clipboard input) |
| `search`        | Search for bookmarks by name or URL           |
| `edit <id>`     | Edit a bookmark by ID                         |
| `remove <id>`   | Remove a bookmark by ID                       |
| `clip`          | Copy a bookmark URL to clipboard              |
| `import <file>` | Import bookmarks from an `.html` file         |

### Example Usage

#### Add a bookmark from the clipboard

```sh
bookman add --clipboard
```

#### Search for bookmarks

```sh
bookman search
```

#### Edit a bookmark

```sh
bookman edit 3
```

#### Remove a bookmark

```sh
bookman remove 5
```

#### Import bookmarks from Firefox/Chrome export

```sh
bookman import bookmarks.html
```

## Configuration

The database is stored at:

```sh
$HOME/.local/share/bookman/bookmarks.db
```

The program will automatically create the directory/database at the first start.

## Contributing

Pull requests are welcome! If you find any bugs or have feature requests, open an issue.

## License

This project is licensed under the GPL-3.0 License. See `LICENSE` for details.
