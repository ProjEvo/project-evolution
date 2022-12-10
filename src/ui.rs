//! Manages the UI

use std::{ops::RangeInclusive, time::Instant};

use crate::{
    evolver::{Evolver, EvolverState},
    simulation::{
        Simulation, FLOOR_HEIGHT, FLOOR_TOP_Y, SCORE_PER_SCREEN, STEPS_PER_SECOND, WORLD_X_SIZE,
        WORLD_Y_SIZE,
    },
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
const DISTANCE_LINE_THICKNESS: f32 = 5.0;
const WHITE: Color32 = Color32::WHITE;
const TEXT_COLOR: Color32 = WHITE;
const CREATURE_SCORE_TEXT_SIZE: f32 = 20.0;
const SCORE_LINE_TEXT_SIZE: f32 = 30.0;

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

#[derive(Default)]

/// Creates new egui ui struct used to populate objects into new Window
struct App {
    evolver: Evolver,
    last_frame: Option<Instant>,
    speed_setting: usize,
    screen_size: Vec2,
    screen_offset_x: f32,
    max_x: f32,
}

/// Utility method to paint text at a position
fn paint_text(
    text: String,
    mut position: Pos2,
    size: f32,
    color: Color32,
    center_x: bool,
    painter: &Painter,
) {
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

    if center_x {
        position.x -= galley.rect.width() / 2.0;
    }

    let text = TextShape::new(position, galley);

    painter.add(text);
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

    /// Paints a [Simulation] using the provided [Painter]
    fn paint_simulation(&self, simulation: &Simulation, painter: &Painter) {
        let creature = simulation.creature();
        let colors = creature.colors();
        let movement_parameters = creature.movement_parameters();

        // Paint muscles
        for (id, muscle) in creature.muscles() {
            let from_position = &simulation.get_position_of_node(muscle.from_id);
            let to_position = &simulation.get_position_of_node(muscle.to_id);
            let is_muscle_extending = simulation.is_muscle_extending(*id);
            let mut from = util::transform_position_from_world_to_screen_pos2(
                from_position,
                &self.screen_size,
            );
            let mut to =
                util::transform_position_from_world_to_screen_pos2(to_position, &self.screen_size);

            from.x += self.screen_offset_x;
            to.x += self.screen_offset_x;

            let muscle_movement_parameters = movement_parameters.get(id).unwrap();
            let normal_length = muscle_movement_parameters.muscle_length();
            let current_length = util::distance(from_position, to_position);

            let mut thickness_delta = current_length / normal_length;

            if thickness_delta < 0.5 {
                thickness_delta = 0.5;
            }

            if thickness_delta > 1.5 {
                thickness_delta = 1.5;
            }

            let thickness = MIN_MUSCLE_THICKNESS
                + ((1.0 - (thickness_delta - 0.5)) * (MAX_MUSCLE_THICKNESS - MIN_MUSCLE_THICKNESS));

            let muscle_color = if is_muscle_extending {
                colors.muscle_extended()
            } else {
                colors.muscle_contracted()
            };

            let line = egui::Shape::line(
                vec![from, to],
                Stroke::from((
                    util::transform_x_from_world_to_screen(thickness, &self.screen_size),
                    muscle_color,
                )),
            );

            painter.add(line);
        }

        // Paint nodes
        for (id, node) in creature.nodes() {
            let position = simulation.get_position_of_node(*id);
            let mut pos2 =
                util::transform_position_from_world_to_screen_pos2(&position, &self.screen_size);

            pos2.x += self.screen_offset_x;

            let circle = CircleShape {
                center: pos2,
                radius: util::transform_x_from_world_to_screen(node.size / 2.0, &self.screen_size),
                fill: colors.node(),
                stroke: Stroke::none(),
            };

            painter.add(circle);
        }

        // Paint score
        {
            let score = simulation.get_score();
            let position = simulation.get_text_position();
            let mut pos2 =
                util::transform_position_from_world_to_screen_pos2(&position, &self.screen_size);

            pos2.x += self.screen_offset_x;
            pos2.y -= CREATURE_SCORE_TEXT_SIZE;

            paint_text(
                format!("{:.2}m", score),
                pos2,
                CREATURE_SCORE_TEXT_SIZE,
                colors.score_text(),
                true,
                painter,
            );
        }
    }

    /// Paints a generation using the provided [Painter]
    fn paint_generation(&self, generation: &Vec<Simulation>, painter: &Painter) {
        for simulation in generation {
            self.paint_simulation(simulation, painter);
        }
    }

    /// Paints the scenery using the provided [Painter]
    fn paint_scenery(&self, painter: &Painter) {
        // Add sky
        let sky = RectShape {
            rect: Rect {
                min: Pos2::new(0.0, 0.0),
                max: Pos2::new(
                    util::transform_x_from_world_to_screen(WORLD_X_SIZE, &self.screen_size),
                    util::transform_y_from_world_to_screen(WORLD_Y_SIZE, &self.screen_size),
                ),
            },
            rounding: Rounding::none(),
            fill: Color32::from_rgb(122, 233, 255),
            stroke: Stroke::none(),
        };

        painter.add(sky);

        // Add ground
        let ground = RectShape {
            rect: Rect {
                min: Pos2::new(
                    0.0,
                    util::transform_y_from_world_to_screen(FLOOR_TOP_Y, &self.screen_size),
                ),
                max: Pos2::new(
                    util::transform_x_from_world_to_screen(WORLD_X_SIZE, &self.screen_size),
                    util::transform_y_from_world_to_screen(WORLD_Y_SIZE, &self.screen_size),
                ),
            },
            rounding: Rounding::none(),
            fill: Color32::from_rgb(75, 200, 75),
            stroke: Stroke::none(),
        };

        painter.add(ground);

        // Add score lines
        let middle_score = Simulation::x_to_score(f32::floor(self.max_x)) as i32;
        // Intentionally extend range extra to cover edges
        let score_range = RangeInclusive::new(
            middle_score - (SCORE_PER_SCREEN),
            middle_score + (SCORE_PER_SCREEN),
        );

        for score in score_range {
            let mut height_scale = 0.25;
            let minor = score % (SCORE_PER_SCREEN / 2) == 0;
            let major = score % SCORE_PER_SCREEN == 0;

            if major {
                height_scale = 2.0 / 3.0;
            } else if minor {
                height_scale = 1.0 / 3.0;
            }

            let x = Simulation::score_to_x(score as f32);
            let y = FLOOR_TOP_Y + (FLOOR_HEIGHT * height_scale);

            let line = RectShape {
                rect: Rect {
                    min: Pos2::new(
                        util::transform_x_from_world_to_screen(
                            x - (DISTANCE_LINE_THICKNESS / 2.0),
                            &self.screen_size,
                        ) + self.screen_offset_x,
                        util::transform_y_from_world_to_screen(FLOOR_TOP_Y, &self.screen_size),
                    ),
                    max: Pos2::new(
                        util::transform_x_from_world_to_screen(
                            x + (DISTANCE_LINE_THICKNESS / 2.0),
                            &self.screen_size,
                        ) + self.screen_offset_x,
                        util::transform_y_from_world_to_screen(y, &self.screen_size),
                    ),
                },
                rounding: Rounding::none(),
                fill: WHITE,
                stroke: Stroke::none(),
            };

            painter.add(line);

            if minor || major {
                let pos = Pos2::new(
                    util::transform_x_from_world_to_screen(x, &self.screen_size)
                        + self.screen_offset_x,
                    util::transform_y_from_world_to_screen(y, &self.screen_size),
                );

                paint_text(
                    format!("{:.2}m", score),
                    pos,
                    SCORE_LINE_TEXT_SIZE,
                    WHITE,
                    true,
                    painter,
                );
            }
        }
    }

    /// Renders the scene
    fn render(&mut self, painter: &Painter) {
        let generation = self.evolver.get_current_generation();
        self.max_x = generation
            .iter()
            .map(|simulation| simulation.get_bounds().1.x)
            .max_by(util::cmp_f32)
            .unwrap();
        self.screen_offset_x = util::transform_x_from_world_to_screen(
            (WORLD_X_SIZE * (2.0 / 3.0)) - self.max_x,
            &self.screen_size,
        );
        self.paint_scenery(painter);
        self.paint_generation(generation, painter);
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
                self.screen_size = ui.available_size();

                let now = Instant::now();

                if let Some(last_frame) = self.last_frame {
                    self.evolver.run(now.duration_since(last_frame).mul_f32(SPEEDS[self.speed_setting]));

                    self.render(ui.painter());
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
