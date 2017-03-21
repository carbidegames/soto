#[derive(Deserialize)]
pub struct Prop {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Model {
    pub reference: String,
}

#[derive(Deserialize)]
pub struct SotoFbxTask {
    pub prop: Prop,
    pub model: Model,
}
