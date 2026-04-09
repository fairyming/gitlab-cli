use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pipeline {
    pub id: u64,
}

#[derive(Debug, Deserialize)]
pub struct Job {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub id: u64,
    pub name: String,
    pub version: String,
    pub package_type: String,
}

pub(crate) fn encode_project(project: &str) -> String {
    urlencoding::encode(project).to_string()
}
