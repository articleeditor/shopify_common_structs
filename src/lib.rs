use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Image {
    pub url: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct Product {
    pub id: String,
    pub title: String,
    pub handle: String,
    pub createdAt: String,
    pub description: String,
    pub descriptionHtml: String,
    pub featuredImage: Option<Image>,
}
