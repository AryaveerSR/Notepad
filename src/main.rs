#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::{
    egui::{
        self, Button, CentralPanel, Context, Direction, FontSelection, Layout, Response, Style,
        TextBuffer, TextEdit, TopBottomPanel, Ui, Visuals, Widget,
    },
    epaint::{vec2, Color32, FontFamily, FontId, Rounding, Stroke},
    App, Frame, NativeOptions,
};

use rfd::FileDialog;
use std::fs;

fn main() {
    let options = NativeOptions::default();
    eframe::run_native(
        "Notepad",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    content: String,
    opened_file: Option<String>,
    title: String,
    is_dark_mode: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            content: "".to_owned(),
            opened_file: None,
            title: "Notepad".to_owned(),
            is_dark_mode: true,
        }
    }
}

impl MyApp {
    fn open_file(&mut self) {
        if let Some(path) = FileDialog::new().pick_file() {
            self.opened_file = Some(path.display().to_string());
            self.title = format!("Notepad - {}", path.display().to_string());
            self.content = fs::read_to_string(path).unwrap();
        }
    }

    fn save_file(&mut self, path: String) {
        fs::write(path, self.content.clone()).unwrap();
    }

    fn save_file_as(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("Text file", &["txt"])
            .save_file()
        {
            self.save_file(path.display().to_string());
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        frame.set_window_title(&self.title);
        ctx.set_visuals(get_visuals(self.is_dark_mode));

        TopBottomPanel::top("file_btns").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if action_button("Open", ui).clicked() {
                    self.open_file();
                }
                if action_button("Save As", ui).clicked() {
                    self.save_file_as();
                }
                if action_button("Save", ui).clicked() {
                    if let Some(path) = &self.opened_file {
                        self.save_file(path.to_string());
                    } else {
                        self.save_file_as();
                    }
                }
                if action_button(
                    match self.is_dark_mode {
                        true => "Light Mode",
                        false => "Dark Mode",
                    },
                    ui,
                )
                .clicked()
                {
                    self.is_dark_mode = !self.is_dark_mode;
                }
            });
        });

        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("{} characters.", self.content.len()));
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                Layout::centered_and_justified(Direction::LeftToRight),
                |ui| {
                    ui.add(text_editor(&mut self.content, self.is_dark_mode));
                },
            );
        });
    }
}

fn action_button(text: &str, ui: &mut Ui) -> Response {
    Button::new(text)
        .fill(Color32::TRANSPARENT)
        .stroke(Stroke::new(1.0, Color32::TRANSPARENT))
        .ui(ui)
}

fn text_editor(text: &mut String, is_dark_mode: bool) -> TextEdit {
    TextEdit::multiline(text)
        .font(FontSelection::FontId(FontId::new(
            16.0,
            FontFamily::Monospace,
        )))
        .lock_focus(true)
        .margin(vec2(8.0, 8.0))
        .frame(false)
        .text_color(if is_dark_mode {
            Color32::WHITE
        } else {
            Color32::BLACK
        })
}

fn get_visuals(is_dark_mode: bool) -> Visuals {
    let mut visuals = match is_dark_mode {
        true => eframe::egui::Visuals::dark(),
        false => eframe::egui::Visuals::light(),
    };
    visuals.override_text_color = Some(match is_dark_mode {
        true => Color32::WHITE,
        false => Color32::BLACK,
    });
    visuals.widgets.noninteractive.bg_fill = match is_dark_mode {
        true => Color32::from_gray(25),
        false => Color32::from_gray(240),
    };
    visuals.widgets.noninteractive.bg_stroke = match is_dark_mode {
        true => Stroke::new(1.0, Color32::from_gray(50)),
        false => Stroke::new(1.0, Color32::from_gray(200)),
    };
    visuals
}
