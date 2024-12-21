#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Vendor struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
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

// Service struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Service {
    id: u64,
    vendor_id: u64,
    name: String,
    description: String,
    price: u64,
    is_available: bool,
}

// Contract struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Contract {
    id: u64,
    vendor_id: u64,
    department_id: u64,
    start_date: u64,
    end_date: u64,
    terms: String,
    is_active: bool,
}

// Feedback struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Feedback {
    id: u64,
    vendor_id: u64,
    user_id: u64,
    rating: f32,
    comment: String,
    timestamp: u64,
}

// Payload structs
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateVendorPayload {
    name: String,
    services: Vec<String>,
    contact: String,
    email: String,
    address: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateServicePayload {
    vendor_id: u64,
    name: String,
    description: String,
    price: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateContractPayload {
    vendor_id: u64,
    department_id: u64,
    start_date: u64,
    end_date: u64,
    terms: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateFeedbackPayload {
    vendor_id: u64,
    user_id: u64,
    rating: f32,
    comment: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
}

// Implementing Storable and BoundedStorable for each entity (Vendor, Service, Contract, Feedback)
macro_rules! impl_storable {
    ($struct:ty) => {
        impl Storable for $struct {
            fn to_bytes(&self) -> Cow<[u8]> {
                Cow::Owned(Encode!(self).unwrap())
            }
            fn from_bytes(bytes: Cow<[u8]>) -> Self {
                Decode!(bytes.as_ref(), Self).unwrap()
            }
        }
        impl BoundedStorable for $struct {
            const MAX_SIZE: u32 = 512;
            const IS_FIXED_SIZE: bool = false;
        }
    };
}

impl_storable!(Vendor);
impl_storable!(Service);
impl_storable!(Contract);
impl_storable!(Feedback);

// Memory management
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static VENDORS: RefCell<StableBTreeMap<u64, Vendor, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(10)))
        ));

    static SERVICES: RefCell<StableBTreeMap<u64, Service, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(11)))
        ));

    static CONTRACTS: RefCell<StableBTreeMap<u64, Contract, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(12)))
        ));

    static FEEDBACKS: RefCell<StableBTreeMap<u64, Feedback, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(13)))
        ));
}

// Functions

// Create Vendor
#[ic_cdk::update]
fn create_vendor(payload: CreateVendorPayload) -> Result<Vendor, Message> {
    if payload.name.is_empty() || payload.contact.is_empty() || payload.email.is_empty() {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let vendor_id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    }).expect("Counter increment failed");

    let vendor = Vendor {
        id: vendor_id,
        name: payload.name,
        services: payload.services,
        contact: payload.contact,
        email: payload.email,
        address: payload.address,
        ratings: vec![],
        created_at: time(),
    };

    VENDORS.with(|vendors| {
        vendors.borrow_mut().insert(vendor_id, vendor.clone());
    });

    Ok(vendor)
}

// Get Vendor by ID
#[ic_cdk::query]
fn get_vendor_by_id(id: u64) -> Result<Vendor, Message> {
    VENDORS.with(|vendors| match vendors.borrow().get(&id) {
        Some(vendor) => Ok(vendor.clone()),
        None => Err(Message::NotFound("Vendor not found".to_string())),
    })
}

// List All Vendors
#[ic_cdk::query]
fn list_all_vendors() -> Result<Vec<Vendor>, Message> {
    VENDORS.with(|vendors| {
        let all_vendors: Vec<Vendor> = vendors
            .borrow()
            .iter()
            .map(|(_, vendor)| vendor.clone())
            .collect();

        if all_vendors.is_empty() {
            Err(Message::NotFound("No vendors found".to_string()))
        } else {
            Ok(all_vendors)
        }
    })
}

// Create Service
#[ic_cdk::update]
fn create_service(payload: CreateServicePayload) -> Result<Service, Message> {
    if payload.name.is_empty() || payload.description.is_empty() || payload.price == 0 {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let vendor_exists = VENDORS.with(|vendors| vendors.borrow().contains_key(&payload.vendor_id));
    if !vendor_exists {
        return Err(Message::NotFound("Vendor not found".to_string()));
    }

    let service_id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    }).expect("Counter increment failed");

    let service = Service {
        id: service_id,
        vendor_id: payload.vendor_id,
        name: payload.name,
        description: payload.description,
        price: payload.price,
        is_available: true,
    };

    SERVICES.with(|services| {
        services.borrow_mut().insert(service_id, service.clone());
    });

    Ok(service)
}

// Get Services by Vendor ID
#[ic_cdk::query]
fn get_services_by_vendor_id(vendor_id: u64) -> Result<Vec<Service>, Message> {
    SERVICES.with(|services| {
        let vendor_services: Vec<Service> = services
            .borrow()
            .iter()
            .filter(|(_, service)| service.vendor_id == vendor_id)
            .map(|(_, service)| service.clone())
            .collect();

        if vendor_services.is_empty() {
            Err(Message::NotFound("No services found for this vendor".to_string()))
        } else {
            Ok(vendor_services)
        }
    })
}

// Create Contract
#[ic_cdk::update]
fn create_contract(payload: CreateContractPayload) -> Result<Contract, Message> {
    if payload.vendor_id == 0 || payload.department_id == 0 || payload.start_date == 0 || payload.end_date == 0 {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let vendor_exists = VENDORS.with(|vendors| vendors.borrow().contains_key(&payload.vendor_id));
    if !vendor_exists {
        return Err(Message::NotFound("Vendor not found".to_string()));
    }

    let contract_id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    }).expect("Counter increment failed");

    let contract = Contract {
        id: contract_id,
        vendor_id: payload.vendor_id,
        department_id: payload.department_id,
        start_date: payload.start_date,
        end_date: payload.end_date,
        terms: payload.terms,
        is_active: true,
    };

    CONTRACTS.with(|contracts| {
        contracts.borrow_mut().insert(contract_id, contract.clone());
    });

    Ok(contract)
}

// Get Contracts by Vendor ID
#[ic_cdk::query]
fn get_contracts_by_vendor_id(vendor_id: u64) -> Result<Vec<Contract>, Message> {
    CONTRACTS.with(|contracts| {
        let vendor_contracts: Vec<Contract> = contracts
            .borrow()
            .iter()
            .filter(|(_, contract)| contract.vendor_id == vendor_id)
            .map(|(_, contract)| contract.clone())
            .collect();

        if vendor_contracts.is_empty() {
            Err(Message::NotFound("No contracts found for this vendor".to_string()))
        } else {
            Ok(vendor_contracts)
        }
    })
}

// Create Feedback
#[ic_cdk::update]
fn create_feedback(payload: CreateFeedbackPayload) -> Result<Feedback, Message> {
    if payload.vendor_id == 0 || payload.user_id == 0 || payload.rating < 0.0 || payload.rating > 5.0 {
        return Err(Message::InvalidPayload("Invalid feedback data".to_string()));
    }

    let vendor_exists = VENDORS.with(|vendors| vendors.borrow().contains_key(&payload.vendor_id));
    if !vendor_exists {
        return Err(Message::NotFound("Vendor not found".to_string()));
    }

    let feedback_id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    }).expect("Counter increment failed");

    let feedback = Feedback {
        id: feedback_id,
        vendor_id: payload.vendor_id,
        user_id: payload.user_id,
        rating: payload.rating,
        comment: payload.comment,
        timestamp: time(),
    };

    // First store the feedback
    FEEDBACKS.with(|feedbacks| {
        feedbacks.borrow_mut().insert(feedback_id, feedback.clone());
    });

    // Then update the vendor's ratings
    VENDORS.with(|vendors| {
        let mut vendors = vendors.borrow_mut();
        if let Some(vendor) = vendors.get(&payload.vendor_id) {
            let mut updated_vendor = vendor.clone();
            updated_vendor.ratings.push(payload.rating);
            vendors.insert(payload.vendor_id, updated_vendor);
        }
    });

    Ok(feedback)
}

// Get Feedback by Vendor ID
#[ic_cdk::query]
fn get_feedback_by_vendor_id(vendor_id: u64) -> Result<Vec<Feedback>, Message> {
    FEEDBACKS.with(|feedbacks| {
        let vendor_feedbacks: Vec<Feedback> = feedbacks
            .borrow()
            .iter()
            .filter(|(_, feedback)| feedback.vendor_id == vendor_id)
            .map(|(_, feedback)| feedback.clone())
            .collect();

        if vendor_feedbacks.is_empty() {
            Err(Message::NotFound("No feedback found for this vendor".to_string()))
        } else {
            Ok(vendor_feedbacks)
        }
    })
}

// Calculate Average Rating for a Vendor
#[ic_cdk::query]
fn calculate_average_rating(vendor_id: u64) -> Result<f32, Message> {
    VENDORS.with(|vendors| {
        if let Some(vendor) = vendors.borrow().get(&vendor_id) {
            if vendor.ratings.is_empty() {
                return Err(Message::NotFound("No ratings available for this vendor".to_string()));
            }

            let average_rating: f32 = vendor.ratings.iter().sum::<f32>() / vendor.ratings.len() as f32;
            Ok(average_rating)
        } else {
            Err(Message::NotFound("Vendor not found".to_string()))
        }
    })
}

// Exporting the candid interface
ic_cdk::export_candid!();
