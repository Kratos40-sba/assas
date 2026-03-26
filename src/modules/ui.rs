use eframe::{egui, App, Frame};
use crate::modules::security::SecurityManager;
use crate::modules::config::Config;
use std::fs;
use std::sync::Arc;
use anyhow::{Result, anyhow};
use image::load_from_memory;
use egui::load::SizedTexture;

pub struct VaultApp {
    password: String,
    authenticated: bool,
    images: Vec<(String, egui::TextureHandle)>,
    security: Option<Arc<SecurityManager>>,
    config: Config,
    new_title_input: String,
    status_msg: String,
    tab: Tab,
}

#[derive(PartialEq)]
enum Tab {
    Gallery,
    Settings,
}

impl VaultApp {
    pub fn new() -> Self {
        let config = Config::load().unwrap_or_default();
        Self {
            password: String::new(),
            authenticated: false,
            images: Vec::new(),
            security: None,
            config,
            new_title_input: String::new(),
            status_msg: "Veuillez entrer le mot de passe maître".to_string(),
            tab: Tab::Gallery,
        }
    }

    fn load_images(&mut self, ctx: &egui::Context) -> Result<()> {
        let security = self.security.as_ref()
            .ok_or_else(|| anyhow!("Gestionnaire de sécurité non initialisé"))?;
        
        let path = security.get_log_dir().map_err(|e| anyhow!(e))?;
        if !path.exists() { return Ok(()); }

        let mut loaded = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let encrypted_data = fs::read(entry.path())?;
            
            if let Ok(decrypted_data) = security.decrypt(&encrypted_data) {
                if let Ok(img) = load_from_memory(&decrypted_data) {
                    let rgba = img.to_rgba8();
                    let (width, height) = rgba.dimensions();
                    let pixels = rgba.into_raw();
                    let color_image = egui::ColorImage::from_rgba_unmultiplied(
                        [width as usize, height as usize], 
                        &pixels
                    );
                    
                    let name = entry.file_name().to_string_lossy().to_string();
                    let texture = ctx.load_texture(&name, color_image, Default::default());
                    loaded.push((name, texture));
                }
            }
        }
        
        loaded.sort_by(|a, b| b.0.cmp(&a.0));
        self.images = loaded;
        Ok(())
    }
}

impl App for VaultApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.authenticated {
                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    ui.heading("Coffre ASSAS - Accès Propriétaire");
                    ui.add_space(20.0);
                    
                    ui.add(egui::TextEdit::singleline(&mut self.password)
                        .password(true)
                        .hint_text("Mot de passe"));
                        
                    ui.add_space(10.0);
                    if ui.button("Déverrouiller le coffre").clicked() {
                        let salt = self.config.salt.as_ref().cloned().unwrap_or_else(|| SecurityManager::generate_random_salt());
                        match SecurityManager::new(&self.password, &salt) {
                            Ok(sm) => {
                                let arc_sm = Arc::new(sm);
                                self.security = Some(arc_sm);
                                if let Err(e) = self.load_images(ctx) {
                                    self.status_msg = format!("Erreur lors du chargement : {:?}", e);
                                } else {
                                    self.authenticated = true;
                                    self.status_msg = "Coffre déverrouillé".to_string();
                                }
                            }
                            Err(_) => self.status_msg = "Mot de passe invalide".to_string(),
                        }
                    }
                    ui.label(&self.status_msg);
                });
            } else {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.tab, Tab::Gallery, "Galerie");
                    ui.selectable_value(&mut self.tab, Tab::Settings, "Paramètres");
                });
                ui.separator();
                ui.add_space(10.0);

                match self.tab {
                    Tab::Gallery => {
                        ui.heading("Journaux d'Audit");
                        ui.add_space(10.0);
                        
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (name, texture) in &self.images {
                                ui.group(|ui| {
                                    ui.label(name);
                                    ui.add(egui::Image::new(SizedTexture::new(
                                        texture.id(),
                                        [texture.size_vec2().x * 0.2, texture.size_vec2().y * 0.2]
                                    )));
                                });
                                ui.add_space(10.0);
                            }
                        });
                    }
                    Tab::Settings => {
                        ui.heading("Configuration du Contrôle");
                        ui.add_space(10.0);
                        
                        ui.label("Titres des fenêtres POS à surveiller :");
                        let mut to_remove = None;
                        for (i, title) in self.config.target_titles.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("• {}", title));
                                if ui.button("🗑").clicked() {
                                    to_remove = Some(i);
                                }
                            });
                        }
                        if let Some(i) = to_remove {
                            self.config.target_titles.remove(i);
                        }

                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut self.new_title_input);
                            if ui.button("Ajouter").clicked() && !self.new_title_input.is_empty() {
                                self.config.target_titles.push(self.new_title_input.clone());
                                self.new_title_input.clear();
                            }
                        });

                        ui.add_space(20.0);
                        if ui.button("Enregistrer la configuration").clicked() {
                            if let Err(e) = self.config.save() {
                                self.status_msg = format!("Erreur sauvegarde : {:?}", e);
                            } else {
                                self.status_msg = "Configuration enregistrée. Redémarrez pour appliquer.".to_string();
                            }
                        }
                        ui.label(&self.status_msg);
                    }
                }
            }
        });
    }
}

pub fn run_vault_viewer() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Visionneuse de Coffre ASSAS",
        native_options,
        Box::new(|_cc| Box::new(VaultApp::new())),
    );
}
