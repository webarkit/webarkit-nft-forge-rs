#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self};
use webarkit_nft_forge_rs::generate_nft_marker;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "WebARKit NFT Forge App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Default)]
struct MyApp {
    texture: Option<egui::TextureHandle>,
    image_bytes: Vec<u8>,
    image_pixels: Vec<u8>,
    image_width: i32,
    image_height: i32,
    image_nc: i32,
    button_color: egui::Color32,
    button_text: egui::WidgetText,
    button_text_color: egui::Color32,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Load the image once via the `image` crate so you can apply transformations
        if self.texture.is_none() {
            let img = image::load_from_memory(include_bytes!("pinball.jpg")).unwrap();

            // Apply transformations here, for example:
            let img = img.resize(720, 400, image::imageops::FilterType::Lanczos3);

            self.button_color = egui::Color32::from_rgb(220, 50, 10); // Change button color based on transformations
            self.button_text_color = egui::Color32::WHITE; // Change button text color based on transformations
            self.button_text = egui::WidgetText::from("Generate").color(self.button_text_color); // Change button text based on transformations

            let rgba = img.to_rgba8();
            let rgb = img.to_rgb8();
            self.image_bytes = rgba.as_raw().to_vec();
            self.image_height = rgb.height() as i32;
            self.image_width = rgb.width() as i32;
            self.image_nc = 3;
            self.image_pixels = rgb.into_raw();
            let size = [rgba.width() as usize, rgba.height() as usize];
            self.texture = Some(ui.ctx().load_texture(
                "pinball",
                egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw()),
                Default::default(),
            ));
        }

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Generate a pinball NFT marker");
                    ui.add_space(4.0);
                    if let Some(texture) = &self.texture {
                        ui.add(egui::Image::new(texture))
                            .on_hover_text_at_pointer("Transformed image");
                        ui.add_space(4.0);

                        ui.scope(|ui| {
                            ui.spacing_mut().button_padding = egui::vec2(12.0, 8.0);

                            if ui
                                .add(
                                    egui::Button::new(self.button_text.clone())
                                        .fill(self.button_color)
                                        .corner_radius(4.0)
                                        .gap(4.0),
                                )
                                .clicked()
                            {
                                println!("NFT marker start to generate...");
                                match generate_nft_marker(
                                    &self.image_pixels,
                                    self.image_width,
                                    self.image_height,
                                    self.image_nc,
                                ) {
                                    Ok(_marker_data) => {
                                        println!("NFT marker generated");
                                        //println!("NFT marker generated: {} bytes", marker_data.len());
                                        /*if let Err(e) = fs::write("marker.iset", 0) {
                                            eprintln!("Failed to write marker file: {}", e);
                                        } else {
                                            println!("NFT marker file 'marker.iset' created successfully.");
                                        }*/
                                    }
                                    Err(e) => eprintln!("Generation error: {}", e),
                                }
                            }
                        });
                    }
                });
            });
        });
    }
}
