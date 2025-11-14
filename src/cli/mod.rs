use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "r-games-launcher")]
#[command(author, version, about = "Epic Games launcher for Linux written in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Authenticate with Epic Games Store
    Auth {
        /// Logout instead of login
        #[arg(short, long)]
        logout: bool,
    },

    /// List games in your library
    List {
        /// Show installed games only
        #[arg(short, long)]
        installed: bool,
    },

    /// Install a game
    Install {
        /// App name of the game to install
        app_name: String,
    },

    /// Launch a game
    Launch {
        /// App name of the game to launch
        app_name: String,
    },

    /// Uninstall a game
    Uninstall {
        /// App name of the game to uninstall
        app_name: String,
    },

    /// Show information about a game
    Info {
        /// App name of the game
        app_name: String,
    },

    /// Show status and configuration
    Status,
}
