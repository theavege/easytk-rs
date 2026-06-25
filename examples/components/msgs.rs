pub enum Converter {
    Cel(f64),
    Far(f64),
}

pub enum Curl {
    Url(String),
    Body(String),
    Responce(String),
    Run,
}

pub enum Pictures {
    Add(Vec<String>),
    Idx(usize),
    Scale(f64),
    Del(bool),
    Shift(bool),
}

pub enum Sudoku {
    Push(usize, usize, i32),
    Solve,
    Clear,
}

pub enum Dialect {
    Run,
    Quit,
    Switch,
    Source(String),
    Target(String),
    SaveAs(String),
    Open(String),
    To(i32),
    From(i32),
    Lang(Vec<(String, String)>),
}
