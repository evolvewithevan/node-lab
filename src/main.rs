use eframe::egui;

struct MovableBox {
    position: egui::Pos2,
    size: egui::Vec2,
    is_dragging: bool,
}

impl MovableBox {
    fn new() -> Self {
        Self {
            position: egui::Pos2::new(100.0, 100.0),
            size: egui::Vec2::new(100.0, 100.0),
            is_dragging: false,
        }
    }
}

impl eframe::App for MovableBox {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = egui::Rect::from_min_size(self.position, self.size);
            let response = ui.allocate_rect(rect, egui::Sense::drag());

            if response.dragged() {
                self.position += response.drag_delta();
            }

            let painter = ui.painter();
            painter.rect_filled(
                rect,
                0.0,
                egui::Color32::from_rgb(100, 150, 250),
            );
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Movable Box",
        options,
        Box::new(|_cc| Box::new(MovableBox::new())),
    )
} 