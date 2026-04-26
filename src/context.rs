use dotenv_codegen::dotenv;
use minijinja::{Environment, path_loader};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string};
use toml::from_str;

pub(crate) struct Context<'a> {
    pub(crate) jinja_env: minijinja::Environment<'a>,
    pub(crate) projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    src: String,
    link: String,
    name: String,
    description: String,
    tools: Vec<String>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        let mut jinja_env = Environment::new();
        jinja_env.set_loader(path_loader("templates"));
        jinja_env.add_global("BASE_URL", dotenv!("STATIC_HOST"));

        // err if data is misisng
        let project_file = read_to_string(dotenv!("PROJECT_FILE")).unwrap();
        // err if wrong toml format
        let mut projects_table: HashMap<String, Vec<Project>> = from_str(&project_file).unwrap();
        let projects = projects_table.remove("projects").unwrap(); // err if projects header missing 

        Context {
            jinja_env,
            projects,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_array_of_tables_toml() {
        let project_file =
            read_to_string(dotenv!("PROJECT_FILE")).expect("Err: Unable to read project data!");
        let mut projects_table: HashMap<String, Vec<Project>> = from_str(&project_file).unwrap();
        let projects: Vec<Project> = projects_table.remove("projects").unwrap();
        println!("{:?}", projects_table);
        println!("{:?}", projects);
    }
}
