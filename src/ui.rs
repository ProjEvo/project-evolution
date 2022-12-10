//! Manages the UI

use std::time::Instant;

use crate::{
    evolver::{Evolver, EvolverState},
    simulation::{Simulation, FLOOR_TOP_Y, MAX_WORLD_X, MAX_WORLD_Y, STEPS_PER_SECOND},
    util,
};
use eframe::{
    egui,
    epaint::{CircleShape, RectShape, TextShape},
    Theme,
};
use egui::{
    text::LayoutJob, Align, Color32, FontFamily, FontId, Layout, Painter, Pos2, Rect, RichText,
    Rounding, Stroke, TextFormat, Vec2,
};

use crate::res;

const SPEEDS: [f32; 9] = [0.0, 0.25, 0.5, 0.75, 1.0, 1.5, 2.0, 3.0, 5.0];
const DEFAULT_SPEED: usize = 4;
const MIN_MUSCLE_THICKNESS: f32 = 1.5;
const MAX_MUSCLE_THICKNESS: f32 = 3.0;
const TEXT_COLOR: Color32 = Color32::WHITE;
const CREATURE_SCORE_TEXT_SIZE: f32 = 20.0;

/// Initializes the UI
pub fn init() {
    let native_options = eframe::NativeOptions {
        icon_data: Some(res::load_icon_data()),
        follow_system_theme: false,
        default_theme: Theme::Light,
        maximized: true,
        initial_window_size: Some(Vec2::new(500.0, 500.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Project Evolution",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}

/// Utility method to convert physics x to screen x
fn transform_x_from_world_to_pos2(x: f32, available_size: Vec2) -> f32 {
    let x_factor = available_size.x / MAX_WORLD_X;

    x * x_factor
}

/// Utility method to convert physics y to screen y
fn transform_y_from_world_to_pos2(y: f32, available_size: Vec2) -> f32 {
    let y_factor = available_size.y / MAX_WORLD_Y;

    y * y_factor
}

/// Utility method to convert physics coordinates to where they should be on the screen
fn transform_position_from_world_to_pos2(
    position: &rapier::prelude::Vector<f32>,
    available_size: Vec2,
) -> Pos2 {
    Pos2 {
        x: transform_x_from_world_to_pos2(position.x, available_size),
        y: transform_y_from_world_to_pos2(position.y, available_size),
    }
}

fn distance(a: &rapier::prelude::Vector<f32>, b: &rapier::prelude::Vector<f32>) -> f32 {
    f32::sqrt(f32::powi(a.x - b.x, 2) + f32::powi(a.y - b.y, 2))
}

/// Utility method to paint text at a position
fn paint_text(text: String, position: Pos2, size: f32, color: Color32, painter: &Painter) {
    let mut job = LayoutJob::default();

    job.append(
        text.as_str(),
        0.0,
        TextFormat {
            font_id: FontId::new(size, FontFamily::Proportional),
            color,
            ..Default::default()
        },
    );

    let galley = painter.ctx().fonts().layout_job(job);

    let text = TextShape::new(position, galley);

    painter.add(text);
}

/// Paints a [Simulation] using the provided [Painter]
fn paint_simulation(
    simulation: &Simulation,
    painter: &Painter,
    available_size: Vec2,
    offset_x: f32,
) {
    let creature = simulation.creature();
    let colors = creature.colors();
    let movement_parameters = creature.movement_parameters();

    // Paint muscles
    for (id, muscle) in creature.muscles() {
        let from_position = &simulation.get_position_of_node(muscle.from_id);
        let to_position = &simulation.get_position_of_node(muscle.to_id);
        let extension_delta = simulation.get_extension_delta_of_muscle(*id);
        let mut from = transform_position_from_world_to_pos2(from_position, available_size);
        let mut to = transform_position_from_world_to_pos2(to_position, available_size);

        from.x += offset_x;
        to.x += offset_x;

        let muscle_movement_parameters = movement_parameters.get(id).unwrap();
        let normal_length = muscle_movement_parameters.muscle_length();
        let current_length = distance(from_position, to_position);

        let mut thickness_delta = current_length / normal_length;

        if thickness_delta < 0.5 {
            thickness_delta = 0.5;
        }

        if thickness_delta > 1.5 {
            thickness_delta = 1.5;
        }

        let thickness = MIN_MUSCLE_THICKNESS
            + ((1.0 - (thickness_delta - 0.5)) * (MAX_MUSCLE_THICKNESS - MIN_MUSCLE_THICKNESS));

        let muscle_color = if extension_delta > 0.5 {
            colors.muscle_extended()
        } else {
            colors.muscle_contracted()
        };

        let line = egui::Shape::line(
            vec![from, to],
            Stroke::from((
                transform_x_from_world_to_pos2(thickness, available_size),
                muscle_color,
            )),
        );

        painter.add(line);
    }

    // Paint nodes
    for (id, node) in creature.nodes() {
        let position = simulation.get_position_of_node(*id);
        let mut pos2 = transform_position_from_world_to_pos2(&position, available_size);

        pos2.x += offset_x;

        let circle = CircleShape {
            center: pos2,
            radius: transform_x_from_world_to_pos2(node.size / 2.0, available_size),
            fill: colors.node(),
            stroke: Stroke::none(),
        };

        painter.add(circle);
    }

    // Paint distance
    {
        let score = simulation.get_score();
        let mut position = simulation.get_text_position();

        position.y -= CREATURE_SCORE_TEXT_SIZE;

        let mut pos2 = transform_position_from_world_to_pos2(&position, available_size);

        pos2.x += offset_x;

        paint_text(
            format!("{:.2}m", score),
            pos2,
            CREATURE_SCORE_TEXT_SIZE,
            colors.score_text(),
            painter,
        );
    }
}

/// Paints a generation using the provided [Painter]
fn paint_generation(generation: &Vec<Simulation>, painter: &Painter, available_size: Vec2) {
    let offset_x = (MAX_WORLD_X * (2.0 / 3.0))
        - generation
            .iter()
            .map(|simulation| simulation.get_bounds().1.x)
            .max_by(util::cmp_f32)
            .unwrap();

    for simulation in generation {
        paint_simulation(simulation, painter, available_size, offset_x);
    }
}

/// Paints the scenery using the provided [Painter]
fn paint_scenery(painter: &Painter, available_size: Vec2) {
    let sky = RectShape {
        rect: Rect {
            min: Pos2::new(0.0, 0.0),
            max: Pos2::new(
                transform_x_from_world_to_pos2(MAX_WORLD_X, available_size),
                transform_y_from_world_to_pos2(MAX_WORLD_Y, available_size),
            ),
        },
        rounding: Rounding::none(),
        fill: Color32::from_rgb(122, 233, 255),
        stroke: Stroke::none(),
    };

    painter.add(sky);

    let floor = RectShape {
        rect: Rect {
            min: Pos2::new(
                0.0,
                transform_y_from_world_to_pos2(FLOOR_TOP_Y, available_size),
            ),
            max: Pos2::new(
                transform_x_from_world_to_pos2(MAX_WORLD_X, available_size),
                transform_y_from_world_to_pos2(MAX_WORLD_Y, available_size),
            ),
        },
        rounding: Rounding::none(),
        fill: Color32::from_rgb(75, 200, 75),
        stroke: Stroke::none(),
    };

    painter.add(floor);
}

#[derive(Default)]

/// Creates new egui ui struct used to populate objects into new Window
struct App {
    evolver: Evolver,
    last_frame: Option<Instant>,
    speed_setting: usize,
}

impl App {
    /// Initializes the egui app
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        App {
            last_frame: Some(Instant::now()),
            speed_setting: DEFAULT_SPEED,
            ..Default::default()
        }
    }

    /// Renders the scene
    fn render(&self, painter: &Painter, available_size: Vec2) {
        paint_scenery(painter, available_size);
        paint_generation(
            self.evolver.get_current_generation(),
            painter,
            available_size,
        );
    }
}

impl eframe::App for App {
    /// Called every frame to update the screen
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let central_frame = egui::containers::Frame {
            inner_margin: egui::style::Margin {
                left: 0.0,
                right: 0.0,
                top: 0.0,
                bottom: 0.0,
            },
            ..Default::default()
        };

        egui::CentralPanel::default()
            .frame(central_frame)
            .show(ctx, |ui| {
                let total_size = ui.available_size();
                let now = Instant::now();

                if let Some(last_frame) = self.last_frame {
                    self.evolver.run(now.duration_since(last_frame).mul_f32(SPEEDS[self.speed_setting]));

                    self.render(ui.painter(), total_size);
                }

                self.last_frame = Some(now);

                let state = self.evolver.state();

                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    match state {
                        EvolverState::SimulatingGeneration { steps_left } => {
                            ui.heading(
                                RichText::new(format!(
                                    "{:.2}s",
                                    steps_left as f32 / STEPS_PER_SECOND as f32
                                ))
                                .font(FontId::proportional(40.0))
                                .color(TEXT_COLOR),
                            );
                        }
                        EvolverState::Evolving { steps_left: _ } => {
                            ui.heading(
                                RichText::new("Evolving...")
                                    .font(FontId::proportional(40.0))
                                    .color(TEXT_COLOR),
                            );
                            ui.label(
                                RichText::new("The best creature's offspring are being generated.")
                                    .font(FontId::proportional(25.0))
                                    .color(TEXT_COLOR),
                            );
                        }
                    };
                    ui.horizontal_top(|ui| {
                        if ui.button("<").clicked() {
                            self.speed_setting = usize::max(0, self.speed_setting - 1);
                        }
                        ui.label(
                            RichText::new(
                                format! {"Speed: {}x", SPEEDS.get(self.speed_setting as usize).unwrap()},
                            )
                            .font(FontId::proportional(25.0))
                            .color(TEXT_COLOR),
                        );
                        if ui.button(">").clicked() {
                            self.speed_setting = usize::min(SPEEDS.len() - 1, self.speed_setting + 1);
                        }
                    });
                });
            });

        // Logic to continuously re-render the UI
        ctx.request_repaint();
    }
}
