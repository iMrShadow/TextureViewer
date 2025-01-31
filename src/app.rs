use crate::{
    codecs::codec_manager::CodecManager, graphics::texture::Texture, io::file_manager::FileManager,
};
use egui::{Image, Modifiers, Rect, Scene, TextureHandle, Vec2};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TextureViewerApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)]
    file_manager: FileManager,

    request_texture_source_update: bool,
    request_texture_update: bool,

    #[serde(skip)]
    display_texture: Option<TextureHandle>,

    #[serde(skip)]
    image: Option<Image<'static>>,

    #[serde(skip)]
    enable_red_channel: bool,
    #[serde(skip)]
    enable_green_channel: bool,
    #[serde(skip)]
    enable_blue_channel: bool,
    #[serde(skip)]
    enable_alpha_channel: bool,

    #[serde(skip)]
    current_mip: u32,
    #[serde(skip)]
    current_slice: u32,
    #[serde(skip)]
    max_mip: u32,
    #[serde(skip)]
    max_slice: u32,

    #[serde(skip)]
    gamma: f32,
    #[serde(skip)]
    drag_value: f32,
    #[serde(skip)]
    scene_rect: Rect,
    #[serde(skip)]
    texture_source: Option<Texture>,

    #[serde(skip)]
    codec_manager: CodecManager,
}

impl Default for TextureViewerApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            file_manager: {
                let mut file_manager = FileManager::new().unwrap();
                file_manager.set_filter_extensions(
                    CodecManager::new_codec_manager().get_registered_extensions(),
                );

                file_manager
            },
            request_texture_update: false,
            request_texture_source_update: false,
            display_texture: None,
            image: None,
            enable_red_channel: true,
            enable_green_channel: true,
            enable_blue_channel: true,
            enable_alpha_channel: true,
            current_mip: 0,
            current_slice: 0,
            max_mip: 0,
            max_slice: 0,
            gamma: 0.0,
            drag_value: 0.0,
            scene_rect: Rect::ZERO,
            codec_manager: CodecManager::new_codec_manager(),
            texture_source: None,
        }
    }
}

impl TextureViewerApp {
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

impl eframe::App for TextureViewerApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    let reset_shortcut = egui::KeyboardShortcut::new(
                        Modifiers::CTRL | Modifiers::SHIFT,
                        egui::Key::R,
                    );

                    let next_image_shortcut =
                        egui::KeyboardShortcut::new(Modifiers::NONE, egui::Key::ArrowRight);
                    let previous_image_shortcut =
                        egui::KeyboardShortcut::new(Modifiers::NONE, egui::Key::ArrowLeft);

                    if ui
                        .add(
                            egui::Button::new("Open")
                                .shortcut_text(ui.ctx().format_shortcut(&reset_shortcut)),
                        )
                        .clicked()
                    {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.request_texture_update = true;

                            match self
                                .file_manager
                                .from_folder(path.parent().unwrap().to_path_buf())
                            {
                                Ok(filemanager) => {
                                    self.request_texture_source_update = true;
                                    self.request_texture_update = true;
                                    filemanager
                                }
                                Err(e) => {
                                    eprintln!("Error: {}", e);
                                }
                            };

                            self.file_manager.set_selected_file(path.to_path_buf());
                        }
                    }

                    if ui
                        .add(
                            egui::Button::new("Open Directory")
                                .shortcut_text(ui.ctx().format_shortcut(&reset_shortcut)),
                        )
                        .clicked()
                    {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            match self.file_manager.from_folder(path.to_path_buf()) {
                                Ok(filemanager) => {
                                    self.request_texture_source_update = true;
                                    self.request_texture_update = true;
                                    filemanager
                                }
                                Err(e) => {
                                    eprintln!("Error: {}", e);
                                }
                            };
                        }
                    }

                    ui.separator();

                    if ui
                        .add(
                            egui::Button::new("Previous")
                                .shortcut_text(ui.ctx().format_shortcut(&next_image_shortcut)),
                        )
                        .clicked()
                    {
                        self.file_manager.previous_file();
                        self.request_texture_update = true;
                        self.request_texture_source_update = true;
                    }

                    if ui
                        .add(
                            egui::Button::new("Next")
                                .shortcut_text(ui.ctx().format_shortcut(&previous_image_shortcut)),
                        )
                        .clicked()
                    {
                        self.file_manager.previous_file();
                        self.request_texture_update = true;
                        self.request_texture_source_update = true;
                    }

                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Edit", |ui| {});

                ui.menu_button("Help", |ui| {});

                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::TopBottomPanel::top("tools_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            ui.horizontal(|ui| {
                if ui
                    .toggle_value(&mut self.enable_red_channel, "red")
                    .changed()
                {
                    self.request_texture_update = true;
                };

                if ui
                    .toggle_value(&mut self.enable_green_channel, "green")
                    .changed()
                {
                    self.request_texture_update = true;
                };

                if ui
                    .toggle_value(&mut self.enable_blue_channel, "blue")
                    .changed()
                {
                    self.request_texture_update = true;
                };

                if ui
                    .toggle_value(&mut self.enable_alpha_channel, "alpha")
                    .changed()
                {
                    self.request_texture_update = true;
                };

                let left_arrow_icon = egui::include_image!("../assets/left-arrow.png");
                let right_arrow_icon = egui::include_image!("../assets/right-arrow.png");

                if ui
                    .add(egui::ImageButton::new(left_arrow_icon))
                    .clicked()
                {
                    self.file_manager.previous_file();
                    if self.file_manager.get_selected_file().is_some() {
                        self.request_texture_update = true;
                        self.request_texture_source_update = true;
                    }
                }

                if ui
                .add(egui::ImageButton::new(right_arrow_icon))
                    .clicked()
                {
                    self.file_manager.next_file();
                    if self.file_manager.get_selected_file().is_some() {
                        self.request_texture_update = true;
                        self.request_texture_source_update = true;
                    }
                }

                // let _ = ui.button("rotate 90 left");
                // let _ = ui.button("rotate 90 right");
                // let _ = ui.button("flip up/down");
                // let _ = ui.button("flip left/right");
                if ui
                    .add(egui::Slider::new(&mut self.current_mip, 0..=self.max_mip).text("Mip"))
                    .changed()
                {
                    self.request_texture_update = true;
                }

                if ui
                    .add(
                        egui::Slider::new(&mut self.current_slice, 0..=self.max_slice)
                            .text("Slice"),
                    )
                    .changed()
                {
                    self.request_texture_update = true;
                }

                // egui::ComboBox::from_label("Pixel Format")
                //     .selected_text(format!("{:?}", &mut self.pixel_format))
                //     .show_ui(ui, |ui| {
                //         let _ = ui.button("WTF");
                //         ui.selectable_value(
                //             &mut self.pixel_format,
                //             PixelFormat::R8Unorm,
                //             "R8Unorm",
                //         );
                //         ui.selectable_value(
                //             &mut self.pixel_format,
                //             PixelFormat::LA8Unorm,
                //             "LA8Unorm",
                //         );
                //         ui.selectable_value(
                //             &mut self.pixel_format,
                //             PixelFormat::RG8Unorm,
                //             "RG8Unorm",
                //         );
                //     });
                // egui::ComboBox::from_label("Gamma")
                //     .selected_text(format!("{:?}", &mut self.gamma))
                //     .show_ui(ui, |ui| {
                //         ui.add(egui::Slider::new(&mut self.gamma, 0.0..=10.0));
                //         ui.selectable_value(&mut self.gamma, 1.0, "1.0");
                //         ui.selectable_value(&mut self.gamma, 1.2, "1.2");
                //         ui.selectable_value(&mut self.gamma, 1.4, "1.4");
                //     });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let current_path = match self.file_manager.get_selected_file() {
                    Some(path) => path.display().to_string(),
                    None => "No Selected File".to_owned(),
                };

                ui.heading(current_path);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut reset_view = false;

            if self.request_texture_update {
                if self.request_texture_source_update {
                    let path = self.file_manager.get_selected_file().unwrap();

                    self.texture_source =
                        match CodecManager::load_from_file(&self.codec_manager, &path) {
                            Ok(texture) => Some(texture),
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                Some(Texture::new())
                            }
                        };

                    self.current_mip = 0;
                    self.current_slice = 0;
                    self.max_mip = self
                        .texture_source
                        .as_ref()
                        .unwrap_or(&Texture::new())
                        .metadata
                        .mip_levels
                        - 1;
                    self.max_slice = self
                        .texture_source
                        .as_ref()
                        .unwrap_or(&Texture::new())
                        .metadata
                        .array_size
                        - 1;
                    self.request_texture_source_update = false;
                    reset_view = true;
                }

                if let Some(texture_source) = &self.texture_source {
                    let (width, height, mut pixels) =
                        texture_source.get_rgba8_data(self.current_mip, self.current_slice, 0);

                    for pixel in pixels.chunks_exact_mut(4) {
                        pixel[0] = pixel[0] * (self.enable_red_channel as u8);
                        pixel[1] = pixel[1] * (self.enable_green_channel as u8);
                        pixel[2] = pixel[2] * (self.enable_blue_channel as u8);
                        pixel[3] = if self.enable_alpha_channel {
                            pixel[3]
                        } else {
                            255
                        };
                    }

                    let image = egui::ColorImage::from_rgba_unmultiplied(
                        [width as usize, height as usize],
                        &pixels,
                    );

                    let texture_options = egui::TextureOptions {
                        magnification: egui::TextureFilter::Linear,
                        minification: egui::TextureFilter::Linear,
                        wrap_mode: egui::TextureWrapMode::ClampToEdge,
                        mipmap_mode: None,
                    };

                    let texture = ctx.load_texture("example_texture", image, texture_options);
                    self.display_texture = Some(texture);

                    self.request_texture_update = false;
                }
            }

            // Could be useful for mip and slice selection
            //     egui::Area::new(egui::Id::new("my_area"))
            //    // .anchor(egui::Align2::LEFT_TOP, [0.0, 0.0]).order(egui::Order::Foreground)
            //     .show(ctx, |ui| {
            //         ui.label("Floating text!");
            //     });

            let scene = Scene::new()
                .max_inner_size([350.0, 1000.0])
                .zoom_range(0.1..=20.0);

            let mut inner_rect = Rect::NAN;

            let response = scene
                .show(ui, &mut self.scene_rect, |ui| {
                    if let Some(texture) = &self.display_texture {
                        let size = texture.size_vec2();

                        if let Some(texture_source) = &self.texture_source {
                            let size = Vec2::new(
                                texture_source.metadata.width as f32,
                                texture_source.metadata.height as f32,
                            );

                            let sized_texture = egui::load::SizedTexture::new(texture, size);
                            ui.add(egui::Image::new(sized_texture).fit_to_exact_size(size));
                        }
                    }

                    inner_rect = ui.min_rect();
                })
                .response;

            if reset_view || response.double_clicked() {
                self.scene_rect = inner_rect;
            }
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
