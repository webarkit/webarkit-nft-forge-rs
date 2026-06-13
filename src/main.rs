#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use webarkit_nft_forge_rs::generate_nft_marker;

/// Helper to load Noto Monochrome Emoji font as fallback for egui
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Load the emoji font data from assets/fonts/NotoEmoji-Regular.ttf
    fonts.font_data.insert(
        "noto_emoji".to_owned(),
        egui::FontData::from_static(include_bytes!("assets/fonts/NotoEmoji-Regular.ttf")).into(),
    );

    // Append to Proportional and Monospace families as fallback
    fonts.families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .push("noto_emoji".to_owned());

    fonts.families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("noto_emoji".to_owned());

    ctx.set_fonts(fonts);
}

/// The main entry point for the WebARKit NFT Forge application.
/// It configures the eframe native options and starts the application.
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "WebARKit NFT Forge App",
        options,
        Box::new(|cc| {
            // Load custom fallback font for emojis
            setup_custom_fonts(&cc.egui_ctx);
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}

/// The application state for the WebARKit NFT Forge GUI.
/// Maintains UI state, configuration, and image generation progress.
struct MyApp {
    texture: Option<egui::TextureHandle>,
    image_pixels: Vec<u8>,
    image_width: i32,
    image_height: i32,
    image_nc: i32,
    image_path: Option<std::path::PathBuf>,
    output_dir: Option<std::path::PathBuf>,
    marker_name: String,
    dpi: f32,
    status_message: String,
    progress_val: Arc<AtomicU32>,
    is_generating: bool,
    result_rx: Option<std::sync::mpsc::Receiver<Result<(), String>>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            texture: None,
            image_pixels: Vec::new(),
            image_width: 0,
            image_height: 0,
            image_nc: 3,
            image_path: None,
            output_dir: None,
            marker_name: "marker".to_string(),
            dpi: 220.0,
            status_message: "Ready".to_string(),
            progress_val: Arc::new(AtomicU32::new(0)),
            is_generating: false,
            result_rx: None,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // Check for results from the background thread
        if let Some(rx) = &self.result_rx {
            if let Ok(res) = rx.try_recv() {
                self.is_generating = false;
                match res {
                    Ok(_) => {
                        self.status_message = "Success! Marker generated.".to_string();
                        self.progress_val.store(100, Ordering::SeqCst);
                    }
                    Err(e) => {
                        self.status_message = format!("Error: {}", e);
                    }
                }
                self.result_rx = None;
            }
        }

        if self.is_generating {
            ctx.request_repaint();
        }

        egui::Panel::bottom("footer_panel").show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(4.0);
                ui.label(egui::RichText::new("Proudly made with Rust 🦀").italics().color(ui.visuals().weak_text_color()));
                ui.add_space(4.0);
            });
        });

        egui::Panel::left("controls_panel").show_inside(ui, |ui| {
            ui.add_enabled_ui(!self.is_generating, |ui| {
                ui.heading("Marker Settings");
                ui.add_space(10.0);

                if ui.button("📁 Select Image").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Image Files", &["jpg", "jpeg", "png"])
                        .pick_file()
                    {
                        self.image_path = Some(path.clone());
                        self.status_message = format!("Selected image: {:?}", path.file_name().unwrap());
                        self.load_image_preview(&ctx, &path);
                    }
                }

                if let Some(path) = &self.image_path {
                    ui.label(format!("File: {}", path.display()));
                }

                ui.separator();

                if ui.button("📂 Output Directory").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.output_dir = Some(path);
                    }
                }

                if let Some(path) = &self.output_dir {
                    ui.label(format!("Save to: {}", path.display()));
                } else {
                    ui.label("Save to: Current Directory");
                }

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Marker Name:");
                    ui.text_edit_singleline(&mut self.marker_name);
                });

                ui.horizontal(|ui| {
                    ui.label("DPI:");
                    ui.add(egui::Slider::new(&mut self.dpi, 72.0..=600.0));
                });

                ui.separator();

                ui.add_space(10.0);

                let can_generate = self.image_path.is_some() && !self.marker_name.is_empty();
                let button = egui::Button::new(egui::WidgetText::from("🚀 Generate Marker").heading())
                    .fill(egui::Color32::from_rgb(0, 150, 255))
                    .corner_radius(8.0);

                if ui.add_enabled(can_generate, button).clicked() {
                    self.is_generating = true;
                    self.status_message = "Generating...".to_string();
                    self.progress_val.store(0, Ordering::SeqCst);

                    let output_dir = self
                        .output_dir
                        .clone()
                        .unwrap_or_else(|| std::env::current_dir().unwrap());
                    let marker_name = self.marker_name.clone();
                    let dpi = self.dpi;
                    let pixels = self.image_pixels.clone();
                    let width = self.image_width;
                    let height = self.image_height;
                    let nc = self.image_nc;
                    let progress = self.progress_val.clone();

                    let (tx, rx) = std::sync::mpsc::channel();
                    self.result_rx = Some(rx);

                    std::thread::spawn(move || {
                        let res = generate_nft_marker(
                            &pixels,
                            width,
                            height,
                            nc,
                            &output_dir,
                            &marker_name,
                            dpi,
                            Some(progress),
                        );
                        let _ = tx.send(res.map_err(|e| e.to_string()));
                    });
                }
            });

            ui.add_space(20.0);
            
            let progress = self.progress_val.load(Ordering::SeqCst) as f32 / 100.0;
            ui.add(egui::ProgressBar::new(progress).text(format!("{}%", (progress * 100.0) as u32)));
            
            ui.add_space(10.0);
            ui.label(&self.status_message);
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Image Preview");
                ui.add_space(10.0);
                if let Some(texture) = &self.texture {
                    let max_size = ui.available_size();
                    ui.add(egui::Image::new(texture).max_size(max_size).corner_radius(8.0));
                } else {
                    ui.add_space(100.0);
                    ui.label(egui::WidgetText::from("No image selected").italics().color(egui::Color32::GRAY));
                }
            });
        });
    }
}

impl MyApp {
    /// Loads the selected image, extracts pixels for generating the marker,
    /// and creates a preview texture to display in the UI.
    fn load_image_preview(&mut self, ctx: &egui::Context, path: &std::path::Path) {
        if let Ok(img) = image::open(path) {
            let rgb = img.to_rgb8();
            self.image_width = rgb.width() as i32;
            self.image_height = rgb.height() as i32;
            self.image_nc = 3;
            self.image_pixels = rgb.into_raw();

            let rgba = img.to_rgba8();
            let size = [rgba.width() as usize, rgba.height() as usize];
            self.texture = Some(ctx.load_texture(
                "preview",
                egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw()),
                Default::default(),
            ));
        } else {
            self.status_message = "Failed to load image".to_string();
        }
    }
}
