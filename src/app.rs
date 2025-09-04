use crate::tree::{self, Tree, Index};
use egui::{Frame, Ui};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    root: Tree,
    focus: Index,
}

impl Default for App {
    fn default() -> Self {
        Self {
            root: tree::big_tree(2, 3),
            focus: Index::default(),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if false {
            if let Some(storage) = cc.storage {
                eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
            } else {
                Default::default()
            }
        } else {
            Default::default()
        }
    }

    fn render_tree(&mut self, ui: &mut Ui) {
        fn go(app: &mut App, ui: &mut Ui, outside_focus: bool, tree: &Tree, index: Index) {
            Frame::new()
                .inner_margin(12)
                .outer_margin(12)
                .corner_radius(12)
                .shadow(egui::Shadow {
                    offset: [8, 12],
                    blur: 12,
                    spread: 0,
                    color: egui::Color32::from_black_alpha(180),
                })
                .fill(egui::Color32::BLACK)
                .stroke(if outside_focus && index.len() == app.focus.len() {
                    egui::Stroke::new(2.0, egui::Color32::WHITE)
                } else {
                    egui::Stroke::new(2.0, egui::Color32::DARK_GRAY)
                })
                .show(ui, |ui| {
                    ui.label(egui::RichText::new(tree.label.clone()).color(egui::Color32::WHITE));
                    for (i, kid) in tree.kids.iter().enumerate() {
                        let mut index_kid = index.clone();
                        index_kid.push(i);

                        let outside_focus_kid = outside_focus
                            && match app.focus.get(index_kid.len() - 1) {
                                None => false,
                                Some(j) => i == j,
                            };

                        go(app, ui, outside_focus_kid, kid, index_kid);
                    }
                });
        }

        go(self, ui, true, &self.root.clone(), Index::default());
    }
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let focus_old = self.focus.clone();
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            self.focus.move_up();
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            self.focus.move_down(0);
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            self.focus.move_left_sibling();
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            self.focus.move_right_sibling();
        }

        // if move went out of bounds, then reset it
        if !self.root.index_in_bounds(&self.focus) {
            self.focus = focus_old;
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
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

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("tree-editor-with-egui");

            ui.label(format!("focus: {:?}", self.focus));

            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                self.render_tree(ui);
            });
        });
    }
}
