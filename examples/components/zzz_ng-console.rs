mod models {
    use rand::Rng;
    use simple_gui::prelude::{COLS, ROWS};

    #[derive(Default)]
    pub enum Direc {
        #[default]
        Left,
        Right,
        Down,
        Up,
    }
    impl Direc
        pub fn next(&self, last: (i32, i32)) -> (i32, i32) {
            match self {
                &Self::Up => last.1 -= 1,
                &Self::Down => last.1 += 1,
                &Self::Left => last.0 -= 1,
                &Self::Right => last.0 += 1,
            }
            (check_limit(x, COLS), check_limit(y, ROWS))
        }
    }
    fn check_limit(coord: i32, limit: i32) -> i32 {
        if coord < 0 {
            limit - 1
        } else if coord > limit {
            0
        } else {
            coord
        }
    }
    fn new_apple() -> (i32, i32) {
        (
            rand::rng().random_range(0..COLS),
            rand::rng().random_range(0..ROWS),
        )
    }
}

#[cfg(feature = "new")]
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
    pub struct Snake {
        direc: Dir,
        apple: (i32, i32),
        apples: Vec<Apple>,
        page: Page,
        timer: f32,
    }
    impl Snake {
        fn eat(&mut self, next: (i32, i32)) {
            self.body.insert(0, next);
            while self.body.contains(&self.apple) {
                self.apple = Apple::default();
            }
        }
    }
    impl Console for Snake {
        fn load(&mut self, _path: &str) {
            self.eat(self.apple);
        }
        fn update(&mut self, dt: f32) {
            if let Page::Main = self.page {
                self.timer += dt;
                if self.timer > 1.0 / (3.0 + self.snake.len() as f32 / 5.0) {
                    self.timer = 0.0;
                    let next = self.dir.next(&self.body.last().unwrap());
                    if self.body.contains(&next) {
                        return false
                    } else {
                        if next != self.apple {
                            self.snake.pop();
                        }
                        self.eat(next);
                    }
                }
            }
            true
        }
        fn exit(&self, _path: &str) {}
        fn handle(&mut self, key: Key) {
            match (&self.page, key) {
                (&Page::Welcome, Key::Escape) => std::process::exit(0),
                (&Page::Welcome, Key::a) => {
                    self.page = Page::Main;
                    self.apple = Apple::default();
                    self.snake.push(Apple::default());
                }
                (&Page::Main, Key::Up) => {
                    if self.direc != Dir::Down {
                        self.dir = Dir::Up;
                    }
                }
                (&Page::Main, Key::Down) => {
                    if self.direc != Dir::Up {
                        self.dir = Dir::Down;
                    }
                }
                (&Page::Main, Key::Left) => {
                    if self.direc != Dir::Right {
                        self.dir = Dir::Left;
                    }
                }
                (&Page::Main, Key::Right) => {
                    if self.direc != Dir::Left {
                        self.dir = Dir::Right;
                    }
                }
                _ => {}
            }
        }
        fn draw(&self, context: &Context, width: i32, height: i32) {
            context.draw_rectangle(0, 0, width, height, solarized::BASE2);
            if let Page::Main = self.page {
                let cell = SCREEN.0 / 16;
                for x in 0..COLS {
                    for y in 0..ROWS {
                        if (x + y) % 2 == 0 {
                            context.draw_rectangle(
                                x * cell,
                                y * cell,
                                cell,
                                cell,
                                solarized::BASE3,
                            );
                        }
                    }
                }
                for Apple(x, y) in &self.snake {
                    context.draw_rectangle(x * cell, y * cell, cell, cell, solarized::CYAN);
                }
                context.draw_rectangle(
                    self.snake[0].0 * cell,
                    self.snake[0].1 * cell,
                    cell,
                    cell,
                    solarized::GREEN,
                );
                context.draw_rectangle(
                    self.apple.0 * cell,
                    self.apple.1 * cell,
                    cell,
                    cell,
                    solarized::RED,
                );
            }
        }
    }
}

pub mod solarized {
    const CAIRO: f64 = 255.0;
    pub const BASE3: (f64, f64, f64) = (
        0xFD as f64 / CAIRO,
        0xF6 as f64 / CAIRO,
        0xE3 as f64 / CAIRO,
    );
    pub const BASE2: (f64, f64, f64) = (
        0xEE as f64 / CAIRO,
        0xE8 as f64 / CAIRO,
        0xD5 as f64 / CAIRO,
    );
    pub const BASE1: (f64, f64, f64) = (
        0x93 as f64 / CAIRO,
        0xA1 as f64 / CAIRO,
        0xA1 as f64 / CAIRO,
    );
    pub const RED: (f64, f64, f64) = (
        0xDC as f64 / CAIRO,
        0x32 as f64 / CAIRO,
        0x2F as f64 / CAIRO,
    );
    pub const GREEN: (f64, f64, f64) = (
        0x85 as f64 / CAIRO,
        0x99 as f64 / CAIRO,
        0x00 as f64 / CAIRO,
    );
    pub const CYAN: (f64, f64, f64) = (
        0x2A as f64 / CAIRO,
        0xA1 as f64 / CAIRO,
        0x98 as f64 / CAIRO,
    );
}

use simple_gui::prelude::*;
fn main() -> glib::ExitCode {
    crate::games::World::run("io.gitlab.kbit")
}

//~ // Single Model Application
pub trait Console
where
Self: Default + 'static,
{
fn load(&mut self, path: &str);
fn exit(&self, path: &str);
fn handle(&mut self, key: Key);
fn update(&mut self, dt: f32);
fn draw(&self, context: &Context, width: i32, height: i32);
fn run(id: &'static str) -> glib::ExitCode {
cascade!(
Application::default();
..set_application_id(Some(id));
..connect_startup(|_| {
libadwaita::init().unwrap();
});
..connect_activate(move |app| {
let path = format!("{}/.config/{id}", std::env::var("HOME").unwrap());
let state = Arc::new(RwLock::new(Self::default()));
state.write().unwrap().load(&path);
let key = libadwaita::gtk::EventControllerKey::new();
key.connect_key_pressed({
let state = state.clone();
move |_, key, _keycode, _state| {
state.write().unwrap().handle(key);
libadwaita::glib::Propagation::Stop
}
});
let draw = DrawingArea::builder()
.content_width(crate::prelude::SCREEN_WIDTH)
.content_height(crate::prelude::SCREEN_HEIGHT)
.hexpand(true)
.vexpand(true)
.focusable(true)
.build();
draw.add_controller(key);
draw.set_draw_func({
let state = state.clone();
move |_, context, width, height| {
state.read().unwrap().draw(context, width, height);
}
});
let window = ApplicationWindow::builder()
.application(app)
.content(&draw)
.build();
window.connect_destroy({
let state = state.clone();
move |window| {
state.read().unwrap().exit(&path);
window.close();
}
});
window.present();
let mut time = Instant::now();
libadwaita::glib::timeout_add_local(Duration::from_millis(20), {
let state = state.clone();
let draw = draw.clone();
move || {
state.write().unwrap().update(time.elapsed().as_secs_f32());
draw.queue_draw();
time = Instant::now();
libadwaita::glib::ControlFlow::Continue
}
});
});
)
.run()
}
}

pub trait Paint {
    fn draw_rectangle(&self, x: i32, y: i32, width: i32, height: i32, color: (f64, f64, f64));
}

impl Paint for Context {
    fn draw_rectangle(&self, x: i32, y: i32, width: i32, height: i32, color: (f64, f64, f64)) {
        self.set_source_rgb(color.0, color.1, color.2);
        self.rectangle(x as f64, y as f64, width as f64, height as f64);
        self.fill().unwrap();
        //self.stroke().unwrap();
    }
}
