use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const PRINTFUL_API_BASE: &str = "https://api.printful.com";

#[derive(Debug, Clone)]
pub struct PrintfulClient {
    client: Client,
    api_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Product {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: u64,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub product_type: Option<String>,
    pub main_category_id: Option<u32>,
    pub description: Option<String>,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub image: Option<String>,
    pub variant_count: Option<u32>,
    pub currency: Option<String>,
    pub files: Option<Vec<File>>,
}

impl Default for Product {
    fn default() -> Self {
        Self {
            id: 0,
            name: None,
            product_type: None,
            main_category_id: None,
            description: None,
            brand: None,
            model: None,
            image: None,
            variant_count: None,
            currency: None,
            files: None,
        }
    }
}

fn deserialize_id<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Id {
        Number(u64),
        String(String),
    }
    
    match Id::deserialize(deserializer)? {
        Id::Number(n) => Ok(n),
        Id::String(s) => {
            // Try to parse string as number, or return 0 for non-numeric strings like "default"
            Ok(s.parse().unwrap_or(0))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct File {
    #[serde(deserialize_with = "deserialize_optional_id")]
    pub id: Option<u64>,
    #[serde(rename = "type")]
    pub file_type: Option<String>,
    pub title: Option<String>,
    pub additional: Option<Vec<String>>,
    pub options: Option<Vec<ProductOption>>,
}

impl Default for File {
    fn default() -> Self {
        Self {
            id: None,
            file_type: None,
            title: None,
            additional: None,
            options: None,
        }
    }
}

fn deserialize_optional_id<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Id {
        Number(u64),
        String(String),
        Null,
    }
    
    match Id::deserialize(deserializer)? {
        Id::Number(n) => Ok(Some(n)),
        Id::String(s) => {
            // Try to parse string as number, or return None for non-numeric strings
            Ok(s.parse().ok())
        }
        Id::Null => Ok(None),
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ProductOption {
    pub id: Option<String>,
    pub value: Option<String>,
}

impl Default for ProductOption {
    fn default() -> Self {
        Self {
            id: None,
            value: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductListResponse {
    pub code: u32,
    pub result: Vec<Product>,
    pub paging: Option<Paging>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Paging {
    pub total: u32,
    pub offset: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponse {
    pub code: u32,
    pub result: Product,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: u64,
    pub external_id: Option<String>,
    pub shipping: Option<String>,
    pub status: String,
    pub created: u64,
    pub updated: u64,
    pub recipient: Option<Recipient>,
    pub items: Option<Vec<OrderItem>>,
    pub costs: Option<Costs>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipient {
    pub name: String,
    pub company: Option<String>,
    pub address1: String,
    pub address2: Option<String>,
    pub city: String,
    pub state_code: Option<String>,
    pub state_name: Option<String>,
    pub country_code: String,
    pub country_name: Option<String>,
    pub zip: String,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: u64,
    pub external_id: Option<String>,
    pub variant_id: u64,
    pub quantity: u32,
    pub price: Option<String>,
    pub product: Option<Product>,
    pub files: Option<Vec<File>>,
    pub options: Option<Vec<ProductOption>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Costs {
    pub subtotal: String,
    pub discount: String,
    pub shipping: String,
    pub tax: String,
    pub total: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    pub code: u32,
    pub result: Order,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderListResponse {
    pub code: u32,
    pub result: Vec<Order>,
    pub paging: Option<Paging>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub external_id: Option<String>,
    pub recipient: Recipient,
    pub items: Vec<CreateOrderItem>,
    pub shipping: Option<String>,
    pub confirm: Option<bool>,
    pub update_existing: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderItem {
    pub variant_id: u64,
    pub quantity: u32,
    pub external_id: Option<String>,
    pub files: Option<Vec<CreateFile>>,
    pub options: Option<Vec<ProductOption>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFile {
    #[serde(rename = "type")]
    pub file_type: String,
    pub url: Option<String>,
    pub id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingRate {
    pub id: String,
    pub name: String,
    pub rate: String,
    pub currency: String,
    pub min_days: u32,
    pub max_days: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingRateRequest {
    pub recipient: Recipient,
    pub items: Vec<ShippingRateItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingRateItem {
    pub variant_id: u64,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingRateResponse {
    pub code: u32,
    pub result: Vec<ShippingRate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: u32,
    pub result: Option<String>,
    pub error: Option<ErrorDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub message: String,
    pub reason: Option<String>,
}

impl PrintfulClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    fn get_auth_header(&self) -> String {
        format!("Bearer {}", self.api_key)
    }

    async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
    ) -> Result<T> {
        let url = format!("{}{}", PRINTFUL_API_BASE, endpoint);
        let response = self
            .client
            .get(&url)
            .header("Authorization", self.get_auth_header())
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            let error: ApiError = serde_json::from_str(&text)
                .unwrap_or_else(|_| ApiError {
                    code: status.as_u16() as u32,
                    result: Some(text.clone()),
                    error: Some(ErrorDetail {
                        message: text,
                        reason: None,
                    }),
                });
            anyhow::bail!("API Error {}: {:?}", error.code, error.error);
        }

        serde_json::from_str(&text)
            .with_context(|| format!("Failed to parse response. Response text: {}", &text[..text.len().min(500)]))
    }

    async fn post<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        body: &impl Serialize,
    ) -> Result<T> {
        let url = format!("{}{}", PRINTFUL_API_BASE, endpoint);
        let response = self
            .client
            .post(&url)
            .header("Authorization", self.get_auth_header())
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            let error: ApiError = serde_json::from_str(&text)
                .unwrap_or_else(|_| ApiError {
                    code: status.as_u16() as u32,
                    result: Some(text.clone()),
                    error: Some(ErrorDetail {
                        message: text,
                        reason: None,
                    }),
                });
            anyhow::bail!("API Error {}: {:?}", error.code, error.error);
        }

        serde_json::from_str(&text)
            .with_context(|| format!("Failed to parse response. Response text: {}", &text[..text.len().min(500)]))
    }

    async fn delete<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
    ) -> Result<T> {
        let url = format!("{}{}", PRINTFUL_API_BASE, endpoint);
        let response = self
            .client
            .delete(&url)
            .header("Authorization", self.get_auth_header())
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            let error: ApiError = serde_json::from_str(&text)
                .unwrap_or_else(|_| ApiError {
                    code: status.as_u16() as u32,
                    result: Some(text.clone()),
                    error: Some(ErrorDetail {
                        message: text,
                        reason: None,
                    }),
                });
            anyhow::bail!("API Error {}: {:?}", error.code, error.error);
        }

        serde_json::from_str(&text)
            .with_context(|| format!("Failed to parse response. Response text: {}", &text[..text.len().min(500)]))
    }

    // Product endpoints
    pub async fn get_products(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<ProductListResponse> {
        let mut params = HashMap::new();
        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(offset) = offset {
            params.insert("offset", offset.to_string());
        }

        let query = if params.is_empty() {
            String::new()
        } else {
            format!(
                "?{}",
                params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&")
            )
        };

        self.get(&format!("/products{}", query)).await
    }

    pub async fn get_product(&self, product_id: u64) -> Result<ProductResponse> {
        self.get(&format!("/products/{}", product_id)).await
    }

    // Order endpoints
    pub async fn get_orders(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<OrderListResponse> {
        let mut params = HashMap::new();
        if let Some(limit) = limit {
            params.insert("limit", limit.to_string());
        }
        if let Some(offset) = offset {
            params.insert("offset", offset.to_string());
        }

        let query = if params.is_empty() {
            String::new()
        } else {
            format!(
                "?{}",
                params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&")
            )
        };

        self.get(&format!("/orders{}", query)).await
    }

    pub async fn get_order(&self, order_id: u64) -> Result<OrderResponse> {
        self.get(&format!("/orders/{}", order_id)).await
    }

    pub async fn create_order(
        &self,
        request: CreateOrderRequest,
    ) -> Result<OrderResponse> {
        self.post("/orders", &request).await
    }

    pub async fn cancel_order(&self, order_id: u64) -> Result<OrderResponse> {
        self.delete(&format!("/orders/{}", order_id)).await
    }

    // Shipping rate endpoint
    pub async fn get_shipping_rates(
        &self,
        request: ShippingRateRequest,
    ) -> Result<ShippingRateResponse> {
        self.post("/shipping/rates", &request).await
    }
}

