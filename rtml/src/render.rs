pub trait Render: ToString {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl Render for &'static str {}

impl Render for std::fmt::Arguments<'_> {
    fn render(&self) -> String {
        format!("{}", self)
    }
}
