#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Address, Map, symbol_short, Symbol};

// Structure to store attendance record for an individual
#[contracttype]
#[derive(Clone)]
pub struct AttendanceRecord {
    pub user_address: Address,
    pub timestamp: u64,
    pub event_id: u64,
    pub is_present: bool,
}

// Structure to track overall attendance statistics
#[contracttype]
#[derive(Clone)]
pub struct AttendanceStats {
    pub total_attendees: u64,
    pub total_events: u64,
}

// Symbol for storing overall stats
const STATS: Symbol = symbol_short!("STATS");

// Enum for mapping attendance records
#[contracttype]
pub enum AttendanceBook {
    Record(Address, u64), // (user_address, event_id)
}

#[contract]
pub struct AttendanceContract;

#[contractimpl]
impl AttendanceContract {
    
    /// Mark attendance for a user at a specific event
    /// Returns true if attendance was successfully marked
    pub fn mark_attendance(env: Env, user: Address, event_id: u64) -> bool {
        // Authenticate the user
        user.require_auth();
        
        let key = AttendanceBook::Record(user.clone(), event_id);
        
        // Check if attendance already marked
        let existing: Option<AttendanceRecord> = env.storage().instance().get(&key);
        
        if existing.is_some() {
            log!(&env, "Attendance already marked for this user at this event");
            return false;
        }
        
        // Get current timestamp
        let timestamp = env.ledger().timestamp();
        
        // Create attendance record
        let record = AttendanceRecord {
            user_address: user.clone(),
            timestamp,
            event_id,
            is_present: true,
        };
        
        // Update statistics
        let mut stats = Self::get_stats(env.clone());
        stats.total_attendees += 1;
        
        // Store the record
        env.storage().instance().set(&key, &record);
        env.storage().instance().set(&STATS, &stats);
        
        // Extend TTL for storage
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Attendance marked successfully for event: {}", event_id);
        true
    }
    
    /// Check if a user attended a specific event
    /// Returns the attendance record if found
    pub fn get_attendance(env: Env, user: Address, event_id: u64) -> AttendanceRecord {
        let key = AttendanceBook::Record(user.clone(), event_id);
        
        env.storage().instance().get(&key).unwrap_or(AttendanceRecord {
            user_address: user,
            timestamp: 0,
            event_id,
            is_present: false,
        })
    }
    
    /// Get overall attendance statistics
    pub fn get_stats(env: Env) -> AttendanceStats {
        env.storage().instance().get(&STATS).unwrap_or(AttendanceStats {
            total_attendees: 0,
            total_events: 0,
        })
    }
    
    /// Register a new event (admin function)
    /// Increments the total event count
    pub fn register_event(env: Env, admin: Address) -> u64 {
        // Authenticate admin
        admin.require_auth();
        
        let mut stats = Self::get_stats(env.clone());
        stats.total_events += 1;
        
        let event_id = stats.total_events;
        
        // Store updated stats
        env.storage().instance().set(&STATS, &stats);
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "New event registered with ID: {}", event_id);
        event_id
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_mark_attendance() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AttendanceContract);
        let client = AttendanceContractClient::new(&env, &contract_id);
        
        let user = Address::generate(&env);
        let event_id = 1u64;
        
        // Mark attendance
        let result = client.mark_attendance(&user, &event_id);
        assert_eq!(result, true);
        
        // Try marking again (should fail)
        let result2 = client.mark_attendance(&user, &event_id);
        assert_eq!(result2, false);
    }
}
