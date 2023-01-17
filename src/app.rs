use egui::Vec2;

pub struct MinesweeperApp {
    field_width: u32,
    field_height: u32,
    num_mines: u32,
    minefield_button_size: Vec2,
    minefield_spacing: f32,
}

impl Default for MinesweeperApp {
    fn default() -> Self {
        Self {
            field_width: 12,
            field_height: 8,
            num_mines: 20,
            minefield_button_size: Vec2 { x: 32.0, y: 32.0 },
            minefield_spacing: 4.0,
        }
    }
}

impl MinesweeperApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for MinesweeperApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            field_width,
            field_height,
            num_mines,
            minefield_button_size,
            minefield_spacing,
        } = self;

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");
            egui::warn_if_debug_build(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("minefield")
                .min_col_width(minefield_button_size.x)
                .min_row_height(minefield_button_size.y)
                .spacing(Vec2 {
                    x: *minefield_spacing,
                    y: *minefield_spacing,
                })
                .show(ui, |ui| {
                    for r in 0..*field_height {
                        for c in 0..*field_width {
                            let mf_button =
                                egui::widgets::Button::new("").min_size(*minefield_button_size);
                            if ui.add(mf_button).clicked() {
                                println!("click");
                            };
                        }
                        ui.end_row();
                    }
                });
        });
    }
}
