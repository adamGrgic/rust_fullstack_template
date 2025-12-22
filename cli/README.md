# Atom Platform CLI

A CLI tool for testing API connections to various services, starting with Printful.

## Setup

1. Install dependencies:
```bash
cargo build
```

2. Set up your Printful API key:
   - Get your API key from [Printful Dashboard](https://www.printful.com/dashboard/api)
   - Create a `.env` file in the project root (parent directory of `cli/`):
   ```
   PRINTFUL_API_KEY=your_api_key_here
   ```
   The `.env` file should be located at: `atomplatform/.env`

## Usage

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

## Available Commands

- `printful products` - List all products
- `printful product <id>` - Get product details
- `printful orders` - List all orders
- `printful order <id>` - Get order details
- `printful shipping-rates` - Get shipping rates for an order

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
cargo run -- printful products
```

Or install it:
```bash
cargo install --path .
```

