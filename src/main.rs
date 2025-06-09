use eframe::egui;
use uuid::Uuid;

struct Connection {
    start_box_id: Uuid,
    start_circle_id: Uuid,
    end_box_id: Uuid,
    end_circle_id: Uuid,
}

struct BoxWithCircle {
    id: Uuid,
    circle_id: Uuid,
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
            id: Uuid::new_v4(),
            circle_id: Uuid::new_v4(),
            position,
            size,
            circle_center,
            is_dragging: false,
        }
    }

    fn is_point_in_circle(&self, point: egui::Pos2) -> bool {
        let circle_radius = 15.0;
        let dx = point.x - self.circle_center.x;
        let dy = point.y - self.circle_center.y;
        let distance_squared = dx * dx + dy * dy;
        let radius_squared = circle_radius * circle_radius;
        
        // Debug print for circle intersection
        println!(
            "Circle check - Point: ({:.1}, {:.1}), Center: ({:.1}, {:.1}), Distance: {:.1}, Radius: {:.1}, Inside: {}",
            point.x, point.y, self.circle_center.x, self.circle_center.y,
            distance_squared.sqrt(), circle_radius,
            distance_squared <= radius_squared
        );
        
        distance_squared <= radius_squared
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
    connections: Vec<Connection>,
    current_connection_start: Option<(Uuid, Uuid)>, // (box_id, circle_id)
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
            connections: Vec::new(),
            current_connection_start: None,
        }
    }

    fn update_box_position(box_with_circle: &mut BoxWithCircle, drag_delta: egui::Vec2) {
        box_with_circle.position += drag_delta;
        box_with_circle.circle_center = egui::Pos2::new(
            box_with_circle.position.x + box_with_circle.size.x / 2.0,
            box_with_circle.position.y + box_with_circle.size.y / 2.0,
        );
    }

    fn handle_box_drag(box_with_circle: &mut BoxWithCircle, response: &egui::Response, pointer_pos: Option<egui::Pos2>, is_circle_dragging: bool) {
        if !response.dragged() {
            return;
        }

        if let Some(pos) = pointer_pos {
            if !box_with_circle.is_point_in_circle(pos) && !is_circle_dragging {
                Self::update_box_position(box_with_circle, response.drag_delta());
            }
        }
    }

    fn handle_circle_click(&mut self, pointer_pos: egui::Pos2) {
        let in_box1_circle = self.box1.is_point_in_circle(pointer_pos);
        let in_box2_circle = self.box2.is_point_in_circle(pointer_pos);

        if !in_box1_circle && !in_box2_circle {
            self.clear_line_state();
            return;
        }

        self.is_drawing_line = true;
        if in_box1_circle {
            self.line_start = Some(self.box1.circle_center);
            self.current_connection_start = Some((self.box1.id, self.box1.circle_id));
        } else {
            self.line_start = Some(self.box2.circle_center);
            self.current_connection_start = Some((self.box2.id, self.box2.circle_id));
        }
    }

    fn clear_line_state(&mut self) {
        self.is_drawing_line = false;
        self.line_start = None;
        self.line_end = None;
        self.current_connection_start = None;
    }

    fn handle_connection_creation(&mut self, pointer_pos: egui::Pos2) {
        let Some((start_box_id, start_circle_id)) = self.current_connection_start else { return };
        let _start = self.line_start; // Keep for future use if needed

        let in_box1_circle = self.box1.is_point_in_circle(pointer_pos);
        let in_box2_circle = self.box2.is_point_in_circle(pointer_pos);

        if !in_box1_circle && !in_box2_circle {
            return;
        }

        if in_box1_circle && start_box_id != self.box1.id {
            self.create_connection(start_box_id, start_circle_id, self.box1.id, self.box1.circle_id);
            self.line_end = Some(self.box1.circle_center);
        } else if in_box2_circle && start_box_id != self.box2.id {
            self.create_connection(start_box_id, start_circle_id, self.box2.id, self.box2.circle_id);
            self.line_end = Some(self.box2.circle_center);
        }
    }

    fn create_connection(&mut self, start_box_id: Uuid, start_circle_id: Uuid, end_box_id: Uuid, end_circle_id: Uuid) {
        let connection = Connection {
            start_box_id,
            start_circle_id,
            end_box_id,
            end_circle_id,
        };
        self.connections.push(connection);
    }

    fn draw_connections(&self, painter: &egui::Painter) {
        for connection in &self.connections {
            let start = if connection.start_box_id == self.box1.id {
                self.box1.circle_center
            } else {
                self.box2.circle_center
            };
            let end = if connection.end_box_id == self.box1.id {
                self.box1.circle_center
            } else {
                self.box2.circle_center
            };
            painter.line_segment([start, end], egui::Stroke::new(4.0, egui::Color32::WHITE));
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
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("Mouse1 pressed: {}", self.mouse1_pressed));
                    if let Some(pos) = self.last_mouse1_click {
                        ui.label(format!("Last Mouse1 click: ({:.1}, {:.1})", pos.x, pos.y));
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label(format!("Last click in circle: {}", self.last_click_in_circle));
                    ui.label(format!("Circle dragging: {}", self.is_circle_dragging));
                });

                ui.horizontal(|ui| {
                    ui.label(format!("Box1 position: ({:.1}, {:.1})", self.box1.position.x, self.box1.position.y));
                    ui.label(format!("Box2 position: ({:.1}, {:.1})", self.box2.position.x, self.box2.position.y));
                });

                ui.horizontal(|ui| {
                    ui.label(format!("Box1 UUID: {}", self.box1.id));
                    ui.label(format!("Box1 Circle UUID: {}", self.box1.circle_id));
                });

                ui.horizontal(|ui| {
                    ui.label(format!("Box2 UUID: {}", self.box2.id));
                    ui.label(format!("Box2 Circle UUID: {}", self.box2.circle_id));
                });

                // Debug information
                if self.is_drawing_line {
                    if let Some((start_box_id, _)) = self.current_connection_start {
                        ui.label(format!("Drawing line from box: {}", start_box_id));
                        if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                            ui.label(format!("Pointer pos: ({:.1}, {:.1})", pos.x, pos.y));
                            ui.label(format!("Box1 circle center: ({:.1}, {:.1})", self.box1.circle_center.x, self.box1.circle_center.y));
                            ui.label(format!("Box2 circle center: ({:.1}, {:.1})", self.box2.circle_center.x, self.box2.circle_center.y));
                        }
                    }
                }
                ui.label(format!("Total connections: {}", self.connections.len()));
            });

            // Draw first box and circle
            let rect1 = egui::Rect::from_min_size(self.box1.position, self.box1.size);
            let response1 = ui.allocate_rect(rect1, egui::Sense::click_and_drag());
            let pointer_pos = ui.input(|i| i.pointer.hover_pos());
            let is_circle_dragging = self.is_circle_dragging;
            Self::handle_box_drag(&mut self.box1, &response1, pointer_pos, is_circle_dragging);

            // Draw second box and circle
            let rect2 = egui::Rect::from_min_size(self.box2.position, self.box2.size);
            let response2 = ui.allocate_rect(rect2, egui::Sense::click_and_drag());
            Self::handle_box_drag(&mut self.box2, &response2, pointer_pos, is_circle_dragging);

            let painter = ui.painter();

            // Draw boxes
            painter.rect_filled(rect1, 0.0, egui::Color32::from_rgb(100, 150, 250));
            painter.rect_filled(rect2, 0.0, egui::Color32::from_rgb(150, 100, 250));

            // Draw circles
            painter.circle_filled(self.box1.circle_center, 10.0, egui::Color32::WHITE);
            painter.circle_filled(self.box2.circle_center, 10.0, egui::Color32::WHITE);

            // Handle line drawing
            if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
                if ui.input(|i| i.pointer.primary_clicked()) {
                    self.handle_circle_click(pointer_pos);
                }
            }

            // Draw line from circle to cursor when circle dragging
            if self.is_drawing_line {
                if let Some(start) = self.line_start {
                    let end = ui.input(|i| i.pointer.hover_pos()).unwrap_or(start);
                    painter.line_segment([start, end], egui::Stroke::new(2.0, egui::Color32::WHITE));
                }
            }

            // Handle mouse release
            if ui.input(|i| i.pointer.any_released()) {
                if self.is_drawing_line {
                    if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
                        self.handle_connection_creation(pointer_pos);
                    }
                }
            }

            // Draw all connections
            self.draw_connections(&painter);

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