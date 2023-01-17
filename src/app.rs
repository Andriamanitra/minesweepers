use egui::Vec2;
use rand::seq;

#[derive(Clone, Debug, PartialEq)]
enum Place {
    MarkedMine,
    IncorrectlyMarked(usize),
    Mine,
    Boom,
    Hidden(usize),
    Visible(usize),
}

type Minefield = Vec<Vec<Place>>;

pub struct MinesweeperApp {
    game_started: bool,
    field_width: usize,
    field_height: usize,
    num_mines: usize,
    minefield: Minefield,
    minefield_button_size: Vec2,
    minefield_spacing: f32,
}

impl Default for MinesweeperApp {
    fn default() -> Self {
        let field_width = 12;
        let field_height = 8;
        Self {
            game_started: false,
            field_width,
            field_height,
            num_mines: 20,
            minefield: vec![vec![Place::Hidden(0); field_width]; field_height],
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
    pub fn reset(&mut self) {
        for row in self.minefield.as_mut_slice() {
            row.fill(Place::Hidden(0));
        }
        self.game_started = false;
    }
}

impl eframe::App for MinesweeperApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New game").clicked() {
                        self.reset();
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");
            if ui.button("New game").clicked() {
                self.reset();
            }
            egui::warn_if_debug_build(ui);
        });

        let started: bool = self.game_started;
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("minefield")
                .min_col_width(self.minefield_button_size.x)
                .min_row_height(self.minefield_button_size.y)
                .spacing(Vec2 {
                    x: self.minefield_spacing,
                    y: self.minefield_spacing,
                })
                .show(ui, |ui| {
                    for r in 0..self.field_height {
                        for c in 0..self.field_width {
                            use egui::{Color32, RichText};
                            let state = &self.minefield[r][c];
                            let mf_button = ui.add(
                                egui::widgets::Button::new(match state {
                                    Place::Visible(x) => RichText::new(x.to_string()),
                                    Place::MarkedMine | Place::IncorrectlyMarked(_) => {
                                        RichText::new("🚩").color(Color32::RED)
                                    }
                                    Place::Boom => {
                                        RichText::new("💥").color(Color32::from_rgb(255, 135, 0))
                                    }
                                    _ => RichText::new(""),
                                })
                                .min_size(self.minefield_button_size),
                            );
                            if mf_button.clicked() {
                                if !started {
                                    populate(&mut self.minefield, self.num_mines, (r, c));
                                    self.game_started = true;
                                }
                                match self.minefield[r][c] {
                                    Place::Mine => {
                                        println!("boom!");
                                        self.minefield[r][c] = Place::Boom;
                                        self.game_started = false;
                                    }
                                    Place::Hidden(x) => {
                                        self.minefield[r][c] = Place::Visible(x);
                                    }
                                    _ => {}
                                }
                            };
                            if mf_button.secondary_clicked() {
                                self.minefield[r][c] = match self.minefield[r][c] {
                                    Place::Mine => Place::MarkedMine,
                                    Place::Hidden(x) => Place::IncorrectlyMarked(x),
                                    Place::MarkedMine => Place::Mine,
                                    Place::IncorrectlyMarked(x) => Place::Hidden(x),
                                    Place::Visible(x) => Place::Visible(x),
                                    Place::Boom => Place::Boom,
                                }
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    }
}

fn populate(field: &mut Minefield, num_mines: usize, clicked_pos: (usize, usize)) {
    for row in field.as_mut_slice() {
        row.fill(Place::Hidden(0));
    }
    let height = field.len();
    let width = field.first().unwrap().len();
    let mut rng = rand::thread_rng();
    for (r, c) in seq::index::sample(&mut rng, width * height, num_mines + 1)
        .iter()
        .map(|i| (i / width, i % width))
        .filter(|x| *x != clicked_pos)
        .take(num_mines)
    {
        field[r][c] = Place::Mine;
        for row in field.iter_mut().take(r + 2).skip(r.saturating_sub(1)) {
            for elem in row.iter_mut().take(c + 2).skip(c.saturating_sub(1)) {
                if let Place::Hidden(x) = elem {
                    *elem = Place::Hidden(*x + 1);
                }
            }
        }
    }
}
