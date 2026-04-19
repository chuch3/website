use minijinja::{Environment, path_loader};

#[derive(Debug)]
pub(crate) struct Context<'a> {
    jinja_env: minijinja::Environment<'a>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        let mut jinja_env = Environment::new();
        jinja_env.set_loader(path_loader("template"));
        Context { jinja_env }
    }
}
