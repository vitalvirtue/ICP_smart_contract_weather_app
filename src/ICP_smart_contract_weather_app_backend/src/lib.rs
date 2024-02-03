// External libraries used in Rust are defined here
use candid::{CandidType, Decode, Deserialize, Encode}; // Enables the availability of external libraries
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory}; // Contains necessary structures for memory management
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable}; // Contains structures for storage operations
use std::{borrow::Cow, cell::RefCell}; // Includes certain structures from the standard library

// Defines the type of virtual memory
type Memory = VirtualMemory<DefaultMemoryImpl>;

