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
pub struct CollectonMeta {
    pub last_page: u32,
}

#[derive(Debug, Deserialize)]
pub struct CollectionData {
    pub meta: CollectonMeta,
    pub data: Vec<Wallpaper>,
}

#[derive(Debug, Deserialize)]
pub struct PageData {
    pub data: Vec<Wallpaper>,
}

impl CollectionsData {
    pub fn find_collection_id(&self, search: &str) -> Option<u32> {
        self.data
            .iter()
            .find(|&collection| collection.label == search)
            .map(|collection| collection.id)
    }
}
