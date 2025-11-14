use egui::{Align, Layout, RichText};

use crate::auth::AuthManager;

#[derive(Default)]
pub struct AuthView {
    email_input: String,
    password_input: String,
    auth_status: String,
    is_loading: bool,
}

impl AuthView {
    pub fn ui(&mut self, ui: &mut egui::Ui, auth: &mut AuthManager) -> bool {
        let mut should_login = false;
        
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            
            ui.heading(RichText::new("Epic Games Store").size(32.0));
            ui.add_space(10.0);
            ui.label(RichText::new("Sign in to your account").size(16.0));
            
            ui.add_space(40.0);
            
            // Center the form
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.set_max_width(400.0);
                
                ui.label("Email Address");
                ui.add_space(5.0);
                let email_response = ui.text_edit_singleline(&mut self.email_input);
                
                ui.add_space(15.0);
                
                ui.label("Password");
                ui.add_space(5.0);
                let password_response = ui.add(
                    egui::TextEdit::singleline(&mut self.password_input)
                        .password(true)
                );
                
                ui.add_space(25.0);
                
                // Login button
                let button_enabled = !self.email_input.is_empty() 
                    && !self.password_input.is_empty() 
                    && !self.is_loading;
                
                ui.add_enabled_ui(button_enabled, |ui| {
                    if ui.button(RichText::new(if self.is_loading { 
                        "Logging in..." 
                    } else { 
                        "Sign In" 
                    }).size(16.0))
                    .clicked() {
                        should_login = true;
                        self.is_loading = true;
                    }
                });
                
                ui.add_space(20.0);
                
                // Status message
                if !self.auth_status.is_empty() {
                    ui.colored_label(
                        if self.auth_status.contains("Success") {
                            egui::Color32::GREEN
                        } else {
                            egui::Color32::RED
                        },
                        &self.auth_status
                    );
                }
                
                ui.add_space(20.0);
                
                // Information message
                ui.label(RichText::new("Note: Full OAuth authentication is not yet implemented.")
                    .size(12.0)
                    .color(egui::Color32::GRAY));
                ui.label(RichText::new("This is a demo login screen.")
                    .size(12.0)
                    .color(egui::Color32::GRAY));
                
                // For demo purposes, allow any input to "login"
                // In a real implementation, this would perform OAuth authentication
                if should_login {
                    // Demo: Create a mock authentication token
                    use chrono::Utc;
                    use crate::auth::AuthToken;
                    
                    let demo_token = AuthToken {
                        access_token: "demo_access_token".to_string(),
                        refresh_token: "demo_refresh_token".to_string(),
                        expires_at: Utc::now() + chrono::Duration::hours(24),
                        account_id: "demo_user_id".to_string(),
                    };
                    
                    if let Err(e) = auth.set_token(demo_token) {
                        self.auth_status = format!("Error saving token: {}", e);
                    } else {
                        self.auth_status = "Successfully authenticated! (Demo mode)".to_string();
                    }
                    self.is_loading = false;
                }
                
                // Check if Enter was pressed
                if (email_response.lost_focus() || password_response.lost_focus()) 
                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                    && button_enabled {
                    should_login = true;
                }
            });
        });
        
        should_login
    }
}
