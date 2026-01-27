mod etsy;
mod printful;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use etsy::EtsyClient;
use printful::PrintfulClient;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "atomplatform")]
#[command(about = "CLI tool for testing API connections to various services")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Printful API commands
    Printful {
        #[command(subcommand)]
        command: PrintfulCommands,
    },
    /// Etsy API commands
    Etsy {
        #[command(subcommand)]
        command: EtsyCommands,
    },
}

#[derive(Subcommand)]
enum PrintfulCommands {
    /// List all products
    Products {
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<u32>,
        /// Offset for pagination
        #[arg(short, long)]
        offset: Option<u32>,
    },
    /// Get product details by ID
    Product {
        /// Product ID
        id: u64,
    },
    /// List all orders
    Orders {
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<u32>,
        /// Offset for pagination
        #[arg(short, long)]
        offset: Option<u32>,
    },
    /// Get order details by ID
    Order {
        /// Order ID
        id: u64,
    },
    /// Get shipping rates for an order
    ShippingRates {
        /// Variant ID
        #[arg(long)]
        variant_id: u64,
        /// Quantity
        #[arg(long)]
        quantity: u32,
        /// Recipient name
        #[arg(long)]
        name: String,
        /// Address line 1
        #[arg(long)]
        address1: String,
        /// City
        #[arg(long)]
        city: String,
        /// Country code (e.g., US, GB)
        #[arg(long)]
        country_code: String,
        /// ZIP/Postal code
        #[arg(long)]
        zip: String,
        /// State code (optional)
        #[arg(long)]
        state_code: Option<String>,
        /// Address line 2 (optional)
        #[arg(long)]
        address2: Option<String>,
        /// Phone number (optional)
        #[arg(long)]
        phone: Option<String>,
        /// Email (optional)
        #[arg(long)]
        email: Option<String>,
    },
}

#[derive(Subcommand)]
enum EtsyCommands {
    /// Test API connectivity (ping)
    Ping,

    /// Get OAuth authorization URL
    AuthUrl {
        /// Redirect URI for OAuth callback
        #[arg(long)]
        redirect_uri: String,
        /// OAuth scopes (space-separated)
        #[arg(long, value_delimiter = ' ', num_args = 1..)]
        scopes: Vec<String>,
        /// State parameter for CSRF protection
        #[arg(long, default_value = "state123")]
        state: String,
        /// Code challenge for PKCE (base64url encoded SHA256 of verifier)
        #[arg(long)]
        code_challenge: String,
    },

    /// Exchange authorization code for access token
    ExchangeToken {
        /// Redirect URI used in authorization
        #[arg(long)]
        redirect_uri: String,
        /// Authorization code from OAuth callback
        #[arg(long)]
        code: String,
        /// Code verifier for PKCE
        #[arg(long)]
        code_verifier: String,
    },

    /// Refresh access token
    RefreshToken {
        /// Refresh token from previous authentication
        #[arg(long)]
        refresh_token: String,
    },

    /// Get authenticated user info
    Me,

    /// Get user by ID
    User {
        /// User ID
        user_id: i64,
    },

    // ========================================================================
    // Shop Commands
    // ========================================================================
    /// Get shop by ID
    Shop {
        /// Shop ID
        shop_id: i64,
    },

    /// Get shop by owner user ID
    ShopByUser {
        /// User ID
        user_id: i64,
    },

    /// Find shops by name
    FindShops {
        /// Shop name to search
        #[arg(long)]
        name: String,
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<i32>,
        /// Offset for pagination
        #[arg(short, long)]
        offset: Option<i32>,
    },

    /// Get shop sections
    ShopSections {
        /// Shop ID
        shop_id: i64,
    },

    // ========================================================================
    // Listing Commands
    // ========================================================================
    /// Get listing by ID
    Listing {
        /// Listing ID
        listing_id: i64,
        /// Include additional resources (e.g., images,shop,user)
        #[arg(long, value_delimiter = ',')]
        includes: Option<Vec<String>>,
    },

    /// Get listings by shop
    Listings {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Listing state (active, inactive, draft, expired, sold_out)
        #[arg(long)]
        state: Option<String>,
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<i32>,
        /// Offset for pagination
        #[arg(short, long)]
        offset: Option<i32>,
        /// Sort field (created, updated, price, score)
        #[arg(long)]
        sort_on: Option<String>,
        /// Sort order (asc, desc)
        #[arg(long)]
        sort_order: Option<String>,
        /// Include additional resources
        #[arg(long, value_delimiter = ',')]
        includes: Option<Vec<String>>,
    },

    /// Get active listings by shop
    ActiveListings {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<i32>,
        /// Offset for pagination
        #[arg(short, long)]
        offset: Option<i32>,
        /// Search keywords
        #[arg(long)]
        keywords: Option<String>,
    },

    /// Get featured listings by shop
    FeaturedListings {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<i32>,
        /// Offset for pagination
        #[arg(short, long)]
        offset: Option<i32>,
    },

    /// Create a draft listing
    CreateListing {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Listing title
        #[arg(long)]
        title: String,
        /// Listing description
        #[arg(long)]
        description: String,
        /// Price in shop currency
        #[arg(long)]
        price: f64,
        /// Quantity available
        #[arg(long)]
        quantity: i32,
        /// Who made it (i_did, someone_else, collective)
        #[arg(long)]
        who_made: String,
        /// When was it made (made_to_order, 2020_2024, 2010_2019, etc.)
        #[arg(long)]
        when_made: String,
        /// Taxonomy ID for categorization
        #[arg(long)]
        taxonomy_id: i64,
        /// Shipping profile ID
        #[arg(long)]
        shipping_profile_id: Option<i64>,
        /// Return policy ID
        #[arg(long)]
        return_policy_id: Option<i64>,
        /// Tags (comma-separated)
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
        /// Materials (comma-separated)
        #[arg(long, value_delimiter = ',')]
        materials: Option<Vec<String>>,
        /// Listing type (physical, download)
        #[arg(long)]
        listing_type: Option<String>,
    },

    /// Update a listing
    UpdateListing {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Listing ID
        #[arg(long)]
        listing_id: i64,
        /// Listing title
        #[arg(long)]
        title: Option<String>,
        /// Listing description
        #[arg(long)]
        description: Option<String>,
        /// Price in shop currency
        #[arg(long)]
        price: Option<f64>,
        /// Quantity available
        #[arg(long)]
        quantity: Option<i32>,
        /// Listing state (active, inactive, draft)
        #[arg(long)]
        state: Option<String>,
        /// Tags (comma-separated)
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },

    /// Delete a listing
    DeleteListing {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Listing ID
        #[arg(long)]
        listing_id: i64,
    },

    // ========================================================================
    // Listing Images Commands
    // ========================================================================
    /// Get listing images
    ListingImages {
        /// Listing ID
        listing_id: i64,
    },

    /// Get a specific listing image
    ListingImage {
        /// Listing ID
        #[arg(long)]
        listing_id: i64,
        /// Image ID
        #[arg(long)]
        image_id: i64,
    },

    /// Delete a listing image
    DeleteListingImage {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Listing ID
        #[arg(long)]
        listing_id: i64,
        /// Image ID
        #[arg(long)]
        image_id: i64,
    },

    // ========================================================================
    // Listing Inventory Commands
    // ========================================================================
    /// Get listing inventory
    ListingInventory {
        /// Listing ID
        listing_id: i64,
    },

    /// Get listing product
    ListingProduct {
        /// Listing ID
        #[arg(long)]
        listing_id: i64,
        /// Product ID
        #[arg(long)]
        product_id: i64,
    },

    // ========================================================================
    // Receipt/Order Commands
    // ========================================================================
    /// Get shop receipts (orders)
    Receipts {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<i32>,
        /// Offset for pagination
        #[arg(short, long)]
        offset: Option<i32>,
        /// Sort field (created, updated)
        #[arg(long)]
        sort_on: Option<String>,
        /// Sort order (asc, desc)
        #[arg(long)]
        sort_order: Option<String>,
        /// Filter: was paid
        #[arg(long)]
        was_paid: Option<bool>,
        /// Filter: was shipped
        #[arg(long)]
        was_shipped: Option<bool>,
        /// Filter: was delivered
        #[arg(long)]
        was_delivered: Option<bool>,
        /// Minimum created timestamp
        #[arg(long)]
        min_created: Option<i64>,
        /// Maximum created timestamp
        #[arg(long)]
        max_created: Option<i64>,
    },

    /// Get a specific receipt
    Receipt {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Receipt ID
        #[arg(long)]
        receipt_id: i64,
    },

    /// Create shipment tracking for a receipt
    CreateShipment {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Receipt ID
        #[arg(long)]
        receipt_id: i64,
        /// Tracking code
        #[arg(long)]
        tracking_code: Option<String>,
        /// Carrier name
        #[arg(long)]
        carrier_name: Option<String>,
        /// Send BCC email to seller
        #[arg(long)]
        send_bcc: Option<bool>,
        /// Note to buyer
        #[arg(long)]
        note_to_buyer: Option<String>,
    },

    // ========================================================================
    // Transaction Commands
    // ========================================================================
    /// Get shop transactions
    Transactions {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<i32>,
        /// Offset for pagination
        #[arg(short, long)]
        offset: Option<i32>,
    },

    /// Get a specific transaction
    Transaction {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Transaction ID
        #[arg(long)]
        transaction_id: i64,
    },

    /// Get transactions for a receipt
    ReceiptTransactions {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Receipt ID
        #[arg(long)]
        receipt_id: i64,
    },

    // ========================================================================
    // Shipping Profile Commands
    // ========================================================================
    /// Get shop shipping profiles
    ShippingProfiles {
        /// Shop ID
        shop_id: i64,
    },

    /// Get a specific shipping profile
    ShippingProfile {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Shipping Profile ID
        #[arg(long)]
        shipping_profile_id: i64,
    },

    /// Create a shipping profile
    CreateShippingProfile {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Profile title
        #[arg(long)]
        title: String,
        /// Origin country ISO code
        #[arg(long)]
        origin_country_iso: String,
        /// Primary cost (first item)
        #[arg(long)]
        primary_cost: f64,
        /// Secondary cost (additional items)
        #[arg(long)]
        secondary_cost: f64,
        /// Minimum processing time in days
        #[arg(long)]
        min_processing_time: i32,
        /// Maximum processing time in days
        #[arg(long)]
        max_processing_time: i32,
        /// Origin postal code
        #[arg(long)]
        origin_postal_code: Option<String>,
    },

    /// Delete a shipping profile
    DeleteShippingProfile {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Shipping Profile ID
        #[arg(long)]
        shipping_profile_id: i64,
    },

    // ========================================================================
    // Return Policy Commands
    // ========================================================================
    /// Get shop return policies
    ReturnPolicies {
        /// Shop ID
        shop_id: i64,
    },

    /// Get a specific return policy
    ReturnPolicy {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Return Policy ID
        #[arg(long)]
        return_policy_id: i64,
    },

    // ========================================================================
    // Taxonomy Commands
    // ========================================================================
    /// Get seller taxonomy nodes
    TaxonomyNodes,

    /// Get taxonomy node properties
    TaxonomyProperties {
        /// Taxonomy ID
        taxonomy_id: i64,
    },

    // ========================================================================
    // Review Commands
    // ========================================================================
    /// Get shop reviews
    ShopReviews {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<i32>,
        /// Offset for pagination
        #[arg(short, long)]
        offset: Option<i32>,
        /// Minimum created timestamp
        #[arg(long)]
        min_created: Option<i64>,
        /// Maximum created timestamp
        #[arg(long)]
        max_created: Option<i64>,
    },

    /// Get listing reviews
    ListingReviews {
        /// Listing ID
        #[arg(long)]
        listing_id: i64,
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<i32>,
        /// Offset for pagination
        #[arg(short, long)]
        offset: Option<i32>,
    },

    // ========================================================================
    // Payment/Ledger Commands
    // ========================================================================
    /// Get shop ledger entries
    LedgerEntries {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<i32>,
        /// Offset for pagination
        #[arg(short, long)]
        offset: Option<i32>,
        /// Minimum created timestamp
        #[arg(long)]
        min_created: Option<i64>,
        /// Maximum created timestamp
        #[arg(long)]
        max_created: Option<i64>,
    },

    /// Get receipt payments
    ReceiptPayments {
        /// Shop ID
        #[arg(long)]
        shop_id: i64,
        /// Receipt ID
        #[arg(long)]
        receipt_id: i64,
    },

    // ========================================================================
    // Raw/Generic API Commands
    // ========================================================================
    /// Make a raw GET request to any Etsy API endpoint
    Get {
        /// API path (e.g., shops/12345/listings)
        path: String,
        /// Query parameters (key=value format)
        #[arg(long, value_delimiter = ',')]
        params: Option<Vec<String>>,
    },

    /// Make a raw POST request to any Etsy API endpoint
    Post {
        /// API path (e.g., shops/12345/listings)
        path: String,
        /// JSON body
        #[arg(long)]
        body: String,
    },

    /// Make a raw PUT request to any Etsy API endpoint
    Put {
        /// API path (e.g., listings/12345/inventory)
        path: String,
        /// JSON body
        #[arg(long)]
        body: String,
    },

    /// Make a raw DELETE request to any Etsy API endpoint
    Delete {
        /// API path (e.g., shops/12345/listings/67890)
        path: String,
    },
}

fn get_printful_api_key() -> Result<String> {
    std::env::var("PRINTFUL_API_KEY")
        .context("PRINTFUL_API_KEY not found. Please create a .env file in the project root with PRINTFUL_API_KEY=your_key")
}

fn get_etsy_api_key() -> Result<String> {
    std::env::var("ETSY_API_KEY")
        .context("ETSY_API_KEY not found. Please add ETSY_API_KEY=your_key to the .env file")
}

fn get_etsy_access_token() -> Option<String> {
    std::env::var("ETSY_ACCESS_TOKEN").ok()
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env from project root (parent directory of cli/)
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        if let Some(parent_dir) = std::path::Path::new(&manifest_dir).parent() {
            let env_path = parent_dir.join(".env");
            dotenv::from_path(&env_path).ok();
        }
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Printful { command } => {
            let api_key = get_printful_api_key()?;
            let client = PrintfulClient::new(api_key);

            match command {
                PrintfulCommands::Products { limit, offset } => {
                    let response = client.get_products(limit, offset).await?;
                    println!("Found {} products:", response.result.len());
                    for product in response.result {
                        println!("\nProduct ID: {}", product.id);
                        if let Some(name) = &product.name {
                            println!("  Name: {}", name);
                        }
                        if let Some(product_type) = &product.product_type {
                            println!("  Type: {}", product_type);
                        }
                        if let Some(description) = &product.description {
                            let char_count = description.chars().count();
                            let short_desc: String = description
                                .chars()
                                .take(100)
                                .collect();
                            if char_count > 100 {
                                println!("  Description: {}...", short_desc);
                            } else {
                                println!("  Description: {}", short_desc);
                            }
                        }
                        if let Some(brand) = &product.brand {
                            println!("  Brand: {}", brand);
                        }
                        if let Some(variant_count) = product.variant_count {
                            println!("  Variants: {}", variant_count);
                        }
                    }
                    if let Some(paging) = response.paging {
                        println!("\nTotal: {}, Offset: {}, Limit: {}", paging.total, paging.offset, paging.limit);
                    }
                }
                PrintfulCommands::Product { id } => {
                    let response = client.get_product(id).await?;
                    let product = response.result;
                    println!("Product Details:");
                    println!("  ID: {}", product.id);
                    if let Some(name) = &product.name {
                        println!("  Name: {}", name);
                    }
                    if let Some(product_type) = &product.product_type {
                        println!("  Type: {}", product_type);
                    }
                    if let Some(description) = &product.description {
                        println!("  Description: {}", description);
                    }
                    if let Some(brand) = &product.brand {
                        println!("  Brand: {}", brand);
                    }
                    if let Some(model) = &product.model {
                        println!("  Model: {}", model);
                    }
                    if let Some(image) = &product.image {
                        println!("  Image: {}", image);
                    }
                    if let Some(variant_count) = product.variant_count {
                        println!("  Variants: {}", variant_count);
                    }
                }
                PrintfulCommands::Orders { limit, offset } => {
                    let response = client.get_orders(limit, offset).await?;
                    println!("Found {} orders:", response.result.len());
                    for order in response.result {
                        println!("\nOrder ID: {}", order.id);
                        println!("  Status: {}", order.status);
                        println!("  Created: {}", order.created);
                        if let Some(external_id) = order.external_id {
                            println!("  External ID: {}", external_id);
                        }
                        if let Some(costs) = order.costs {
                            println!("  Total: {}", costs.total);
                        }
                    }
                    if let Some(paging) = response.paging {
                        println!("\nTotal: {}, Offset: {}, Limit: {}", paging.total, paging.offset, paging.limit);
                    }
                }
                PrintfulCommands::Order { id } => {
                    let response = client.get_order(id).await?;
                    let order = response.result;
                    println!("Order Details:");
                    println!("  ID: {}", order.id);
                    println!("  Status: {}", order.status);
                    println!("  Created: {}", order.created);
                    println!("  Updated: {}", order.updated);
                    if let Some(external_id) = order.external_id {
                        println!("  External ID: {}", external_id);
                    }
                    if let Some(recipient) = order.recipient {
                        println!("  Recipient: {}", recipient.name);
                        println!("  Address: {}, {}, {} {}", recipient.address1, recipient.city, recipient.state_code.as_deref().unwrap_or(""), recipient.zip);
                        println!("  Country: {}", recipient.country_code);
                    }
                    if let Some(costs) = order.costs {
                        println!("  Costs:");
                        println!("    Subtotal: {}", costs.subtotal);
                        println!("    Shipping: {}", costs.shipping);
                        println!("    Tax: {}", costs.tax);
                        println!("    Total: {}", costs.total);
                    }
                    if let Some(items) = order.items {
                        println!("  Items: {}", items.len());
                        for item in items {
                            println!("    - Variant ID: {}, Quantity: {}", item.variant_id, item.quantity);
                        }
                    }
                }
                PrintfulCommands::ShippingRates {
                    variant_id,
                    quantity,
                    name,
                    address1,
                    city,
                    country_code,
                    zip,
                    state_code,
                    address2,
                    phone,
                    email,
                } => {
                    let request = printful::ShippingRateRequest {
                        recipient: printful::Recipient {
                            name,
                            company: None,
                            address1,
                            address2,
                            city,
                            state_code: state_code.clone(),
                            state_name: None,
                            country_code,
                            country_name: None,
                            zip,
                            phone,
                            email,
                        },
                        items: vec![printful::ShippingRateItem {
                            variant_id,
                            quantity,
                        }],
                    };

                    let response = client.get_shipping_rates(request).await?;
                    println!("Available Shipping Rates:");
                    for rate in response.result {
                        println!("\n  ID: {}", rate.id);
                        println!("  Name: {}", rate.name);
                        println!("  Rate: {} {}", rate.rate, rate.currency);
                        println!("  Delivery: {} - {} days", rate.min_days, rate.max_days);
                    }
                }
            }
        }
        Commands::Etsy { command } => {
            handle_etsy_command(command).await?;
        }
    }

    Ok(())
}

async fn handle_etsy_command(command: EtsyCommands) -> Result<()> {
    let api_key = get_etsy_api_key()?;
    let access_token = get_etsy_access_token();
    let client = EtsyClient::new(api_key.clone(), access_token);

    match command {
        // ====================================================================
        // Auth Commands
        // ====================================================================
        EtsyCommands::Ping => {
            let response = client.ping().await?;
            println!("Ping successful!");
            println!("  Application ID: {}", response.application_id);
        }

        EtsyCommands::AuthUrl {
            redirect_uri,
            scopes,
            state,
            code_challenge,
        } => {
            let scope_refs: Vec<&str> = scopes.iter().map(|s| s.as_str()).collect();
            let url = EtsyClient::get_oauth_url(
                &api_key,
                &redirect_uri,
                &scope_refs,
                &state,
                &code_challenge,
            );
            println!("Visit this URL to authorize your application:");
            println!("\n{}\n", url);
            println!("After authorizing, you'll be redirected to your callback URL with 'code' and 'state' parameters.");
        }

        EtsyCommands::ExchangeToken {
            redirect_uri,
            code,
            code_verifier,
        } => {
            let response = EtsyClient::exchange_code(&api_key, &redirect_uri, &code, &code_verifier).await?;
            println!("Token exchange successful!");
            println!("\nAccess Token: {}", response.access_token);
            println!("Refresh Token: {}", response.refresh_token);
            println!("Expires In: {} seconds", response.expires_in);
            println!("\nAdd to your .env file:");
            println!("ETSY_ACCESS_TOKEN={}", response.access_token);
            println!("ETSY_REFRESH_TOKEN={}", response.refresh_token);
        }

        EtsyCommands::RefreshToken { refresh_token } => {
            let response = EtsyClient::refresh_token(&api_key, &refresh_token).await?;
            println!("Token refresh successful!");
            println!("\nAccess Token: {}", response.access_token);
            println!("Refresh Token: {}", response.refresh_token);
            println!("Expires In: {} seconds", response.expires_in);
            println!("\nUpdate your .env file:");
            println!("ETSY_ACCESS_TOKEN={}", response.access_token);
            println!("ETSY_REFRESH_TOKEN={}", response.refresh_token);
        }

        // ====================================================================
        // User Commands
        // ====================================================================
        EtsyCommands::Me => {
            let user = client.get_me().await?;
            println!("Authenticated User:");
            println!("  User ID: {}", user.user_id);
            if let Some(email) = user.primary_email {
                println!("  Email: {}", email);
            }
            if let Some(first) = user.first_name {
                println!("  First Name: {}", first);
            }
            if let Some(last) = user.last_name {
                println!("  Last Name: {}", last);
            }
        }

        EtsyCommands::User { user_id } => {
            let user = client.get_user(user_id).await?;
            println!("User {}:", user.user_id);
            if let Some(first) = user.first_name {
                println!("  First Name: {}", first);
            }
            if let Some(last) = user.last_name {
                println!("  Last Name: {}", last);
            }
        }

        // ====================================================================
        // Shop Commands
        // ====================================================================
        EtsyCommands::Shop { shop_id } => {
            let shop = client.get_shop(shop_id).await?;
            print_shop(&shop);
        }

        EtsyCommands::ShopByUser { user_id } => {
            let shop = client.get_shop_by_owner_user_id(user_id).await?;
            print_shop(&shop);
        }

        EtsyCommands::FindShops { name, limit, offset } => {
            let response = client.find_shops(&name, limit, offset).await?;
            println!("{}", serde_json::to_string_pretty(&response)?);
        }

        EtsyCommands::ShopSections { shop_id } => {
            let response = client.get_shop_sections(shop_id).await?;
            println!("Found {} sections:", response.count);
            for section in response.results {
                println!("\n  Section ID: {}", section.shop_section_id);
                if let Some(title) = section.title {
                    println!("  Title: {}", title);
                }
                if let Some(rank) = section.rank {
                    println!("  Rank: {}", rank);
                }
                if let Some(count) = section.active_listing_count {
                    println!("  Active Listings: {}", count);
                }
            }
        }

        // ====================================================================
        // Listing Commands
        // ====================================================================
        EtsyCommands::Listing { listing_id, includes } => {
            let inc_refs: Option<Vec<&str>> = includes.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect());
            let listing = client.get_listing(listing_id, inc_refs.as_deref()).await?;
            print_listing(&listing);
        }

        EtsyCommands::Listings {
            shop_id,
            state,
            limit,
            offset,
            sort_on,
            sort_order,
            includes,
        } => {
            let inc_refs: Option<Vec<&str>> = includes.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect());
            let response = client
                .get_listings_by_shop(
                    shop_id,
                    state.as_deref(),
                    limit,
                    offset,
                    sort_on.as_deref(),
                    sort_order.as_deref(),
                    inc_refs.as_deref(),
                )
                .await?;
            println!("Found {} listings:", response.count);
            for listing in response.results {
                print_listing_summary(&listing);
            }
        }

        EtsyCommands::ActiveListings {
            shop_id,
            limit,
            offset,
            keywords,
        } => {
            let response = client
                .get_active_listings_by_shop(shop_id, limit, offset, keywords.as_deref())
                .await?;
            println!("Found {} active listings:", response.count);
            for listing in response.results {
                print_listing_summary(&listing);
            }
        }

        EtsyCommands::FeaturedListings {
            shop_id,
            limit,
            offset,
        } => {
            let response = client.get_featured_listings_by_shop(shop_id, limit, offset).await?;
            println!("Found {} featured listings:", response.count);
            for listing in response.results {
                print_listing_summary(&listing);
            }
        }

        EtsyCommands::CreateListing {
            shop_id,
            title,
            description,
            price,
            quantity,
            who_made,
            when_made,
            taxonomy_id,
            shipping_profile_id,
            return_policy_id,
            tags,
            materials,
            listing_type,
        } => {
            let request = etsy::CreateDraftListingRequest {
                quantity,
                title,
                description,
                price,
                who_made,
                when_made,
                taxonomy_id,
                shipping_profile_id,
                return_policy_id,
                materials,
                shop_section_id: None,
                processing_min: None,
                processing_max: None,
                tags,
                styles: None,
                item_weight: None,
                item_length: None,
                item_width: None,
                item_height: None,
                item_weight_unit: None,
                item_dimensions_unit: None,
                is_personalizable: None,
                personalization_is_required: None,
                personalization_char_count_max: None,
                personalization_instructions: None,
                is_supply: None,
                is_customizable: None,
                should_auto_renew: None,
                is_taxable: None,
                r#type: listing_type,
            };
            let listing = client.create_draft_listing(shop_id, &request).await?;
            println!("Listing created successfully!");
            print_listing(&listing);
        }

        EtsyCommands::UpdateListing {
            shop_id,
            listing_id,
            title,
            description,
            price,
            quantity,
            state,
            tags,
        } => {
            let request = etsy::UpdateListingRequest {
                quantity,
                title,
                description,
                price,
                who_made: None,
                when_made: None,
                taxonomy_id: None,
                shipping_profile_id: None,
                return_policy_id: None,
                materials: None,
                shop_section_id: None,
                processing_min: None,
                processing_max: None,
                tags,
                styles: None,
                item_weight: None,
                item_length: None,
                item_width: None,
                item_height: None,
                item_weight_unit: None,
                item_dimensions_unit: None,
                is_personalizable: None,
                personalization_is_required: None,
                personalization_char_count_max: None,
                personalization_instructions: None,
                state,
                is_supply: None,
                is_customizable: None,
                should_auto_renew: None,
                is_taxable: None,
                r#type: None,
            };
            let listing = client.update_listing(shop_id, listing_id, &request).await?;
            println!("Listing updated successfully!");
            print_listing(&listing);
        }

        EtsyCommands::DeleteListing { shop_id, listing_id } => {
            client.delete_listing(shop_id, listing_id).await?;
            println!("Listing {} deleted successfully!", listing_id);
        }

        // ====================================================================
        // Listing Images Commands
        // ====================================================================
        EtsyCommands::ListingImages { listing_id } => {
            let response = client.get_listing_images(listing_id).await?;
            println!("Found {} images:", response.count);
            for image in response.results {
                println!("\n  Image ID: {}", image.listing_image_id);
                if let Some(rank) = image.rank {
                    println!("  Rank: {}", rank);
                }
                if let Some(url) = image.url_570x_n {
                    println!("  URL (570xN): {}", url);
                }
                if let Some(alt) = image.alt_text {
                    println!("  Alt Text: {}", alt);
                }
            }
        }

        EtsyCommands::ListingImage { listing_id, image_id } => {
            let image = client.get_listing_image(listing_id, image_id).await?;
            println!("Image {}:", image.listing_image_id);
            println!("  Listing ID: {}", image.listing_id);
            if let Some(url) = image.url_fullxfull {
                println!("  Full URL: {}", url);
            }
            if let Some(w) = image.full_width {
                println!("  Width: {}", w);
            }
            if let Some(h) = image.full_height {
                println!("  Height: {}", h);
            }
        }

        EtsyCommands::DeleteListingImage {
            shop_id,
            listing_id,
            image_id,
        } => {
            client.delete_listing_image(shop_id, listing_id, image_id).await?;
            println!("Image {} deleted successfully!", image_id);
        }

        // ====================================================================
        // Listing Inventory Commands
        // ====================================================================
        EtsyCommands::ListingInventory { listing_id } => {
            let inventory = client.get_listing_inventory(listing_id).await?;
            println!("Inventory for listing {}:", listing_id);
            println!("  Products: {}", inventory.products.len());
            for product in inventory.products {
                println!("\n  Product ID: {}", product.product_id);
                if let Some(sku) = product.sku {
                    println!("    SKU: {}", sku);
                }
                if let Some(offerings) = product.offerings {
                    for offering in offerings {
                        println!("    Offering ID: {}", offering.offering_id);
                        if let Some(qty) = offering.quantity {
                            println!("      Quantity: {}", qty);
                        }
                        if let Some(price) = offering.price {
                            println!("      Price: {}.{} {}", price.amount / price.divisor as i64, price.amount % price.divisor as i64, price.currency_code);
                        }
                    }
                }
            }
        }

        EtsyCommands::ListingProduct {
            listing_id,
            product_id,
        } => {
            let product = client.get_listing_product(listing_id, product_id).await?;
            println!("Product {}:", product.product_id);
            if let Some(sku) = product.sku {
                println!("  SKU: {}", sku);
            }
            if let Some(offerings) = product.offerings {
                for offering in offerings {
                    println!("\n  Offering ID: {}", offering.offering_id);
                    if let Some(qty) = offering.quantity {
                        println!("    Quantity: {}", qty);
                    }
                    if let Some(enabled) = offering.is_enabled {
                        println!("    Enabled: {}", enabled);
                    }
                }
            }
        }

        // ====================================================================
        // Receipt Commands
        // ====================================================================
        EtsyCommands::Receipts {
            shop_id,
            limit,
            offset,
            sort_on,
            sort_order,
            was_paid,
            was_shipped,
            was_delivered,
            min_created,
            max_created,
        } => {
            let response = client
                .get_shop_receipts(
                    shop_id,
                    min_created,
                    max_created,
                    None,
                    None,
                    limit,
                    offset,
                    sort_on.as_deref(),
                    sort_order.as_deref(),
                    was_paid,
                    was_shipped,
                    was_delivered,
                )
                .await?;
            println!("Found {} receipts:", response.count);
            for receipt in response.results {
                print_receipt_summary(&receipt);
            }
        }

        EtsyCommands::Receipt { shop_id, receipt_id } => {
            let receipt = client.get_shop_receipt(shop_id, receipt_id).await?;
            print_receipt(&receipt);
        }

        EtsyCommands::CreateShipment {
            shop_id,
            receipt_id,
            tracking_code,
            carrier_name,
            send_bcc,
            note_to_buyer,
        } => {
            let request = etsy::CreateReceiptShipmentRequest {
                tracking_code,
                carrier_name,
                send_bcc,
                note_to_buyer,
            };
            let receipt = client.create_receipt_shipment(shop_id, receipt_id, &request).await?;
            println!("Shipment created successfully!");
            print_receipt(&receipt);
        }

        // ====================================================================
        // Transaction Commands
        // ====================================================================
        EtsyCommands::Transactions {
            shop_id,
            limit,
            offset,
        } => {
            let response = client.get_shop_transactions(shop_id, limit, offset).await?;
            println!("Found {} transactions:", response.count);
            for txn in response.results {
                print_transaction_summary(&txn);
            }
        }

        EtsyCommands::Transaction {
            shop_id,
            transaction_id,
        } => {
            let txn = client.get_shop_receipt_transaction(shop_id, transaction_id).await?;
            print_transaction(&txn);
        }

        EtsyCommands::ReceiptTransactions { shop_id, receipt_id } => {
            let response = client.get_shop_receipt_transactions(shop_id, receipt_id).await?;
            println!("Found {} transactions for receipt {}:", response.count, receipt_id);
            for txn in response.results {
                print_transaction_summary(&txn);
            }
        }

        // ====================================================================
        // Shipping Profile Commands
        // ====================================================================
        EtsyCommands::ShippingProfiles { shop_id } => {
            let response = client.get_shop_shipping_profiles(shop_id).await?;
            println!("Found {} shipping profiles:", response.count);
            for profile in response.results {
                println!("\n  Profile ID: {}", profile.shipping_profile_id);
                if let Some(title) = profile.title {
                    println!("  Title: {}", title);
                }
                if let Some(origin) = profile.origin_country_iso {
                    println!("  Origin: {}", origin);
                }
                if let Some(min) = profile.min_processing_days {
                    if let Some(max) = profile.max_processing_days {
                        println!("  Processing: {} - {} days", min, max);
                    }
                }
            }
        }

        EtsyCommands::ShippingProfile {
            shop_id,
            shipping_profile_id,
        } => {
            let profile = client.get_shop_shipping_profile(shop_id, shipping_profile_id).await?;
            println!("Shipping Profile {}:", profile.shipping_profile_id);
            if let Some(title) = profile.title {
                println!("  Title: {}", title);
            }
            if let Some(origin) = profile.origin_country_iso {
                println!("  Origin Country: {}", origin);
            }
            if let Some(postal) = profile.origin_postal_code {
                println!("  Origin Postal Code: {}", postal);
            }
            if let Some(min) = profile.min_processing_days {
                println!("  Min Processing Days: {}", min);
            }
            if let Some(max) = profile.max_processing_days {
                println!("  Max Processing Days: {}", max);
            }
        }

        EtsyCommands::CreateShippingProfile {
            shop_id,
            title,
            origin_country_iso,
            primary_cost,
            secondary_cost,
            min_processing_time,
            max_processing_time,
            origin_postal_code,
        } => {
            let request = etsy::CreateShippingProfileRequest {
                title,
                origin_country_iso,
                primary_cost,
                secondary_cost,
                min_processing_time,
                max_processing_time,
                origin_postal_code,
                destination_country_iso: None,
                destination_region: None,
                shipping_carrier_id: None,
                mail_class: None,
                min_delivery_days: None,
                max_delivery_days: None,
            };
            let profile = client.create_shop_shipping_profile(shop_id, &request).await?;
            println!("Shipping profile created successfully!");
            println!("  Profile ID: {}", profile.shipping_profile_id);
        }

        EtsyCommands::DeleteShippingProfile {
            shop_id,
            shipping_profile_id,
        } => {
            client.delete_shop_shipping_profile(shop_id, shipping_profile_id).await?;
            println!("Shipping profile {} deleted successfully!", shipping_profile_id);
        }

        // ====================================================================
        // Return Policy Commands
        // ====================================================================
        EtsyCommands::ReturnPolicies { shop_id } => {
            let response = client.get_shop_return_policies(shop_id).await?;
            println!("Found {} return policies:", response.count);
            for policy in response.results {
                println!("\n  Policy ID: {}", policy.return_policy_id);
                if let Some(accepts) = policy.accepts_returns {
                    println!("  Accepts Returns: {}", accepts);
                }
                if let Some(accepts) = policy.accepts_exchanges {
                    println!("  Accepts Exchanges: {}", accepts);
                }
                if let Some(deadline) = policy.return_deadline {
                    println!("  Return Deadline: {} days", deadline);
                }
            }
        }

        EtsyCommands::ReturnPolicy {
            shop_id,
            return_policy_id,
        } => {
            let policy = client.get_shop_return_policy(shop_id, return_policy_id).await?;
            println!("Return Policy {}:", policy.return_policy_id);
            if let Some(accepts) = policy.accepts_returns {
                println!("  Accepts Returns: {}", accepts);
            }
            if let Some(accepts) = policy.accepts_exchanges {
                println!("  Accepts Exchanges: {}", accepts);
            }
            if let Some(deadline) = policy.return_deadline {
                println!("  Return Deadline: {} days", deadline);
            }
        }

        // ====================================================================
        // Taxonomy Commands
        // ====================================================================
        EtsyCommands::TaxonomyNodes => {
            let response = client.get_seller_taxonomy_nodes().await?;
            println!("Found {} taxonomy nodes:", response.count);
            for node in response.results {
                print_taxonomy_node(&node, 0);
            }
        }

        EtsyCommands::TaxonomyProperties { taxonomy_id } => {
            let response = client.get_properties_by_taxonomy_id(taxonomy_id).await?;
            println!("Found {} properties for taxonomy {}:", response.count, taxonomy_id);
            for prop in response.results {
                println!("\n  Property ID: {}", prop.property_id);
                if let Some(name) = prop.name {
                    println!("  Name: {}", name);
                }
                if let Some(display) = prop.display_name {
                    println!("  Display Name: {}", display);
                }
                if let Some(required) = prop.is_required {
                    println!("  Required: {}", required);
                }
            }
        }

        // ====================================================================
        // Review Commands
        // ====================================================================
        EtsyCommands::ShopReviews {
            shop_id,
            limit,
            offset,
            min_created,
            max_created,
        } => {
            let response = client
                .get_reviews_by_shop(shop_id, limit, offset, min_created, max_created)
                .await?;
            println!("Found {} reviews:", response.count);
            for review in response.results {
                println!("\n  Listing ID: {}", review.listing_id);
                if let Some(rating) = review.rating {
                    println!("  Rating: {}/5", rating);
                }
                if let Some(text) = review.review {
                    let short = if text.len() > 100 {
                        format!("{}...", &text[..100])
                    } else {
                        text
                    };
                    println!("  Review: {}", short);
                }
            }
        }

        EtsyCommands::ListingReviews {
            listing_id,
            limit,
            offset,
        } => {
            let response = client.get_reviews_by_listing(listing_id, limit, offset).await?;
            println!("Found {} reviews for listing {}:", response.count, listing_id);
            for review in response.results {
                if let Some(rating) = review.rating {
                    println!("\n  Rating: {}/5", rating);
                }
                if let Some(text) = review.review {
                    println!("  Review: {}", text);
                }
            }
        }

        // ====================================================================
        // Payment/Ledger Commands
        // ====================================================================
        EtsyCommands::LedgerEntries {
            shop_id,
            limit,
            offset,
            min_created,
            max_created,
        } => {
            let response = client
                .get_shop_payment_account_ledger_entries(shop_id, min_created, max_created, limit, offset)
                .await?;
            println!("Found {} ledger entries:", response.count);
            for entry in response.results {
                println!("\n  Entry ID: {}", entry.entry_id);
                if let Some(desc) = entry.description {
                    println!("  Description: {}", desc);
                }
                if let Some(amount) = entry.amount {
                    if let Some(currency) = &entry.currency {
                        println!("  Amount: {} {}", amount, currency);
                    }
                }
                if let Some(balance) = entry.balance {
                    println!("  Balance: {}", balance);
                }
            }
        }

        EtsyCommands::ReceiptPayments { shop_id, receipt_id } => {
            let response = client
                .get_shop_receipt_transactions_by_receipt(shop_id, receipt_id)
                .await?;
            println!("Found {} payments for receipt {}:", response.count, receipt_id);
            for payment in response.results {
                println!("\n  Payment ID: {}", payment.payment_id);
                if let Some(status) = payment.status {
                    println!("  Status: {}", status);
                }
                if let Some(amount) = payment.amount_gross {
                    println!("  Gross: {}.{} {}", amount.amount / amount.divisor as i64, amount.amount % amount.divisor as i64, amount.currency_code);
                }
            }
        }

        // ====================================================================
        // Raw API Commands
        // ====================================================================
        EtsyCommands::Get { path, params } => {
            let mut param_map = HashMap::new();
            if let Some(p) = params {
                for kv in p {
                    if let Some((k, v)) = kv.split_once('=') {
                        param_map.insert(k.to_string(), v.to_string());
                    }
                }
            }
            let response = client.raw_get(&path, &param_map).await?;
            println!("{}", serde_json::to_string_pretty(&response)?);
        }

        EtsyCommands::Post { path, body } => {
            let body_value: Value = serde_json::from_str(&body)
                .context("Invalid JSON body")?;
            let response = client.raw_post(&path, body_value).await?;
            println!("{}", serde_json::to_string_pretty(&response)?);
        }

        EtsyCommands::Put { path, body } => {
            let body_value: Value = serde_json::from_str(&body)
                .context("Invalid JSON body")?;
            let response = client.raw_put(&path, body_value).await?;
            println!("{}", serde_json::to_string_pretty(&response)?);
        }

        EtsyCommands::Delete { path } => {
            let response = client.raw_delete(&path).await?;
            if response.is_null() {
                println!("Deleted successfully!");
            } else {
                println!("{}", serde_json::to_string_pretty(&response)?);
            }
        }
    }

    Ok(())
}

// ============================================================================
// Print Helpers
// ============================================================================

fn print_shop(shop: &etsy::ShopResponse) {
    println!("Shop {}:", shop.shop_id);
    if let Some(name) = &shop.shop_name {
        println!("  Name: {}", name);
    }
    if let Some(title) = &shop.title {
        println!("  Title: {}", title);
    }
    if let Some(url) = &shop.url {
        println!("  URL: {}", url);
    }
    if let Some(currency) = &shop.currency_code {
        println!("  Currency: {}", currency);
    }
    if let Some(count) = shop.listing_active_count {
        println!("  Active Listings: {}", count);
    }
    if let Some(count) = shop.num_favorers {
        println!("  Favorers: {}", count);
    }
    if let Some(count) = shop.review_count {
        println!("  Reviews: {}", count);
    }
    if let Some(avg) = shop.review_average {
        println!("  Review Average: {:.2}", avg);
    }
    if let Some(count) = shop.transaction_sold_count {
        println!("  Sales: {}", count);
    }
    if let Some(vacation) = shop.is_vacation {
        println!("  On Vacation: {}", vacation);
    }
}

fn print_listing(listing: &etsy::Listing) {
    println!("Listing {}:", listing.listing_id);
    if let Some(title) = &listing.title {
        println!("  Title: {}", title);
    }
    if let Some(state) = &listing.state {
        println!("  State: {}", state);
    }
    if let Some(qty) = listing.quantity {
        println!("  Quantity: {}", qty);
    }
    if let Some(price) = &listing.price {
        println!("  Price: {}.{} {}", price.amount / price.divisor as i64, price.amount % price.divisor as i64, price.currency_code);
    }
    if let Some(url) = &listing.url {
        println!("  URL: {}", url);
    }
    if let Some(desc) = &listing.description {
        let short = if desc.len() > 200 {
            format!("{}...", &desc[..200])
        } else {
            desc.clone()
        };
        println!("  Description: {}", short);
    }
    if let Some(tags) = &listing.tags {
        println!("  Tags: {}", tags.join(", "));
    }
    if let Some(views) = listing.views {
        println!("  Views: {}", views);
    }
    if let Some(favs) = listing.num_favorers {
        println!("  Favorers: {}", favs);
    }
}

fn print_listing_summary(listing: &etsy::Listing) {
    print!("\n  ID: {}", listing.listing_id);
    if let Some(title) = &listing.title {
        print!(" - {}", title);
    }
    println!();
    if let Some(state) = &listing.state {
        println!("    State: {}", state);
    }
    if let Some(qty) = listing.quantity {
        println!("    Quantity: {}", qty);
    }
    if let Some(price) = &listing.price {
        println!("    Price: {}.{} {}", price.amount / price.divisor as i64, price.amount % price.divisor as i64, price.currency_code);
    }
}

fn print_receipt(receipt: &etsy::Receipt) {
    println!("Receipt {}:", receipt.receipt_id);
    if let Some(status) = &receipt.status {
        println!("  Status: {}", status);
    }
    if let Some(name) = &receipt.name {
        println!("  Buyer: {}", name);
    }
    if let Some(paid) = receipt.is_paid {
        println!("  Paid: {}", paid);
    }
    if let Some(shipped) = receipt.is_shipped {
        println!("  Shipped: {}", shipped);
    }
    if let Some(total) = &receipt.grandtotal {
        println!("  Total: {}.{} {}", total.amount / total.divisor as i64, total.amount % total.divisor as i64, total.currency_code);
    }
    if let Some(addr) = &receipt.formatted_address {
        println!("  Address: {}", addr);
    }
    if let Some(msg) = &receipt.message_from_buyer {
        if !msg.is_empty() {
            println!("  Buyer Message: {}", msg);
        }
    }
}

fn print_receipt_summary(receipt: &etsy::Receipt) {
    print!("\n  Receipt ID: {}", receipt.receipt_id);
    if let Some(status) = &receipt.status {
        print!(" - {}", status);
    }
    println!();
    if let Some(name) = &receipt.name {
        println!("    Buyer: {}", name);
    }
    if let Some(total) = &receipt.grandtotal {
        println!("    Total: {}.{} {}", total.amount / total.divisor as i64, total.amount % total.divisor as i64, total.currency_code);
    }
}

fn print_transaction(txn: &etsy::Transaction) {
    println!("Transaction {}:", txn.transaction_id);
    if let Some(title) = &txn.title {
        println!("  Title: {}", title);
    }
    if let Some(qty) = txn.quantity {
        println!("  Quantity: {}", qty);
    }
    if let Some(price) = &txn.price {
        println!("  Price: {}.{} {}", price.amount / price.divisor as i64, price.amount % price.divisor as i64, price.currency_code);
    }
    if let Some(sku) = &txn.sku {
        println!("  SKU: {}", sku);
    }
    if let Some(listing_id) = txn.listing_id {
        println!("  Listing ID: {}", listing_id);
    }
}

fn print_transaction_summary(txn: &etsy::Transaction) {
    print!("\n  Transaction ID: {}", txn.transaction_id);
    if let Some(title) = &txn.title {
        print!(" - {}", title);
    }
    println!();
    if let Some(qty) = txn.quantity {
        println!("    Quantity: {}", qty);
    }
    if let Some(price) = &txn.price {
        println!("    Price: {}.{} {}", price.amount / price.divisor as i64, price.amount % price.divisor as i64, price.currency_code);
    }
}

fn print_taxonomy_node(node: &etsy::TaxonomyNode, indent: usize) {
    let prefix = "  ".repeat(indent);
    println!("{}ID: {} - {:?}", prefix, node.id, node.name);
    if let Some(children) = &node.children {
        for child in children {
            print_taxonomy_node(child, indent + 1);
        }
    }
}
