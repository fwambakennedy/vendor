# Vendor Management System

The Vendor Management System is a Rust-based backend system for managing vendors, services, contracts, and feedback. It uses the Internet Computer (IC) SDK to implement functionalities for managing data in a stable and structured format.

## Features

- **Vendor Management:**
  - Create vendors with details like name, services, contact information, and ratings.
  - Retrieve vendor details by ID.
  - List all vendors.

- **Service Management:**
  - Create services associated with vendors.
  - Retrieve services by vendor ID.

- **Contract Management:**
  - Create contracts between vendors and departments.
  - Retrieve contracts by vendor ID.

- **Feedback Management:**
  - Add feedback for vendors with a rating and comment.
  - Retrieve feedback for vendors.

- **Ratings:**
  - Calculate average ratings for vendors based on feedback.

## Project Structure

- **`Vendor`**: Represents a vendor with details like services offered, contact information, and ratings.
- **`Service`**: Represents a service provided by a vendor.
- **`Contract`**: Represents a contract between a vendor and a department.
- **`Feedback`**: Represents user feedback on vendors.

## Data Storage

The system uses stable structures provided by the Internet Computer SDK:

- **StableBTreeMap**: Used for managing vendors, services, contracts, and feedback data efficiently.
- **MemoryManager**: Ensures memory is managed and allocated properly.

## API Endpoints

### Vendor Management

- `create_vendor(payload: CreateVendorPayload) -> Result<Vendor, Message>`: Create a new vendor.
- `get_vendor_by_id(id: u64) -> Result<Vendor, Message>`: Retrieve vendor details by ID.
- `list_all_vendors() -> Result<Vec<Vendor>, Message>`: List all vendors.

### Service Management

- `create_service(payload: CreateServicePayload) -> Result<Service, Message>`: Create a new service.
- `get_services_by_vendor_id(vendor_id: u64) -> Result<Vec<Service>, Message>`: Retrieve services for a vendor.

### Contract Management

- `create_contract(payload: CreateContractPayload) -> Result<Contract, Message>`: Create a new contract.
- `get_contracts_by_vendor_id(vendor_id: u64) -> Result<Vec<Contract>, Message>`: Retrieve contracts for a vendor.

### Feedback Management

- `create_feedback(payload: CreateFeedbackPayload) -> Result<Feedback, Message>`: Add feedback for a vendor.
- `get_feedback_by_vendor_id(vendor_id: u64) -> Result<Vec<Feedback>, Message>`: Retrieve feedback for a vendor.
- `calculate_average_rating(vendor_id: u64) -> Result<f32, Message>`: Calculate the average rating for a vendor.

## Data Models

### Vendor
```rust
struct Vendor {
    id: u64,
    name: String,
    services: Vec<String>,
    contact: String,
    email: String,
    address: String,
    ratings: Vec<f32>,
    created_at: u64,
}
```

### Service
```rust
struct Service {
    id: u64,
    vendor_id: u64,
    name: String,
    description: String,
    price: u64,
    is_available: bool,
}
```

### Contract
```rust
struct Contract {
    id: u64,
    vendor_id: u64,
    department_id: u64,
    start_date: u64,
    end_date: u64,
    terms: String,
    is_active: bool,
}
```

### Feedback
```rust
struct Feedback {
    id: u64,
    vendor_id: u64,
    user_id: u64,
    rating: f32,
    comment: String,
    timestamp: u64,
}
```


## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown targetz
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ git clone  https://github.com/fwambakennedy/vendor.git
$ cd vendor/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```