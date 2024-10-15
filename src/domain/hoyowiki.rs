use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct HoyowikiResponse {
    pub data: Data,
    pub message: String,
    pub retcode: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub page: Page,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Page {
    pub id: String,
    pub modules: Vec<Module>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Module {
    pub name: String,
    pub components: Vec<Component>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Component {
    pub data: String,
}
