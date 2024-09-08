use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Wallpaper {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Collection {
    pub id: u32,
    pub label: String,
}

#[derive(Debug, Deserialize)]
pub struct CollectionsData {
    pub data: Vec<Collection>,
}

#[derive(Debug, Deserialize)]
pub struct CollectionMeta {
    pub last_page: u32,
}

#[derive(Debug, Deserialize)]
pub struct CollectionData {
    pub meta: CollectionMeta,
}

#[derive(Debug, Deserialize)]
pub struct PageData {
    pub data: Vec<Wallpaper>,
}
