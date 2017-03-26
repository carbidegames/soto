use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Prop {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Model {
    pub reference: PathBuf,
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
    pub sequences: HashMap<String, Sequence>,
}
