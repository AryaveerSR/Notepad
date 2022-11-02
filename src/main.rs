#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{
    egui::{self, Direction, FontSelection, TextFormat, Ui},
    epaint::{Color32, FontFamily, FontId},
};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Notepad",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    content: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            content: "".to_owned(),
        }
    }
}

fn highlighter(string: &str) -> egui::text::LayoutJob {
    let mut layout_job = egui::text::LayoutJob::default();
    string.lines().for_each(|line| {
        let mut line = line.to_owned();
        line.push_str("\n");
        layout_job.append(
            &line,
            0 as f32,
            TextFormat::simple(
                FontId::monospace(18 as f32),
                match line.chars().next().unwrap() {
                    '#' => Color32::BLUE,
                    '>' => Color32::GRAY,
                    '*' => Color32::GREEN,
                    _ => Color32::WHITE,
                },
            ),
        );
    });
    layout_job
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // if ui.button("E").clicked() {} **TODO**
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(Direction::LeftToRight),
                |ui| {
                    let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                        let mut layout_job: egui::text::LayoutJob = highlighter(string);
                        layout_job.wrap.max_width = wrap_width;
                        ui.fonts().layout_job(layout_job)
                    };

                    ui.add(
                        egui::TextEdit::multiline(&mut self.content)
                            .font(FontSelection::FontId(FontId::new(
                                18 as f32,
                                FontFamily::Monospace,
                            )))
                            .text_color(Color32::from_gray(200))
                            .layouter(&mut layouter),
                    );
                },
            );
        });
    }
}
