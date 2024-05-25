use crate::sections::{Resume, Entry};
use crate::utils::{format_phone};
use crate::generate::generate;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ResumeBuilderApp {
    resume: Resume,
}

impl Default for ResumeBuilderApp {
    fn default() -> Self {
        Self {
            resume: Resume::default(),
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

    fn add_entry(&mut self, entries_name: &str) {
        let entries = match entries_name {
            "Experience" => &mut self.resume.experience,
            "Education" => &mut self.resume.education,
            _ => panic!("Invalid entries name")
        };
        entries.push(Entry::default());
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
            // Scrollable area
            egui::ScrollArea::vertical()
            // Max height a bit less than the screen height:
            .max_height(0.9 * ctx.screen_rect().height())
            .show(ui, |ui| {

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
                        ui.text_edit_singleline(&mut self.resume.personal_info.name);
                        ui.end_row();
                        ui.label("Email: ");
                        ui.text_edit_singleline(&mut self.resume.personal_info.email);
                        ui.label("Phone: ");
                        ui.text_edit_singleline(&mut self.resume.personal_info.phone);
                        ui.end_row();
                    });
                });
                ui.separator();

                // Work Experience Section
                egui::CollapsingHeader::new("Work Experience")
                .default_open(true)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Add Experience").clicked() {
                            self.add_entry("Experience");
                        }
                        if ui.button("Remove Experience").clicked() {
                            self.resume.experience.pop();
                        }
                    });
                    
                    // For each education entry, display a collapsible header
                    for (i, entry) in self.resume.experience.iter_mut().enumerate() {
                        egui::CollapsingHeader::new(format!("Experience {}", i + 1))
                        .default_open(true)
                        .show(ui, |ui| {
                            egui::Grid::new("experience_grid")
                            .spacing([5.0, 5.0])
                            .min_col_width(50.0)
                            .max_col_width(200.0)
                            .show(ui, |ui| {
                                ui.label("Company: ");
                                ui.text_edit_singleline(&mut entry.institution);
                                ui.label("Location: ");
                                ui.text_edit_singleline(&mut entry.location);
                                ui.end_row();
                                ui.label("Title: ");
                                ui.text_edit_singleline(&mut entry.title);
                                ui.label("Start Date: ");
                                ui.text_edit_singleline(&mut entry.start_date);
                                ui.label("End Date: ");
                                ui.text_edit_singleline(&mut entry.end_date);
                                ui.end_row();
                                ui.label("Details: ");
                                ui.horizontal(|ui| {
                                    if ui.button("Add Bullet").clicked() {
                                        entry.description.push("".to_owned());
                                    }
                                    if ui.button("Remove Bullet").clicked() {
                                        entry.description.pop();
                                    }
                                });
                                ui.end_row();
                                for (_, bullet) in entry.description.iter_mut().enumerate() {
                                    ui.label("o ");
                                    ui.text_edit_multiline(bullet);
                                    ui.end_row();
                                }
                            });
                        });
                    }
                });
                ui.separator();

                // Education Section
                egui::CollapsingHeader::new("Education")
                .default_open(true)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Add Education").clicked() {
                            self.add_entry("Education");
                        }
                        if ui.button("Remove Education").clicked() {
                            self.resume.education.pop();
                        }
                    });
                    
                    // For each education entry, display a collapsible header
                    for (i, entry) in self.resume.education.iter_mut().enumerate() {
                        egui::CollapsingHeader::new(format!("Education {}", i + 1))
                        .default_open(true)
                        .show(ui, |ui| {
                            egui::Grid::new("education_grid")
                            .spacing([5.0, 5.0])
                            .min_col_width(50.0)
                            .max_col_width(200.0)
                            .show(ui, |ui| {
                                ui.label("Institution: ");
                                ui.text_edit_singleline(&mut entry.institution);
                                ui.label("Location: ");
                                ui.text_edit_singleline(&mut entry.location);
                                ui.end_row();
                                ui.label("Degree: ");
                                ui.text_edit_singleline(&mut entry.title);
                                ui.label("End Date: ");
                                ui.text_edit_singleline(&mut entry.end_date);
                                ui.end_row();
                            });
                        });
                    }
                });
                
            });
            // Format as needed
            self.resume.personal_info.phone = format_phone(&self.resume.personal_info.phone);
            self.resume.personal_info.email = self.resume.personal_info.email.to_lowercase();
            self.resume.personal_info.linkedin = self.resume.personal_info.linkedin.to_lowercase();
            self.resume.personal_info.github = self.resume.personal_info.github.to_lowercase();
            
            ui.separator();
            if ui.button("Generate").clicked() {
                generate(&self.resume);
            }
            ui.separator();
        });
    }
}