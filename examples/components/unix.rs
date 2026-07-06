use easytk::prelude::*;

#[derive(Default)]
pub struct Application {}
impl Component for Application {
    type Event = bool;
    type State = bool;
    fn handle(msg: Self::Event, _: &mut Self::State, _: Sender<Self::Event>) -> bool {
        msg
    }
    fn update(&self, _: &Self::State) {}
    fn view(&self, _: Sender<Self::Event>) -> gtk::Box {
        let stack = gtk::Stack::default();
        let sidebar = gtk::StackSidebar::builder().stack(&stack).build();
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, PAD);
        hbox.add(&sidebar);
        hbox.add(&stack);
        stack.add_titled(&Converter::mount(), "Conv", "Conv");
        stack.add_titled(&Curl::mount(), "Curl", "Curl");
        stack.add_titled(&Calc::mount(), "Calc", "Calc");
        stack.add_titled(&Dial::mount(), "Dial", "Dial");
        stack.add_titled(&Sudo::mount(), "Sudo", "Sudo");
        stack.add_titled(
            &{
                let wgt = gtk::Box::new(gtk::Orientation::Vertical, PAD);
                wgt.set_margin(PAD);
                let list_store = gtk::ListStore::new(&[
                    gtk::glib::types::Type::BOOL,
                    gtk::glib::types::Type::STRING,
                    gtk::glib::types::Type::STRING,
                    gtk::glib::Type::U32,
                ]);
                for url in ["https://ipinfo.io/json"] {
                    let name: &str = url.split('/').nth_back(0).unwrap();
                    list_store.set(&list_store.append(), &[(0, &true), (1, &name), (2, &url)]);
                }
                let tree_view = gtk::TreeView::with_model(&list_store);
                for (ord, name) in ["STATUS", "NAME", "URL"].into_iter().enumerate() {
                    match name {
                        "STATUS" => {
                            let renderer = gtk::CellRendererToggle::new();
                            renderer.connect_toggled({
                                let list_store = list_store.clone();
                                move |_, path| {
                                    let iter = list_store.iter(&path).unwrap();
                                    list_store.set_value(
                                        &iter,
                                        ord as u32,
                                        &(!list_store
                                            .value(&iter, ord as i32)
                                            .get::<bool>()
                                            .unwrap())
                                        .to_value(),
                                    );
                                }
                            });
                            tree_view.append_column(&gtk::TreeViewColumn::with_attributes(
                                name,
                                &renderer,
                                &[("active", ord as i32)],
                            ))
                        }
                        _ => tree_view.append_column(&gtk::TreeViewColumn::with_attributes(
                            name,
                            &gtk::CellRendererText::new(),
                            &[("text", ord as i32)],
                        )),
                    };
                }
                let run = gtk::Button::with_mnemonic("Run");
                let result = gtk::Label::with_mnemonic("Result");
                wgt.add(&tree_view);
                wgt.add(&{
                    let wgt = gtk::Box::new(gtk::Orientation::Horizontal, PAD);
                    wgt.add(&run);
                    wgt.add(&result);
                    wgt
                });
                run.connect_clicked({
                    let run = run.clone();
                    move |_| {
                        list_store.foreach(|store, _, iter| {
                            if store.value(iter, 0).get::<bool>().unwrap() {
                                println!("{}", list_store.value(iter, 2).get::<String>().unwrap());
                            };
                            false
                        });
                        let value = result.text();
                        result.set_text(&(value.to_string() + " Done!"));
                        run.set_sensitive(false);
                    }
                });
                let adjustment = gtk::Adjustment::new(0.0, 0.0, 255.0, 1.0, 10.0, 0.0);
                wgt.add(&{
                    let wgt = gtk::SpinButton::with_range(0.0, 255.0, 1.0);
                    wgt.set_adjustment(&adjustment);
                    wgt
                });
                wgt.add(&{
                    let wgt = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 255.0, 1.0);
                    wgt.set_adjustment(&adjustment);
                    wgt
                });
                wgt
            },
            "Othe",
            "Othe",
        );
        hbox
    }
}

#[derive(Default)]
pub struct Converter(gtk::Entry, gtk::Entry);
impl Component for Converter {
    type Event = f64;
    type State = f64;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        *model = msg;
        true
    }
    fn update(&self, model: &Self::State) {
        self.0.update(&((*model - 32.0) * 5.0 / 9.0).to_string());
        self.1.update(&((*model * 9.0 / 5.0) + 32.0).to_string());
    }
    fn view(&self, sender: Sender<Self::Event>) -> gtk::Box {
        let wgt = gtk::Box::new(gtk::Orientation::Vertical, PAD);
        wgt.set_margin(PAD);
        wgt.add(&{
            let wgt = gtk::Box::new(gtk::Orientation::Horizontal, PAD);
            wgt.add({
                self.0.set_placeholder_text(Some("°C"));
                self.0.set_hexpand(true);
                self.0.connect_changed({
                    let sender = sender.clone();
                    move |wgt| {
                        if wgt.has_visible_focus()
                            && let Ok(value) = wgt.buffer().text().parse::<f64>()
                        {
                            sender.send(value).unwrap();
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
                    move |wgt| {
                        if wgt.has_visible_focus()
                            && let Ok(value) = wgt.buffer().text().parse::<f64>()
                        {
                            sender.send(value).unwrap();
                        }
                    }
                });
                &self.1
            });
            wgt
        });
        wgt
    }
}

const PAD: i32 = 10;

#[derive(Default)]
pub struct Calc(gtk::TextBuffer, gtk::Label, gtk::Label, gtk::Label);
impl Component for Calc {
    type Event = String;
    type State = super::models::Calculator;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        model.click(&msg);
        true
    }
    fn update(&self, model: &Self::State) {
        self.0.set_text(&model.output);
        self.1.set_label(&model.operation);
        self.2.set_label(&model.prev.to_string());
        self.3.set_label(&model.current);
    }
    fn view(&self, _sender: Sender<Self::Event>) -> gtk::Box {
        let wgt = gtk::Box::new(gtk::Orientation::Vertical, PAD);
        wgt.set_margin(PAD);
        wgt.add(
            &gtk::ScrolledWindow::builder()
                .hscrollbar_policy(gtk::PolicyType::Automatic)
                .vscrollbar_policy(gtk::PolicyType::Automatic)
                .vexpand(true)
                .child(
                    &gtk::TextView::builder()
                        .buffer(&self.0)
                        .monospace(true)
                        .build(),
                )
                .build(),
        );
        wgt.add(&{
            let wgt = gtk::Box::new(gtk::Orientation::Horizontal, PAD);
            wgt.set_margin(PAD);
            wgt.add(&self.1);
            wgt.add(&self.2);
            wgt.add(&self.3);
            wgt
        });
        wgt
    }
}

#[derive(Default)]
pub struct Curl(gtk::TextBuffer);
impl Component for Curl {
    type Event = super::msgs::Curl;
    type State = (String, String, String);
    fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool {
        match msg {
            Self::Event::Url(value) => model.0 = value,
            Self::Event::Body(value) => model.1 = value,
            Self::Event::Responce(value) => {
                model.2 = value;
                return true;
            }
            Self::Event::Run => {
                let url = model.0.clone();
                std::thread::spawn({
                    let sender = sender.clone();
                    move || {
                        let value = match reqwest::blocking::get(&url) {
                            Ok(value) => value.text().unwrap(),
                            Err(value) => value.to_string(),
                        };
                        sender.send(Self::Event::Responce(value)).unwrap();
                    }
                });
            }
        };
        false
    }
    fn update(&self, model: &Self::State) {
        self.0.set_text(&model.2);
    }
    fn view(&self, sender: Sender<Self::Event>) -> gtk::Box {
        let wgt = gtk::Box::new(gtk::Orientation::Vertical, PAD);
        wgt.set_margin(PAD);
        wgt.add(&{
            let list = gtk::ListStore::new(&[gtk::glib::Type::STRING]);
            for line in [
                r#"https://jsonplaceholder.typicode.com/users"#,
                r#"https://jsonplaceholder.typicode.com/posts"#,
                r#"https://jsonplaceholder.typicode.com/albums"#,
                r#"https://jsonplaceholder.typicode.com/todos"#,
                r#"https://jsonplaceholder.typicode.com/comments"#,
                r#"https://jsonplaceholder.typicode.com/posts"#,
                r#"https://lingva.ml/api/v1/languages"#,
                r#"https://lingva.ml/api/v1/en/de/rust"#,
                r#"https://ipinfo.io/json"#,
            ] {
                list.set(&list.append(), &[(0, &line)]);
            }
            let wgt = gtk::Box::new(gtk::Orientation::Horizontal, PAD);
            wgt.add(&{
                let wgt = gtk::ComboBoxText::new();
                wgt.set_tooltip_text(Some("Method"));
                wgt.append_text("GET");
                wgt.append_text("POST");
                wgt
            });
            wgt.add(&{
                let wgt = gtk::Entry::default();
                wgt.connect_changed({
                    let sender = sender.clone();
                    move |entry| {
                        if entry.is_sensitive() {
                            sender
                                .send(Self::Event::Url(entry.buffer().text().to_string()))
                                .unwrap();
                        }
                    }
                });
                wgt.set_placeholder_text(Some("URL"));
                wgt.set_completion(Some(&{
                    let wgt = gtk::EntryCompletion::default();
                    wgt.set_popup_completion(true);
                    wgt.set_model(Some(&list));
                    wgt.set_text_column(0);
                    wgt
                }));
                wgt.set_hexpand(true);
                wgt
            });
            wgt.add(&{
                let wgt = gtk::Button::with_mnemonic("list-add");
                wgt.set_tooltip_text(Some("Run"));
                wgt.connect_clicked({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Self::Event::Run).unwrap();
                    }
                });
                wgt
            });
            wgt
        });
        wgt.add(&{
            let wgt = gtk::Notebook::new();
            wgt.append_page(
                &gtk::ScrolledWindow::builder()
                    .hscrollbar_policy(gtk::PolicyType::Automatic)
                    .vscrollbar_policy(gtk::PolicyType::Automatic)
                    .child(&{
                        let wgt = gtk::TextView::with_buffer(&{
                            let wgt = gtk::TextBuffer::default();
                            wgt.connect_text_notify({
                                let sender = sender.clone();
                                move |buffer| {
                                    let value = buffer
                                        .start_iter()
                                        .text(&buffer.end_iter())
                                        .unwrap()
                                        .to_string();
                                    sender.send(Self::Event::Body(value)).unwrap();
                                }
                            });
                            wgt
                        });
                        wgt.set_hexpand(true);
                        wgt.set_tooltip_text(Some("Body"));
                        wgt
                    })
                    .vexpand(true)
                    .build(),
                Some(&gtk::Label::with_mnemonic("Body")),
            );
            wgt.append_page(
                &gtk::ScrolledWindow::builder()
                    .hscrollbar_policy(gtk::PolicyType::Automatic)
                    .vscrollbar_policy(gtk::PolicyType::Automatic)
                    .child(&{
                        let wgt = gtk::TextView::with_buffer(&self.0);
                        wgt.set_hexpand(true);
                        wgt.set_monospace(true);
                        wgt
                    })
                    .vexpand(true)
                    .build(),
                Some(&gtk::Label::with_mnemonic("Request")),
            );
            wgt
        });
        wgt
    }
}

#[derive(Default)]
pub struct Dial(
    gtk::ComboBoxText,
    gtk::ComboBoxText,
    gtk::TextBuffer,
    gtk::TextBuffer,
    gtk::Button,
);
impl Component for Dial {
    type Event = super::msgs::Dialect;
    type State = super::models::Dialect;
    fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool {
        match msg {
            Self::Event::Switch => {
                model.switch();
                return true;
            }
            Self::Event::Open(value) => {
                model.source = std::fs::read_to_string(value).unwrap();
                return true;
            }
            Self::Event::Target(value) => {
                model.target = value;
                return true;
            }
            Self::Event::Lang(value) => {
                model.read(value);
                return true;
            }
            Self::Event::SaveAs(value) => std::fs::write(value, model.target.as_bytes()).unwrap(),
            Self::Event::To(value) => model.to = value,
            Self::Event::From(value) => model.from = value,
            Self::Event::Source(value) => model.source = value,
            Self::Event::Quit => model.save(),
            Self::Event::Run => {
                if model.from != model.to && !model.source.is_empty() {
                    let url = model.url();
                    std::thread::spawn({
                        let sender = sender.clone();
                        move || {
                            if let Ok(value) = reqwest::blocking::get(&url) {
                                let target = value
                                    .json::<std::collections::HashMap<String, String>>()
                                    .unwrap()
                                    .get("translation")
                                    .unwrap()
                                    .to_string();
                                sender.send(Self::Event::Target(target)).unwrap();
                            }
                        }
                    });
                };
            }
        }
        false
    }
    fn update(&self, _model: &Self::State) {
        //~ self.0.set_text(&model.output);
        //~ self.1.set_label(&model.operation);
        //~ self.2.set_label(&model.prev.to_string());
        //~ self.3.set_label(&model.current);
    }
    fn view(&self, sender: Sender<Self::Event>) -> gtk::Box {
        self.0.set_tooltip_text(Some("From"));
        self.0.set_hexpand(true);
        self.1.set_tooltip_text(Some("To"));
        self.1.set_hexpand(true);
        let wgt = gtk::Box::new(gtk::Orientation::Vertical, PAD);
        wgt.set_margin(PAD);
        wgt.add(&{
            let wgt = gtk::Box::new(gtk::Orientation::Horizontal, PAD);
            wgt.add(&self.0);
            wgt.add(&{
                let wgt = gtk::Button::with_mnemonic("Switch");
                wgt.connect_clicked({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Self::Event::Switch).unwrap();
                    }
                });
                wgt
            });
            wgt.add(&self.1);
            wgt
        });
        wgt.add(&{
            let wgt = gtk::Box::new(gtk::Orientation::Horizontal, PAD);
            wgt.add(
                &gtk::ScrolledWindow::builder()
                    .hscrollbar_policy(gtk::PolicyType::Automatic)
                    .vscrollbar_policy(gtk::PolicyType::Automatic)
                    .vexpand(true)
                    .child(
                        &gtk::TextView::builder()
                            .buffer(&self.2)
                            .hexpand(true)
                            .monospace(true)
                            .build(),
                    )
                    .build(),
            );
            wgt.add(
                &gtk::ScrolledWindow::builder()
                    .hscrollbar_policy(gtk::PolicyType::Automatic)
                    .vscrollbar_policy(gtk::PolicyType::Automatic)
                    .vexpand(true)
                    .child(
                        &gtk::TextView::builder()
                            .buffer(&self.3)
                            .hexpand(true)
                            .monospace(true)
                            .build(),
                    )
                    .build(),
            );
            wgt
        });
        wgt
    }
}

#[derive(Default)]
pub struct Sudo([[gtk::Button; 9]; 9]);
impl Component for Sudo {
    type Event = super::msgs::Sudoku;
    type State = super::models::Sudoku;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Self::Event::Push(row, col, value) => model.0[row][col] = value,
            Self::Event::Clear => model.clear(),
            Self::Event::Solve => model.answer(),
        };
        true
    }
    fn update(&self, model: &Self::State) {
        for row in 0..9 {
            for col in 0..9 {
                self.0[row][col].set_label(&match model.0[row][col] {
                    0 => String::new(),
                    _ => model.0[row][col].to_string(),
                });
            }
        }
    }
    fn view(&self, _sender: Sender<Self::Event>) -> gtk::Box {
        let wgt = gtk::Box::new(gtk::Orientation::Vertical, PAD);
        wgt.set_margin(PAD);
        for row in 0..9 {
            for col in 0..9 {
                wgt.add(&self.0[row][col]);
            }
        }
        wgt
    }
}
