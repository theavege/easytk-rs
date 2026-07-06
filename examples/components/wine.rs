use easytk::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct Converter(Input, Input);
impl Component for Converter {
    type Event = f64;
    type State = f64;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        *model = msg;
        true
    }
    fn update(&mut self, model: &Self::State) {
        self.0.update(&((*model - 32.0) * 5.0 / 9.0).to_string());
        self.1.update(&((*model * 9.0 / 5.0) + 32.0).to_string());
    }
    fn view(&mut self, sender: Sender<Self::Event>) -> impl WidgetExt {
        let mut wgt = Wizard::default_fill();
        wgt.set_frame(FrameType::FlatBox);
        wgt.add(&info().with_label("Info"));
        wgt.add(&{
            let mut wgt = Flex::default_fill().with_label("Converter");
            wgt.set_frame(FrameType::FlatBox);
            wgt.set_margin(10);
            wgt.add({
                self.0.set_tooltip("Cel");
                self.0.set_type(InputType::Float);
                self.0.set_trigger(CallbackTrigger::Changed);
                self.0.set_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        if wgt.has_focus()
                            && let Ok(value) = wgt.value().parse::<f64>()
                        {
                            sender.send(value).unwrap();
                        }
                    }
                });
                &self.0
            });
            wgt.add({
                self.1.set_tooltip("Far");
                self.1.set_type(InputType::Float);
                self.1.set_trigger(CallbackTrigger::Changed);
                self.1.set_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        if wgt.has_focus()
                            && let Ok(value) = wgt.value().parse::<f64>()
                        {
                            sender.send(value).unwrap();
                        }
                    }
                });
                &self.1
            });
            wgt
        });
        wgt.add(&Calculator::default().mount());
        wgt.add(&Sudoku::default().mount());
        wgt.add(&Pictures::default().mount());
        wgt.add(&Dialect::default().mount());
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
            wgt.set_value(include_str!("../../assets/README.html"));
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
            wgt.insert(include_str!("../../LICENSE"));
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

#[derive(Clone, Default)]
pub struct Calculator {
    prev: Frame,
    oper: Frame,
    curr: Frame,
    outp: TextDisplay,
}

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;

impl Component for Calculator {
    type Event = String;
    type State = super::mdls::Calculator;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        model.click(&msg);
        true
    }
    fn update(&mut self, model: &Self::State) {
        self.prev.set_label(&model.prev.to_string());
        self.oper.set_label(&model.operation);
        self.curr.set_label(&model.current);
        self.outp.update(&model.output);
    }
    fn view(&mut self, sender: Sender<Self::Event>) -> impl WidgetExt {
        let mut wgt = Flex::default_fill();
        wgt.set_label("Calculator");
        wgt.set_frame(FrameType::FlatBox);
        wgt.set_margin(0);
        wgt.set_pad(0);
        wgt.add(&{
            let mut wgt = Flex::default_fill();
            wgt.set_margin(PAD);
            wgt.add(&Frame::default()); //LEFT
            wgt.fixed(
                &{
                    let mut wgt = Flex::default_fill().column();
                    wgt.add(&{
                        let mut wgt = Flex::default_fill().column(); //UP
                        wgt.set_margin(0);
                        wgt.set_pad(PAD);
                        wgt.add(&{
                            let mut wgt = Flex::default_fill().column(); //OUTPUT
                            wgt.set_frame(FrameType::RFlatBox);
                            wgt.set_color(Color::Background2);
                            wgt.set_margin(5);
                            wgt.set_pad(0);
                            wgt.add({
                                self.outp.set_tooltip("Output");
                                self.outp.set_text_size(HEIGHT - 2);
                                self.outp.set_scrollbar_size(3);
                                self.outp.set_frame(FrameType::FlatBox);
                                self.outp.config();
                                &self.outp
                            });
                            wgt.fixed(
                                &{
                                    let mut wgt = Frame::default();
                                    wgt.set_frame(FrameType::FlatBox);
                                    wgt
                                },
                                3,
                            );
                            wgt.fixed(
                                &{
                                    let mut wgt = Flex::default_fill();
                                    wgt.set_pad(0);
                                    wgt.set_margin(0);
                                    wgt.add({
                                        config_frame(&mut self.oper, "Operation");
                                        &self.oper
                                    });
                                    wgt.fixed(&self.oper, HEIGHT);
                                    wgt.add(&{
                                        let mut wgt = Flex::default_fill().column();
                                        wgt.set_pad(0);
                                        wgt.set_margin(0);
                                        wgt.add({
                                            config_frame(&mut self.prev, "Previous");
                                            &self.prev
                                        });
                                        wgt.add({
                                            config_frame(&mut self.curr, "Current");
                                            &self.curr
                                        });
                                        wgt.end();
                                        wgt
                                    });
                                    wgt.end();
                                    wgt
                                },
                                HEIGHT * 2,
                            );
                            wgt.end();
                            wgt
                        });
                        wgt.fixed(
                            &{
                                let mut buttons = Flex::default_fill().column();
                                buttons.set_pad(PAD);
                                buttons.set_margin(0);
                                for line in [
                                    ["CE", "C", "%", "/"],
                                    ["7", "8", "9", "x"],
                                    ["4", "5", "6", "-"],
                                    ["1", "2", "3", "+"],
                                    ["0", ".", "@<-", "="],
                                ] {
                                    let mut hbox = Flex::default_fill();
                                    for label in line {
                                        hbox.add(&{
                                            let mut wgt = btn();
                                            wgt.set_label(label);
                                            wgt.set_label_size(HEIGHT);
                                            wgt.set_frame(FrameType::OFlatBox);
                                            wgt.set_color(
                                                match [
                                                    ".", "@<-", "CE", "C", "x", "/", "+", "-", "%",
                                                ]
                                                .contains(&label)
                                                {
                                                    false => Color::Selection,
                                                    true => Color::Inactive,
                                                },
                                            );
                                            wgt.set_callback({
                                                let sender = sender.clone();
                                                move |wgt| {
                                                    sender.send(wgt.label()).unwrap();
                                                }
                                            });
                                            wgt
                                        });
                                    }
                                    hbox.end();
                                    hbox.set_pad(PAD);
                                    hbox.set_margin(0);
                                }
                                buttons.end();
                                buttons
                            },
                            425,
                        );
                        wgt.end();
                        wgt
                    });
                    wgt.end();
                    wgt
                },
                340,
            );
            wgt.add(&Frame::default());
            wgt.end();
            wgt
        });
        wgt.end();
        wgt.handle(add_wizard);
        wgt
    }
}

fn config_frame(wgt: &mut Frame, tooltip: &str) {
    wgt.set_tooltip(tooltip);
    wgt.set_align(Align::Right | Align::Inside);
    wgt.set_color(Color::Background2);
    wgt.set_frame(FrameType::FlatBox);
    wgt.set_label_size(HEIGHT);
}

fn btn() -> Frame {
    let mut color = Color::Background2;
    let mut wgt = Frame::default();
    wgt.set_frame(FrameType::RFlatBox);
    wgt.set_label_color(color);
    wgt.handle(move |wgt, event| match event {
        Event::Push => {
            wgt.do_callback();
            color = wgt.color();
            wgt.set_color(color.lighter());
            wgt.redraw();
            true
        }
        Event::Released => {
            wgt.set_color(color);
            wgt.redraw();
            true
        }
        Event::Enter => {
            draw::set_cursor(Cursor::Hand);
            true
        }
        Event::Leave => {
            draw::set_cursor(Cursor::Arrow);
            true
        }
        _ => false,
    });
    wgt
}

#[derive(Clone, Default)]
pub struct Pictures {
    prev: Button,
    next: Button,
    remo: Button,
    canv: Frame,
    list: Choice,
    valu: Slider,
}

impl Component for Pictures {
    type Event = super::msgs::Pictures;
    type State = super::mdls::Pictures;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Self::Event::Add(value) => model.set_list(value),
            Self::Event::Idx(value) => model.idx = value,
            Self::Event::Shift(value) => model.shift(value),
            Self::Event::Del(value) => model.del(value),
            Self::Event::Scale(value) => model.scale = value,
        };
        true
    }
    fn update(&mut self, model: &Self::State) {
        self.prev.update(model.list.len() > 1);
        self.next.update(model.list.len() > 1);
        self.remo.update(model.list.len() > 1);
        self.valu.update(!model.list.is_empty());
        self.list.update((model.list.clone(), model.idx as i32));
        self.canv.update((model.path(), model.scale as i32));
    }
    fn view(&mut self, sender: Sender<Self::Event>) -> impl WidgetExt {
        let mut wgt = Flex::default_fill().column();
        wgt.set_label("Pictures");
        wgt.set_frame(FrameType::FlatBox);
        wgt.set_margin(PAD);
        wgt.set_pad(PAD);
        wgt.fixed(
            &{
                let mut wgt = Flex::default();
                wgt.set_pad(0);
                wgt.fixed(
                    &{
                        let mut wgt = Button::default();
                        wgt.set_label("@+");
                        wgt.set_tooltip("Add ...");
                        wgt.set_label_color(Color::Green);
                        wgt.set_shortcut(Shortcut::Ctrl | 'a');
                        wgt.set_callback({
                            let sender = sender.clone();
                            move |_| {
                                if let Some(value) = choice_files("*.{png,svg}") {
                                    sender.send(Self::Event::Add(value)).unwrap();
                                }
                            }
                        });
                        wgt
                    },
                    HEIGHT,
                );
                wgt.add(&self.prev);
                wgt.fixed(
                    {
                        self.prev.set_label("@<-");
                        self.prev.set_tooltip("Prev ...");
                        self.prev.set_shortcut(Shortcut::Ctrl | 'j');
                        self.prev.set_callback({
                            let sender = sender.clone();
                            move |_| {
                                sender.send(Self::Event::Shift(false)).unwrap();
                            }
                        });
                        &self.prev
                    },
                    HEIGHT,
                );
                wgt.add({
                    self.list.set_tooltip("List");
                    self.list.set_callback({
                        let sender = sender.clone();
                        move |menu| {
                            sender
                                .send(Self::Event::Idx(menu.value() as usize))
                                .unwrap();
                        }
                    });
                    &self.list
                });
                wgt.add(&self.next);
                wgt.fixed(
                    {
                        self.next.set_label("@->");
                        self.next.set_tooltip("Next ...");
                        self.next.set_shortcut(Shortcut::Ctrl | 'k');
                        self.next.set_callback({
                            let sender = sender.clone();
                            move |_| {
                                sender.send(Self::Event::Shift(true)).unwrap();
                            }
                        });
                        &self.next
                    },
                    HEIGHT,
                );
                wgt.add(&self.remo);
                wgt.fixed(
                    {
                        self.remo.set_label("@1+");
                        self.remo.set_tooltip("Remove ...");
                        self.remo.set_shortcut(Shortcut::Ctrl | 'd');
                        self.remo.set_label_color(Color::Red);
                        self.remo.set_callback({
                            let sender = sender.clone();
                            move |_| {
                                match choice2_default(
                                    "Remove ...?",
                                    "Remove",
                                    "Cancel",
                                    "Permanent",
                                ) {
                                    Some(0) => sender.send(Self::Event::Del(false)).unwrap(),
                                    Some(2) => sender.send(Self::Event::Del(true)).unwrap(),
                                    _ => {}
                                };
                            }
                        });
                        &self.remo
                    },
                    HEIGHT,
                );
                wgt.end();
                wgt
            },
            HEIGHT,
        );
        wgt.add({
            config_canvas(&mut self.canv);
            &self.canv
        });
        wgt.add(&self.valu);
        wgt.fixed(
            {
                self.valu.config();
                self.valu.set_value(0.5);
                self.valu.set_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        sender
                            .send(Self::Event::Scale(wgt.value() * 100.0))
                            .unwrap()
                    }
                });
                self.valu.do_callback();
                &self.valu
            },
            HEIGHT,
        );
        wgt.end();
        wgt.handle(add_wizard);
        wgt
    }
}

fn config_canvas(wgt: &mut Frame) {
    use std::collections::HashMap;
    let mut cash: HashMap<String, SharedImage> = HashMap::new();
    wgt.set_frame(FrameType::FlatBox);
    wgt.set_color(Color::Background2);
    wgt.set_callback(move |frame| {
        let scale = frame.label_size() as f64;
        match frame.tooltip() {
            None => frame.set_image(None::<SharedImage>),
            Some(file) => {
                frame.set_image(None::<SharedImage>);
                if let Some(mut window) = frame.window() {
                    window.redraw();
                };
                if !file.is_empty() {
                    if !cash.contains_key(&file)
                        && let Ok(image) = SharedImage::load(&file)
                    {
                        cash.insert(file.clone(), image);
                    }
                    let mut image = cash.get(&file).unwrap().clone();
                    image.scale(
                        (scale / 100.0 * (frame.w()) as f64) as i32,
                        (scale / 100.0 * (frame.h()) as f64) as i32,
                        true,
                        false,
                    );
                    frame.set_image(Some(image));
                }
            }
        }
        frame.redraw();
    });
}

#[derive(Clone, Default)]
pub struct Sudoku([[Frame; 9]; 9]);

impl Component for Sudoku {
    type Event = super::msgs::Sudoku;
    type State = super::mdls::Sudoku;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Self::Event::Push(row, col, value) => model.0[row][col] = value,
            Self::Event::Clear => model.clear(),
            Self::Event::Solve => model.answer(),
        };
        true
    }
    fn update(&mut self, model: &Self::State) {
        for row in 0..9 {
            for col in 0..9 {
                self.0[row][col].set_label(&match model.0[row][col] {
                    0 => String::new(),
                    _ => model.0[row][col].to_string(),
                });
            }
        }
    }
    fn view(&mut self, sender: Sender<Self::Event>) -> impl WidgetExt {
        let mut wgt = Flex::default_fill();
        wgt.set_label("Sudoku");
        wgt.set_frame(FrameType::FlatBox);
        wgt.set_margin(0);
        wgt.set_pad(0);
        wgt.add(&{
                let mut wgt = Flex::default_fill();
                wgt.add(&Frame::default_fill()); //LEFT
                wgt.add(&{
                    let mut wgt = Flex::default_fill().column(); // CENTER
                    wgt.set_pad(PAD);
                    wgt.set_margin(0);
                    wgt.add(&Frame::default()); //UP
                    wgt.add(&{
                        let pad = 4;
                        let mut vbox = Flex::default_fill().column(); // CENTER
                        vbox.set_frame(FrameType::RoundedFrame);
                        vbox.set_color(Color::Foreground);
                        vbox.set_pad(pad);
                        vbox.set_margin(5);
                        for row in 0..9 {
                            if row > 0 && row % 3 == 0 {
                                vbox.fixed(&Frame::default(), pad * 2);
                            };
                            let mut flex = Flex::default_fill();
                            flex.set_pad(pad);
                            flex.set_margin(0);
                            for col in 0..9 {
                                if col > 0 && col % 3 == 0 {
                                    flex.fixed(&Frame::default(), pad * 2);
                                };
                                flex.add({
                                    self.0[row][col].draw(draw_frame);
                                    self.0[row][col].handle({
                                        let sender = sender.clone();
                                        move |frm, event| match event {
                                            Event::Push => match event_mouse_button() {
                                                MouseButton::Left => {
                                                        let mut wgt = MenuButton::default();
                                                        wgt.set_pos(frm.x(),frm.y());
                                                        wgt.set_size(frm.w(),frm.h());
                                                        wgt.add_choice(
                                                            "  X  |  1  |  2  |  3  |  4  |  5  |  6  |  7  |  8  |  9  "
                                                        );
                                                        wgt.set_callback({
                                                            let sender = sender.clone();
                                                            move |menu| {
                                                                sender.send(Self::Event::Push(row, col, menu.value())).unwrap();
                                                            }
                                                        });
                                                    wgt.popup();
                                                    true
                                                }
                                                MouseButton::Right => {
                                                        let mut wgt = MenuButton::default();
                                                        wgt.set_pos(frm.x(),frm.y());
                                                        wgt.set_size(frm.w(),frm.h());
                                                        wgt.add_choice("@search  Solve|@1+  Clear");
                                                        wgt.set_callback({
                                                            let sender = sender.clone();
                                                            move |menu| {
                                                                sender.send(match menu.value() {
                                                                    0 => Self::Event::Solve,
                                                                    _ => Self::Event::Clear,
                                                                }).unwrap();
                                                            }
                                                        });
                                                        wgt.popup();
                                                    true
                                                }
                                                _ => false,
                                            }
                                            Event::Enter => {
                                                draw::set_cursor(Cursor::Hand);
                                                true
                                            }
                                            Event::Leave => {
                                                draw::set_cursor(Cursor::Arrow);
                                                true
                                            }
                                            _ => false,
                                        }
                                    });
                                    &self.0[row][col]
                                });
                            }
                            flex.end();
                        }
                        vbox.end();
                        vbox
                    });
                    wgt.add(&Frame::default()); //BOTTOM
                    wgt.end();
                    wgt.handle(add_resize); // RESIZE CENTER's HEIGHT
                    wgt
                });
                wgt.add(&Frame::default()); //RIGHT
                wgt.end();
                wgt.handle(add_resize);  // RESIZE CENTER's WIDTH
                wgt.handle_event(Event::Resize);
                wgt
            });
        wgt.end();
        wgt.handle(add_wizard);
        wgt
    }
}

fn draw_frame(frame: &mut Frame) {
    draw::set_draw_color(Color::Background2);
    draw::set_font(Font::CourierBold, (frame.h() as f64 / 4.0 * 3.0) as i32);
    draw::draw_circle_fill(frame.x(), frame.y(), frame.h(), Color::Inactive);
    draw::draw_text2(
        &frame.label(),
        frame.x(),
        frame.y(),
        frame.w(),
        frame.h(),
        Align::Center,
    );
}

fn add_resize(flex: &mut Flex, event: Event) -> bool {
    if event == Event::Resize {
        if flex.children() == 3
            && let Some(child) = flex.child(1)
            && let Some(window) = flex.window()
        {
            flex.fixed(&child, window.w().min(window.h()));
        }
        return true;
    }
    false
}

#[derive(Clone, Default)]
pub struct Dialect {
    translate: Button,
    source: TextEditor,
    target: TextDisplay,
    from: Choice,
    to: Choice,
}

impl Component for Dialect {
    type Event = super::msgs::Dialect;
    type State = super::mdls::Dialect;
    fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool {
        match msg {
            Self::Event::Switch => {
                model.switch();
                true
            }
            Self::Event::Open(value) => {
                model.source = std::fs::read_to_string(value).unwrap();
                true
            }
            Self::Event::SaveAs(value) => {
                std::fs::write(value, model.target.as_bytes()).unwrap();
                false
            }
            Self::Event::Target(value) => {
                model.target = value;
                true
            }
            Self::Event::Lang(value) => {
                model.read(value);
                true
            }
            Self::Event::To(value) => {
                model.to = value;
                false
            }
            Self::Event::From(value) => {
                model.from = value;
                true
            }
            Self::Event::Source(value) => {
                model.source = value;
                true
            }
            Self::Event::Quit => {
                model.save();
                false
            }
            Self::Event::Run => {
                if model.from != model.to && !model.source.is_empty() {
                    let url = model.url();
                    std::thread::spawn({
                        let sender = sender.clone();
                        move || {
                            if let Ok(value) = reqwest::blocking::get(&url) {
                                let target = value
                                    .json::<HashMap<String, String>>()
                                    .unwrap()
                                    .get("translation")
                                    .unwrap()
                                    .to_string();
                                sender.send(Self::Event::Target(target)).unwrap();
                            }
                        }
                    });
                };
                false
            }
        }
    }
    fn update(&mut self, state: &Self::State) {
        self.to.update((state.lang(), state.to));
        self.from.update((state.lang(), state.from));
        self.source.update(&state.source);
        self.target.update(&state.target);
        self.translate.update(!state.lang.is_empty());
    }
    fn view(&mut self, sender: Sender<Self::Event>) -> impl WidgetExt {
        std::thread::spawn({
            let sender = sender.clone();
            move || {
                if let Ok(get) = reqwest::blocking::get("https://lingva.ml/api/v1/languages") {
                    let value = get
                        .json::<HashMap<String, Vec<HashMap<String, String>>>>()
                        .unwrap()
                        .get("languages")
                        .unwrap()
                        .iter()
                        .map(|lang| (lang["code"].clone(), lang["name"].clone()))
                        .collect::<Vec<(String, String)>>();
                    if !value.is_empty() {
                        sender.send(Self::Event::Lang(value)).unwrap();
                    }
                }
            }
        });
        let mut wgt = Flex::default_fill();
        wgt.set_label("Dialect");
        wgt.set_frame(FrameType::FlatBox);
        wgt.set_type(FlexType::Column);
        wgt.set_margin(PAD);
        wgt.set_pad(PAD);
        wgt.fixed(
            &{
                let mut wgt = Flex::default();
                wgt.set_type(FlexType::Column);
                wgt.set_margin(0);
                wgt.set_pad(PAD);
                wgt.set_type(FlexType::Row);
                wgt.fixed(
                    &{
                        let mut wgt = Frame::default();
                        wgt.set_label("@menu");
                        wgt.set_tooltip("Menu");
                        wgt.set_color(Color::Red);
                        wgt.set_callback({
                            let sender = sender.clone();
                            move |wgt| {
                                let mut wgt =
                                    MenuButton::default().with_pos(wgt.x(), wgt.y() + wgt.h());
                                wgt.add(
                                    "@fileopen  &Open...",
                                    Shortcut::Ctrl | 'o',
                                    MenuFlag::Normal,
                                    {
                                        let sender = sender.clone();
                                        move |_| {
                                            if let Some(value) = choice_file("*.{txt,md}") {
                                                sender.send(Self::Event::Open(value)).unwrap();
                                            };
                                        }
                                    },
                                );
                                wgt.add(
                                    "@filesaveas  &Save as...",
                                    Shortcut::Ctrl | 's',
                                    MenuFlag::MenuDivider,
                                    {
                                        let sender = sender.clone();
                                        move |_| {
                                            if let Some(value) = choice_file("*.{txt,md}") {
                                                sender.send(Self::Event::SaveAs(value)).unwrap();
                                            };
                                        }
                                    },
                                );
                                wgt.add("@1+  &Quit\t", Shortcut::Ctrl | 'q', MenuFlag::Normal, {
                                    let sender = sender.clone();
                                    move |_| sender.send(Self::Event::Quit).unwrap()
                                });
                                wgt.popup();
                            }
                        });
                        wgt
                    },
                    HEIGHT,
                );
                wgt.add(&Frame::default());
                wgt.add({
                    self.from.set_tooltip("From");
                    self.from.set_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            sender.send(Self::Event::From(wgt.value())).unwrap();
                        }
                    });
                    &self.from
                });
                wgt.fixed(&self.from, 105);
                wgt.fixed(
                    &{
                        let mut wgt = Button::default();
                        wgt.set_label("@refresh");
                        wgt.set_tooltip("Refresh");
                        wgt.set_color(Color::Blue);
                        wgt.set_callback({
                            let sender = sender.clone();
                            move |_| {
                                sender.send(Self::Event::Switch).unwrap();
                            }
                        });
                        wgt
                    },
                    HEIGHT,
                );
                wgt.add({
                    self.to.set_tooltip("To");
                    self.to.set_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            sender.send(Self::Event::To(wgt.value())).unwrap();
                        }
                    });
                    &self.to
                });
                wgt.fixed(&self.to, 105);
                wgt.add(&Frame::default());
                wgt.add({
                    self.translate.set_label("@search");
                    self.translate.set_tooltip("Translate");
                    self.translate.set_color(Color::Green);
                    self.translate.set_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            wgt.deactivate();
                            sender.send(Self::Event::Run).unwrap();
                        }
                    });
                    &self.translate
                });
                wgt.fixed(&self.translate, HEIGHT);
                wgt.end();
                wgt
            },
            HEIGHT,
        );
        wgt.add(&{
            let mut wgt = Flex::default();
            wgt.set_type(FlexType::Column);
            wgt.set_color(Color::Background2);
            wgt.set_frame(FrameType::RFlatBox);
            wgt.set_margin(5);
            wgt.set_pad(0);
            wgt.handle(add_resize);
            wgt.add({
                self.source.config();
                self.source.set_tooltip("Source");
                self.source.set_text_size(16);
                self.source.set_frame(FrameType::FlatBox);
                self.source.set_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        sender
                            .send(Self::Event::Source(wgt.buffer().unwrap().text()))
                            .unwrap();
                    }
                });
                &self.source
            });
            wgt.add(&{
                let mut wgt = Frame::default();
                wgt.config();
                wgt
            });
            wgt.add({
                self.target.config();
                self.target.set_text_size(16);
                self.target.set_tooltip("Target");
                self.target.set_frame(FrameType::FlatBox);
                &self.target
            });
            wgt.end();
            wgt.handle(Flex::listen);
            wgt.handle_event(Event::Resize);
            wgt
        });
        //~ wgt.close({
        //~ let sender = sender.clone();
        //~ move |_| {
        //~ sender.send(Self::Event::Quit).unwrap();
        //~ }
        //~ });
        wgt.end();
        wgt
    }
}

//~ const TO_JSON: Event = Event::from_i32(101);
//~ fn handle_highlight(display: &mut TextDisplay, event: Event) -> bool {
//~ let text = display.buffer().unwrap().text();
//~ match event {
//~ TO_JSON => {
//~ use json_tools::{Buffer, BufferType, Lexer, Span, TokenType};
//~ let mut buffer = vec![b'A'; text.len()];
//~ for token in Lexer::new(text.bytes(), BufferType::Span) {
//~ let c = match token.kind {
//~ TokenType::String => 'B',
//~ TokenType::BooleanTrue | TokenType::BooleanFalse | TokenType::Null => 'C',
//~ TokenType::Number => 'D',
//~ _ => 'A',
//~ };
//~ if let Buffer::Span(Span { first, end }) = token.buf {
//~ let start = first as _;
//~ let last = end as _;
//~ buffer[start..last]
//~ .copy_from_slice(c.to_string().repeat(last - start).as_bytes());
//~ }
//~ }
//~ let mut buf = TextBuffer::default();
//~ buf.set_text(&String::from_utf8_lossy(&buffer));
//~ let styles: Vec<StyleTableEntryExt> =
//~ [Color::Red, Color::Blue, Color::Green, Color::Yellow]
//~ .into_iter()
//~ .map(|color| StyleTableEntryExt {
//~ color,
//~ font: display.text_font(),
//~ size: display.text_size(),
//~ attr: TextAttr::None,
//~ bgcolor: Color::TransparentBg,
//~ })
//~ .collect();
//~ display.set_highlight_data_ext(buf, styles);
//~ true
//~ }
//~ _ => false,
//~ }
//~ }
