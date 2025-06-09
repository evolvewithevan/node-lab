use eframe::egui;
use uuid::Uuid;

struct Connection {
    start_box_id: Uuid,
    start_circle_id: Uuid,
    end_box_id: Uuid,
    end_circle_id: Uuid,
}

struct BoxWithCircles {
    id: Uuid,
    left_circle_id: Uuid,
    right_circle_id: Uuid,
    position: egui::Pos2,
    size: egui::Vec2,
    left_circle_center: egui::Pos2,
    right_circle_center: egui::Pos2,
    is_dragging: bool,
}

impl BoxWithCircles {
    fn new(x: f32, y: f32) -> Self {
        let size = egui::Vec2::new(100.0, 100.0);
        let position = egui::Pos2::new(x, y);
        let left_circle_center = egui::Pos2::new(
            position.x,
            position.y + size.y / 2.0,
        );
        let right_circle_center = egui::Pos2::new(
            position.x + size.x,
            position.y + size.y / 2.0,
        );
        
        Self {
            id: Uuid::new_v4(),
            left_circle_id: Uuid::new_v4(),
            right_circle_id: Uuid::new_v4(),
            position,
            size,
            left_circle_center,
            right_circle_center,
            is_dragging: false,
        }
    }

    fn is_point_in_circle(&self, point: egui::Pos2) -> Option<Uuid> {
        let circle_radius = 15.0;
        let radius_squared = circle_radius * circle_radius;

        // Check left circle
        let dx_left = point.x - self.left_circle_center.x;
        let dy_left = point.y - self.left_circle_center.y;
        let distance_squared_left = dx_left * dx_left + dy_left * dy_left;
        
        if distance_squared_left <= radius_squared {
            return Some(self.left_circle_id);
        }

        // Check right circle
        let dx_right = point.x - self.right_circle_center.x;
        let dy_right = point.y - self.right_circle_center.y;
        let distance_squared_right = dx_right * dx_right + dy_right * dy_right;
        
        if distance_squared_right <= radius_squared {
            return Some(self.right_circle_id);
        }

        None
    }

    fn get_circle_center(&self, circle_id: Uuid) -> Option<egui::Pos2> {
        if circle_id == self.left_circle_id {
            Some(self.left_circle_center)
        } else if circle_id == self.right_circle_id {
            Some(self.right_circle_center)
        } else {
            None
        }
    }
}

struct App {
    box1: BoxWithCircles,
    box2: BoxWithCircles,
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
            box1: BoxWithCircles::new(100.0, 100.0),
            box2: BoxWithCircles::new(400.0, 100.0),
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

    fn update_box_position(box_with_circles: &mut BoxWithCircles, drag_delta: egui::Vec2) {
        box_with_circles.position += drag_delta;
        box_with_circles.left_circle_center = egui::Pos2::new(
            box_with_circles.position.x,
            box_with_circles.position.y + box_with_circles.size.y / 2.0,
        );
        box_with_circles.right_circle_center = egui::Pos2::new(
            box_with_circles.position.x + box_with_circles.size.x,
            box_with_circles.position.y + box_with_circles.size.y / 2.0,
        );
    }

    fn handle_box_drag(box_with_circles: &mut BoxWithCircles, response: &egui::Response, pointer_pos: Option<egui::Pos2>, is_circle_dragging: bool) {
        if !response.dragged() {
            return;
        }

        if let Some(pos) = pointer_pos {
            if box_with_circles.is_point_in_circle(pos).is_none() && !is_circle_dragging {
                Self::update_box_position(box_with_circles, response.drag_delta());
            }
        }
    }

    fn handle_circle_click(&mut self, pointer_pos: egui::Pos2) {
        let box1_circle = self.box1.is_point_in_circle(pointer_pos);
        let box2_circle = self.box2.is_point_in_circle(pointer_pos);

        if box1_circle.is_none() && box2_circle.is_none() {
            self.clear_line_state();
            return;
        }

        self.is_drawing_line = true;
        if let Some(circle_id) = box1_circle {
            self.line_start = self.box1.get_circle_center(circle_id);
            self.current_connection_start = Some((self.box1.id, circle_id));
        } else if let Some(circle_id) = box2_circle {
            self.line_start = self.box2.get_circle_center(circle_id);
            self.current_connection_start = Some((self.box2.id, circle_id));
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
        let _start = self.line_start;

        let box1_circle = self.box1.is_point_in_circle(pointer_pos);
        let box2_circle = self.box2.is_point_in_circle(pointer_pos);

        if box1_circle.is_none() && box2_circle.is_none() {
            return;
        }

        if let Some(circle_id) = box1_circle {
            if start_box_id != self.box1.id {
                self.create_connection(start_box_id, start_circle_id, self.box1.id, circle_id);
                self.line_end = self.box1.get_circle_center(circle_id);
            }
        } else if let Some(circle_id) = box2_circle {
            if start_box_id != self.box2.id {
                self.create_connection(start_box_id, start_circle_id, self.box2.id, circle_id);
                self.line_end = self.box2.get_circle_center(circle_id);
            }
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
                self.box1.get_circle_center(connection.start_circle_id).unwrap()
            } else {
                self.box2.get_circle_center(connection.start_circle_id).unwrap()
            };
            let end = if connection.end_box_id == self.box1.id {
                self.box1.get_circle_center(connection.end_circle_id).unwrap()
            } else {
                self.box2.get_circle_center(connection.end_circle_id).unwrap()
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
                self.last_click_in_circle = self.box1.is_point_in_circle(pos).is_some() || self.box2.is_point_in_circle(pos).is_some();
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
                    ui.label(format!("Box1 Left Circle UUID: {}", self.box1.left_circle_id));
                    ui.label(format!("Box1 Right Circle UUID: {}", self.box1.right_circle_id));
                });

                ui.horizontal(|ui| {
                    ui.label(format!("Box2 UUID: {}", self.box2.id));
                    ui.label(format!("Box2 Left Circle UUID: {}", self.box2.left_circle_id));
                    ui.label(format!("Box2 Right Circle UUID: {}", self.box2.right_circle_id));
                });

                // Debug information
                if self.is_drawing_line {
                    if let Some((start_box_id, _)) = self.current_connection_start {
                        ui.label(format!("Drawing line from box: {}", start_box_id));
                        if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                            ui.label(format!("Pointer pos: ({:.1}, {:.1})", pos.x, pos.y));
                            ui.label(format!("Box1 left circle center: ({:.1}, {:.1})", self.box1.left_circle_center.x, self.box1.left_circle_center.y));
                            ui.label(format!("Box1 right circle center: ({:.1}, {:.1})", self.box1.right_circle_center.x, self.box1.right_circle_center.y));
                            ui.label(format!("Box2 left circle center: ({:.1}, {:.1})", self.box2.left_circle_center.x, self.box2.left_circle_center.y));
                            ui.label(format!("Box2 right circle center: ({:.1}, {:.1})", self.box2.right_circle_center.x, self.box2.right_circle_center.y));
                        }
                    }
                }
                ui.label(format!("Total connections: {}", self.connections.len()));
            });

            // Draw first box and circles
            let rect1 = egui::Rect::from_min_size(self.box1.position, self.box1.size);
            let response1 = ui.allocate_rect(rect1, egui::Sense::click_and_drag());
            let pointer_pos = ui.input(|i| i.pointer.hover_pos());
            let is_circle_dragging = self.is_circle_dragging;
            Self::handle_box_drag(&mut self.box1, &response1, pointer_pos, is_circle_dragging);

            // Draw second box and circles
            let rect2 = egui::Rect::from_min_size(self.box2.position, self.box2.size);
            let response2 = ui.allocate_rect(rect2, egui::Sense::click_and_drag());
            Self::handle_box_drag(&mut self.box2, &response2, pointer_pos, is_circle_dragging);

            let painter = ui.painter();

            // Draw boxes
            painter.rect_filled(rect1, 0.0, egui::Color32::from_rgb(100, 150, 250));
            painter.rect_filled(rect2, 0.0, egui::Color32::from_rgb(150, 100, 250));

            // Draw circles
            painter.circle_filled(self.box1.left_circle_center, 10.0, egui::Color32::WHITE);
            painter.circle_filled(self.box1.right_circle_center, 10.0, egui::Color32::WHITE);
            painter.circle_filled(self.box2.left_circle_center, 10.0, egui::Color32::WHITE);
            painter.circle_filled(self.box2.right_circle_center, 10.0, egui::Color32::WHITE);

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