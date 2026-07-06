mod components;

use easytk::prelude::*;

#[cfg(target_os = "linux")]
fn main() -> gtk::glib::ExitCode {
    components::unix::Application::run("io.gitlab.kbit.rust", 640, 400)
}

#[cfg(target_os = "windows")]
fn main() -> Result<(), FltkError> {
    components::wine::Converter::run(Settings {
        xclass: Some("io.gitlab.kbit.rust"),
        icon: Some(SvgImage::from_data(include_str!("../assets/logo.svg")).unwrap()),
        ..Default::default()
    })
}
