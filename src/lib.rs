#[cfg(target_os = "windows")]
pub mod prelude {
    const LINE: i32 = 3;
    use fltk::app;
    pub use {
        fltk::{
            app::{MouseButton, awake, event_coords, event_key, event_mouse_button},
            browser::{Browser, BrowserType},
            button::{Button, ButtonType},
            dialog::{FileChooser, FileChooserType, alert_default, choice2_default},
            draw,
            enums::{Align, CallbackTrigger, Color, Cursor, Event, Font, FrameType, Key, Shortcut},
            frame::Frame,
            group::{Flex, FlexType, Scroll, Wizard},
            image::{SharedImage, SvgImage},
            input::{Input, InputType},
            menu::{Choice, MenuButton, MenuButtonType, MenuFlag},
            misc::{HelpView, Progress, Tooltip},
            prelude::*,
            text::{
                StyleTableEntry, StyleTableEntryExt, TextAttr, TextBuffer, TextDisplay, TextEditor,
                WrapMode,
            },
            valuator::{Counter, CounterType, Slider, SliderType},
            window::Window,
        },
        std::{
            sync::{
                Arc, RwLock,
                mpsc::{Receiver, Sender, channel},
            },
            thread,
            time::{Duration, Instant},
        },
    };

    #[derive(Default)]
    pub struct Settings {
        pub fullscreen: bool,
        pub size: Option<(i32, i32)>,
        pub font_size: Option<u8>,
        pub font: Option<Font>,
        pub xclass: Option<&'static str>,
        pub icon: Option<SvgImage>,
    }
    impl Settings {
        pub fn config(&self) -> Window {
            app::set_scheme(app::Scheme::Base);
            app::set_visible_focus(false);
            app::set_frame_type2(FrameType::UpBox, FrameType::ThinUpBox);
            app::set_frame_type2(FrameType::DownBox, FrameType::ThinDownBox);
            //~ app::set_background_color(238, 232, 213);
            app::set_background_color(212, 208, 200);
            //~ app::set_background2_color(253, 246, 227);
            //~ app::set_foreground_color(88, 110, 117);
            //~ app::set_selection_color(203, 75, 22);
            app::set_selection_color(10, 36, 106);
            //~ app::set_inactive_color(181, 137, 0);
            Tooltip::set_color(Color::Background2);
            Tooltip::set_text_color(Color::Foreground);
            for (color, (r, g, b)) in [
                (Color::Red, (220, 50, 47)),
                (Color::Magenta, (211, 54, 130)),
                (Color::Blue, (38, 139, 210)),
                (Color::Cyan, (42, 161, 152)),
                (Color::Green, (133, 153, 0)),
                (Color::Yellow, (181, 137, 0)),
            ] {
                app::set_color(color, r, g, b);
            }
            app::set_font(match cfg!(target_os = "windows") {
                true => Font::by_name("BCascadia Mono"),
                false => self.font.unwrap_or(Font::CourierBold),
            });
            app::set_font_size(self.font_size.unwrap_or(14));
            let (w, h) = self.size.unwrap_or((400, 640));
            let mut wgt = Window::default().with_size(w, h).center_screen();
            wgt.set_xclass(self.xclass.unwrap_or("FLTK"));
            wgt.set_label(self.xclass.unwrap_or("FLTK"));
            wgt.size_range(w, h, 0, 0);
            wgt.fullscreen(self.fullscreen);
            wgt.make_resizable(true);
            wgt.set_icon(self.icon.clone());
            wgt.end();
            wgt.show();
            wgt
        }
    }

    pub trait Component
    where
        Self: Default + Clone + 'static,
    {
        type State: Default;
        type Event: 'static;
        fn view(&mut self, sender: Sender<Self::Event>) -> impl WidgetExt;
        fn update(&mut self, model: &Self::State);
        fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool;
        fn mount(&mut self) -> impl WidgetExt {
            let (sender, resiver) = channel::<Self::Event>();
            let mut model = Self::State::default();
            self.update(&model);
            const TICK: f64 = 0.02;
            app::add_timeout3(TICK, {
                let mut page = self.clone();
                let sender = sender.clone();
                move |handle| {
                    if let Ok(msg) = resiver.try_recv()
                        && Self::handle(msg, &mut model, sender.clone())
                    {
                        page.update(&model);
                    }
                    app::repeat_timeout3(TICK, handle);
                }
            });
            self.view(sender)
        }
        fn run(settings: Settings) -> Result<(), FltkError> {
            let app = app::App::default().load_system_fonts();
            let mut wgt = settings.config();
            wgt.begin();
            wgt.add(&Self::default().mount());
            wgt.end();
            wgt.set_callback(move |wgt| {
                if app::event() == Event::Close {
                    wgt.hide();
                }
            });
            app.run()
        }
    }
    pub trait Update<T>
    where
        Self: WidgetExt,
    {
        fn update(&mut self, value: T);
    }

    impl Update<&String> for TextDisplay {
        fn update(&mut self, value: &String) {
            let mut buffer = match self.buffer() {
                Some(buf) => buf,
                None => {
                    self.set_buffer(TextBuffer::default());
                    self.buffer().unwrap()
                }
            };
            if buffer.text() != *value {
                buffer.set_text(value);
                self.scroll(buffer.text().lines().count() as i32, 0);
            };
        }
    }

    impl Update<&String> for TextEditor {
        fn update(&mut self, value: &String) {
            if !self.has_focus() {
                let mut buffer = match self.buffer() {
                    Some(buf) => buf,
                    None => {
                        self.set_buffer(TextBuffer::default());
                        self.buffer().unwrap()
                    }
                };
                if buffer.text() != *value {
                    buffer.set_text(value);
                }
            };
        }
    }

    impl Update<&String> for Input {
        fn update(&mut self, value: &String) {
            if !self.has_focus() && self.value() != *value {
                self.set_value(value);
            };
        }
    }

    impl Update<(Vec<String>, i32)> for Choice {
        fn update(&mut self, value: (Vec<String>, i32)) {
            if self.size() != value.0.len() as i32 {
                self.clear();
                if !value.0.is_empty() {
                    self.add_choice(&value.0.join("|").replace(r#"/"#, r#"\/"#));
                }
            };
            match self.size() > 0 {
                true => self.activate(),
                false => self.deactivate(),
            }
            if self.value() != value.1 {
                self.set_value(value.1);
            };
        }
    }

    impl Update<i32> for Choice {
        fn update(&mut self, value: i32) {
            if self.value() != value {
                self.set_value(value);
            };
        }
    }

    impl Update<(f64, String)> for Progress {
        fn update(&mut self, value: (f64, String)) {
            if self.value() != value.0 {
                self.set_value(value.0);
            };
            if self.label() != value.1 {
                self.set_label(&value.1)
            };
        }
    }

    impl Update<f64> for Slider {
        fn update(&mut self, value: f64) {
            if !self.has_focus() && self.value() != value {
                self.set_value(value);
            };
        }
    }

    impl Update<bool> for Button {
        fn update(&mut self, value: bool) {
            if self.active() != value {
                match value {
                    true => self.activate(),
                    false => self.deactivate(),
                };
            };
        }
    }

    impl Update<bool> for Slider {
        fn update(&mut self, value: bool) {
            if self.active() != value {
                match value {
                    true => self.activate(),
                    false => self.deactivate(),
                };
            };
        }
    }

    impl Update<(Option<String>, i32)> for Frame {
        fn update(&mut self, value: (Option<String>, i32)) {
            if self.tooltip() != value.0 {
                if let Some(tooltip) = value.0 {
                    self.set_tooltip(&tooltip);
                };
                self.do_callback();
            };
            if self.label_size() != value.1 {
                self.set_label_size(value.1);
                self.do_callback();
            };
        }
    }

    pub trait Config
    where
        Self: WidgetExt,
    {
        fn config(&mut self);
    }

    impl Config for TextDisplay {
        fn config(&mut self) {
            self.set_align(Align::Left);
            self.set_trigger(CallbackTrigger::Changed);
            self.set_buffer(TextBuffer::default());
            self.set_linenumber_width(0);
            self.set_text_size(16);
            self.set_scrollbar_size(LINE);
            self.wrap_mode(WrapMode::AtBounds, 0);
        }
    }

    impl Config for Slider {
        fn config(&mut self) {
            self.set_align(Align::Left);
            self.set_type(SliderType::HorizontalNice);
            self.set_color(Color::Background2);
        }
    }

    impl Config for Frame {
        fn config(&mut self) {
            self.set_frame(FrameType::FlatBox);
            self.handle(Self::listen);
        }
    }

    impl Config for Counter {
        fn config(&mut self) {
            self.set_align(Align::Left);
            self.set_type(CounterType::Simple);
            self.set_precision(0);
        }
    }

    impl Config for Progress {
        fn config(&mut self) {
            self.set_selection_color(Color::Selection);
        }
    }

    impl Config for TextEditor {
        fn config(&mut self) {
            self.set_align(Align::Left);
            self.set_trigger(CallbackTrigger::Changed);
            self.set_linenumber_width(0);
            self.set_buffer(TextBuffer::default());
            self.set_text_size(14);
            self.set_scrollbar_size(LINE);
            self.wrap_mode(WrapMode::AtBounds, 0);
            self.kf_end();
        }
    }

    pub trait Listener
    where
        Self: WidgetExt,
    {
        fn listen(wgt: &mut Self, event: Event) -> bool;
    }

    impl Listener for Frame {
        fn listen(wgt: &mut Self, event: Event) -> bool {
            let mut flex = Flex::from_dyn_widget(&wgt.parent().unwrap()).unwrap();
            match event {
                Event::Push => true,
                Event::Drag => {
                    let child0 = flex.child(0).unwrap();
                    let child2 = flex.child(2).unwrap();
                    match flex.get_type() {
                        FlexType::Column => {
                            let y = flex.margin() * 2 + wgt.h();
                            if (y..(flex.h() - y) / 2).contains(&app::event_y()) {
                                flex.fixed(&child2, flex.h() - app::event_y());
                                flex.fixed(&child0, 0);
                            } else if ((flex.h() - y) / 2..=(flex.h() - y))
                                .contains(&app::event_y())
                            {
                                flex.fixed(&child2, 0);
                                flex.fixed(&child0, app::event_y());
                            }
                        }
                        FlexType::Row => {
                            let x = flex.margin() * 2 + wgt.w();
                            if (x..=(flex.w() - x) / 2).contains(&app::event_x()) {
                                flex.fixed(&child2, flex.w() - app::event_x());
                                flex.fixed(&child0, 0);
                            } else if ((flex.w() - x) / 2..=(flex.w() - x))
                                .contains(&app::event_x())
                            {
                                flex.fixed(&child0, app::event_x());
                                flex.fixed(&child2, 0);
                            }
                        }
                    }
                    app::redraw();
                    true
                }
                Event::Enter => {
                    draw::set_cursor(match flex.get_type() {
                        FlexType::Column => Cursor::NS,
                        FlexType::Row => Cursor::WE,
                    });
                    true
                }
                Event::Leave => {
                    draw::set_cursor(Cursor::Arrow);
                    true
                }
                _ => false,
            }
        }
    }

    impl Listener for Flex {
        fn listen(wgt: &mut Self, event: Event) -> bool {
            if event == Event::Resize {
                if wgt.children() == 3 {
                    wgt.set_type(match wgt.w() < wgt.h() {
                        true => FlexType::Column,
                        false => FlexType::Row,
                    });
                    wgt.fixed(&wgt.child(0).unwrap(), 0);
                    wgt.fixed(&wgt.child(1).unwrap(), LINE);
                    wgt.fixed(&wgt.child(2).unwrap(), 0);
                }
                return true;
            }
            false
        }
    }

    pub fn choice_file(filter: &str) -> Option<String> {
        let mut dialog = FileChooser::new(
            std::env::var(match cfg!(target_os = "windows") {
                true => "HOMEPATH",
                false => "HOME",
            })
            .unwrap(),
            filter,
            match filter.is_empty() {
                true => FileChooserType::Directory,
                false => FileChooserType::Create,
            },
            "Choce ...",
        );
        dialog.show();
        while dialog.shown() {
            app::wait();
        }
        match dialog.count() {
            0 => None,
            _ => dialog.value(1),
        }
    }

    pub fn choice_files(filter: &str) -> Option<Vec<String>> {
        let mut dialog = FileChooser::new(
            std::env::var(match cfg!(target_os = "windows") {
                true => "HOMEPATH",
                false => "HOME",
            })
            .unwrap(),
            filter,
            FileChooserType::Multi,
            "Choose File...",
        );
        dialog.show();
        while dialog.shown() {
            app::wait();
        }
        match dialog.count() {
            0 => None,
            _ => Some(
                (1..=dialog.count())
                    .filter_map(|x| dialog.value(x))
                    .collect(),
            ),
        }
    }
}

#[cfg(target_os = "linux")]
pub mod prelude {
    pub use {gtk::prelude::*, std::sync::mpsc::Sender};

    pub trait Component
    where
        Self: Default + 'static,
    {
        type Event: 'static;
        type State: Default + 'static;
        fn view(&self, sender: Sender<Self::Event>) -> gtk::Box;
        fn update(&self, model: &Self::State);
        fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool;
        fn mount() -> gtk::Box {
            let (sender, resiver) = std::sync::mpsc::channel::<Self::Event>();
            let mut model = Self::State::default();
            let page = Self::default();
            let view = page.view(sender.clone());
            page.update(&model);
            gtk::glib::timeout_add_local(std::time::Duration::from_millis(20), {
                let sender = sender.clone();
                move || {
                    if let Ok(msg) = resiver.try_recv()
                        && Self::handle(msg, &mut model, sender.clone())
                    {
                        page.update(&model);
                    }
                    gtk::glib::ControlFlow::Continue
                }
            });
            view
        }
        fn run() -> gtk::glib::ExitCode {
            let app = gtk::Application::builder()
                .application_id("io.gitlab.kbit")
                .build();
            app.connect_activate(move |app| {
                gtk::ApplicationWindow::builder()
                    .application(app)
                    .width_request(640)
                    .height_request(400)
                    .child(&Self::mount())
                    .build()
                    .show_all();
            });
            app.run()
        }
    }

    pub trait Update<T>
    where
        Self: WidgetExt,
    {
        fn update(&self, value: T);
    }

    impl Update<&String> for gtk::Entry {
        fn update(&self, value: &String) {
            if !self.has_visible_focus() {
                self.buffer().set_text(value);
            }
        }
    }
}
