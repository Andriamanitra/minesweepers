use std::cmp::min;

use egui::Vec2;
use rand::seq;

#[derive(Clone, Debug)]
enum Place {
    CorrectlyMarked,
    IncorrectlyMarked,
    Mine,
    Hidden,
    Visible(usize),
}

enum BoomOrNoBoom {
    Boom,
    NoBoom,
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
            field_width: field_width,
            field_height: field_height,
            num_mines: 20,
            minefield: vec![vec![Place::Hidden; field_width]; field_height],
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
            row.fill(Place::Hidden);
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
                            let button_label = match self.minefield[r][c] {
                                Place::Visible(x) => x.to_string(),
                                Place::CorrectlyMarked => "🚩".to_string(),
                                Place::IncorrectlyMarked => "🚩".to_string(),
                                _ => "".to_string(),
                            };
                            let mf_button = ui.add(
                                egui::widgets::Button::new(button_label)
                                    .min_size(self.minefield_button_size),
                            );
                            if mf_button.clicked() {
                                if !started {
                                    populate(&mut self.minefield, self.num_mines);
                                    self.game_started = true;
                                }
                                if let BoomOrNoBoom::Boom = click(&mut self.minefield, r, c) {
                                    self.reset()
                                }
                            };
                            if mf_button.secondary_clicked() {
                                self.minefield[r][c] = match self.minefield[r][c] {
                                    Place::Mine => Place::CorrectlyMarked,
                                    Place::Hidden => Place::IncorrectlyMarked,
                                    Place::CorrectlyMarked => Place::Mine,
                                    Place::IncorrectlyMarked => Place::Hidden,
                                    Place::Visible(x) => Place::Visible(x),
                                }
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    }
}

fn click(field: &mut Minefield, r: usize, c: usize) -> BoomOrNoBoom {
    use BoomOrNoBoom::*;
    let height = field.len();
    let width = field.first().unwrap().len();
    match field[r][c] {
        Place::Mine => {
            println!("boom!");
            return Boom;
        }
        Place::Hidden => {
            let mut mine_count = 0;
            for nr in r.saturating_sub(1)..=min(r + 1, height - 1) {
                for nc in c.saturating_sub(1)..=min(c + 1, width - 1) {
                    match field[nr][nc] {
                        Place::Mine => mine_count += 1,
                        Place::CorrectlyMarked => mine_count += 1,
                        _ => {}
                    }
                }
            }
            field[r][c] = Place::Visible(mine_count);
        }
        _ => {}
    }
    return NoBoom;
}

fn populate(field: &mut Minefield, num_mines: usize) {
    let height = field.len();
    let width = field.first().unwrap().len();
    let mut rng = rand::thread_rng();
    for i in seq::index::sample(&mut rng, width * height, num_mines) {
        field[i / width][i % width] = Place::Mine;
    }
}
