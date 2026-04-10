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

#[derive(Debug, Deserialize)]
pub struct PackageFile {
    #[allow(dead_code)]
    pub id: u64,
    #[allow(dead_code)]
    pub package_id: u64,
    pub file_name: String,
    pub size: u64,
    pub created_at: String,
}

pub(crate) fn encode_project(project: &str) -> String {
    urlencoding::encode(project).to_string()
}
