use crate::creature::{Creature, Muscle, Node, Position};
/// Manages User Interface (UI)
use eframe::{egui, epaint::CircleShape};
use egui::{Color32, Painter, Pos2, Shape, Stroke};

use crate::res;

pub fn init() {
    let native_options = eframe::NativeOptions {
        icon_data: Some(res::load_icon_data()),
        ..Default::default()
    };

    eframe::run_native(
        "Project Evolution",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}

#[derive(Default)]

/// Creates new egui ui struct used to populate objects into new Window
struct App {
    creatures: Vec<Creature>,
}

/// Initializes the new interface that will create the objects on the screen
impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}
fn test_creature() -> (Vec<CircleShape>, Vec<Shape>) {
    let mut c = Creature::new();

    let nodes = Vec::from([
        Node::new(Position::new(300.0, 300.0), 3),
        Node::new(Position::new(320.0, 220.0), 3),
        Node::new(Position::new(360.0, 360.0), 3),
    ]);

    let id1 = nodes.get(0).unwrap().id;
    let id2 = nodes.get(1).unwrap().id;
    let id3 = nodes.get(2).unwrap().id;

    let muscles = Vec::from([
        Muscle::new(id1, id2),
        Muscle::new(id2, id3),
        Muscle::new(id3, id1),
    ]);
    let _id4 = muscles.get(0).unwrap().id;

    c.add_nodes(nodes);
    c.add_muscles(muscles);
    let list_nodes = get_nodes(&c);
    let list_muscles = get_muscle(&c);
    return (list_nodes, list_muscles);
}

pub fn get_nodes(c: &Creature) -> Vec<CircleShape> {
    let mut list = Vec::new();
    for node in c.nodes() {
        let circle = CircleShape {
            center: egui::Pos2 {
                x: node.1.position.x as f32,
                y: node.1.position.y as f32,
            },
            radius: 10.0,
            fill: Color32::BLUE,
            stroke: Stroke::default(),
        };
        list.push(circle);
    }
    return list;
}

pub fn get_muscle(c: &Creature) -> Vec<Shape> {
    let mut list = Vec::new();

    for m in c.muscles() {
        let from_node = c.nodes().get(&m.1.from_id).unwrap();
        let to_node = c.nodes().get(&m.1.to_id).unwrap();
        let mut list_points = Vec::new();
        list_points.push(Pos2 {
            x: from_node.position.x as f32,
            y: from_node.position.y as f32,
        });
        list_points.push(Pos2 {
            x: to_node.position.x as f32,
            y: to_node.position.y as f32,
        });
        let line = egui::Shape::line(list_points, Stroke::from((2.5, Color32::RED)));
        list.push(line);
    }
    return list;
}

pub fn paint_creatures(painter: &Painter) {
    let get_nodes = test_creature().0;
    let get_muscles = test_creature().1;
    for muscle in get_muscles {
        painter.add(muscle);
    }
    for node in get_nodes {
        painter.add(node);
    }
}

impl eframe::App for App {
    /// Updates the screen that is to be blitted (currently very underdeveloped, needs to be fully realized soon)
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // initializes a central panel of the UI with contents to be added
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.label("This is should be a blank UI with a couple of buttons");

            paint_creatures(ui.painter());
        });
    }
}
