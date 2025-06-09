use eframe::egui;

struct BoxWithCircle {
    position: egui::Pos2,
    size: egui::Vec2,
    circle_center: egui::Pos2,
    is_dragging: bool,
}

impl BoxWithCircle {
    fn new(x: f32, y: f32) -> Self {
        let size = egui::Vec2::new(100.0, 100.0);
        let position = egui::Pos2::new(x, y);
        let circle_center = egui::Pos2::new(
            position.x + size.x / 2.0,
            position.y + size.y / 2.0,
        );
        
        Self {
            position,
            size,
            circle_center,
            is_dragging: false,
        }
    }

    fn is_point_in_circle(&self, point: egui::Pos2) -> bool {
        let circle_radius = 10.0;
        let dx = point.x - self.circle_center.x;
        let dy = point.y - self.circle_center.y;
        dx * dx + dy * dy <= circle_radius * circle_radius
    }
}

struct App {
    box1: BoxWithCircle,
    box2: BoxWithCircle,
    is_drawing_line: bool,
    line_start: Option<egui::Pos2>,
    line_end: Option<egui::Pos2>,
    mouse1_pressed: bool,
    last_mouse1_click: Option<egui::Pos2>,
    last_click_in_circle: bool,
    is_circle_dragging: bool,
    circle_drag_origin: Option<egui::Pos2>,
}

impl App {
    fn new() -> Self {
        Self {
            box1: BoxWithCircle::new(100.0, 100.0),
            box2: BoxWithCircle::new(400.0, 100.0),
            is_drawing_line: false,
            line_start: None,
            line_end: None,
            mouse1_pressed: false,
            last_mouse1_click: None,
            last_click_in_circle: false,
            is_circle_dragging: false,
            circle_drag_origin: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update mouse state
        self.mouse1_pressed = ctx.input(|i| i.pointer.primary_down());
        if ctx.input(|i| i.pointer.primary_clicked()) {
            if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
                self.last_mouse1_click = Some(pos);
                self.last_click_in_circle = self.box1.is_point_in_circle(pos) || self.box2.is_point_in_circle(pos);
                if self.last_click_in_circle {
                    self.is_circle_dragging = true;
                    self.circle_drag_origin = Some(pos);
                }
            }
        }
        
        // Reset circle dragging state when mouse is released
        if ctx.input(|i| i.pointer.any_released()) {
            self.is_circle_dragging = false;
            self.circle_drag_origin = None;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Display info text at top left
            ui.horizontal(|ui| {
                ui.label(format!("Mouse1 pressed: {}", self.mouse1_pressed));
                if let Some(pos) = self.last_mouse1_click {
                    ui.label(format!("Last Mouse1 click: ({:.1}, {:.1})", pos.x, pos.y));
                }
                ui.label(format!("Last click in circle: {}", self.last_click_in_circle));
                ui.label(format!("Circle dragging: {}", self.is_circle_dragging));
                ui.label(format!("Box1 position: ({:.1}, {:.1})", self.box1.position.x, self.box1.position.y));
                ui.label(format!("Box2 position: ({:.1}, {:.1})", self.box2.position.x, self.box2.position.y));
            });

            // Draw first box and circle
            let rect1 = egui::Rect::from_min_size(self.box1.position, self.box1.size);
            let response1 = ui.allocate_rect(rect1, egui::Sense::click_and_drag());
            
            if response1.dragged() {
                // Only move the box if we're not clicking on the circle and not in circle dragging state
                if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
                    if !self.box1.is_point_in_circle(pointer_pos) && !self.is_circle_dragging {
                        self.box1.position += response1.drag_delta();
                        self.box1.circle_center = egui::Pos2::new(
                            self.box1.position.x + self.box1.size.x / 2.0,
                            self.box1.position.y + self.box1.size.y / 2.0,
                        );
                    }
                }
            }

            // Draw second box and circle
            let rect2 = egui::Rect::from_min_size(self.box2.position, self.box2.size);
            let response2 = ui.allocate_rect(rect2, egui::Sense::click_and_drag());
            
            if response2.dragged() {
                // Only move the box if we're not clicking on the circle and not in circle dragging state
                if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
                    if !self.box2.is_point_in_circle(pointer_pos) && !self.is_circle_dragging {
                        self.box2.position += response2.drag_delta();
                        self.box2.circle_center = egui::Pos2::new(
                            self.box2.position.x + self.box2.size.x / 2.0,
                            self.box2.position.y + self.box2.size.y / 2.0,
                        );
                    }
                }
            }

            let painter = ui.painter();

            // Draw boxes
            painter.rect_filled(rect1, 0.0, egui::Color32::from_rgb(100, 150, 250));
            painter.rect_filled(rect2, 0.0, egui::Color32::from_rgb(150, 100, 250));

            // Draw circles
            painter.circle_filled(self.box1.circle_center, 10.0, egui::Color32::WHITE);
            painter.circle_filled(self.box2.circle_center, 10.0, egui::Color32::WHITE);

            // Draw line from circle to cursor when circle dragging
            if self.is_circle_dragging {
                if let (Some(origin), Some(cursor_pos)) = (self.circle_drag_origin, ui.input(|i| i.pointer.hover_pos())) {
                    painter.line_segment([origin, cursor_pos], egui::Stroke::new(2.0, egui::Color32::YELLOW));
                }
            }

            // Handle line drawing
            if response1.clicked() {
                if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
                    if self.box1.is_point_in_circle(pointer_pos) {
                        self.is_drawing_line = true;
                        self.line_start = Some(self.box1.circle_center);
                    }
                }
            } else if response2.clicked() {
                if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
                    if self.box2.is_point_in_circle(pointer_pos) {
                        self.is_drawing_line = true;
                        self.line_start = Some(self.box2.circle_center);
                    }
                }
            }

            if ui.input(|i| i.pointer.any_released()) {
                if self.is_drawing_line {
                    if let Some(start) = self.line_start {
                        if response1.hovered() {
                            self.line_end = Some(self.box1.circle_center);
                        } else if response2.hovered() {
                            self.line_end = Some(self.box2.circle_center);
                        }
                    }
                }
                self.is_drawing_line = false;
                self.line_start = None;
                self.line_end = None;
            }

            // Draw the line while dragging
            if self.is_drawing_line {
                if let Some(start) = self.line_start {
                    let end = ui.input(|i| i.pointer.hover_pos()).unwrap_or(start);
                    painter.line_segment([start, end], egui::Stroke::new(2.0, egui::Color32::WHITE));
                }
            }

            // Draw the final line if both points are set
            if let (Some(start), Some(end)) = (self.line_start, self.line_end) {
                painter.line_segment([start, end], egui::Stroke::new(2.0, egui::Color32::WHITE));
            }
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
        "Connected Boxes",
        options,
        Box::new(|_cc| Box::new(App::new())),
    )
} 