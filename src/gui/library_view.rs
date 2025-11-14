use egui::{Color32, RichText, ScrollArea, Vec2};
use std::sync::{Arc, Mutex};

use crate::api::Game;
use crate::games::InstalledGame;

#[derive(Clone, PartialEq)]
pub enum GameFilter {
    All,
    Installed,
}

#[derive(Clone)]
pub struct LibraryView {
    filter: GameFilter,
    search_query: String,
    installing_games: Arc<Mutex<Vec<String>>>,
}

impl Default for LibraryView {
    fn default() -> Self {
        Self {
            filter: GameFilter::All,
            search_query: String::new(),
            installing_games: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl LibraryView {
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        library_games: &[Game],
        installed_games: &[InstalledGame],
    ) -> Option<LibraryAction> {
        let mut action = None;

        // Top bar with search and filters
        ui.horizontal(|ui| {
            ui.heading("Library");
            ui.add_space(20.0);

            // Search box
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search_query);

            ui.add_space(20.0);

            // Filters
            if ui.selectable_label(self.filter == GameFilter::All, "All Games").clicked() {
                self.filter = GameFilter::All;
            }
            if ui.selectable_label(self.filter == GameFilter::Installed, "Installed").clicked() {
                self.filter = GameFilter::Installed;
            }
        });

        ui.separator();
        ui.add_space(10.0);

        // Game grid
        ScrollArea::vertical().show(ui, |ui| {
            let available_width = ui.available_width();
            let card_width = 250.0;
            let cards_per_row = (available_width / (card_width + 10.0)).floor().max(1.0) as usize;

            let games_to_show: Vec<_> = match self.filter {
                GameFilter::All => library_games
                    .iter()
                    .filter(|g| {
                        self.search_query.is_empty()
                            || g.app_title
                                .to_lowercase()
                                .contains(&self.search_query.to_lowercase())
                    })
                    .collect(),
                GameFilter::Installed => library_games
                    .iter()
                    .filter(|g| {
                        installed_games.iter().any(|ig| ig.app_name == g.app_name)
                            && (self.search_query.is_empty()
                                || g.app_title
                                    .to_lowercase()
                                    .contains(&self.search_query.to_lowercase()))
                    })
                    .collect(),
            };

            if games_to_show.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    ui.label(RichText::new("No games found").size(18.0).color(Color32::GRAY));
                    if self.search_query.is_empty() {
                        ui.label(RichText::new("Your library is empty or not yet loaded").color(Color32::GRAY));
                    }
                });
            } else {
                // Display games in a grid
                for row_games in games_to_show.chunks(cards_per_row) {
                    ui.horizontal(|ui| {
                        for game in row_games {
                            if let Some(game_action) = self.game_card(ui, game, installed_games) {
                                action = Some(game_action);
                            }
                        }
                    });
                    ui.add_space(10.0);
                }
            }
        });

        action
    }

    fn game_card(
        &mut self,
        ui: &mut egui::Ui,
        game: &Game,
        installed_games: &[InstalledGame],
    ) -> Option<LibraryAction> {
        let mut action = None;
        let is_installed = installed_games.iter().any(|ig| ig.app_name == game.app_name);
        let is_installing = self.installing_games
            .lock()
            .unwrap()
            .contains(&game.app_name);

        egui::Frame::none()
            .fill(super::styles::CARD_BG)
            .rounding(egui::Rounding::same(4.0))
            .inner_margin(egui::Margin::same(10.0))
            .show(ui, |ui| {
                ui.set_min_size(Vec2::new(250.0, 200.0));
                ui.set_max_size(Vec2::new(250.0, 200.0));

                ui.vertical(|ui| {
                    // Game image placeholder
                    let (rect, _) = ui.allocate_exact_size(
                        Vec2::new(230.0, 130.0),
                        egui::Sense::click(),
                    );
                    ui.painter().rect_filled(
                        rect,
                        egui::Rounding::same(4.0),
                        Color32::from_rgb(50, 50, 55),
                    );
                    
                    // Game title placeholder (centered text on image)
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        &game.app_title,
                        egui::FontId::proportional(14.0),
                        Color32::WHITE,
                    );

                    ui.add_space(10.0);

                    // Game title
                    ui.label(
                        RichText::new(&game.app_title)
                            .size(14.0)
                            .strong(),
                    );

                    ui.add_space(5.0);

                    // Action buttons
                    ui.horizontal(|ui| {
                        if is_installed {
                            if ui.button(RichText::new("Play").size(14.0)).clicked() {
                                action = Some(LibraryAction::Launch(game.app_name.clone()));
                            }
                            if ui.button(RichText::new("Uninstall").size(12.0)).clicked() {
                                action = Some(LibraryAction::Uninstall(game.app_name.clone()));
                            }
                        } else if is_installing {
                            ui.add_enabled_ui(false, |ui| {
                                let _ = ui.button(RichText::new("Installing...").size(14.0));
                            });
                        } else if ui.button(RichText::new("Install").size(14.0)).clicked() {
                            self.installing_games.lock().unwrap().push(game.app_name.clone());
                            action = Some(LibraryAction::Install(game.app_name.clone()));
                        }
                    });
                });
            });

        action
    }

    pub fn mark_installation_complete(&mut self, app_name: &str) {
        self.installing_games.lock().unwrap().retain(|name| name != app_name);
    }
}

pub enum LibraryAction {
    Install(String),
    Launch(String),
    Uninstall(String),
}
