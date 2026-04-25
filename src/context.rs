use dotenv_codegen::dotenv;
use minijinja::{Environment, path_loader};

#[derive(Debug)]
pub(crate) struct Context<'a> {
    pub(crate) jinja_env: minijinja::Environment<'a>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        let mut jinja_env = Environment::new();
        jinja_env.set_loader(path_loader("templates"));
        jinja_env.add_global("BASE_URL", dotenv!("STATIC_HOST"));
        dbg!(dotenv!("STATIC_HOST"));
        Context { jinja_env }
    }
}

pub struct Project {}
