mod models {
    #[derive(Default)]
    pub struct Model(pub f64, pub f64);

    impl Model {
        pub fn set_cel(&mut self, value: f64) {
            self.1 = (value * 9.0 / 5.0) + 32.0;
        }
        pub fn set_far(&mut self, value: f64) {
            self.0 = (value - 32.0) * 5.0 / 9.0;
        }
    }
}

use easytk::prelude::*;

#[cfg(target_os = "linux")]
mod views {
    use super::*;

    pub enum Msg {
        Cel(f64),
        Far(f64),
    }

    #[derive(Default)]
    pub struct View(gtk::Entry, gtk::Entry);

    impl Component for View {
        type Event = Msg;
        type State = models::Model;
        fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
            match msg {
                Msg::Cel(value) => model.set_cel(value),
                Msg::Far(value) => model.set_far(value),
            };
            true
        }
        fn update(&self, model: &Self::State) {
            self.0.update(&model.0.to_string());
            self.1.update(&model.1.to_string());
        }
        fn view(&self, sender: Sender<Self::Event>) -> gtk::Box {
            let pad = 10;
            let wgt = gtk::Box::new(gtk::Orientation::Vertical, pad);
            wgt.set_margin(pad);
            wgt.add({
                self.0.set_placeholder_text(Some("°C"));
                self.0.set_hexpand(true);
                self.0.connect_changed({
                    let sender = sender.clone();
                    move |entry| {
                        if entry.has_visible_focus() {
                            let value = entry.buffer().text().parse::<f64>().unwrap_or_default();
                            sender.send(Msg::Cel(value)).unwrap();
                        }
                    }
                });
                &self.0
            });
            wgt.add({
                self.1.set_placeholder_text(Some("°F"));
                self.1.set_hexpand(true);
                self.1.connect_changed({
                    let sender = sender.clone();
                    move |entry| {
                        if entry.has_visible_focus() {
                            let value = entry.buffer().text().parse::<f64>().unwrap_or_default();
                            sender.send(Msg::Far(value)).unwrap();
                        }
                    }
                });
                &self.1
            });
            wgt
        }
    }
}

#[cfg(target_os = "linux")]
fn main() -> gtk::glib::ExitCode {
    views::View::run()
}

#[cfg(target_os = "windows")]
mod views {
    use super::*;

    pub enum Msg {}

    #[derive(Clone, Default)]
    pub struct View();

    impl Component for View {
        type Event = Msg;
        type State = bool;
        fn handle(_msg: Self::Event, _model: &mut Self::State, _: Sender<Self::Event>) -> bool {
            false
        }
        fn update(&mut self, _model: &Self::State) {}
        fn view(&mut self, _sender: Sender<Self::Event>) -> impl WidgetExt {
            let mut wgt = Wizard::default_fill();
            wgt.set_frame(FrameType::FlatBox);
            wgt.add(&info().with_label("Info"));
            wgt.end();
            wgt
        }
    }

    fn info() -> Flex {
        let mut wgt = Flex::default_fill();
        wgt.set_frame(FrameType::FlatBox);
        wgt.set_margin(10);
        wgt.add(&{
            let mut wgt = Flex::default().column();
            wgt.set_frame(FrameType::RFlatBox);
            wgt.set_color(Color::Background2);
            wgt.set_pad(0);
            wgt.set_margin(5);
            wgt.add(&{
                let mut wgt = HelpView::default();
                wgt.set_color(Color::Background2);
                wgt.set_scrollbar_size(3);
                wgt.set_text_size(16);
                wgt.set_frame(FrameType::FlatBox);
                wgt.set_tooltip("README");
                wgt.set_value(include_str!("../assets/README.html"));
                wgt
            });
            wgt.add(&{
                let mut wgt = Frame::default();
                wgt.config();
                wgt
            });
            wgt.add(&{
                let mut wgt = TextDisplay::default();
                wgt.config();
                wgt.set_frame(FrameType::FlatBox);
                wgt.set_tooltip("LICENSE");
                wgt.insert(include_str!("../LICENSE"));
                wgt
            });
            wgt.end();
            wgt.handle(Flex::listen);
            wgt.handle_event(Event::Resize);
            wgt
        });
        wgt.end();
        wgt.handle(add_wizard);
        wgt
    }

    fn add_wizard(wgt: &mut Flex, event: Event) -> bool {
        let wizard = Wizard::from_dyn_widget(&wgt.parent().unwrap()).unwrap();
        match event {
            Event::Push => match event_mouse_button() {
                MouseButton::Right => {
                    let mut wgt = MenuButton::default();
                    wgt.add_choice(
                        &(0..wizard.children())
                            .map(|x| {
                                let label = wizard.child(x).unwrap().label();
                                match wizard.try_current_widget().unwrap().label() == label {
                                    true => format!("@->  {label}"),
                                    false => format!("@-  {label}"),
                                }
                            })
                            .collect::<Vec<String>>()
                            .join("|"),
                    );
                    wgt.set_callback({
                        let mut wizard = wizard.clone();
                        move |menu| {
                            if let Some(mut child) = wizard.child(menu.value()) {
                                wizard.set_current_widget(&child);
                                if let Some(mut window) = wizard.window() {
                                    window.set_label(&format!(
                                        "{}::{}",
                                        window.xclass().unwrap(),
                                        child.label()
                                    ));
                                };
                                child.handle_event(Event::Resize);
                            }
                        }
                    });
                    wgt.popup();
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }
}

#[cfg(target_os = "windows")]
fn main() -> Result<(), FltkError> {
    views::View::run(Settings {
        xclass: Some("Simple"),
        icon: Some(SvgImage::from_data(include_str!("../assets/logo.svg")).unwrap()),
        ..Default::default()
    })
}
