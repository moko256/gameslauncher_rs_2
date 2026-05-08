use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct LaunchItem {
    pub id: usize,
    pub name: String,
    pub path: PathBuf,
}
