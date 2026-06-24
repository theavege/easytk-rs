mod models {
    use rand::Rng;

    pub const SIZE: (usize, usize) = (15, 30);
    pub const FIGURES: [[[(i32, usize); 4]; 4]; 7] = [
        [[(0, 0), (1, 0), (0, 1), (1, 1)]; 4], // Square (No rotation)
        [
            [(0, 0), (0, 1), (0, 2), (0, 3)], // Rotation 0
            [(0, 0), (1, 0), (2, 0), (3, 0)], // Rotation 1
            [(0, 0), (0, 1), (0, 2), (0, 3)], // Rotation 0
            [(0, 0), (1, 0), (2, 0), (3, 0)], // Rotation 1
        ], // Straight
        [
            [(0, 1), (1, 1), (1, 0), (2, 0)], // Rotation 0
            [(0, 0), (0, 1), (1, 1), (1, 2)], // Rotation 1
            [(0, 1), (1, 1), (1, 0), (2, 0)], // Rotation 0
            [(0, 0), (0, 1), (1, 1), (1, 2)], // Rotation 1
        ], // S
        [
            [(0, 0), (1, 0), (1, 1), (2, 1)], // Rotation 0
            [(1, 0), (1, 1), (0, 1), (0, 2)], // Rotation 1
            [(0, 0), (1, 0), (1, 1), (2, 1)], // Rotation 0
            [(1, 0), (1, 1), (0, 1), (0, 2)], // Rotation 1
        ], // Z
        [
            [(0, 0), (1, 0), (1, 1), (2, 0)], // Rotation 0
            [(0, 1), (1, 0), (1, 1), (1, 2)], // Rotation 1
            [(0, 1), (1, 0), (1, 1), (2, 1)], // Rotation 2
            [(0, 0), (0, 1), (1, 1), (0, 2)], // Rotation 3
        ], // T-Shaped Quadshape
        [
            [(1, 0), (1, 1), (1, 2), (0, 2)], // Rotation 0
            [(0, 0), (0, 1), (1, 1), (2, 1)], // Rotation 1
            [(0, 0), (1, 0), (0, 1), (0, 2)], // Rotation 2
            [(0, 0), (1, 0), (2, 0), (2, 1)], // Rotation 3
        ], // J
        [
            [(0, 0), (0, 1), (0, 2), (1, 2)], // Rotation 0
            [(0, 0), (1, 0), (2, 0), (0, 1)], // Rotation 1
            [(0, 0), (1, 0), (1, 1), (1, 2)], // Rotation 2
            [(0, 1), (1, 1), (2, 1), (2, 0)], // Rotation 3
        ], // L
    ];

    #[derive(Default)]
    pub struct Figure {
        pub shape: (usize, usize),
        pub body: [(i32, usize); 4],
    }

    pub struct Grid(pub Vec<[Option<usize>; SIZE.0]>, pub usize);

    impl Figure {
        pub fn new(color: usize) -> Self {
            Self {
                shape: (color, 0),
                body: FIGURES[color][0].map(|(i, j)| (8 + i, j)),
            }
        }
        pub fn check(self, grid: &Grid) -> Option<Self> {
            for (x, y) in self.body {
                if !(0..SIZE.1).contains(&y)
                    || !(0..SIZE.0 as i32).contains(&x)
                    || grid.0[y][x as usize].is_some()
                {
                    return None;
                }
            }
            Some(self)
        }
        pub fn shift(&self, x: i32, y: usize) -> Self {
            Self {
                shape: self.shape,
                body: self.body.map(|(i, j)| (x + i, y + j)),
            }
        }
        pub fn rotate(&self) -> Self {
            let position = match self.shape.1 < FIGURES[self.shape.0].len() - 1 {
                true => self.shape.1 + 1,
                false => 0,
            };
            let (x, y) = self
                .body
                .iter()
                .fold((SIZE.0 as i32, SIZE.0), |(i, j), (x, y)| {
                    (i.min(*x), j.min(*y))
                });
            Self {
                shape: (self.shape.0, position),
                body: FIGURES[self.shape.0][position].map(|(i, j)| (x + i, y + j)),
            }
        }
    }

    impl Default for Grid {
        fn default() -> Self {
            Self(
                vec![[None; SIZE.0]; SIZE.1],
                rand::rng().random_range(0..FIGURES.len()),
            )
        }
    }

    impl Grid {
        pub fn check(&mut self) -> Option<i32> {
            if self.0[0].iter().any(|cell: &Option<usize>| cell.is_some()) {
                return None;
            }
            let mut count = 0;
            for line in 0..SIZE.1 {
                if self.0[line]
                    .iter()
                    .all(|cell: &Option<usize>| cell.is_some())
                {
                    self.0.remove(line);
                    self.0.insert(0, [None; SIZE.0]);
                    count += 1;
                }
            }
            Some(count)
        }
        pub fn append(&mut self, curr: &Figure) -> Figure {
            for (x, y) in curr.body {
                self.0[y][x as usize] = Some(curr.shape.0);
            }
            Figure::new(self.next())
        }
        pub fn draw(&self, curr: &Figure) -> Vec<[Option<usize>; SIZE.0]> {
            let mut field = self.0.clone();
            for (x, y) in curr.body {
                field[y][x as usize] = Some(curr.shape.0);
            }
            field
        }
        pub fn next(&mut self) -> usize {
            let tmp = self.1;
            while tmp == self.1 {
                self.1 = rand::rng().random_range(0..FIGURES.len());
            }
            tmp
        }
    }
}

#[cfg(feature = "old")]
mod games {
    use crate::models::*;
    use simple_gui::prelude::*;

    #[derive(Default)]
    pub enum Page {
        #[default]
        Welcome,
        Main,
    }

    #[derive(Default)]
    pub struct World {
        page: Page,
        timer: f32,
        level: i32,
        curr: Figure,
        grid: Grid,
    }

    impl Console for World {
        fn load(&mut self, _path: &str) {}
        fn exit(&self, _path: &str) {}
        fn handle(&mut self, _: &mut Window, event: Event) -> bool {
            match event {
                Event::KeyDown => {
                    const UP: Key = Key::from_char('w');
                    const RIGHT: Key = Key::from_char('d');
                    const DOWN: Key = Key::from_char('s');
                    const LEFT: Key = Key::from_char('a');
                    match (&self.page, event_key()) {
                        (&Page::Welcome, Key::Escape) => std::process::exit(0),
                        (&Page::Welcome, Key::Enter) => {
                            self.grid = Grid::default();
                            self.curr = Figure::new(self.grid.next());
                            self.page = Page::Main;
                        }
                        (&Page::Main, Key::Up | UP) => {
                            if let Some(temp) = self.curr.rotate().check(&self.grid) {
                                self.curr = temp;
                            }
                        }
                        (&Page::Main, Key::Left | LEFT) => {
                            if let Some(temp) = self.curr.shift(-1, 0).check(&self.grid) {
                                self.curr = temp;
                            }
                        }
                        (&Page::Main, Key::Right | RIGHT) => {
                            if let Some(temp) = self.curr.shift(1, 0).check(&self.grid) {
                                self.curr = temp;
                            }
                        }
                        (&Page::Main, Key::Down | DOWN) => {
                            if let Some(temp) = self.curr.shift(0, 1).check(&self.grid) {
                                self.curr = temp;
                            }
                        }
                        _ => return false,
                    };
                    true
                }
                _ => false,
            }
        }
        fn update(&mut self, dt: f32) {
            if let Page::Main = self.page {
                self.timer += dt;
                if self.timer > 1.0 / (3.0 + (self.level / 2) as f32) {
                    self.timer = 0.0;
                    if let Some(val) = self.curr.shift(0, 1).check(&self.grid) {
                        self.curr = val;
                    } else {
                        self.curr = self.grid.append(&self.curr);
                        match self.grid.check() {
                            Some(val) => self.level += val,
                            None => *self = Self::default(),
                        }
                    };
                }
            }
        }
        fn draw(&self, window: &mut Window) {
            if let Page::Main = self.page {
                window.draw_background(Color::Foreground);
                let (x, y, h) = draw_field(window, &self.grid.draw(&self.curr));
                let (x, y) = draw_next(x, y, h, self.grid.1);
                draw_level(x, y, h, self.level);
            } else {
                window.draw_background(Color::Background);
                window.draw_welcome(
                    "Tetris",
                    &[&["PRESS <ENTER>", "for play"], &["PRESS <ESC>", "for exit"]],
                );
            }
        }
    }

    const COLORS: [Color; 7] = [
        Color::Green,
        Color::Cyan,
        Color::Blue,
        Color::from_hex(0x6C71C4), //violet
        Color::Magenta,
        Color::Yellow,
        Color::from_hex(0xCB4B16), //orange
    ];

    fn draw_field(flex: &impl WidgetExt, table: &Vec<[Option<usize>; SIZE.0]>) -> (i32, i32, i32) {
        let pad: i32 = 1;
        let height: i32 =
            (flex.height() - 2 * PAD - pad * (table.len() as i32 + 1)) / table.len() as i32;
        let ww = height * table[0].len() as i32 + pad * (table[0].len() as i32 - 1) + 2 * PAD;
        let hh = height * table.len() as i32 + pad * (table.len() as i32 - 1) + 2 * PAD;
        let x = (flex.width() - ww) / 2;
        let y = (flex.height() - hh) / 2;
        let mut xx = x;
        let mut yy = y;
        let mut xxx = 0;
        xx += PAD;
        yy += PAD;
        for line in table {
            for cell in line {
                draw::draw_rect_fill(
                    xx,
                    yy,
                    height,
                    height,
                    match cell {
                        None => Color::Background2,
                        Some(idx) => COLORS[*idx],
                    },
                );
                xx += pad + height;
                xxx = xx
            }
            yy += pad + height;
            xx = x + PAD;
        }
        (xxx + PAD, y, height)
    }

    fn draw_next(x: i32, y: i32, height: i32, color: usize) -> (i32, i32) {
        let pad: i32 = 1;
        let mut xx = x;
        let mut yy = y;
        xx += PAD;
        yy += PAD;
        let mut table = [[None; 4]; 4];
        for (x, y) in FIGURES[color][0] {
            table[y][x as usize] = Some(color);
        }
        for line in table {
            for cell in line {
                draw::draw_rect_fill(
                    xx,
                    yy,
                    height,
                    height,
                    match cell {
                        None => Color::Foreground,
                        Some(idx) => COLORS[idx],
                    },
                );
                xx += pad + height;
            }
            yy += pad + height;
            xx = x + PAD;
        }
        (xx, yy)
    }

    fn draw_level(x: i32, y: i32, h: i32, v: i32) {
        draw::set_draw_color(Color::Background2);
        draw::set_font(Font::CourierBold, h);
        let mut yy = y;
        for line in [
            &format!("Level:\t{v}"),
            "PRESS:",
            "  <UP>    rotate",
            "  <DOWN>  fast down",
            "  <LEFT>  move left",
            "  <RIGHT> move right",
            "  <ESC>   exit from game",
        ] {
            let (w, h) = draw::measure(line, false);
            yy += h;
            draw::draw_text2(line, x, yy, w, h, Align::Left);
        }
    }
}

use simple_gui::prelude::*;
fn main() -> Result<(), FltkError> {
    games::World::run(Settings {
        fullscreen: true,
        xclass: Some("Tetris"),
        size: Some((SCREEN.0, SCREEN.1)),
        icon: Some(SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap()),
        ..Default::default()
    })
}

pub trait Painter {
    fn draw_rect(&self, x: i32, y: i32, w: i32, h: i32, r: i32, color: Color);
    fn draw_text(&self, line: &str, x: i32, y: i32, color: Color, align: Align, size: i32);
    fn draw_welcome(&self, title: &str, menu: &[&[&str]]);
    fn draw_background(&self, color: Color);
    fn draw_overlay(&self, title: &str, subtitle: &str, color: Color);
}

impl Painter for Window {
    fn draw_rect(&self, x: i32, y: i32, w: i32, h: i32, r: i32, color: Color) {
        draw::set_draw_color(color);
        draw::draw_rounded_rectf(x, y, w, h, r);
    }
    fn draw_text(&self, line: &str, x: i32, y: i32, color: Color, align: Align, size: i32) {
        draw::set_font(Font::CourierBold, size);
        draw::set_draw_color(color);
        let (w, h) = draw::measure(line, false);
        draw::draw_text2(line, x, y, w, h, align);
    }
    fn draw_overlay(&self, title: &str, subtitle: &str, color: Color) {
        draw::set_draw_color(color);
        draw::set_font(Font::CourierBold, 42);
        let (mut w, mut h) = draw::measure(title, false);
        draw::draw_text2(
            title,
            self.w() / 2 - w / 2,
            self.h() / 3 - h,
            w,
            h,
            Align::Left,
        );
        draw::set_font(Font::CourierBold, 24);
        (w, h) = draw::measure(subtitle, false);
        draw::draw_text2(
            subtitle,
            self.w() / 2 - w / 2,
            self.h() / 2 - h,
            w,
            h,
            Align::Left,
        );
    }
    fn draw_background(&self, color: Color) {
        draw::draw_rect_fill(0, 0, self.w(), self.h(), color);
    }
    fn draw_welcome(&self, title: &str, menu: &[&[&str]]) {
        draw::set_draw_color(Color::Green);
        draw::set_font(Font::CourierBold, 20);
        draw::draw_text2(
            &figleter::FIGfont::standard()
                .unwrap()
                .convert(title)
                .unwrap()
                .to_string(),
            0,
            self.h() / 4,
            self.w(),
            crate::prelude::HEIGHT,
            Align::Center,
        );
        draw::set_draw_color(Color::Red);
        draw::draw_text2(
            &{
                let mut table = comfy_table::Table::new();
                table.load_preset(comfy_table::presets::UTF8_FULL);
                table.apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS);
                for row in menu {
                    table.add_row(*row);
                }
                table
            }
            .to_string(),
            0,
            self.h() / 3 * 2,
            self.w(),
            crate::prelude::HEIGHT,
            Align::Center,
        );
    }
}
