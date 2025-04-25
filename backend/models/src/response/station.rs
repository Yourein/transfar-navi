use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct ResStation {
    pub id: String,
    pub name: String,
    pub pronounce: String,
}