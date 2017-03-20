#[derive(Deserialize, Debug)]
pub struct SotoProjectFile {
    pub project: SotoProjectFileProject
}

#[derive(Deserialize, Debug)]
pub struct SotoProjectFileProject {
    pub prefix: String,
}

#[derive(Deserialize, Debug)]
pub struct SotoLocalFile {
    pub game: SotoLocalFileGame,
}

#[derive(Deserialize, Debug)]
pub struct SotoLocalFileGame {
    pub bin: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct SotoTaskFile {
    pub soto: Option<SotoTaskFileSoto>
}

#[derive(Deserialize, Debug)]
pub struct SotoTaskFileSoto {
    pub runner: String
}
