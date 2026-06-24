#[derive(Default)]
pub struct Converter(gtk::Entry, gtk::Entry);

impl Component for Converter {
    type Event = super::msgs::Converter;
    type State = super::mdls::Converter;
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
        let adjustment = Adjustment::new(0.0, 0.0, 255.0, 1.0, 10.0, 0.0);
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
        wgt.add({
            let wgt = SpinButton::with_range(0.0, 255.0, 1.0);
            wgt.set_adjustment(&adjustment);
            wgt
        });
        wgt.add({
            let wgt = Scale::with_range(0.0, 255.0, 1.0);
            wgt.set_adjustment(&adjustment);
            wgt
        });
        wgt
    }
}

#[derive(Default)]
pub struct Curl(gtk::TextBuffer);

impl Component for Curl {
    type Event = Msg;
    type State = (String, String, String);
    fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Url(value) => model.0 = value,
            Msg::Body(value) => model.1 = value,
            Msg::Responce(value) => {
                model.2 = value;
                return true;
            }
            Msg::Run => {
                let url = model.url.clone();
                std::thread::spawn(glib::clone!(
                    @strong sender =>
                    move || {
                        let value = match http_request(&url) {
                            Ok(value) => value,
                            Err(value) => value.to_string(),
                        };
                        sender.send(Msg::Responce(value)).unwrap();
                    }
                ));
            }
        };
        false
    }
    fn update(&self, model: &Self::State) {
        self.responce.update(&model.responce);
    }
    fn view(&self, sender: Sender<Self::Event>) -> Flex {
        let list = ListStore::new(&[glib::Type::STRING]);
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
        let wgt = Flex::new(Orientation::Vertical, PAD);
        wgt.set_margin(PAD);
        wgt.add(&{
            let wgt = Flex::new(Orientation::Horizontal, PAD);
            wgt.add(&{
                let wgt = gtk::ComboBox::new();
                wgt.set_tooltip_text(Some("Method"));
                wgt
            });
            wgt.add(&{
                let wgt = Entry::default();
                wgt.connect_changed(glib::clone!(@strong sender => move |entry| {
                    if entry.is_sensitive() {
                        sender.send(Msg::Url(entry.buffer().text().to_string())).unwrap();
                    }
                }));
                wgt.set_placeholder_text(Some("URL"));
                wgt.set_completion(Some(&{
                    let wgt = EntryCompletion::default();
                    wgt.set_popup_completion(true);
                    wgt.set_model(Some(&list));
                    wgt.set_text_column(0);
                    wgt
                }));
                wgt.set_hexpand(true);
                wgt
            });
            wgt.add(&{
                let wgt = Button::with_mnemonic("list-add");
                wgt.set_tooltip_text(Some("Run"));
                wgt.connect_clicked({
                    glib::clone!(@strong sender => move |_| {
                        sender.send(Msg::Run).unwrap();
                    })
                });
                wgt
            });
            wgt
        });
        wgt.add(&ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Automatic)
            .vscrollbar_policy(PolicyType::Automatic)
            .child(&{
                let wgt = TextView::with_buffer(&{
                    let wgt = TextBuffer::default();
                    wgt.connect_text_notify({
                        let sender = sender.clone();
                        move |buffer| {
                            let value = buffer.start_iter().text(&buffer.end_iter()).unwrap().to_string();
                            sender.send(Msg::Body(value)).unwrap();
                        }
                    });
                    wgt
                });
                wgt.set_tooltip_text(Some("Body"));
                wgt
            })
            .vexpand(true)
            .build()
        );
        wgt.add(
            &ScrolledWindow::builder()
                .hscrollbar_policy(PolicyType::Automatic)
                .vscrollbar_policy(PolicyType::Automatic)
                .child(&{
                    let wgt = TextView::with_buffer(&self.responce);
                    wgt.set_monospace(true);
                    wgt
                })
                .vexpand(true)
                .build(),
        );
        wgt
    }
}

fn window(application: &Application) {
    let list_store = ListStore::new(&[
        glib::types::Type::BOOL,
        glib::types::Type::STRING,
        glib::types::Type::STRING,
        glib::Type::U32,
    ]);
    for url in [
        "https://www.libreoffice.org/donate/dl/win-x86_64/7.6.0/en-US/LibreOffice_7.6.0_Win_x86-64.msi",
    ] {
        let name: &str = url.split('/').nth_back(0).unwrap();
        list_store.set(&list_store.append(), &[(0, &true), (1, &name), (2, &url)]);
    }
    let tree_view = TreeView::with_model(&list_store);
    for (ord, name) in ["STATUS", "NAME", "URL"].into_iter().enumerate() {
        match name {
            "STATUS" => {
                let renderer = CellRendererToggle::new();
                renderer.connect_toggled(glib::clone!( @strong list_store => move |_, path| {
                    let iter = list_store.iter(&path).unwrap();
                    list_store.set_value(
                        &iter,
                        ord as u32,
                        &(!list_store.value(&iter, ord as i32).get::<bool>().unwrap()).to_value(),
                    );
                }));
                tree_view.append_column(&TreeViewColumn::with_attributes(
                    name,
                    &renderer,
                    &[("active", ord as i32)],
                ))
            }
            _ => tree_view.append_column(&TreeViewColumn::with_attributes(
                name,
                &CellRendererText::new(),
                &[("text", ord as i32)],
            )),
        };
    }
    let run = Button::with_mnemonic("Run");
    let result = Label::with_mnemonic("Result");
    run.connect_clicked(glib::clone!(@strong run => move |_| {
        list_store.foreach(|store, _, iter| {
            if store.value(iter, 0).get::<bool>().unwrap() {
                gtk_nsis::run(list_store.value(iter, 2).get::<String>().unwrap());
            };
            false
        });
        let value = result.text();
        result.set_text(&(value.to_string() + " Done!"));
        run.set_sensitive(false);
    }));
}
