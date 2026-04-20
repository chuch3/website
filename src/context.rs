use minijinja::{Environment, path_loader};

#[derive(Debug)]
pub(crate) struct Context<'a> {
    pub(crate) jinja_env: minijinja::Environment<'a>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        let mut jinja_env = Environment::new();
        jinja_env.set_loader(path_loader("templates"));
        Context { jinja_env }
    }
}
