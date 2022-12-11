//! Contains resources used by the program, such as the application icon

use eframe::IconData;
use egui::ColorImage;

#[cfg(COMPILING_PLATFORM = "UNIX")]
macro_rules! main_separator {
    () => {
        "/"
    };
}

#[cfg(COMPILING_PLATFORM = "WINDOWS")]
macro_rules! main_separator {
    () => {
        "\\"
    };
}

const ICON: &[u8] = include_bytes!(concat!(
    "..",
    main_separator!(),
    "res",
    main_separator!(),
    "icon.png"
));

const BANNER: &[u8] = include_bytes!(concat!(
    "..",
    main_separator!(),
    "res",
    main_separator!(),
    "banner.png"
));

/// Loads the icon into [IconData]
pub fn load_icon_data() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(ICON)
            .expect("Failed to load icon image")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

/// Loads the banner into [ColorImage]
pub fn load_banner_data() -> ColorImage {
    let image = image::load_from_memory(BANNER)
        .expect("Failed to load banner image")
        .to_rgba8();

    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.as_flat_samples();

    ColorImage::from_rgba_unmultiplied(size, pixels.as_slice())
}
