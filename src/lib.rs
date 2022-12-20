use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct MoneyV2 {
    pub amount: f32,
    pub currencyCode: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct ProductPriceRangeV2 {
    pub maxVariantPrice: MoneyV2,
    pub minVariantPrice: MoneyV2,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct ProductVariantContextualPricing {
    pub price: f32,
    pub compareAtPrice: Option<f32>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct ProductContextualPricing {
    pub maxVariantPricing: ProductVariantContextualPricing,
    pub minVariantPricing: ProductVariantContextualPricing,
    pub priceRange: ProductPriceRangeV2,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct MetafieldDefintion {
    pub description: Option<String>,
    pub id: String,
    pub key: String,
    pub metafieldsCount: i32,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct Metafield {
    pub createdAt: String,
    pub defintion: Option<MetafieldDefintion>,
    pub description: Option<String>,
    pub id: String,
    pub key: String,
    pub namespace: String,
    pub updatedAt: String,
    pub value: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct Image {
    pub altText: String,
    pub height: Option<i32>,
    pub id: String,
    pub url: String,
    pub width: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct Product {
    pub availablePublicationCount: i32,
    pub contextualPricing: ProductContextualPricing,
    pub createdAt: String,
    pub defaultCursor: String,
    pub description: String,
    pub descriptionHtml: String,
    pub featuredImage: Option<Image>,
    pub id: String,
    pub title: String,
    pub handle: String,
}
