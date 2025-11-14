# R Games Launcher

An Epic Games Store launcher for Linux written in Rust, inspired by [Legendary](https://github.com/derrod/legendary).

## Features

- **Cross-platform support**: Built for Linux with Rust's performance and safety guarantees
- **Epic Games Store Integration**: Authenticate and access your Epic Games library
- **Game Management**: List, install, launch, and uninstall games
- **Configuration Management**: Persistent configuration and authentication
- **GUI Interface**: Minimal, Epic Games-inspired graphical user interface
- **CLI Interface**: Easy-to-use command-line interface

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/kairosci/r-games-launcher.git
cd r-games-launcher

# Build with cargo
cargo build --release

# Install (optional)
cargo install --path .
```

## Usage

### GUI Mode

Launch the graphical user interface:

```bash
r-games-launcher gui
```

The GUI provides an Epic Games Store-like experience with:
- **Login Screen**: Authenticate with your Epic Games account
- **Game Library**: Browse all your games with search and filter capabilities
- **Game Cards**: Visual representation of each game with installation status
- **Quick Actions**: Install, launch, or uninstall games with one click
- **Dark Theme**: Modern dark interface inspired by Epic Games Store

### CLI Mode

The launcher also provides a full command-line interface for all operations.

### Authentication

Authenticate with Epic Games Store (currently in development):

```bash
r-games-launcher auth
```

Logout:

```bash
r-games-launcher auth --logout
```

### List Games

List all games in your library:

```bash
r-games-launcher list
```

List only installed games:

```bash
r-games-launcher list --installed
```

### Install a Game

Install a game from your library:

```bash
r-games-launcher install <app_name>
```

### Launch a Game

Launch an installed game:

```bash
r-games-launcher launch <app_name>
```

### Game Information

Show information about a game:

```bash
r-games-launcher info <app_name>
```

### Uninstall a Game

Remove a game:

```bash
r-games-launcher uninstall <app_name>
```

### Status

Check the launcher status and configuration:

```bash
r-games-launcher status
```

### Options

Enable verbose logging for any command:

```bash
r-games-launcher --verbose <command>
```

## Architecture

The launcher is built with a modular architecture:

- **API Module** (`src/api/`): Epic Games Store API client
- **Auth Module** (`src/auth/`): Authentication and token management
- **Config Module** (`src/config/`): Configuration management
- **Games Module** (`src/games/`): Game installation, launching, and management
- **CLI Module** (`src/cli/`): Command-line interface
- **Error Module** (`src/error.rs`): Error handling

## Configuration

Configuration is stored in:
- **Linux**: `~/.config/r-games-launcher/config.toml`

Default configuration:

```toml
install_dir = "~/.local/share/r-games-launcher/games"
log_level = "info"
```

Authentication tokens are stored securely in:
- **Linux**: `~/.local/share/r-games-launcher/auth.json`

## Development Status

This project is currently in early development. The following features are planned or in progress:

- [x] Project structure and core modules
- [x] CLI interface
- [x] GUI interface with Epic Games-inspired design
- [x] Configuration management
- [x] Authentication framework
- [x] Game library display (demo mode)
- [x] Game installation workflow (framework in place)
- [x] Game launching (for installed games)
- [x] Game uninstallation
- [ ] Epic Games OAuth integration (full implementation)
- [ ] Real Epic Games API integration
- [ ] Game manifest parsing
- [ ] Game download and installation (full implementation)
- [ ] Update management
- [ ] Cloud saves support

## Inspiration

This project is inspired by [Legendary](https://github.com/derrod/legendary), an excellent Python-based Epic Games launcher. R Games Launcher aims to provide similar functionality with the performance and safety benefits of Rust.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues.

## License

MIT License

## Disclaimer

This project is not affiliated with or endorsed by Epic Games. Use at your own risk.