use clap::Parser;
use r_games_launcher::{
    auth::AuthManager,
    cli::{Cli, Commands},
    config::Config,
    games::GameManager,
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    // Load configuration
    let config = Config::load()?;
    log::debug!("Configuration loaded");

    // Initialize auth manager
    let mut auth = AuthManager::new()?;

    match cli.command {
        Commands::Auth { logout } => {
            if logout {
                auth.logout()?;
                println!("Successfully logged out");
            } else {
                println!("Epic Games Store Authentication");
                println!("================================");
                println!();
                println!("Authentication is not yet fully implemented.");
                println!("This requires setting up OAuth with Epic Games Store.");
                println!();
                println!("In the future, this will:");
                println!("  1. Display a device code and URL");
                println!("  2. Wait for you to authenticate in your browser");
                println!("  3. Save your authentication token");
            }
        }

        Commands::List { installed } => {
            if installed {
                let manager = GameManager::new(config, auth)?;
                let games = manager.list_installed()?;

                if games.is_empty() {
                    println!("No games installed");
                } else {
                    println!("Installed Games:");
                    println!("================");
                    for game in games {
                        println!(
                            "  {} - {} (v{})",
                            game.app_name, game.app_title, game.app_version
                        );
                        println!("    Path: {:?}", game.install_path);
                    }
                }
            } else {
                if !auth.is_authenticated() {
                    eprintln!("Error: Not authenticated. Run 'r-games-launcher auth' first.");
                    std::process::exit(1);
                }

                let manager = GameManager::new(config, auth)?;
                let games = manager.list_library().await?;

                if games.is_empty() {
                    println!("No games in library (or authentication required)");
                } else {
                    println!("Library:");
                    println!("========");
                    for game in games {
                        println!(
                            "  {} - {} (v{})",
                            game.app_name, game.app_title, game.app_version
                        );
                    }
                }
            }
        }

        Commands::Install { app_name } => {
            if !auth.is_authenticated() {
                eprintln!("Error: Not authenticated. Run 'r-games-launcher auth' first.");
                std::process::exit(1);
            }

            let manager = GameManager::new(config, auth)?;
            println!("Installing game: {}", app_name);

            match manager.install_game(&app_name).await {
                Ok(()) => println!("Game installed successfully!"),
                Err(e) => {
                    eprintln!("Failed to install game: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Launch { app_name } => {
            let manager = GameManager::new(config, auth)?;

            match manager.launch_game(&app_name) {
                Ok(()) => println!("Game launched successfully!"),
                Err(e) => {
                    eprintln!("Failed to launch game: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Uninstall { app_name } => {
            let manager = GameManager::new(config, auth)?;

            match manager.uninstall_game(&app_name) {
                Ok(()) => println!("Game uninstalled successfully!"),
                Err(e) => {
                    eprintln!("Failed to uninstall game: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Info { app_name } => {
            let manager = GameManager::new(config, auth)?;

            match manager
                .list_installed()?
                .iter()
                .find(|g| g.app_name == app_name)
            {
                Some(game) => {
                    println!("Game Information:");
                    println!("================");
                    println!("Name: {}", game.app_name);
                    println!("Title: {}", game.app_title);
                    println!("Version: {}", game.app_version);
                    println!("Install Path: {:?}", game.install_path);
                    println!("Executable: {}", game.executable);
                }
                None => {
                    eprintln!("Game not found: {}", app_name);
                    std::process::exit(1);
                }
            }
        }

        Commands::Status => {
            println!("R Games Launcher Status");
            println!("=======================");
            println!();
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            println!(
                "Authenticated: {}",
                if auth.is_authenticated() { "Yes" } else { "No" }
            );
            println!();
            println!("Configuration:");
            println!("  Install Directory: {:?}", config.install_dir);
            println!("  Log Level: {}", config.log_level);
            println!();

            if let Ok(config_path) = Config::config_path() {
                println!("Config Path: {:?}", config_path);
            }

            if let Ok(data_dir) = Config::data_dir() {
                println!("Data Directory: {:?}", data_dir);
            }
        }
    }

    Ok(())
}
