pub struct RenderFn<F>(pub F);

impl<F: Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result> std::fmt::Display for RenderFn<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0)(f)
    }
}

#[macro_export]
macro_rules! render_fn {
    ($($exp:tt)*) => {
        RenderFn(move |f: &mut std::fmt::Formatter<'_>| write!(f, $($exp)*))
    };
}

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

impl<F> Render for RenderFn<F>
where
    F: Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
{
    fn render(&self) -> String {
        self.to_string()
    }
}
