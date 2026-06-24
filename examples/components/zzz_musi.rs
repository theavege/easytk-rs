use crate::*;

#[derive(Clone, Default)]
pub struct Model {}

#[derive(Clone, Default)]
pub struct Music {}

pub enum Msg {
    Song(String),
    Volume(f64),
    Play(bool),
}

impl Component for Music {
    type Event = Msg;
    type State = Model;
    fn handle(msg: Self::Event, _: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Song(_value) => {}
            Msg::Volume(_value) => {}
            Msg::Play(_value) => {}
        };
        true
    }
    fn update(&mut self, _: &Self::State) {}
    fn view(&mut self, sender: Sender<Self::Event>) -> impl WidgetExt {
        let mut wgt = Flex::default_fill().column();
        wgt.set_label("Music");
        wgt.set_frame(FrameType::FlatBox);
        wgt.set_margin(PAD);
        wgt.set_pad(PAD);
        wgt.fixed(
            &{
                let mut wgt = Flex::default_fill(); //HEADER
                wgt.set_margin(0);
                wgt.set_pad(PAD);
                wgt.fixed(
                    &{
                        let mut wgt = Button::default();
                        wgt.set_type(ButtonType::Toggle);
                        wgt.set_value(false);
                        wgt.set_callback({
                            let sender = sender.clone();
                            move |wgt| {
                                sender.send(Msg::Play(wgt.value())).unwrap();
                            }
                        });
                        wgt.do_callback();
                        wgt
                    },
                    HEIGHT,
                );
                wgt.add(&{
                    let mut wgt = Progress::build();
                    wgt.set_tooltip("Duration");
                    wgt
                });
                wgt.fixed(
                    &{
                        let mut wgt = Slider::build();
                        wgt.set_tooltip("Volume");
                        wgt.set_value(25f64);
                        wgt.set_callback({
                            let sender = sender.clone();
                            move |wgt| {
                                sender.send(Msg::Volume(wgt.value() * 0.03)).unwrap();
                            }
                        });
                        wgt.do_callback();
                        wgt
                    },
                    WIDTH * 2,
                );
                wgt.end();
                wgt
            },
            HEIGHT,
        );
        wgt.add(&{
            let mut wgt = Flex::default().column();
            wgt.set_frame(FrameType::RFlatBox);
            wgt.set_color(Color::Background2);
            wgt.set_pad(0);
            wgt.set_margin(5);
            wgt.add(&{
                let mut wgt = Browser::build();
                wgt.set_tooltip("Songs");
                wgt.set_frame(FrameType::FlatBox);
                wgt.set_scrollbar_size(LINE);
                wgt.set_text_size(16);
                wgt.set_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        if let Some(value) = wgt.selected_text() {
                            sender.send(Msg::Song(value)).unwrap();
                        }
                    }
                });
                wgt
            });
            wgt.end();
            wgt
        });
        wgt.close(move |wgt| {
            if let Some(mut window) = wgt.window() {
                window.hide();
            }
        });
        wgt.end();
        wgt
    }
}

impl Build for Browser {
    fn build() -> Self {
        let cash = Arc::new(RwLock::new(Vec::<String>::new()));
        let mut wgt = Self::default();
        wgt.set_type(BrowserType::Hold);
        wgt.handle(move |list, event| match event {
            Event::Push => match event_mouse_button() {
                MouseButton::Right => {
                    let mut wgt = MenuButton::default();
                    wgt.add_choice("@#+  Add ...|@#1+  Remove ...");
                    wgt.set_type(MenuButtonType::Popup1);
                    wgt.set_text_size(14);
                    wgt.set_callback({
                        let mut list = list.clone();
                        let cash = cash.clone();
                        move |menu| {
                            let mut current = list.value();
                            match menu.value() {
                                0 => {
                                    if let Some(value) = choice_files("*.{png}") {
                                        for item in value {
                                            if !cash.read().unwrap().contains(&item) {
                                                cash.write().unwrap().push(item);
                                            }
                                        }
                                        current = 1;
                                    }
                                }
                                1 => {
                                    if let Some(file) = list.selected_text() {
                                        match choice2_default(
                                            &format!("Remove {file}?"),
                                            "Remove",
                                            "Cancel",
                                            "Permanent",
                                        ) {
                                            Some(0) => {
                                                cash.write()
                                                    .unwrap()
                                                    .remove((list.value() - 1) as usize);
                                                current -= 1;
                                            }
                                            Some(2) => {
                                                if std::fs::remove_file(&file).is_ok() {
                                                    cash.write()
                                                        .unwrap()
                                                        .remove((list.value() - 1) as usize);
                                                    current -= 1
                                                }
                                            }
                                            _ => {}
                                        };
                                    }
                                }
                                _ => {}
                            };
                            cash.write().unwrap().sort();
                            list.clear();
                            for item in &cash.read().unwrap().clone() {
                                list.add(item);
                            }
                            list.select(current);
                            list.do_callback();
                        }
                    });
                    wgt.popup();
                    true
                }
                _ => false,
            },
            _ => false,
        });
        wgt
    }
}
