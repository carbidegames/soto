use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Prop {
    pub name: String,
    pub dynamic: String,
}

#[derive(Deserialize)]
pub struct Model {
    pub reference: PathBuf,
    pub flip_fix_list: Vec<String>,
}

#[derive(Deserialize)]
pub struct Sequence {
    pub file: PathBuf,
    pub params: String,
}

#[derive(Deserialize)]
pub struct SotoFbxTask {
    pub prop: Prop,
    pub model: Model,
    pub sequences: Option<HashMap<String, Sequence>>,
}
