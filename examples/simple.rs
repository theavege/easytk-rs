mod components;

use easytk::prelude::*;

#[cfg(target_os = "linux")]
fn main() -> gtk::glib::ExitCode {
    components::unix::Converter::run()
}

#[cfg(target_os = "windows")]
fn main() -> Result<(), FltkError> {
    components::wine::Converter::run(Settings {
        xclass: Some("Simple"),
        icon: Some(SvgImage::from_data(include_str!("../assets/logo.svg")).unwrap()),
        ..Default::default()
    })
}
