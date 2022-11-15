use eframe::IconData;

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

const ICON: &[u8] = include_bytes!(concat!("res", main_separator!(), "icon.png"));

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
