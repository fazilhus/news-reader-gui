use eframe::{
    egui::{self, Align, Layout, Context, Rect, Rgba, ScrollArea, TopBottomPanel, Ui},
    Frame,
};
use newsapi::Article;

struct NewsReaderApp {
    articles: Vec<Article>,
    break_anywhere: bool,
    max_rows_in_card_desc: usize,
    overflow_char: Option<char>,
}

impl NewsReaderApp {
    pub fn new() -> NewsReaderApp { 
        let iter = (0..20).map(|elem| Article {
            title: format!("title{}", elem),
            content: format!("desc{}", elem),
            url: format!("https://example.com/{}", elem),
        });
        NewsReaderApp {
            articles: Vec::from_iter(iter),
            //articles: NewsAPI::fetch_async(),
            break_anywhere: false,
            max_rows_in_card_desc: 6,
            overflow_char: Some('‥'),
        }
    }

    fn render_newscard(&self, context: &Context, card: &Article, ui: &mut Ui) {
        ui.add_space(5.0);
        ui.label(&card.title);
        ui.label(&card.content);
        ui.hyperlink_to("Read more", &card.url);
        ui.separator();
    }
}

impl eframe::App for NewsReaderApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        Rgba::TRANSPARENT.to_array()
    }

    fn update(&mut self, context: &Context, _frame: &mut Frame) {
        use egui::scroll_area::ScrollBarVisibility;

        frame(context, |ui| {
            ScrollArea::vertical()
                .max_height(context.available_rect().height() - 10.0)
                .auto_shrink(false)
                .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
                .show(ui, |ui| {
                for card in &self.articles {
                    self.render_newscard(context, &card, ui);
                }
            });
            //render_footer(context, ui);
        });
    }
}


fn frame(context: &Context, add_contents: impl FnOnce(&mut Ui)) {
    use egui::CentralPanel;

    let panel_frame = egui::Frame {
        fill: context.style().visuals.window_fill(),
        rounding: 10.0.into(),
        stroke: context.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(),
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(context, |ui| {
        let app_rect = ui.max_rect();
        let tbar_height = 32.0;
        let tbar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + tbar_height;
            rect
        };
        render_titlebar(ui, tbar_rect, "News Reader");

        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = tbar_rect.max.y;
            //rect.max.y -= 16.0;
            rect
        }
        .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);

        //render_footer(context, ui);
    });
}

fn render_titlebar(ui: &mut Ui, tbar_rect: Rect, title: &str) {
    use egui::{vec2, Align2, FontId, Id, Sense, ViewportCommand};

    let painter = ui.painter();
    let tbar_response = ui.interact(tbar_rect, Id::new("title bar"), Sense::click());

    painter.text(
        tbar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color()
    );

    painter.line_segment(
        [
            tbar_rect.left_bottom() + vec2(1.0, 0.0),
            tbar_rect.right_bottom() + vec2(-1.0, 0.0)
        ],
        ui.visuals().widgets.noninteractive.bg_stroke
    );
    
    if tbar_response.double_clicked() {
        let is_maximized = ui.input(|i| i.viewport()
                                    .maximized.unwrap_or(false));
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));

    }

    if tbar_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }

    ui.allocate_ui_at_rect(tbar_rect, |ui| {
        ui.horizontal_centered(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.visuals_mut().button_frame = false;
                ui.add_space(8.0);
                minimize_maximize_close(ui);
            });
        });
    });
}

fn minimize_maximize_close(ui: &mut Ui) {
    use egui::{Button, RichText, ViewportCommand};

    let button_height = 16.0;
    
    let close_response = ui
        .add(Button::new(RichText::new("🗙").size(button_height)))
        .on_hover_text("Close window");
    if close_response.clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Close);
    }

    let is_max = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    if is_max {
        let max_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Restore window");
        if max_response.clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(false));
        }
    } else {
        let max_response = ui
            .add(Button::new(RichText::new("🗖").size(button_height)))
            .on_hover_text("Maximize window");
        if max_response.clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(true));
        }
    }

    let min_response = ui
        .add(Button::new(RichText::new("🗕").size(button_height)))
        .on_hover_text("Minimize window");
    if min_response.clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
    }
}

fn main() -> Result<(), eframe::Error> {
    let opt = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 720.0])
            .with_min_inner_size([480.0, 720.0])
            .with_decorations(false)
            .with_transparent(true),
        ..Default::default()
    };
    eframe::run_native(
        "News Reader",
        opt,
        Box::new(|_| Box::new(NewsReaderApp::new()))
    )
}
