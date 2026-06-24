mod models {

    #[derive(Default)]
    pub struct Model {
        pub source: String,
        pub target: String,
        pub style: String,
        pub result: String,
    }
    impl Model {
        fn update(&mut self) {
            let (text, style) = diff(&self.source, &self.target);
            self.result = text;
            self.style = style;
        }
        pub fn set_source(&mut self, value: String) {
            self.source = value;
            self.update();
        }
        pub fn set_target(&mut self, value: String) {
            self.target = value;
            self.update();
        }
    }

    fn diff(source: &str, target: &str) -> (String, String) {
        use similar::{Algorithm, ChangeTag, utils};
        let mut source_text = String::new();
        let mut source_style = String::new();
        let mut target_text = String::new();
        let mut target_style = String::new();
        for (tag, text) in utils::diff_chars(Algorithm::Myers, source, target) {
            match tag {
                ChangeTag::Delete => {
                    source_text.push_str(text);
                    source_style.push_str(&"C".repeat(text.len()));
                    target_text.push_str(&" ".repeat(text.len()));
                    target_style.push_str(&" ".repeat(text.len()));
                }
                ChangeTag::Insert => {
                    source_text.push_str(&" ".repeat(text.len()));
                    source_style.push_str(&" ".repeat(text.len()));
                    target_text.push_str(text);
                    target_style.push_str(&"B".repeat(text.len()));
                }
                ChangeTag::Equal => {
                    source_text.push_str(text);
                    source_style.push_str(&"A".repeat(text.len()));
                    target_text.push_str(text);
                    target_style.push_str(&"A".repeat(text.len()));
                }
            }
        }
        (
            format!("{source_text}\n{target_text}"),
            format!("{source_style}\n{target_style}"),
        )
    }
}

use crate::*;

const STYLE_TABLE: [StyleTableEntryExt; 3] = [
    StyleTableEntryExt {
        color: Color::Green,
        font: Font::Courier,
        size: 16,
        attr: TextAttr::None,
        bgcolor: Color::TransparentBg,
    },
    StyleTableEntryExt {
        color: Color::Red,
        font: Font::Courier,
        size: 16,
        attr: TextAttr::None,
        bgcolor: Color::TransparentBg,
    },
    StyleTableEntryExt {
        color: Color::Blue,
        font: Font::Courier,
        size: 16,
        attr: TextAttr::None,
        bgcolor: Color::TransparentBg,
    },
];

#[derive(Clone, Default)]
pub struct View {
    source: TextBuffer,
    target: TextBuffer,
    style: TextBuffer,
    result: TextBuffer,
}

pub enum Msg {
    Source(String),
    Target(String),
}

impl Component for View {
    type Event = Msg;
    type State = models::Model;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Source(value) => model.set_source(value),
            Msg::Target(value) => model.set_target(value),
        };
        true
    }
    fn update(&mut self, model: &Self::State) {
        self.source.set_text(&model.source);
        self.target.set_text(&model.target);
        self.result.set_text(&model.result);
        self.style.set_text(&model.style);
    }
    fn view(&mut self, sender: Sender<Self::Event>) -> impl WidgetExt {
        let mut wgt = Flex::default_fill();
        wgt.set_margin(PAD);
        wgt.set_label("Similar");
        wgt.add(&{
            let mut wgt = Flex::default_fill().column();
            wgt.set_frame(FrameType::RFlatBox);
            wgt.set_color(Color::Background2);
            wgt.set_margin(5);
            wgt.set_pad(0);
            wgt.fixed(
                &{
                    let mut wgt = Input::build();
                    wgt.set_tooltip("Source");
                    wgt.set_callback({
                        let sender = sender.clone();
                        move |wgt| sender.send(Msg::Source(wgt.value())).unwrap()
                    });
                    wgt
                },
                HEIGHT,
            );
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
                    let mut wgt = Input::build();
                    wgt.set_tooltip("Target");
                    wgt.set_callback({
                        let sender = sender.clone();
                        move |wgt| sender.send(Msg::Target(wgt.value())).unwrap()
                    });
                    wgt
                },
                HEIGHT,
            );
            wgt.fixed(
                &{
                    let mut wgt = Frame::default();
                    wgt.set_frame(FrameType::FlatBox);
                    wgt
                },
                3,
            );
            wgt.add(&{
                let mut wgt = TextDisplay::default();
                wgt.set_scrollbar_size(LINE);
                wgt.set_frame(FrameType::FlatBox);
                wgt.set_buffer(self.result.clone());
                wgt.set_highlight_data_ext(self.style.clone(), STYLE_TABLE);
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
