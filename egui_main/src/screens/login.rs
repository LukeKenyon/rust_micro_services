use crate::services::authentication::AuthService;
use crate::{AppEvent, LoginResponse};
use egui::{Context, Ui};
use std::time::Instant;
use tokio::sync::broadcast;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

pub struct LoginScreen {
    email: String,
    password: String,
    show_error: bool,
    warning_shown_at: Option<Instant>,
}
impl Default for LoginScreen {
    fn default() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            show_error: false,
            warning_shown_at: None,
        }
    }
}

impl LoginScreen {
    pub fn show(
        &mut self,
        _ctx: &Context,
        ui: &mut Ui,
        auth_service: &mut AuthService,
        event_tx: broadcast::Sender<AppEvent>,
    ) {
        ui.horizontal_centered(|ui| {
            ui.vertical_centered(|ui| {
                ui.style_mut().spacing.button_padding = egui::vec2(20.0, 10.0);
                ui.heading("🔐 Login");
                ui.label("Username:");
                ui.text_edit_singleline(&mut self.email);
                ui.add_space(5.0);
                ui.label("Password:");
                ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                ui.add_space(5.0);
                //Error Label for missing fields, only shows for 3 seconds
                if self.show_error {
                    if let Some(warning_shown_at) = self.warning_shown_at {
                        if warning_shown_at.elapsed().as_millis() < 3000 {
                            ui.colored_label(
                                egui::Color32::RED,
                                "⚠️ Please enter both fields".to_string(),
                            );
                        } else {
                            self.show_error = false;
                            self.warning_shown_at = None;
                        }
                    }
                }
                ui.horizontal_centered(|ui| {
                    if ui.button("Log In").clicked() {
                        if !self.email.is_empty() && !self.password.is_empty() {
                            tracing::info!(
                                "Logging in with username: {} and password: {}",
                                self.email,
                                self.password
                            );
                            let (tx, mut rx): (
                                UnboundedSender<Result<LoginResponse, String>>,
                                UnboundedReceiver<Result<LoginResponse, String>>,
                            ) = unbounded_channel();

                            auth_service.login_async(self.email.clone(), self.password.clone(), tx);
                            //
                            //
                            let event_tx = event_tx.clone();
                            tokio::spawn(async move {
                                if let Some(result) = rx.recv().await {
                                    match result {
                                        Ok(login_response) => {
                                            let _ = event_tx.send(AppEvent::LoginSuccess(
                                                login_response.token.clone(),
                                            ));
                                            //auth_service.set_token(login_response.token);
                                            tracing::info!(
                                                "✅ Login successful {}",
                                                login_response.token
                                            );
                                            // TODO: Trigger screen change externally
                                            // Possibly use a shared state manager or message to MainApp
                                        }
                                        Err(e) => {
                                            tracing::warn!("❌ Login error: {}", e);
                                            // Could use a shared state or callback to inform UI
                                        }
                                    }
                                    // ctx.request_repaint(); // Ensure UI updates
                                }
                            });
                        } else {
                            tracing::info!("Missing fields");
                            self.show_error = true;
                            self.warning_shown_at = Some(Instant::now());
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        std::process::exit(0);
                    }

                    if ui.button("Test").clicked() {
                        // Add your test logic here
                    }
                });
            })
        });
    }
}
