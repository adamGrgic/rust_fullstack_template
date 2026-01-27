# Atom Platform CLI

A CLI tool for testing API connections to various services, including Printful and Etsy.

## Setup

1. Install dependencies:
```bash
cargo build
```

2. Set up your API keys in a `.env` file in the project root (parent directory of `cli/`):
```
# Printful
PRINTFUL_API_KEY=your_printful_api_key_here

# Etsy
ETSY_API_KEY=your_etsy_api_key_here
ETSY_ACCESS_TOKEN=your_etsy_access_token_here
ETSY_REFRESH_TOKEN=your_etsy_refresh_token_here
```

The `.env` file should be located at: `atomplatform/.env`

## Etsy API Setup

### Getting Your API Key

1. Go to [Etsy Developer Portal](https://www.etsy.com/developers/your-apps)
2. Create a new app or use an existing one
3. Copy your **Keystring** (this is your `ETSY_API_KEY`)

### OAuth 2.0 Authentication

Etsy uses OAuth 2.0 with PKCE for authentication. Follow these steps:

#### Step 1: Generate PKCE Codes

Generate a code verifier (random 43-128 character string) and code challenge (base64url-encoded SHA256 of verifier):

```bash
# Generate code verifier (save this!)
CODE_VERIFIER=$(openssl rand -base64 32 | tr -d '=' | tr '/+' '_-')
echo "Code Verifier: $CODE_VERIFIER"

# Generate code challenge
CODE_CHALLENGE=$(echo -n "$CODE_VERIFIER" | openssl dgst -sha256 -binary | base64 | tr -d '=' | tr '/+' '_-')
echo "Code Challenge: $CODE_CHALLENGE"
```

#### Step 2: Get Authorization URL

```bash
cargo run -- etsy auth-url \
  --redirect-uri "https://your-app.com/callback" \
  --scopes "listings_r listings_w shops_r transactions_r" \
  --state "random_state_string" \
  --code-challenge "$CODE_CHALLENGE"
```

#### Step 3: Authorize and Get Code

Visit the URL and authorize your app. You'll be redirected to your callback URL with `code` and `state` parameters.

#### Step 4: Exchange Code for Tokens

```bash
cargo run -- etsy exchange-token \
  --redirect-uri "https://your-app.com/callback" \
  --code "your_authorization_code" \
  --code-verifier "$CODE_VERIFIER"
```

Save the returned tokens to your `.env` file.

#### Step 5: Refresh Tokens (when expired)

Access tokens expire after 1 hour. Refresh them with:

```bash
cargo run -- etsy refresh-token --refresh-token "your_refresh_token"
```

### Available Scopes

- `address_r` - Read shipping addresses
- `address_w` - Write shipping addresses
- `billing_r` - Read billing info
- `cart_r` / `cart_w` - Read/write carts
- `email_r` - Read email addresses
- `favorites_r` / `favorites_w` - Read/write favorites
- `feedback_r` - Read feedback
- `listings_d` - Delete listings
- `listings_r` / `listings_w` - Read/write listings
- `profile_r` / `profile_w` - Read/write profiles
- `recommend_r` / `recommend_w` - Read/write recommendations
- `shops_r` / `shops_w` - Read/write shops
- `transactions_r` / `transactions_w` - Read/write transactions

## Usage

### Etsy Commands

#### Test Connection
```bash
cargo run -- etsy ping
```

#### Get Authenticated User
```bash
cargo run -- etsy me
```

#### Shop Commands

Get shop by ID:
```bash
cargo run -- etsy shop 12345678
```

Get shop by user ID:
```bash
cargo run -- etsy shop-by-user 12345678
```

Find shops by name:
```bash
cargo run -- etsy find-shops --name "MyShop" --limit 10
```

Get shop sections:
```bash
cargo run -- etsy shop-sections 12345678
```

#### Listing Commands

Get listing by ID:
```bash
cargo run -- etsy listing 1234567890
```

Get listings by shop (with filters):
```bash
cargo run -- etsy listings --shop-id 12345678 --state active --limit 50 --offset 0
```

Get active listings:
```bash
cargo run -- etsy active-listings --shop-id 12345678 --limit 25 --keywords "vintage"
```

Get featured listings:
```bash
cargo run -- etsy featured-listings --shop-id 12345678
```

Create a draft listing:
```bash
cargo run -- etsy create-listing \
  --shop-id 12345678 \
  --title "Handmade Pottery Mug" \
  --description "Beautiful handcrafted ceramic mug..." \
  --price 29.99 \
  --quantity 10 \
  --who-made "i_did" \
  --when-made "2020_2024" \
  --taxonomy-id 1234 \
  --shipping-profile-id 5678 \
  --tags "pottery,mug,handmade,ceramic"
```

Update a listing:
```bash
cargo run -- etsy update-listing \
  --shop-id 12345678 \
  --listing-id 1234567890 \
  --title "Updated Title" \
  --price 34.99 \
  --state active
```

Delete a listing:
```bash
cargo run -- etsy delete-listing --shop-id 12345678 --listing-id 1234567890
```

#### Listing Images

Get listing images:
```bash
cargo run -- etsy listing-images 1234567890
```

Get specific image:
```bash
cargo run -- etsy listing-image --listing-id 1234567890 --image-id 9876543210
```

Delete image:
```bash
cargo run -- etsy delete-listing-image --shop-id 12345678 --listing-id 1234567890 --image-id 9876543210
```

#### Inventory Commands

Get listing inventory:
```bash
cargo run -- etsy listing-inventory 1234567890
```

Get listing product:
```bash
cargo run -- etsy listing-product --listing-id 1234567890 --product-id 9876543210
```

#### Receipt/Order Commands

Get shop receipts:
```bash
cargo run -- etsy receipts --shop-id 12345678 --limit 50 --was-paid true --was-shipped false
```

Get specific receipt:
```bash
cargo run -- etsy receipt --shop-id 12345678 --receipt-id 9876543210
```

Create shipment tracking:
```bash
cargo run -- etsy create-shipment \
  --shop-id 12345678 \
  --receipt-id 9876543210 \
  --tracking-code "1Z999AA10123456784" \
  --carrier-name "UPS"
```

#### Transaction Commands

Get shop transactions:
```bash
cargo run -- etsy transactions --shop-id 12345678 --limit 100
```

Get specific transaction:
```bash
cargo run -- etsy transaction --shop-id 12345678 --transaction-id 9876543210
```

Get transactions for a receipt:
```bash
cargo run -- etsy receipt-transactions --shop-id 12345678 --receipt-id 9876543210
```

#### Shipping Profile Commands

Get shop shipping profiles:
```bash
cargo run -- etsy shipping-profiles 12345678
```

Get specific profile:
```bash
cargo run -- etsy shipping-profile --shop-id 12345678 --shipping-profile-id 9876543210
```

Create shipping profile:
```bash
cargo run -- etsy create-shipping-profile \
  --shop-id 12345678 \
  --title "Standard Shipping" \
  --origin-country-iso "US" \
  --primary-cost 5.99 \
  --secondary-cost 2.99 \
  --min-processing-time 1 \
  --max-processing-time 3
```

Delete shipping profile:
```bash
cargo run -- etsy delete-shipping-profile --shop-id 12345678 --shipping-profile-id 9876543210
```

#### Return Policy Commands

Get return policies:
```bash
cargo run -- etsy return-policies 12345678
```

Get specific policy:
```bash
cargo run -- etsy return-policy --shop-id 12345678 --return-policy-id 9876543210
```

#### Taxonomy Commands

Get seller taxonomy nodes:
```bash
cargo run -- etsy taxonomy-nodes
```

Get taxonomy properties:
```bash
cargo run -- etsy taxonomy-properties 1234
```

#### Review Commands

Get shop reviews:
```bash
cargo run -- etsy shop-reviews --shop-id 12345678 --limit 50
```

Get listing reviews:
```bash
cargo run -- etsy listing-reviews --listing-id 1234567890 --limit 25
```

#### Payment/Ledger Commands

Get ledger entries:
```bash
cargo run -- etsy ledger-entries --shop-id 12345678 --limit 100
```

Get receipt payments:
```bash
cargo run -- etsy receipt-payments --shop-id 12345678 --receipt-id 9876543210
```

#### Raw API Commands

For any endpoint not covered by specific commands, use raw API calls:

GET request:
```bash
cargo run -- etsy get "shops/12345678/listings" --params "state=active,limit=10"
```

POST request:
```bash
cargo run -- etsy post "shops/12345678/listings" --body '{"title":"Test","description":"Test item","price":10.00}'
```

PUT request:
```bash
cargo run -- etsy put "listings/1234567890/inventory" --body '{"products":[...]}'
```

DELETE request:
```bash
cargo run -- etsy delete "shops/12345678/listings/1234567890"
```

---

### Printful Commands

#### List Products
```bash
cargo run -- printful products
```

With pagination:
```bash
cargo run -- printful products --limit 10 --offset 0
```

#### Get Product Details
```bash
cargo run -- printful product <product_id>
```

Example:
```bash
cargo run -- printful product 1
```

#### List Orders
```bash
cargo run -- printful orders
```

With pagination:
```bash
cargo run -- printful orders --limit 10 --offset 0
```

#### Get Order Details
```bash
cargo run -- printful order <order_id>
```

Example:
```bash
cargo run -- printful order 12345
```

#### Get Shipping Rates
```bash
cargo run -- printful shipping-rates \
  --variant-id <variant_id> \
  --quantity <quantity> \
  --name "John Doe" \
  --address1 "123 Main St" \
  --city "New York" \
  --country-code "US" \
  --zip "10001" \
  --state-code "NY" \
  --email "john@example.com"
```

Example:
```bash
cargo run -- printful shipping-rates \
  --variant-id 4011 \
  --quantity 1 \
  --name "John Doe" \
  --address1 "123 Main St" \
  --city "New York" \
  --country-code "US" \
  --zip "10001" \
  --state-code "NY"
```

## Available Commands Summary

### Printful
- `printful products` - List all products
- `printful product <id>` - Get product details
- `printful orders` - List all orders
- `printful order <id>` - Get order details
- `printful shipping-rates` - Get shipping rates for an order

### Etsy
- `etsy ping` - Test API connectivity
- `etsy auth-url` - Get OAuth authorization URL
- `etsy exchange-token` - Exchange auth code for tokens
- `etsy refresh-token` - Refresh access token
- `etsy me` - Get authenticated user
- `etsy user <id>` - Get user by ID
- `etsy shop <id>` - Get shop by ID
- `etsy shop-by-user <id>` - Get shop by user ID
- `etsy find-shops` - Search for shops
- `etsy shop-sections <id>` - Get shop sections
- `etsy listing <id>` - Get listing by ID
- `etsy listings` - Get listings by shop
- `etsy active-listings` - Get active listings
- `etsy featured-listings` - Get featured listings
- `etsy create-listing` - Create draft listing
- `etsy update-listing` - Update listing
- `etsy delete-listing` - Delete listing
- `etsy listing-images <id>` - Get listing images
- `etsy listing-image` - Get specific image
- `etsy delete-listing-image` - Delete image
- `etsy listing-inventory <id>` - Get listing inventory
- `etsy listing-product` - Get listing product
- `etsy receipts` - Get shop receipts/orders
- `etsy receipt` - Get specific receipt
- `etsy create-shipment` - Create shipment tracking
- `etsy transactions` - Get shop transactions
- `etsy transaction` - Get specific transaction
- `etsy receipt-transactions` - Get receipt transactions
- `etsy shipping-profiles <id>` - Get shipping profiles
- `etsy shipping-profile` - Get specific profile
- `etsy create-shipping-profile` - Create shipping profile
- `etsy delete-shipping-profile` - Delete shipping profile
- `etsy return-policies <id>` - Get return policies
- `etsy return-policy` - Get specific policy
- `etsy taxonomy-nodes` - Get taxonomy nodes
- `etsy taxonomy-properties <id>` - Get taxonomy properties
- `etsy shop-reviews` - Get shop reviews
- `etsy listing-reviews` - Get listing reviews
- `etsy ledger-entries` - Get payment ledger entries
- `etsy receipt-payments` - Get receipt payments
- `etsy get <path>` - Raw GET request
- `etsy post <path>` - Raw POST request
- `etsy put <path>` - Raw PUT request
- `etsy delete <path>` - Raw DELETE request

## Building

Build the project:
```bash
cargo build
```

Build in release mode:
```bash
cargo build --release
```

Run directly:
```bash
cargo run -- etsy ping
cargo run -- printful products
```

Or install it:
```bash
cargo install --path .
```

Then run as:
```bash
cli etsy ping
cli printful products
```

## API Documentation

- [Etsy Open API v3 Documentation](https://developers.etsy.com/documentation/)
- [Etsy API Reference](https://developers.etsy.com/documentation/reference)
- [Printful API Documentation](https://developers.printful.com/docs/)
