use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SotoProjectFile {
    pub project: SotoProjectFileProject
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SotoProjectFileProject {
    pub prefix: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SotoLocalFile {
    pub game: SotoLocalFileGame,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SotoLocalFileGame {
    pub bin: PathBuf,
    pub content: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SotoTaskFile {
    pub soto: Option<SotoTaskFileSoto>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SotoTaskFileSoto {
    pub runner: String
}
