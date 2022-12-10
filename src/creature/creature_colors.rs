use crate::util;
use egui::Color32;
use rand::Rng;
use std::ops::RangeInclusive;

const COLOR_HUE_RANGE: RangeInclusive<u16> = 0..=350;
const MUTATE_COLOR_HUE_RANGE: RangeInclusive<i16> = -10..=10;

/// Represents the colors of a creature
#[derive(Debug, Clone, Copy)]
pub struct CreatureColors {
    hue: u16,
    node: Color32,
    muscle_extended: Color32,
    muscle_contracted: Color32,
    score_text: Color32,
}

impl CreatureColors {
    /// Creates a new random set of creature colors
    pub fn new() -> CreatureColors {
        let mut rng = rand::thread_rng();

        let hue = rng.gen_range(COLOR_HUE_RANGE);

        Self::from_hue(hue)
    }

    /// Creates a set of creature colors from a specific hue [0, 360]
    pub fn from_hue(hue: u16) -> CreatureColors {
        let (nr, ng, nb) = util::hsv_to_rgb(hue, 75, 100);
        let (er, eg, eb) = util::hsv_to_rgb(hue, 75, 75);
        let (cr, cg, cb) = util::hsv_to_rgb(hue, 75, 50);
        let (sr, sg, sb) = util::hsv_to_rgb(hue, 75, 95);

        CreatureColors {
            hue,
            node: Color32::from_rgb(nr, ng, nb),
            muscle_extended: Color32::from_rgb(er, eg, eb),
            muscle_contracted: Color32::from_rgb(cr, cg, cb),
            score_text: Color32::from_rgb(sr, sg, sb),
        }
    }

    /// Creates a new [CreatureColors] that is a mutation of the one passed in
    pub fn mutate(colors: &CreatureColors) -> CreatureColors {
        let mut rng = rand::thread_rng();
        let new_hue = (colors.hue() as i16 + rng.gen_range(MUTATE_COLOR_HUE_RANGE)) as u16 % 360;

        CreatureColors::from_hue(new_hue)
    }

    /// The hue the colors were generated off of
    pub fn hue(&self) -> u16 {
        self.hue
    }

    /// The color to be used for the creature's nodes
    pub fn node(&self) -> Color32 {
        self.node
    }

    /// The color to be used for the creature's muscles when extended
    pub fn muscle_extended(&self) -> Color32 {
        self.muscle_extended
    }

    /// The color to be used for the creature's muscles when contracted
    pub fn muscle_contracted(&self) -> Color32 {
        self.muscle_contracted
    }

    /// The color to be used for the creature's score text
    pub fn score_text(&self) -> Color32 {
        self.score_text
    }
}

impl Default for CreatureColors {
    /// Identical to [CreatureColors::new]
    fn default() -> Self {
        Self::new()
    }
}
