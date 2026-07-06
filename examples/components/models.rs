#[derive(Default)]
pub struct Calculator {
    pub prev: f64,
    pub operation: String,
    pub current: String,
    pub output: String,
}
impl Calculator {
    pub fn click(&mut self, value: &str) {
        match value {
            "/" | "x" | "+" | "-" | "%" => {
                if self.current != "0" {
                    if self.operation.is_empty() {
                        self.prev = self.current.parse().unwrap();
                    } else {
                        self.equil();
                    }
                    self.output.push_str(&format!("{} {}", self.prev, value));
                    self.operation = value.to_string();
                    self.current = String::from("0");
                }
            }
            "=" => {
                if !self.operation.is_empty() {
                    self.equil();
                    self.operation.clear();
                }
            }
            "CE" => {
                self.output.clear();
                self.operation.clear();
                self.current = String::from("0");
                self.prev = 0f64;
            }
            "@<-" => {
                let label = self.current.clone();
                self.current = if label.len() > 1 {
                    String::from(&label[..label.len() - 1])
                } else {
                    String::from("0")
                };
            }
            "C" => self.current = String::from("0"),
            "." => {
                if !self.current.contains('.') {
                    self.current.push('.');
                }
            }
            _ => {
                if self.current == "0" {
                    self.current.clear();
                }
                self.current.push_str(value);
            }
        };
    }
    fn equil(&mut self) {
        self.output.push_str(&format!(" {}\n", self.current));
        let current: f64 = self.current.parse().unwrap();
        self.prev = match self.operation.as_str() {
            "/" => self.prev / current,
            "x" => self.prev * current,
            "+" => self.prev + current,
            "-" => self.prev - current,
            _ => self.prev / 100.0 * current,
        };
        self.output.push_str(&format!("    = {}\n", self.prev));
        self.current = String::from("0");
    }
}

#[derive(Default)]
pub struct Pictures {
    pub list: Vec<String>,
    pub idx: usize,
    pub scale: f64,
}
impl Pictures {
    pub fn shift(&mut self, dir: bool) {
        let step:isize  = match dir {
            true => 1,
            false => - 1,
        };
        let next:isize = self.idx as isize + step;
        self.idx = match (0..self.list.len()).contains(&(next as usize)) {
            true => next,
            false => self.list.len() as isize - next * step,
        } as usize;
    }
    pub fn del(&mut self, value: bool) {
        if let Some(path) = self.path() {
            if value {
                if std::fs::remove_file(path).is_ok() {
                    self.list.remove(self.idx);
                }
            } else {
                self.list.remove(self.idx);
            };
            self.shift(true);
        }
    }
    pub fn path(&self) -> Option<String> {
        match self.list.is_empty() {
            true => None,
            false => Some(self.list[self.idx].clone()),
        }
    }
    pub fn set_list(&mut self, value: Vec<String>) {
        for item in value {
            self.list.push(item);
        }
        self.idx = 0;
    }
}

#[derive(Default)]
pub struct Sudoku(pub [[i32; 9]; 9]);
impl Sudoku {
    pub fn clear(&mut self) {
        self.0 = [[0; 9]; 9];
    }
    fn check_solvable(&mut self) -> bool {
        let mut items: [i32; 9];
        for row in self.0 {
            items = [0; 9];
            for value in row {
                if value > 0 && value < 10 {
                    items[(value - 1) as usize] += 1;
                }
            }
            if items.iter().any(|&n| n > 1) {
                return false;
            }
        }
        for i in 0..9 {
            items = [0; 9];
            for row in self.0 {
                if row[i] > 0 && row[i] < 10 {
                    items[(row[i] - 1) as usize] += 1;
                }
            }
            if items.iter().any(|&n| n > 1) {
                return false;
            }
        }
        for &x in [0, 3, 6].iter() {
            for &y in [0, 3, 6].iter() {
                items = [0; 9];
                for i in 0..3 {
                    for j in 0..3 {
                        if self.0[y + i][x + j] > 0 && self.0[y + i][x + j] < 10 {
                            items[(self.0[y + i][x + j] - 1) as usize] += 1;
                        }
                    }
                }
                if items.iter().any(|&n| n > 1) {
                    return false;
                }
            }
        }
        true
    }
    fn check_possible(&self, y: usize, x: usize, number: i32) -> bool {
        if self.0[y].contains(&number) {
            return false;
        }
        if self.0.iter().any(|n| n[x] == number) {
            return false;
        }
        let x0: usize = (x / 3) * 3;
        let y0: usize = (y / 3) * 3;
        for i in 0..3 {
            for j in 0..3 {
                if self.0[y0 + i][x0 + j] == number {
                    return false;
                }
            }
        }
        true
    }
    fn find_next_cell2fill(&self) -> (usize, usize) {
        for (x, row) in self.0.iter().enumerate() {
            for (y, &val) in row.iter().enumerate() {
                if val == 0 {
                    return (x, y);
                }
            }
        }
        (99, 99)
    }
    fn solve(&mut self) -> bool {
        let (i, j) = self.find_next_cell2fill();
        if i == 99 {
            return true;
        }
        for e in 1..10 {
            if self.check_possible(i, j, e) {
                self.0[i][j] = e;
                if self.solve() {
                    return true;
                }
                self.0[i][j] = 0;
            }
        }
        false
    }
    pub fn answer(&mut self) {
        if self.check_solvable() {
            self.solve();
        } else {
            self.clear();
        }
    }
}

#[derive(Default)]
pub struct Dialect {
    pub lang: Vec<(String, String)>,
    pub source: String,
    pub target: String,
    pub from: i32,
    pub to: i32,
}

impl Dialect {
    const SERVICE: &str = r#"https://lingva.ml/api/v1"#;
    const NAME: &str = "Dialect";
    pub fn read(&mut self, value: Vec<(String, String)>) {
        self.lang = value;
        if let Ok(value) = std::fs::read(Self::file()) {
            self.from = value[0] as i32;
            self.to = value[1] as i32;
        };
    }
    fn file() -> String {
        format!(
            "{}/.config/{}",
            std::env::var("HOMEPATH").unwrap(),
            Self::NAME
        )
    }
    pub fn switch(&mut self) {
        std::mem::swap(&mut self.from, &mut self.to);
    }
    pub fn save(&self) {
        std::fs::write(Self::file(), [self.from as u8, self.to as u8]).unwrap();
        std::process::exit(0);
    }
    pub fn url(&self) -> String {
        format!(
            "{}/{}/{}/{}",
            Self::SERVICE,
            self.lang[self.from as usize].0,
            self.lang[self.to as usize].0,
            &self
                .source
                .replace("%", "%25")
                .replace("/", "%20")
                .replace(r#"\"#, "%20")
                .replace(" ", "%20")
                .replace("\n", "%0A")
                .replace("?", "%3F")
        )
    }
    pub fn lang(&self) -> Vec<&str> {
        self.lang.iter().map(|lang| lang.1.as_str()).collect()
    }
}
