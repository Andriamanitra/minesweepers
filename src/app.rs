use egui::{Color32, FontId, RichText, Vec2};

#[derive(Clone, Debug, PartialEq)]
pub enum Place {
    MarkedMine,
    IncorrectlyMarked(usize),
    Mine,
    Boom,
    Hidden(usize),
    Visible(usize),
}

#[derive(PartialEq)]
pub enum GameState {
    NotStarted,
    InProgress,
    Won,
    Lost,
}

pub struct Minefield {
    field: Vec<Vec<Place>>,
    width: usize,
    height: usize,
}

impl Minefield {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            field: vec![vec![Place::Hidden(0); width]; height],
            width,
            height,
        }
    }

    pub fn populate(&mut self, num_mines: usize, clicked_pos: (usize, usize)) {
        for row in self.field.as_mut_slice() {
            row.fill(Place::Hidden(0));
        }
        let mut rng = rand::thread_rng();
        for (r, c) in rand::seq::index::sample(&mut rng, self.width * self.height, num_mines + 1)
            .iter()
            .map(|i| (i / self.width, i % self.width))
            .filter(|x| *x != clicked_pos)
            .take(num_mines)
        {
            self.field[r][c] = Place::Mine;
            for row in self.field.iter_mut().take(r + 2).skip(r.saturating_sub(1)) {
                for elem in row.iter_mut().take(c + 2).skip(c.saturating_sub(1)) {
                    if let Place::Hidden(x) = elem {
                        *elem = Place::Hidden(*x + 1);
                    }
                }
            }
        }
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&Place> {
        self.field.get(r).and_then(|row| row.get(c))
    }

    fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut Place> {
        self.field.get_mut(r).and_then(|row| row.get_mut(c))
    }

    pub fn reset(&mut self) {
        for row in self.field.as_mut_slice() {
            row.fill(Place::Hidden(0));
        }
    }

    pub fn primary_click(&mut self, r: usize, c: usize) -> Option<&Place> {
        if let Some(place) = self.get_mut(r, c) {
            match place {
                Place::Hidden(x) => {
                    *place = Place::Visible(*x);
                }
                Place::Mine => {
                    *place = Place::Boom;
                }
                _ => (),
            }
            return Some(place);
        }
        None
    }

    pub fn secondary_click(&mut self, r: usize, c: usize) -> Option<&Place> {
        if let Some(place) = self.get_mut(r, c) {
            use Place::*;
            match place {
                Mine => *place = MarkedMine,
                Hidden(x) => *place = IncorrectlyMarked(*x),
                MarkedMine => *place = Mine,
                IncorrectlyMarked(x) => *place = Hidden(*x),
                _ => (),
            }
            return Some(place);
        }
        None
    }

    fn is_solved(&self) -> bool {
        for row in self.field.iter() {
            for place in row {
                match place {
                    Place::Boom => return false,
                    Place::Mine => return false,
                    Place::IncorrectlyMarked(_) => return false,
                    _ => (),
                }
            }
        }
        true
    }

    fn neighbors(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut neighbor_vec = vec![];
        if r > 0 {
            if c > 0 {
                neighbor_vec.push((r - 1, c - 1));
            }
            neighbor_vec.push((r - 1, c));
            if c + 1 < self.width {
                neighbor_vec.push((r - 1, c + 1));
            }
        }
        if c > 0 {
            neighbor_vec.push((r, c - 1));
        }
        if c + 1 < self.width {
            neighbor_vec.push((r, c + 1));
        }
        if r + 1 < self.height {
            if c > 0 {
                neighbor_vec.push((r + 1, c - 1));
            }
            neighbor_vec.push((r + 1, c));
            if c + 1 < self.width {
                neighbor_vec.push((r + 1, c + 1));
            }
        }
        neighbor_vec
    }
}

pub struct MinesweeperApp {
    game_state: GameState,
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
            game_state: GameState::NotStarted,
            num_mines: 20,
            minefield: Minefield::new(field_width, field_height),
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
        self.minefield.reset();
        self.game_state = GameState::NotStarted;
    }

    fn show_minefield(&mut self, ui: &mut egui::Ui) {
        let grid = egui::Grid::new("minefield")
            .min_col_width(self.minefield_button_size.x)
            .min_row_height(self.minefield_button_size.y)
            .spacing(Vec2 {
                x: self.minefield_spacing,
                y: self.minefield_spacing,
            });
        grid.show(ui, |ui| {
            for r in 0..self.minefield.height {
                for c in 0..self.minefield.width {
                    let mf_button = ui.add(
                        egui::widgets::Button::new(match &self.minefield.get(r, c) {
                            Some(Place::Visible(x)) => RichText::new(x.to_string()),
                            Some(Place::MarkedMine | Place::IncorrectlyMarked(_)) => {
                                RichText::new("🚩").color(Color32::RED)
                            }
                            Some(Place::Boom) => {
                                RichText::new("💥").color(Color32::from_rgb(255, 135, 0))
                            }
                            _ => RichText::new(""),
                        })
                        .min_size(self.minefield_button_size),
                    );
                    if mf_button.clicked() {
                        if self.game_state == GameState::NotStarted {
                            self.minefield.populate(self.num_mines, (r, c));
                            self.game_state = GameState::InProgress;
                        }
                        match self.minefield.primary_click(r, c) {
                            Some(Place::Boom) => {
                                self.game_state = GameState::Lost;
                            }
                            Some(Place::Visible(0)) => {
                                let mut q = self.minefield.neighbors(r, c);
                                while !q.is_empty() {
                                    let (r, c) = q.pop().unwrap();
                                    if let Some(&Place::Hidden(x)) = self.minefield.get(r, c) {
                                        self.minefield.primary_click(r, c);
                                        if x == 0 {
                                            q.extend(self.minefield.neighbors(r, c).iter());
                                        }
                                    }
                                }
                            }
                            _ => (),
                        }
                    };
                    if mf_button.secondary_clicked() {
                        if self.game_state == GameState::NotStarted {
                            self.minefield.populate(self.num_mines, (1337, 1337));
                            self.game_state = GameState::InProgress;
                        }
                        self.minefield.secondary_click(r, c);
                        if self.minefield.is_solved() {
                            self.game_state = GameState::Won;
                        }
                    }
                }
                ui.end_row();
            }
        });
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
            if ui.button("New game").clicked() {
                self.reset();
            }
            egui::warn_if_debug_build(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_minefield(ui);
            match self.game_state {
                GameState::Lost => {
                    ui.label(RichText::new("Game over!").font(FontId::proportional(24.0)));
                }
                GameState::Won => {
                    ui.label(RichText::new("You win!").font(FontId::proportional(24.0)));
                }
                _ => (),
            }
        });
    }
}
