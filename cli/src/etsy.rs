use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

const ETSY_API_BASE: &str = "https://api.etsy.com/v3/application";
const ETSY_OAUTH_BASE: &str = "https://api.etsy.com/v3/public/oauth";

#[derive(Debug, Clone)]
pub struct EtsyClient {
    client: Client,
    api_key: String,
    access_token: Option<String>,
}

// ============================================================================
// Response Types
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Shop {
    pub shop_id: i64,
    pub shop_name: Option<String>,
    pub user_id: Option<i64>,
    pub title: Option<String>,
    pub announcement: Option<String>,
    pub currency_code: Option<String>,
    pub is_vacation: Option<bool>,
    pub vacation_message: Option<String>,
    pub sale_message: Option<String>,
    pub digital_sale_message: Option<String>,
    pub listing_active_count: Option<i32>,
    pub digital_listing_count: Option<i32>,
    pub login_name: Option<String>,
    pub url: Option<String>,
    pub image_url_760x100: Option<String>,
    pub num_favorers: Option<i32>,
    pub review_count: Option<i32>,
    pub review_average: Option<f64>,
    pub transaction_sold_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShopResponse {
    pub shop_id: i64,
    pub shop_name: Option<String>,
    pub user_id: Option<i64>,
    pub title: Option<String>,
    pub announcement: Option<String>,
    pub currency_code: Option<String>,
    pub is_vacation: Option<bool>,
    pub vacation_message: Option<String>,
    pub sale_message: Option<String>,
    pub digital_sale_message: Option<String>,
    pub listing_active_count: Option<i32>,
    pub digital_listing_count: Option<i32>,
    pub login_name: Option<String>,
    pub url: Option<String>,
    pub image_url_760x100: Option<String>,
    pub num_favorers: Option<i32>,
    pub review_count: Option<i32>,
    pub review_average: Option<f64>,
    pub transaction_sold_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Listing {
    pub listing_id: i64,
    pub shop_id: Option<i64>,
    pub user_id: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub state: Option<String>,
    pub creation_timestamp: Option<i64>,
    pub ending_timestamp: Option<i64>,
    pub original_creation_timestamp: Option<i64>,
    pub last_modified_timestamp: Option<i64>,
    pub state_timestamp: Option<i64>,
    pub quantity: Option<i32>,
    pub shop_section_id: Option<i64>,
    pub featured_rank: Option<i32>,
    pub url: Option<String>,
    pub num_favorers: Option<i32>,
    pub non_taxable: Option<bool>,
    pub is_taxable: Option<bool>,
    pub is_customizable: Option<bool>,
    pub is_personalizable: Option<bool>,
    pub personalization_is_required: Option<bool>,
    pub personalization_char_count_max: Option<i32>,
    pub personalization_instructions: Option<String>,
    pub listing_type: Option<String>,
    pub tags: Option<Vec<String>>,
    pub materials: Option<Vec<String>>,
    pub shipping_profile_id: Option<i64>,
    pub return_policy_id: Option<i64>,
    pub processing_min: Option<i32>,
    pub processing_max: Option<i32>,
    pub who_made: Option<String>,
    pub when_made: Option<String>,
    pub is_supply: Option<bool>,
    pub item_weight: Option<f64>,
    pub item_weight_unit: Option<String>,
    pub item_length: Option<f64>,
    pub item_width: Option<f64>,
    pub item_height: Option<f64>,
    pub item_dimensions_unit: Option<String>,
    pub is_private: Option<bool>,
    pub style: Option<Vec<String>>,
    pub file_data: Option<String>,
    pub has_variations: Option<bool>,
    pub should_auto_renew: Option<bool>,
    pub language: Option<String>,
    pub price: Option<Price>,
    pub taxonomy_id: Option<i64>,
    pub views: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    pub amount: i64,
    pub divisor: i32,
    pub currency_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingsResponse {
    pub count: i32,
    pub results: Vec<Listing>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Receipt {
    pub receipt_id: i64,
    pub receipt_type: Option<i32>,
    pub seller_user_id: Option<i64>,
    pub seller_email: Option<String>,
    pub buyer_user_id: Option<i64>,
    pub buyer_email: Option<String>,
    pub name: Option<String>,
    pub first_line: Option<String>,
    pub second_line: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub status: Option<String>,
    pub formatted_address: Option<String>,
    pub country_iso: Option<String>,
    pub payment_method: Option<String>,
    pub payment_email: Option<String>,
    pub message_from_seller: Option<String>,
    pub message_from_buyer: Option<String>,
    pub message_from_payment: Option<String>,
    pub is_paid: Option<bool>,
    pub is_shipped: Option<bool>,
    pub create_timestamp: Option<i64>,
    pub created_timestamp: Option<i64>,
    pub update_timestamp: Option<i64>,
    pub updated_timestamp: Option<i64>,
    pub is_gift: Option<bool>,
    pub gift_message: Option<String>,
    pub grandtotal: Option<Price>,
    pub subtotal: Option<Price>,
    pub total_price: Option<Price>,
    pub total_shipping_cost: Option<Price>,
    pub total_tax_cost: Option<Price>,
    pub total_vat_cost: Option<Price>,
    pub discount_amt: Option<Price>,
    pub gift_wrap_price: Option<Price>,
    pub shipments: Option<Vec<Shipment>>,
    pub transactions: Option<Vec<Transaction>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiptsResponse {
    pub count: i32,
    pub results: Vec<Receipt>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shipment {
    pub receipt_shipping_id: Option<i64>,
    pub shipment_notification_timestamp: Option<i64>,
    pub carrier_name: Option<String>,
    pub tracking_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub transaction_id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub seller_user_id: Option<i64>,
    pub buyer_user_id: Option<i64>,
    pub create_timestamp: Option<i64>,
    pub created_timestamp: Option<i64>,
    pub paid_timestamp: Option<i64>,
    pub shipped_timestamp: Option<i64>,
    pub quantity: Option<i32>,
    pub listing_image_id: Option<i64>,
    pub receipt_id: Option<i64>,
    pub is_digital: Option<bool>,
    pub file_data: Option<String>,
    pub listing_id: Option<i64>,
    pub sku: Option<String>,
    pub product_id: Option<i64>,
    pub transaction_type: Option<String>,
    pub price: Option<Price>,
    pub shipping_cost: Option<Price>,
    pub variations: Option<Vec<TransactionVariation>>,
    pub product_data: Option<Value>,
    pub shipping_profile_id: Option<i64>,
    pub min_processing_days: Option<i32>,
    pub max_processing_days: Option<i32>,
    pub shipping_method: Option<String>,
    pub shipping_upgrade: Option<String>,
    pub expected_ship_date: Option<i64>,
    pub buyer_coupon: Option<f64>,
    pub shop_coupon: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsResponse {
    pub count: i32,
    pub results: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionVariation {
    pub property_id: Option<i64>,
    pub value_id: Option<i64>,
    pub formatted_name: Option<String>,
    pub formatted_value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingImage {
    pub listing_id: i64,
    pub listing_image_id: i64,
    pub hex_code: Option<String>,
    pub red: Option<i32>,
    pub green: Option<i32>,
    pub blue: Option<i32>,
    pub hue: Option<i32>,
    pub saturation: Option<i32>,
    pub brightness: Option<i32>,
    pub is_black_and_white: Option<bool>,
    pub creation_tsz: Option<i64>,
    pub created_timestamp: Option<i64>,
    pub rank: Option<i32>,
    pub url_75x75: Option<String>,
    pub url_170x135: Option<String>,
    #[serde(rename = "url_570xN")]
    pub url_570x_n: Option<String>,
    pub url_fullxfull: Option<String>,
    pub full_height: Option<i32>,
    pub full_width: Option<i32>,
    pub alt_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingImagesResponse {
    pub count: i32,
    pub results: Vec<ListingImage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingInventory {
    pub products: Vec<ListingProduct>,
    pub price_on_property: Option<Vec<i64>>,
    pub quantity_on_property: Option<Vec<i64>>,
    pub sku_on_property: Option<Vec<i64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingProduct {
    pub product_id: i64,
    pub sku: Option<String>,
    pub is_deleted: Option<bool>,
    pub offerings: Option<Vec<ProductOffering>>,
    pub property_values: Option<Vec<PropertyValue>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductOffering {
    pub offering_id: i64,
    pub quantity: Option<i32>,
    pub is_enabled: Option<bool>,
    pub is_deleted: Option<bool>,
    pub price: Option<Price>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyValue {
    pub property_id: i64,
    pub property_name: Option<String>,
    pub scale_id: Option<i64>,
    pub scale_name: Option<String>,
    pub value_ids: Option<Vec<i64>>,
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingProfile {
    pub shipping_profile_id: i64,
    pub title: Option<String>,
    pub user_id: Option<i64>,
    pub min_processing_days: Option<i32>,
    pub max_processing_days: Option<i32>,
    pub processing_days_display_label: Option<String>,
    pub origin_country_iso: Option<String>,
    pub origin_postal_code: Option<String>,
    pub profile_type: Option<String>,
    pub domestic_handling_fee: Option<f64>,
    pub international_handling_fee: Option<f64>,
    pub shipping_profile_destinations: Option<Vec<ShippingProfileDestination>>,
    pub shipping_profile_upgrades: Option<Vec<ShippingProfileUpgrade>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingProfilesResponse {
    pub count: i32,
    pub results: Vec<ShippingProfile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingProfileDestination {
    pub shipping_profile_destination_id: Option<i64>,
    pub shipping_profile_id: Option<i64>,
    pub origin_country_iso: Option<String>,
    pub destination_country_iso: Option<String>,
    pub destination_region: Option<String>,
    pub primary_cost: Option<Price>,
    pub secondary_cost: Option<Price>,
    pub shipping_carrier_id: Option<i64>,
    pub mail_class: Option<String>,
    pub min_delivery_days: Option<i32>,
    pub max_delivery_days: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingProfileUpgrade {
    pub shipping_profile_id: Option<i64>,
    pub upgrade_id: Option<i64>,
    pub upgrade_name: Option<String>,
    pub r#type: Option<String>,
    pub rank: Option<i32>,
    pub language: Option<String>,
    pub price: Option<Price>,
    pub secondary_price: Option<Price>,
    pub shipping_carrier_id: Option<i64>,
    pub mail_class: Option<String>,
    pub min_delivery_days: Option<i32>,
    pub max_delivery_days: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxonomyNode {
    pub id: i64,
    pub level: Option<i32>,
    pub name: Option<String>,
    pub parent_id: Option<i64>,
    pub children: Option<Vec<TaxonomyNode>>,
    pub full_path_taxonomy_ids: Option<Vec<i64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxonomyNodesResponse {
    pub count: i32,
    pub results: Vec<TaxonomyNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxonomyProperty {
    pub property_id: i64,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub scales: Option<Vec<TaxonomyPropertyScale>>,
    pub is_required: Option<bool>,
    pub supports_attributes: Option<bool>,
    pub supports_variations: Option<bool>,
    pub is_multivalued: Option<bool>,
    pub max_values_allowed: Option<i32>,
    pub possible_values: Option<Vec<TaxonomyPropertyValue>>,
    pub selected_values: Option<Vec<TaxonomyPropertyValue>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxonomyPropertyScale {
    pub scale_id: i64,
    pub display_name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxonomyPropertyValue {
    pub value_id: Option<i64>,
    pub name: Option<String>,
    pub scale_id: Option<i64>,
    pub equal_to: Option<Vec<i64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxonomyPropertiesResponse {
    pub count: i32,
    pub results: Vec<TaxonomyProperty>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: i64,
    pub primary_email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub image_url_75x75: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    pub shop_id: i64,
    pub listing_id: i64,
    pub transaction_id: Option<i64>,
    pub buyer_user_id: Option<i64>,
    pub rating: Option<i32>,
    pub review: Option<String>,
    pub language: Option<String>,
    pub image_url_fullxfull: Option<String>,
    pub create_timestamp: Option<i64>,
    pub created_timestamp: Option<i64>,
    pub update_timestamp: Option<i64>,
    pub updated_timestamp: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewsResponse {
    pub count: i32,
    pub results: Vec<Review>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShopSection {
    pub shop_section_id: i64,
    pub title: Option<String>,
    pub rank: Option<i32>,
    pub user_id: Option<i64>,
    pub active_listing_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShopSectionsResponse {
    pub count: i32,
    pub results: Vec<ShopSection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnPolicy {
    pub return_policy_id: i64,
    pub shop_id: Option<i64>,
    pub accepts_returns: Option<bool>,
    pub accepts_exchanges: Option<bool>,
    pub return_deadline: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnPoliciesResponse {
    pub count: i32,
    pub results: Vec<ReturnPolicy>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    pub payment_id: i64,
    pub buyer_user_id: Option<i64>,
    pub shop_id: Option<i64>,
    pub receipt_id: Option<i64>,
    pub amount_gross: Option<Price>,
    pub amount_fees: Option<Price>,
    pub amount_net: Option<Price>,
    pub posted_gross: Option<Price>,
    pub posted_fees: Option<Price>,
    pub posted_net: Option<Price>,
    pub adjusted_gross: Option<Price>,
    pub adjusted_fees: Option<Price>,
    pub adjusted_net: Option<Price>,
    pub currency: Option<String>,
    pub shop_currency: Option<String>,
    pub buyer_currency: Option<String>,
    pub shipping_user_id: Option<i64>,
    pub shipping_address_id: Option<i64>,
    pub billing_address_id: Option<i64>,
    pub status: Option<String>,
    pub shipped_timestamp: Option<i64>,
    pub create_timestamp: Option<i64>,
    pub created_timestamp: Option<i64>,
    pub update_timestamp: Option<i64>,
    pub updated_timestamp: Option<i64>,
    pub payment_adjustments: Option<Vec<PaymentAdjustment>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentsResponse {
    pub count: i32,
    pub results: Vec<Payment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentAdjustment {
    pub payment_adjustment_id: i64,
    pub payment_id: Option<i64>,
    pub status: Option<String>,
    pub is_success: Option<bool>,
    pub user_id: Option<i64>,
    pub reason_code: Option<String>,
    pub total_adjustment_amount: Option<i64>,
    pub shop_total_adjustment_amount: Option<i64>,
    pub buyer_total_adjustment_amount: Option<i64>,
    pub total_fee_adjustment_amount: Option<i64>,
    pub create_timestamp: Option<i64>,
    pub created_timestamp: Option<i64>,
    pub update_timestamp: Option<i64>,
    pub updated_timestamp: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub entry_id: i64,
    pub ledger_id: Option<i64>,
    pub sequence_number: Option<i32>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub description: Option<String>,
    pub balance: Option<i64>,
    pub create_date: Option<i64>,
    pub created_timestamp: Option<i64>,
    pub ledger_type: Option<String>,
    pub reference_type: Option<String>,
    pub reference_id: Option<String>,
    pub payment_adjustments: Option<Vec<PaymentAdjustment>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerEntriesResponse {
    pub count: i32,
    pub results: Vec<LedgerEntry>,
}

// ============================================================================
// Request Types
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDraftListingRequest {
    pub quantity: i32,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub who_made: String,       // "i_did", "someone_else", "collective"
    pub when_made: String,      // e.g., "made_to_order", "2020_2024", "2010_2019", etc.
    pub taxonomy_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_profile_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_policy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub materials: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_section_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_min: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_max: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_weight_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_dimensions_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personalizable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalization_is_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalization_char_count_max: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalization_instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_supply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_customizable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_auto_renew: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_taxable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,  // "physical" or "download"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateListingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_made: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_made: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxonomy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_profile_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_policy_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub materials: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_section_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_min: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_max: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_weight_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_dimensions_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personalizable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalization_is_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalization_char_count_max: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalization_instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,  // "active", "inactive", "draft"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_supply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_customizable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_auto_renew: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_taxable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInventoryRequest {
    pub products: Vec<UpdateInventoryProduct>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_on_property: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_on_property: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_on_property: Option<Vec<i64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInventoryProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    pub offerings: Vec<UpdateInventoryOffering>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_values: Option<Vec<UpdatePropertyValue>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInventoryOffering {
    pub price: f64,
    pub quantity: i32,
    pub is_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePropertyValue {
    pub property_id: i64,
    pub value_ids: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReceiptShipmentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_bcc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_to_buyer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateShippingProfileRequest {
    pub title: String,
    pub origin_country_iso: String,
    pub primary_cost: f64,
    pub secondary_cost: f64,
    pub min_processing_time: i32,
    pub max_processing_time: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_country_iso: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_carrier_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_delivery_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_delivery_days: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
    pub error_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingResponse {
    pub application_id: i64,
}

impl EtsyClient {
    pub fn new(api_key: String, access_token: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            access_token,
        }
    }

    fn get_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "x-api-key",
            self.api_key.parse().unwrap(),
        );
        if let Some(ref token) = self.access_token {
            headers.insert(
                "Authorization",
                format!("Bearer {}", token).parse().unwrap(),
            );
        }
        headers
    }

    async fn get<T: for<'de> Deserialize<'de>>(&self, endpoint: &str) -> Result<T> {
        let url = format!("{}{}", ETSY_API_BASE, endpoint);
        let response = self
            .client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            if let Ok(error) = serde_json::from_str::<ApiError>(&text) {
                anyhow::bail!("API Error: {} - {:?}", error.error, error.error_description);
            }
            anyhow::bail!("API Error {}: {}", status, text);
        }

        serde_json::from_str(&text)
            .with_context(|| format!("Failed to parse response: {}", &text[..text.len().min(500)]))
    }

    async fn get_raw(&self, endpoint: &str) -> Result<Value> {
        let url = format!("{}{}", ETSY_API_BASE, endpoint);
        let response = self
            .client
            .get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            anyhow::bail!("API Error {}: {}", status, text);
        }

        serde_json::from_str(&text).context("Failed to parse response as JSON")
    }

    async fn post<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        body: &impl Serialize,
    ) -> Result<T> {
        let url = format!("{}{}", ETSY_API_BASE, endpoint);
        let response = self
            .client
            .post(&url)
            .headers(self.get_headers())
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            if let Ok(error) = serde_json::from_str::<ApiError>(&text) {
                anyhow::bail!("API Error: {} - {:?}", error.error, error.error_description);
            }
            anyhow::bail!("API Error {}: {}", status, text);
        }

        serde_json::from_str(&text)
            .with_context(|| format!("Failed to parse response: {}", &text[..text.len().min(500)]))
    }

    async fn post_form<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        form: &HashMap<String, String>,
    ) -> Result<T> {
        let url = format!("{}{}", ETSY_API_BASE, endpoint);
        let response = self
            .client
            .post(&url)
            .headers(self.get_headers())
            .form(form)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            anyhow::bail!("API Error {}: {}", status, text);
        }

        serde_json::from_str(&text)
            .with_context(|| format!("Failed to parse response: {}", &text[..text.len().min(500)]))
    }

    async fn put<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        body: &impl Serialize,
    ) -> Result<T> {
        let url = format!("{}{}", ETSY_API_BASE, endpoint);
        let response = self
            .client
            .put(&url)
            .headers(self.get_headers())
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            if let Ok(error) = serde_json::from_str::<ApiError>(&text) {
                anyhow::bail!("API Error: {} - {:?}", error.error, error.error_description);
            }
            anyhow::bail!("API Error {}: {}", status, text);
        }

        serde_json::from_str(&text)
            .with_context(|| format!("Failed to parse response: {}", &text[..text.len().min(500)]))
    }

    async fn patch<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        body: &impl Serialize,
    ) -> Result<T> {
        let url = format!("{}{}", ETSY_API_BASE, endpoint);
        let response = self
            .client
            .patch(&url)
            .headers(self.get_headers())
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            if let Ok(error) = serde_json::from_str::<ApiError>(&text) {
                anyhow::bail!("API Error: {} - {:?}", error.error, error.error_description);
            }
            anyhow::bail!("API Error {}: {}", status, text);
        }

        serde_json::from_str(&text)
            .with_context(|| format!("Failed to parse response: {}", &text[..text.len().min(500)]))
    }

    async fn delete<T: for<'de> Deserialize<'de>>(&self, endpoint: &str) -> Result<T> {
        let url = format!("{}{}", ETSY_API_BASE, endpoint);
        let response = self
            .client
            .delete(&url)
            .headers(self.get_headers())
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            if let Ok(error) = serde_json::from_str::<ApiError>(&text) {
                anyhow::bail!("API Error: {} - {:?}", error.error, error.error_description);
            }
            anyhow::bail!("API Error {}: {}", status, text);
        }

        serde_json::from_str(&text)
            .with_context(|| format!("Failed to parse response: {}", &text[..text.len().min(500)]))
    }

    async fn delete_no_response(&self, endpoint: &str) -> Result<()> {
        let url = format!("{}{}", ETSY_API_BASE, endpoint);
        let response = self
            .client
            .delete(&url)
            .headers(self.get_headers())
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        if !status.is_success() {
            let text = response.text().await.context("Failed to read response")?;
            anyhow::bail!("API Error {}: {}", status, text);
        }

        Ok(())
    }

    // ========================================================================
    // Generic raw API call for advanced usage
    // ========================================================================

    pub async fn raw_get(&self, path: &str, params: &HashMap<String, String>) -> Result<Value> {
        let query = if params.is_empty() {
            String::new()
        } else {
            format!(
                "?{}",
                params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
                    .collect::<Vec<_>>()
                    .join("&")
            )
        };
        self.get_raw(&format!("/{}{}", path, query)).await
    }

    pub async fn raw_post(&self, path: &str, body: Value) -> Result<Value> {
        let url = format!("{}/{}", ETSY_API_BASE, path);
        let response = self
            .client
            .post(&url)
            .headers(self.get_headers())
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            anyhow::bail!("API Error {}: {}", status, text);
        }

        serde_json::from_str(&text).context("Failed to parse response as JSON")
    }

    pub async fn raw_put(&self, path: &str, body: Value) -> Result<Value> {
        let url = format!("{}/{}", ETSY_API_BASE, path);
        let response = self
            .client
            .put(&url)
            .headers(self.get_headers())
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            anyhow::bail!("API Error {}: {}", status, text);
        }

        serde_json::from_str(&text).context("Failed to parse response as JSON")
    }

    pub async fn raw_delete(&self, path: &str) -> Result<Value> {
        let url = format!("{}/{}", ETSY_API_BASE, path);
        let response = self
            .client
            .delete(&url)
            .headers(self.get_headers())
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            anyhow::bail!("API Error {}: {}", status, text);
        }

        if text.is_empty() {
            Ok(Value::Null)
        } else {
            serde_json::from_str(&text).context("Failed to parse response as JSON")
        }
    }

    // ========================================================================
    // Application / Auth Endpoints
    // ========================================================================

    /// Test API key connectivity (does not require OAuth token)
    pub async fn ping(&self) -> Result<PingResponse> {
        self.get("/openapi-ping").await
    }

    /// Get OAuth token URL for user to authorize
    pub fn get_oauth_url(
        client_id: &str,
        redirect_uri: &str,
        scopes: &[&str],
        state: &str,
        code_challenge: &str,
    ) -> String {
        let scope = scopes.join("%20");
        format!(
            "https://www.etsy.com/oauth/connect?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
            client_id,
            urlencoding::encode(redirect_uri),
            scope,
            state,
            code_challenge
        )
    }

    /// Exchange authorization code for access token
    pub async fn exchange_code(
        client_id: &str,
        redirect_uri: &str,
        code: &str,
        code_verifier: &str,
    ) -> Result<OAuthTokenResponse> {
        let client = Client::new();
        let url = format!("{}/token", ETSY_OAUTH_BASE);

        let mut params = HashMap::new();
        params.insert("grant_type", "authorization_code");
        params.insert("client_id", client_id);
        params.insert("redirect_uri", redirect_uri);
        params.insert("code", code);
        params.insert("code_verifier", code_verifier);

        let response = client
            .post(&url)
            .form(&params)
            .send()
            .await
            .context("Failed to exchange code")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            anyhow::bail!("OAuth Error {}: {}", status, text);
        }

        serde_json::from_str(&text).context("Failed to parse OAuth response")
    }

    /// Refresh access token
    pub async fn refresh_token(client_id: &str, refresh_token: &str) -> Result<OAuthTokenResponse> {
        let client = Client::new();
        let url = format!("{}/token", ETSY_OAUTH_BASE);

        let mut params = HashMap::new();
        params.insert("grant_type", "refresh_token");
        params.insert("client_id", client_id);
        params.insert("refresh_token", refresh_token);

        let response = client
            .post(&url)
            .form(&params)
            .send()
            .await
            .context("Failed to refresh token")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            anyhow::bail!("OAuth Error {}: {}", status, text);
        }

        serde_json::from_str(&text).context("Failed to parse OAuth response")
    }

    // ========================================================================
    // User Endpoints
    // ========================================================================

    /// Get authenticated user info
    pub async fn get_me(&self) -> Result<User> {
        self.get("/users/me").await
    }

    /// Get user by ID
    pub async fn get_user(&self, user_id: i64) -> Result<User> {
        self.get(&format!("/users/{}", user_id)).await
    }

    // ========================================================================
    // Shop Endpoints
    // ========================================================================

    /// Get shop by ID
    pub async fn get_shop(&self, shop_id: i64) -> Result<ShopResponse> {
        self.get(&format!("/shops/{}", shop_id)).await
    }

    /// Get shop by owner user ID
    pub async fn get_shop_by_owner_user_id(&self, user_id: i64) -> Result<ShopResponse> {
        self.get(&format!("/users/{}/shops", user_id)).await
    }

    /// Find shops by name
    pub async fn find_shops(&self, shop_name: &str, limit: Option<i32>, offset: Option<i32>) -> Result<Value> {
        let mut params = HashMap::new();
        params.insert("shop_name".to_string(), shop_name.to_string());
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        self.raw_get("shops", &params).await
    }

    /// Get shop sections
    pub async fn get_shop_sections(&self, shop_id: i64) -> Result<ShopSectionsResponse> {
        self.get(&format!("/shops/{}/sections", shop_id)).await
    }

    // ========================================================================
    // Listing Endpoints
    // ========================================================================

    /// Get listing by ID
    pub async fn get_listing(&self, listing_id: i64, includes: Option<&[&str]>) -> Result<Listing> {
        let query = if let Some(inc) = includes {
            format!("?includes={}", inc.join(","))
        } else {
            String::new()
        };
        self.get(&format!("/listings/{}{}", listing_id, query)).await
    }

    /// Get listings by shop ID
    pub async fn get_listings_by_shop(
        &self,
        shop_id: i64,
        state: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
        sort_on: Option<&str>,
        sort_order: Option<&str>,
        includes: Option<&[&str]>,
    ) -> Result<ListingsResponse> {
        let mut params = vec![];
        if let Some(s) = state {
            params.push(format!("state={}", s));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(o) = offset {
            params.push(format!("offset={}", o));
        }
        if let Some(s) = sort_on {
            params.push(format!("sort_on={}", s));
        }
        if let Some(s) = sort_order {
            params.push(format!("sort_order={}", s));
        }
        if let Some(inc) = includes {
            params.push(format!("includes={}", inc.join(",")));
        }
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        self.get(&format!("/shops/{}/listings{}", shop_id, query)).await
    }

    /// Get active listings by shop
    pub async fn get_active_listings_by_shop(
        &self,
        shop_id: i64,
        limit: Option<i32>,
        offset: Option<i32>,
        keywords: Option<&str>,
    ) -> Result<ListingsResponse> {
        let mut params = vec![];
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(o) = offset {
            params.push(format!("offset={}", o));
        }
        if let Some(k) = keywords {
            params.push(format!("keywords={}", urlencoding::encode(k)));
        }
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        self.get(&format!("/shops/{}/listings/active{}", shop_id, query)).await
    }

    /// Get featured listings by shop
    pub async fn get_featured_listings_by_shop(
        &self,
        shop_id: i64,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<ListingsResponse> {
        let mut params = vec![];
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(o) = offset {
            params.push(format!("offset={}", o));
        }
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        self.get(&format!("/shops/{}/listings/featured{}", shop_id, query)).await
    }

    /// Create a draft listing
    pub async fn create_draft_listing(
        &self,
        shop_id: i64,
        request: &CreateDraftListingRequest,
    ) -> Result<Listing> {
        self.post(&format!("/shops/{}/listings", shop_id), request).await
    }

    /// Update a listing
    pub async fn update_listing(
        &self,
        shop_id: i64,
        listing_id: i64,
        request: &UpdateListingRequest,
    ) -> Result<Listing> {
        self.patch(&format!("/shops/{}/listings/{}", shop_id, listing_id), request).await
    }

    /// Delete a listing
    pub async fn delete_listing(&self, shop_id: i64, listing_id: i64) -> Result<()> {
        self.delete_no_response(&format!("/shops/{}/listings/{}", shop_id, listing_id)).await
    }

    // ========================================================================
    // Listing Images
    // ========================================================================

    /// Get listing images
    pub async fn get_listing_images(&self, listing_id: i64) -> Result<ListingImagesResponse> {
        self.get(&format!("/listings/{}/images", listing_id)).await
    }

    /// Get a specific listing image
    pub async fn get_listing_image(&self, listing_id: i64, image_id: i64) -> Result<ListingImage> {
        self.get(&format!("/listings/{}/images/{}", listing_id, image_id)).await
    }

    /// Delete a listing image
    pub async fn delete_listing_image(
        &self,
        shop_id: i64,
        listing_id: i64,
        image_id: i64,
    ) -> Result<()> {
        self.delete_no_response(&format!(
            "/shops/{}/listings/{}/images/{}",
            shop_id, listing_id, image_id
        ))
        .await
    }

    // ========================================================================
    // Listing Inventory
    // ========================================================================

    /// Get listing inventory
    pub async fn get_listing_inventory(&self, listing_id: i64) -> Result<ListingInventory> {
        self.get(&format!("/listings/{}/inventory", listing_id)).await
    }

    /// Update listing inventory
    pub async fn update_listing_inventory(
        &self,
        listing_id: i64,
        request: &UpdateInventoryRequest,
    ) -> Result<ListingInventory> {
        self.put(&format!("/listings/{}/inventory", listing_id), request).await
    }

    /// Get listing product
    pub async fn get_listing_product(
        &self,
        listing_id: i64,
        product_id: i64,
    ) -> Result<ListingProduct> {
        self.get(&format!("/listings/{}/inventory/products/{}", listing_id, product_id))
            .await
    }

    // ========================================================================
    // Receipt / Order Endpoints
    // ========================================================================

    /// Get shop receipts (orders)
    pub async fn get_shop_receipts(
        &self,
        shop_id: i64,
        min_created: Option<i64>,
        max_created: Option<i64>,
        min_last_modified: Option<i64>,
        max_last_modified: Option<i64>,
        limit: Option<i32>,
        offset: Option<i32>,
        sort_on: Option<&str>,
        sort_order: Option<&str>,
        was_paid: Option<bool>,
        was_shipped: Option<bool>,
        was_delivered: Option<bool>,
    ) -> Result<ReceiptsResponse> {
        let mut params = vec![];
        if let Some(v) = min_created {
            params.push(format!("min_created={}", v));
        }
        if let Some(v) = max_created {
            params.push(format!("max_created={}", v));
        }
        if let Some(v) = min_last_modified {
            params.push(format!("min_last_modified={}", v));
        }
        if let Some(v) = max_last_modified {
            params.push(format!("max_last_modified={}", v));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(o) = offset {
            params.push(format!("offset={}", o));
        }
        if let Some(s) = sort_on {
            params.push(format!("sort_on={}", s));
        }
        if let Some(s) = sort_order {
            params.push(format!("sort_order={}", s));
        }
        if let Some(v) = was_paid {
            params.push(format!("was_paid={}", v));
        }
        if let Some(v) = was_shipped {
            params.push(format!("was_shipped={}", v));
        }
        if let Some(v) = was_delivered {
            params.push(format!("was_delivered={}", v));
        }
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        self.get(&format!("/shops/{}/receipts{}", shop_id, query)).await
    }

    /// Get a specific receipt
    pub async fn get_shop_receipt(&self, shop_id: i64, receipt_id: i64) -> Result<Receipt> {
        self.get(&format!("/shops/{}/receipts/{}", shop_id, receipt_id)).await
    }

    /// Create shipment for receipt
    pub async fn create_receipt_shipment(
        &self,
        shop_id: i64,
        receipt_id: i64,
        request: &CreateReceiptShipmentRequest,
    ) -> Result<Receipt> {
        self.post(
            &format!("/shops/{}/receipts/{}/tracking", shop_id, receipt_id),
            request,
        )
        .await
    }

    // ========================================================================
    // Transaction Endpoints
    // ========================================================================

    /// Get shop receipt transactions
    pub async fn get_shop_receipt_transactions(
        &self,
        shop_id: i64,
        receipt_id: i64,
    ) -> Result<TransactionsResponse> {
        self.get(&format!("/shops/{}/receipts/{}/transactions", shop_id, receipt_id))
            .await
    }

    /// Get shop transactions
    pub async fn get_shop_transactions(
        &self,
        shop_id: i64,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<TransactionsResponse> {
        let mut params = vec![];
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(o) = offset {
            params.push(format!("offset={}", o));
        }
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        self.get(&format!("/shops/{}/transactions{}", shop_id, query)).await
    }

    /// Get a specific transaction
    pub async fn get_shop_receipt_transaction(
        &self,
        shop_id: i64,
        transaction_id: i64,
    ) -> Result<Transaction> {
        self.get(&format!("/shops/{}/transactions/{}", shop_id, transaction_id))
            .await
    }

    // ========================================================================
    // Shipping Profile Endpoints
    // ========================================================================

    /// Get shop shipping profiles
    pub async fn get_shop_shipping_profiles(
        &self,
        shop_id: i64,
    ) -> Result<ShippingProfilesResponse> {
        self.get(&format!("/shops/{}/shipping-profiles", shop_id)).await
    }

    /// Get a specific shipping profile
    pub async fn get_shop_shipping_profile(
        &self,
        shop_id: i64,
        shipping_profile_id: i64,
    ) -> Result<ShippingProfile> {
        self.get(&format!(
            "/shops/{}/shipping-profiles/{}",
            shop_id, shipping_profile_id
        ))
        .await
    }

    /// Create shipping profile
    pub async fn create_shop_shipping_profile(
        &self,
        shop_id: i64,
        request: &CreateShippingProfileRequest,
    ) -> Result<ShippingProfile> {
        self.post(&format!("/shops/{}/shipping-profiles", shop_id), request)
            .await
    }

    /// Delete shipping profile
    pub async fn delete_shop_shipping_profile(
        &self,
        shop_id: i64,
        shipping_profile_id: i64,
    ) -> Result<()> {
        self.delete_no_response(&format!(
            "/shops/{}/shipping-profiles/{}",
            shop_id, shipping_profile_id
        ))
        .await
    }

    // ========================================================================
    // Return Policy Endpoints
    // ========================================================================

    /// Get shop return policies
    pub async fn get_shop_return_policies(&self, shop_id: i64) -> Result<ReturnPoliciesResponse> {
        self.get(&format!("/shops/{}/policies/return", shop_id)).await
    }

    /// Get a specific return policy
    pub async fn get_shop_return_policy(
        &self,
        shop_id: i64,
        return_policy_id: i64,
    ) -> Result<ReturnPolicy> {
        self.get(&format!(
            "/shops/{}/policies/return/{}",
            shop_id, return_policy_id
        ))
        .await
    }

    // ========================================================================
    // Taxonomy Endpoints
    // ========================================================================

    /// Get seller taxonomy nodes
    pub async fn get_seller_taxonomy_nodes(&self) -> Result<TaxonomyNodesResponse> {
        self.get("/seller-taxonomy/nodes").await
    }

    /// Get taxonomy node properties
    pub async fn get_properties_by_taxonomy_id(
        &self,
        taxonomy_id: i64,
    ) -> Result<TaxonomyPropertiesResponse> {
        self.get(&format!("/seller-taxonomy/nodes/{}/properties", taxonomy_id))
            .await
    }

    // ========================================================================
    // Review Endpoints
    // ========================================================================

    /// Get shop reviews
    pub async fn get_reviews_by_shop(
        &self,
        shop_id: i64,
        limit: Option<i32>,
        offset: Option<i32>,
        min_created: Option<i64>,
        max_created: Option<i64>,
    ) -> Result<ReviewsResponse> {
        let mut params = vec![];
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(o) = offset {
            params.push(format!("offset={}", o));
        }
        if let Some(v) = min_created {
            params.push(format!("min_created={}", v));
        }
        if let Some(v) = max_created {
            params.push(format!("max_created={}", v));
        }
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        self.get(&format!("/shops/{}/reviews{}", shop_id, query)).await
    }

    /// Get reviews by listing
    pub async fn get_reviews_by_listing(
        &self,
        listing_id: i64,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<ReviewsResponse> {
        let mut params = vec![];
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(o) = offset {
            params.push(format!("offset={}", o));
        }
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        self.get(&format!("/listings/{}/reviews{}", listing_id, query)).await
    }

    // ========================================================================
    // Payment Endpoints
    // ========================================================================

    /// Get shop payments
    pub async fn get_shop_payment_account_ledger_entries(
        &self,
        shop_id: i64,
        min_created: Option<i64>,
        max_created: Option<i64>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<LedgerEntriesResponse> {
        let mut params = vec![];
        if let Some(v) = min_created {
            params.push(format!("min_created={}", v));
        }
        if let Some(v) = max_created {
            params.push(format!("max_created={}", v));
        }
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(o) = offset {
            params.push(format!("offset={}", o));
        }
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        self.get(&format!(
            "/shops/{}/payment-account/ledger-entries{}",
            shop_id, query
        ))
        .await
    }

    /// Get payments by receipt
    pub async fn get_shop_receipt_transactions_by_receipt(
        &self,
        shop_id: i64,
        receipt_id: i64,
    ) -> Result<PaymentsResponse> {
        self.get(&format!(
            "/shops/{}/receipts/{}/payments",
            shop_id, receipt_id
        ))
        .await
    }
}

