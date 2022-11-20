#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::{
    egui::{
        self, Button, CentralPanel, Context, Direction, FontSelection, Layout, Response, TextEdit,
        TopBottomPanel, Ui, Visuals, Widget, WidgetText,
    },
    emath::Align,
    epaint::{vec2, Color32, FontFamily, FontId, Rect, Stroke},
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
    title: String,
    is_dark_mode: bool,
    tabs: Vec<Tab>,
    current_tab: usize,
}
#[derive(Clone, Debug)]
struct Tab {
    opened_file: Option<String>,
    content: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            title: "Notepad".to_owned(),
            is_dark_mode: true,
            current_tab: 0,
            tabs: vec![Tab {
                opened_file: None,
                content: "".to_owned(),
            }],
        }
    }
}

impl MyApp {
    fn open_file(&mut self) {
        if let Some(path) = FileDialog::new().pick_file() {
            self.new_tab();
            self.tabs[self.current_tab].opened_file = Some(path.display().to_string());
            self.title = format!("Notepad - {}", path.display().to_string());
            self.tabs[self.current_tab].content = fs::read_to_string(path).unwrap();
        }
    }

    fn save_file(&mut self, path: String) {
        fs::write(path, self.tabs[self.current_tab].content.clone()).unwrap();
    }

    fn save_file_as(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("Text file", &["txt"])
            .save_file()
        {
            self.save_file(path.display().to_string());
            self.tabs[self.current_tab].opened_file = Some(path.display().to_string());
            self.title = format!("Notepad - {}", path.display().to_string());
        }
    }

    fn remove_tab(&mut self, index: usize) {
        if self.tabs.len() > 0 {
            self.tabs.remove(index);
            if (self.current_tab == index) && (self.current_tab == index) && (index != 0) {
                self.current_tab -= 1;
            } else if self.current_tab > index {
                self.current_tab -= 1;
            }
        }
    }

    fn new_tab(&mut self) {
        self.tabs.push(Tab {
            opened_file: None,
            content: "".to_owned(),
        });
        self.current_tab = self.tabs.len() - 1;
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
                ui.add_enabled_ui(self.tabs.len() != 0, |ui| {
                    if action_button("Save As", ui).clicked() {
                        self.save_file_as();
                    }
                    if action_button("Save", ui).clicked() {
                        if let Some(path) = &self.tabs[self.current_tab].opened_file {
                            self.save_file(path.to_string());
                        } else {
                            self.save_file_as();
                        }
                    }
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    toggle_theme(ui, &mut self.is_dark_mode);
                });
            });
        });

        TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                for (i, tab) in self.tabs.clone().iter().enumerate() {
                    let (res, c_res) = tab_widget(
                        ui,
                        match &tab.opened_file {
                            Some(path) => file_from_path(path.to_string()),
                            None => "Untitled".to_owned(),
                        },
                        i == self.current_tab,
                    );
                    if res.clicked() {
                        self.current_tab = i;
                    }
                    if c_res.clicked() {
                        self.remove_tab(i);
                    }
                }
                if ui.button("+").clicked() {
                    self.new_tab();
                }
            });
        });

        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.tabs.len() > 0 {
                    ui.label(format!(
                        "{} characters. {} lines.",
                        self.tabs[self.current_tab].content.len(),
                        self.tabs[self.current_tab].content.lines().count()
                    ));
                } else {
                    ui.label("No file opened.");
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                Layout::centered_and_justified(Direction::LeftToRight),
                |ui| {
                    if self.tabs.len() > 0 {
                        ui.add(text_editor(
                            &mut self.tabs[self.current_tab].content,
                            self.is_dark_mode,
                        ));
                    }
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
    visuals.widgets.hovered.bg_stroke = Stroke::new(
        1.0,
        match is_dark_mode {
            true => Color32::from_gray(25),
            false => Color32::from_gray(240),
        },
    );
    visuals
}

fn file_from_path(path: String) -> String {
    let file = path.split("\\").collect::<Vec<&str>>();
    file[file.len() - 1].to_string()
}

fn tab_widget(ui: &mut egui::Ui, tab: String, is_active: bool) -> (Response, Response) {
    let text = WidgetText::Galley(ui.fonts().layout_no_wrap(
        tab,
        FontId::default(),
        Color32::WHITE,
    ))
    .into_galley(ui, Some(false), 1.0, FontId::default());
    let text_size = text.size() + ui.spacing().button_padding * 4.0;
    let (text_rect, mut text_response) = ui.allocate_exact_size(text_size, egui::Sense::click());

    let cancel_size = ui.spacing().interact_size.y * egui::vec2(1.0, 1.0);
    let (cancel_rect, mut cancel_response) =
        ui.allocate_exact_size(cancel_size, egui::Sense::click());

    if text_response.clicked() || text_response.hovered() {
        text_response.mark_changed();
    }
    if cancel_response.clicked() || cancel_response.hovered() {
        cancel_response.mark_changed();
    }

    let tab_rect = Rect::from_two_pos(text_rect.min, cancel_rect.max);
    if ui.is_rect_visible(tab_rect) {
        let visuals = ui.style().interact(&text_response);
        let rect = tab_rect.expand(visuals.expansion);
        ui.painter().rect(
            rect,
            4.0,
            if is_active {
                Color32::from_rgb(0, 120, 215)
            } else if text_response.hovered() || cancel_response.hovered() {
                visuals.bg_fill
            } else {
                Color32::TRANSPARENT
            },
            Stroke::none(),
        );
    }

    if ui.is_rect_visible(text_rect) {
        let visuals = ui.style().interact(&text_response);
        let rect = text_rect.expand(visuals.expansion);
        let text_pos = rect.center() - vec2(text.size().x / 2.0, text.size().y / 2.0);
        ui.painter()
            .rect(rect, 4.0, Color32::TRANSPARENT, Stroke::none());
        text.paint_with_visuals(ui.painter(), text_pos, visuals)
    }

    if ui.is_rect_visible(cancel_rect) {
        let visuals = ui.style().interact(&cancel_response);
        let rect = cancel_rect.expand(visuals.expansion);

        ui.painter()
            .rect(rect, 4.0, Color32::TRANSPARENT, Stroke::none());

        const OFFSET: f32 = 5.5;
        const WIDTH: f32 = 2.0;
        ui.painter().line_segment(
            [
                egui::pos2(rect.left() + OFFSET, rect.top() + OFFSET),
                egui::pos2(rect.right() - OFFSET, rect.bottom() - OFFSET),
            ],
            Stroke::new(WIDTH, Color32::WHITE),
        );
        ui.painter().line_segment(
            [
                egui::pos2(rect.left() + OFFSET, rect.bottom() - OFFSET),
                egui::pos2(rect.right() - OFFSET, rect.top() + OFFSET),
            ],
            Stroke::new(WIDTH, Color32::WHITE),
        );
    }

    (text_response, cancel_response)
}

fn toggle_theme(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, Color32::from_rgb(0, 120, 215), Stroke::none());
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter().circle(
            center,
            0.75 * radius,
            Color32::from_rgb(0, 120, 215),
            Stroke::new(1.0, Color32::WHITE),
        );
    }

    response
}
