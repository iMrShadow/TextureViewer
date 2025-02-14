use std::time::Duration;

use crate::{
    codecs::codec_manager::CodecManager,
    graphics::{
        pixel_format::PixelFormat, swizzling::Platform, texture::Texture,
        texture_utility::TextureEffects,
    },
    io::file_manager::FileManager,
};
use egui::{Button, OpenUrl, Rect, Scene, TextureHandle, Vec2};
use egui_notify::Toasts;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TextureViewer {
    #[serde(skip)]
    file_manager: FileManager,
    #[serde(skip)]
    request_texture_source_update: bool,
    #[serde(skip)]
    request_texture_update: bool,
    #[serde(skip)]
    display_texture: Option<TextureHandle>,
    #[serde(skip)]
    current_mip: u32,
    #[serde(skip)]
    current_item: u32,
    #[serde(skip)]
    max_mip: u32,
    #[serde(skip)]
    max_item: u32,
    #[serde(skip)]
    scene_rect: Rect,
    #[serde(skip)]
    texture_source: Option<Texture>,
    #[serde(skip)]
    codec_manager: CodecManager,
    #[serde(skip)]
    texture_effects: TextureEffects,
    #[serde(skip)]
    reset_view: bool,
    #[serde(skip)]
    display_compressed: bool,
    #[serde(skip)]
    toasts: Toasts,
}

impl Default for TextureViewer {
    fn default() -> Self {
        Self {
            file_manager: {
                let mut file_manager = FileManager::new().unwrap();
                file_manager
                    .set_filter_extensions(CodecManager::default().get_registered_extensions());

                file_manager
            },
            request_texture_update: false,
            request_texture_source_update: false,
            display_texture: None,
            current_mip: 0,
            current_item: 0,
            max_mip: 0,
            max_item: 0,
            scene_rect: Rect::ZERO,
            codec_manager: CodecManager::default(),
            texture_source: None,
            texture_effects: TextureEffects::default(),
            reset_view: false,
            display_compressed: false,
            toasts: {
                let mut toasts = Toasts::default();
                toasts = toasts.with_anchor(egui_notify::Anchor::BottomRight);
                toasts
            },
        }
    }
}

impl TextureViewer {
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
}

impl eframe::App for TextureViewer {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|input| {
            input.key_pressed(egui::Key::ArrowLeft) || input.key_pressed(egui::Key::A)
        }) {
            self.select_previous_file();
        }

        if ctx.input(|input| {
            input.key_pressed(egui::Key::ArrowRight) || input.key_pressed(egui::Key::D)
        }) {
            self.select_next_file();
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.display_menu(ui, ctx);
        });

        egui::TopBottomPanel::top("tools_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            ui.horizontal(|ui| {
                self.display_tool_bar(ui);
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                self.display_image_information(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.display_scene(ui, ctx);
        });
    }
}

impl TextureViewer {
    fn select_previous_file(&mut self) {
        self.file_manager.previous_file();
        if self.file_manager.get_selected_file().is_some() {
            self.request_texture_update = true;
            self.request_texture_source_update = true;
        }
    }

    fn select_next_file(&mut self) {
        self.file_manager.next_file();
        if self.file_manager.get_selected_file().is_some() {
            self.request_texture_update = true;
            self.request_texture_source_update = true;
        }
    }

    /// Display the toolbar at the top of the window.
    fn display_tool_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui
                .button("◀")
                .on_hover_text("Go to previous file")
                .clicked()
            {
                self.select_previous_file();
            }

            if ui.button("▶").on_hover_text("Go to next file").clicked() {
                self.select_next_file();
            }

            if ui
                .toggle_value(&mut self.texture_effects.channel_filter.0, "R")
                .on_hover_text("Toggle red channel")
                .changed()
            {
                self.request_texture_update = true;
            };

            if ui
                .toggle_value(&mut self.texture_effects.channel_filter.1, "G")
                .on_hover_text("Toggle green channel")
                .changed()
            {
                self.request_texture_update = true;
            };

            if ui
                .toggle_value(&mut self.texture_effects.channel_filter.2, "B")
                .on_hover_text("Toggle blue channel")
                .changed()
            {
                self.request_texture_update = true;
            };

            if ui
                .toggle_value(&mut self.texture_effects.channel_filter.3, "A")
                .on_hover_text("Toggle alpha channel")
                .changed()
            {
                self.request_texture_update = true;
            };

            if ui
                .button("⟲")
                .on_hover_text("Rotate 90 degrees left")
                .clicked()
            {
                self.texture_effects.rotate_90_left_count += 1;
                self.request_texture_update = true;
                self.reset_view = true;
            }
            if ui
                .button("⟳")
                .on_hover_text("Rotate 90 degrees right")
                .clicked()
            {
                self.texture_effects.rotate_90_right_count += 1;
                self.request_texture_update = true;
                self.reset_view = true;
            }
            if ui.button("↕").on_hover_text("Flip vertical").clicked() {
                self.texture_effects.flip_vertical_count += 1;
                self.request_texture_update = true;
                self.reset_view = true;
            }
            if ui.button("↔").on_hover_text("Flip horizontal").clicked() {
                self.texture_effects.flip_horizontal_count += 1;
                self.request_texture_update = true;
                self.reset_view = true;
            }

            if self.max_item > 0 {
                if ui
                    .add(egui::Slider::new(&mut self.current_item, 0..=self.max_item).text("Item"))
                    .on_hover_text("Current array element being displayed")
                    .changed()
                {
                    self.request_texture_update = true;
                }
            }

            if self.max_mip > 0 {
                if ui
                    .add(egui::Slider::new(&mut self.current_mip, 0..=self.max_mip).text("Mip"))
                    .on_hover_text("Current mip being displayed")
                    .changed()
                {
                    self.request_texture_update = true;
                }
            }

            if ui
                .checkbox(&mut self.display_compressed, "Display Compressed")
                .changed()
            {
                self.request_texture_update = true;
            }
        });
    }

    /// Display the menu bar at the top of the window.
    fn display_menu(&mut self, ui: &mut egui::Ui, ctx: &eframe::egui::Context) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter(
                            "Supported Images",
                            &self.codec_manager.get_registered_extensions(),
                        )
                        .pick_file()
                    {
                        match self
                            .file_manager
                            .from_folder(path.parent().unwrap().to_path_buf())
                        {
                            Ok(file_manager) => {
                                self.request_texture_source_update = true;
                                self.request_texture_update = true;
                                file_manager
                            }
                            Err(e) => {
                                self.show_error(&e.to_string());
                                self.log_error(&e.to_string());
                            }
                        };

                        self.file_manager.set_selected_file(path.to_path_buf());
                    }

                    ui.close_menu();
                }

                if ui.button("Open Directory").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        match self.file_manager.from_folder(path.to_path_buf()) {
                            Ok(file_manager) => {
                                self.request_texture_source_update = true;
                                self.request_texture_update = true;
                                file_manager
                            }
                            Err(e) => {
                                self.show_error(&e.to_string());
                                self.log_error(&e.to_string());
                            }
                        };
                    }

                    ui.close_menu();
                }

                if ui
                    .add_enabled(self.texture_source.is_some(), Button::new("Save As"))
                    .clicked()
                {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("PNG", &["png"])
                        .add_filter("JPEG", &["jpg", "jpeg"])
                        .add_filter("BMP", &["bmp"])
                        .add_filter("TGA", &["tga"])
                        .add_filter("DDS", &["dds"])
                        .save_file()
                    {
                        match self.codec_manager.save_to_file(
                            &path.to_path_buf(),
                            &self.texture_source.as_ref().unwrap(),
                            &self.texture_effects,
                        ) {
                            Ok(_) => {}
                            Err(e) => {
                                self.show_error(&e.to_string());
                                self.log_error(&e.to_string());
                            }
                        };
                    }

                    match self.file_manager.refresh() {
                        Ok(file_manager) => file_manager,
                        Err(e) => {
                            self.show_error(&e.to_string());
                            self.log_error(&e.to_string());
                        }
                    };

                    ui.close_menu();
                }

                ui.separator();

                if ui.button("Previous File").clicked() {
                    self.select_previous_file();
                    ui.close_menu();
                }

                if ui.button("Next File").clicked() {
                    self.select_next_file();
                    ui.close_menu();
                }

                ui.separator();
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.menu_button("Edit", |ui| {
                ui.menu_button("Pixel Format", |ui| {
                    let mut selected_pixel_format = self.texture_effects.pixel_format;

                    let vec_pixel_format = vec![
                        PixelFormat::B5G6R5,
                        PixelFormat::B5G5R5A1,
                        PixelFormat::R8G8B8,
                        PixelFormat::B8G8R8,
                        PixelFormat::R8G8B8A8,
                        PixelFormat::B8G8R8A8,
                        PixelFormat::R16G16B16A16,
                        PixelFormat::R32G32B32A32,
                        PixelFormat::BC1,
                        PixelFormat::BC2,
                        PixelFormat::BC3,
                        PixelFormat::BC4,
                        PixelFormat::BC5,
                        PixelFormat::BC6H,
                        // PixelFormat::BC7, currently BC7 takes too long to compress
                    ];

                    for format in vec_pixel_format {
                        if ui
                            .selectable_value(
                                &mut selected_pixel_format,
                                format,
                                format.to_string(),
                            )
                            .clicked()
                        {
                            self.texture_effects.pixel_format = selected_pixel_format;
                            self.request_texture_update = true;
                            ui.close_menu();
                        }
                    }
                });

                ui.separator();
                let platforms = vec![
                    None,
                    Some(Platform::PS4),
                    Some(Platform::Xbox360),
                    Some(Platform::PSVita),
                ];

                ui.menu_button("Swizzle", |ui| {
                    let mut selected_platform = self.texture_effects.swizzle;

                    for platform in &platforms {
                        if ui
                            .selectable_value(
                                &mut selected_platform,
                                *platform,
                                match platform {
                                    Some(p) => p.to_string(),
                                    None => "None".to_string(),
                                },
                            )
                            .clicked()
                        {
                            self.texture_effects.swizzle = *platform;
                            self.request_texture_update = true;
                            ui.close_menu();
                        }
                    }
                });

                ui.menu_button("Deswizzle", |ui| {
                    let mut selected_platform = self.texture_effects.deswizzle;

                    for platform in &platforms {
                        if ui
                            .selectable_value(
                                &mut selected_platform,
                                *platform,
                                match platform {
                                    Some(p) => p.to_string(),
                                    None => "None".to_string(),
                                },
                            )
                            .clicked()
                        {
                            self.texture_effects.deswizzle = *platform;
                            self.request_texture_update = true;
                            ui.close_menu();
                        }
                    }
                });
            });

            if ui.button("About").clicked() {
                ctx.open_url(OpenUrl::new_tab(
                    "https://github.com/iMrShadow/TextureViewer",
                ));
                ui.close_menu();
            }
        });

        self.toasts.show(ctx);
    }

    /// Update the display texture.
    fn update_display_texture(&mut self, ctx: &eframe::egui::Context) {
        if self.request_texture_source_update {
            let path = self.file_manager.get_selected_file().unwrap();

            self.texture_source = match CodecManager::load_from_file(&self.codec_manager, &path) {
                Ok(texture) => Some(texture),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    Some(Texture::new())
                }
            };

            // Reset all texture settings
            self.current_mip = 0;
            self.current_item = 0;
            self.max_mip = self
                .texture_source
                .as_ref()
                .unwrap_or(&Texture::new())
                .metadata
                .mip_levels
                - 1;
            self.max_item = self
                .texture_source
                .as_ref()
                .unwrap_or(&Texture::new())
                .metadata
                .array_size
                - 1;
            self.request_texture_source_update = false;
            self.reset_view = true;
            self.texture_effects = TextureEffects::default();
        }

        if let Some(texture_source) = &self.texture_source {
            let image = match texture_source.get_image(self.current_mip, self.current_item, 0) {
                Ok(image) => image,
                Err(e) => {
                    self.show_error(&e.to_string());
                    self.log_error(&e.to_string());
                    self.request_texture_update = false;
                    return;
                }
            };

            // Get the RGBA8 pixels
            let rgba8_image = match self
                .texture_effects
                .get_transformed_rgba8_pixels(&image, self.display_compressed)
            {
                Ok(rgba8_image) => rgba8_image,
                Err(e) => {
                    self.show_error(&e.to_string());
                    self.log_error(&e.to_string());
                    self.request_texture_update = false;
                    return;
                }
            };

            // Construct the color image
            let image = egui::ColorImage::from_rgba_unmultiplied(
                [rgba8_image.0 as usize, rgba8_image.1 as usize],
                &rgba8_image.2,
            );

            let texture_options = egui::TextureOptions {
                magnification: egui::TextureFilter::Linear,
                minification: egui::TextureFilter::Linear,
                wrap_mode: egui::TextureWrapMode::ClampToEdge,
                mipmap_mode: None,
            };

            // Load the texture
            self.display_texture =
                Some(ctx.load_texture("display_texture", image, texture_options));

            self.request_texture_update = false;
        }
    }

    /// Display the texture in the scene.
    fn display_scene(&mut self, ui: &mut egui::Ui, ctx: &eframe::egui::Context) {
        if self.request_texture_update {
            self.update_display_texture(ctx);
        }

        let scene = Scene::new()
            .max_inner_size([350.0, 1000.0])
            .zoom_range(0.1..=20.0);

        let mut inner_rect = Rect::NAN;

        let response = scene
            .show(ui, &mut self.scene_rect, |ui| {
                if let Some(texture) = &self.display_texture {
                    if let Some(texture_source) = &self.texture_source {
                        let dimensions = Vec2::new(
                            texture_source.metadata.width as f32,
                            texture_source.metadata.height as f32,
                        );
                        let dimensions = match (self.texture_effects.rotate_90_left_count as i32
                            % 2)
                            - (self.texture_effects.rotate_90_right_count as i32 % 2)
                        {
                            0 => dimensions,
                            _ => Vec2::new(dimensions.y, dimensions.x),
                        };

                        let sized_texture = egui::load::SizedTexture::new(texture, dimensions);

                        ui.add(egui::Image::new(sized_texture).fit_to_exact_size(dimensions));
                    }
                }

                inner_rect = ui.min_rect();
            })
            .response;

        if self.reset_view || response.double_clicked() {
            self.scene_rect = inner_rect;
            self.reset_view = false;
        }
    }

    /// Display the image information at the bottom of the window.
    fn display_image_information(&mut self, ui: &mut egui::Ui) {
        let current_path = match self.file_manager.get_selected_file() {
            Some(path) => path.display().to_string(),
            None => "No Selected File".to_owned(),
        };

        if let Some(texture_source) = &self.texture_source {
            ui.label(format!(
                "{} | Size: {} x {} x {} px | Format: {} | Array Elements: {} | Mips: {}",
                current_path,
                texture_source.metadata.width,
                texture_source.metadata.height,
                texture_source.metadata.depth,
                texture_source.metadata.pixel_format_info.pixel_format,
                texture_source.metadata.array_size,
                texture_source.metadata.mip_levels
            ));
        } else {
            ui.label(current_path);
        }
    }

    /// Display an error message as a notification.
    fn show_error(&mut self, message: &str) {
        self.toasts
            .error(message)
            .duration(Some(Duration::from_secs(5)));
    }

    fn log_error(&mut self, message: &str) {
        eprintln!("Error: {}", message);
    }
}
