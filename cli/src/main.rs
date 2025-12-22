mod printful;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use printful::PrintfulClient;

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

fn get_api_key() -> Result<String> {
    std::env::var("PRINTFUL_API_KEY")
        .context("PRINTFUL_API_KEY not found. Please create a .env file in the project root with PRINTFUL_API_KEY=your_key")
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
            let api_key = get_api_key()?;
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
    }

    Ok(())
}
