#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{
    egui::{CentralPanel, Context, Direction, FontSelection, Layout, TextEdit, TopBottomPanel},
    epaint::{Color32, FontFamily, FontId},
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

impl Default for MyApp {
    fn default() -> Self {
        Self {
            content: "".to_owned(),
            opened_file: None,
            title: "Notepad".to_owned(),
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        frame.set_window_title(&self.title);
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Open").clicked() {
                    self.open_file();
                }
                if ui.button("Save As").clicked() {
                    self.save_file_as();
                }
                if ui.button("Save").clicked() {
                    if let Some(path) = &self.opened_file {
                        self.save_file(path.to_string());
                    } else {
                        self.save_file_as();
                    }
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                Layout::centered_and_justified(Direction::LeftToRight),
                |ui| {
                    ui.add(
                        TextEdit::multiline(&mut self.content)
                            .font(FontSelection::FontId(FontId::new(
                                18 as f32,
                                FontFamily::Monospace,
                            )))
                            .text_color(Color32::WHITE),
                    );
                },
            );
        });
    }
}
