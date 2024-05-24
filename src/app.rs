use crate::sections::{PersonalInfo, Entry};
use crate::utils::{format_phone};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ResumeBuilderApp {
    personal_info: PersonalInfo,
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for ResumeBuilderApp {
    fn default() -> Self {
        Self {
            // Fill with empty data
            personal_info: PersonalInfo {
                name: "".to_owned(),
                email: "".to_owned(),
                phone: "".to_owned(),
                location: "".to_owned(),
                linkedin: "".to_owned(),
                github: "".to_owned()
            },
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl ResumeBuilderApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn generate(&mut self) {
        // Print the generated resume to the console
        println!("Generating resume...");
        // Pretty print the personal info on the user's input
        println!("Personal Info:");
        println!("Name: {}", self.personal_info.name);
        println!("Email: {}", self.personal_info.email);
        println!("Phone: {}", self.personal_info.phone);
        println!("Location: {}", self.personal_info.location);
        println!("LinkedIn: {}", self.personal_info.linkedin);
        println!("GitHub: {}", self.personal_info.github);
    }
}

impl eframe::App for ResumeBuilderApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::Label::new("Created by: Jai Wargacki"));
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Personal Info Section
            egui::CollapsingHeader::new("Personal Info")
            .default_open(true)
            .show(ui, |ui| {
                egui::Grid::new("personal_info_grid")
                .spacing([5.0, 5.0])
                .min_col_width(50.0)
                .max_col_width(200.0)
                .show(ui, |ui| {
                    ui.label("Name: ");
                    ui.text_edit_singleline(&mut self.personal_info.name);
                    ui.end_row();
                    ui.label("Email: ");
                    ui.text_edit_singleline(&mut self.personal_info.email);
                    ui.label("Phone: ");
                    ui.text_edit_singleline(&mut self.personal_info.phone);
                    ui.end_row();
                    ui.label("Location: ");
                    ui.text_edit_singleline(&mut self.personal_info.location);
                    ui.end_row();
                    ui.label("LinkedIn: ");
                    ui.text_edit_singleline(&mut self.personal_info.linkedin);
                    ui.label("GitHub: ");
                    ui.text_edit_singleline(&mut self.personal_info.github);
                    ui.end_row();
                });
            });
            ui.separator();

            // Format as needed
            self.personal_info.phone = format_phone(&self.personal_info.phone);
            self.personal_info.email = self.personal_info.email.to_lowercase();
            self.personal_info.linkedin = self.personal_info.linkedin.to_lowercase();
            self.personal_info.github = self.personal_info.github.to_lowercase();

            
            if ui.button("Generate").clicked() {
                self.generate();
            }

            ui.separator();
        });
    }
}