use eframe::egui::{self, CentralPanel};
use egui_main::AppEvent;
use egui_main::ContentScreens;
use egui_main::screens::{home, login, settings};
use egui_main::services::authentication::AuthService;
use tokio::sync::broadcast;
pub struct MainApp {
    pub current_screen: ContentScreens,
    pub login_screen: login::LoginScreen,
    pub auth_service: AuthService,
    pub event_tx: broadcast::Sender<AppEvent>,
    pub event_rx: broadcast::Receiver<AppEvent>,
}

impl Default for MainApp {
    fn default() -> Self {
        let (event_tx, event_rx) = broadcast::channel(16);
        Self {
            current_screen: ContentScreens::Login,
            login_screen: login::LoginScreen::default(),
            auth_service: AuthService::default(),
            event_tx,
            event_rx,
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                AppEvent::LoginSuccess(token) => {
                    tracing::info!("Login successful  {}", token);
                    self.auth_service.set_token(token);
                    self.current_screen = ContentScreens::Home;
                }
                AppEvent::LoginFailed(err) => {
                    tracing::warn!("Login failed: {}", err);
                }
            }
        }

        if self.current_screen != ContentScreens::Login {
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Save").clicked() {
                            // TODO: Add Save functionality
                        }
                        if ui.button("Quit").clicked() {
                            std::process::exit(0);
                        }
                    });

                    ui.menu_button("Edit", |ui| {
                        if ui.button("Cut").clicked() {
                            // TODO: Add Cut functionality
                        }
                        if ui.button("Copy").clicked() {
                            // TODO: Add Copy functionality
                        }
                        if ui.button("Paste").clicked() {
                            // TODO: Add Paste functionality
                        }
                    });
                });
            });
        }

        CentralPanel::default().show(ctx, |ui| match self.current_screen {
            ContentScreens::Login => {
                ui.label("Welcome to the Login Screen!");
                self.login_screen
                    .show(ctx, ui, &mut self.auth_service, self.event_tx.clone());
            }
            ContentScreens::Home => {
                let mut home = home::HomeScreen::default();
                ui.label("Welcome to the Home Screen!");
                home.show(ctx, ui);
            }
            ContentScreens::Settings => {
                let mut settings = settings::SettingsScreen::default();
                ui.label("Welcome to the Settings Screen!");
                settings.show(ctx, ui);
            }
        });
    }
}
