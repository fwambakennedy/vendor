
# tele-health

This project is a decentralized healthcare management system deployed on the Internet Computer (IC) using Rust. The system provides a range of healthcare-related services, enabling patient management, doctor scheduling, and health records while ensuring persistence through stable memory. The project is designed to offer scalable and reliable management of healthcare entities in a decentralized environment.

## Features

### 1. Department Management
   - **Create Department**: Add new departments for organizational structuring.
   - **View Department**: Retrieve information about a specific department.
   - **List Departments**: Retrieve all existing departments.
   - **Search by Name**: Find departments using a partial or full name match.

### 2. Doctor Management
   - **Create Doctor**: Register new doctors and assign them to specific departments.
   - **View Doctor**: Retrieve doctor details by ID.
   - **List Doctors**: Retrieve a list of all doctors.
   - **Doctor Availability**: Update and check doctors' availability.
   - **Search by Name**: Find doctors using a name search.

### 3. Patient Management
   - **Create Patient**: Register patients with detailed information, including emergency contacts and medical history.
   - **View Patient**: Retrieve patient information by ID.
   - **List Patients**: Retrieve all registered patients.

### 4. Consultations
   - **Create Consultation**: Schedule a consultation between a patient and a department.
   - **View Consultation**: Retrieve details about a consultation.
   - **List Consultations**: View all consultations or filter by patient.

### 5. Chat System
   - **Create Chat**: Facilitate communication between doctors and patients.
   - **View Chat**: Retrieve messages by ID.
   - **List Chats**: Retrieve all chat messages or filter by doctor or patient.

### 6. Appointments
   - **Create Appointment**: Schedule an appointment with a doctor, including optional video link support for virtual consultations.
   - **View Appointment**: Retrieve appointment details by ID.
   - **Update Status and Video Link**: Modify the status (scheduled, canceled, completed) or add a video link for virtual meetings.
   - **List Appointments**: Retrieve all appointments or filter by doctor or patient.

### 7. Prescription Management
   - **Create Prescription**: Record doctor-issued prescriptions for patients.
   - **View Prescription**: Retrieve specific prescription details.
   - **List Prescriptions**: Retrieve all prescriptions or filter by patient or doctor.

### 8. Payment Management
   - **Create Payment**: Record payments associated with appointments.
   - **View Payment**: Retrieve payment details.
   - **List Payments**: View all payments or filter by patient or appointment.

### 9. Medical Records
   - **Create Medical Record**: Log a patient’s medical record, including lab results, diagnoses, and treatment details.
   - **View Medical Record**: Retrieve detailed medical records for a patient.
   - **List Medical Records**: View all records or filter by patient or doctor.

## Data Persistence and Memory Management

The project uses stable memory to ensure persistence across canister upgrades. Each entity (departments, doctors, patients, etc.) is stored in separate stable structures, allowing efficient data management. The system ensures the integrity and accessibility of patient records, schedules, and appointments without data loss over time.

## Error Handling

The system provides structured responses for various operations:
   - **Success**: A success message with details of the operation.
   - **Error**: General errors or invalid payloads.
   - **Not Found**: Returned when a requested entity is missing.
   - **Invalid Payload**: Triggers when required fields are missing or incorrect.
   - **Payment Status**: Indications for payment completion or failure.


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
$ cd icp_rust_boilerplate/
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