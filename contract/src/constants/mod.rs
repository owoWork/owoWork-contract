//! Constants and configuration for the OwoWork contract

use soroban_sdk::{symbol_short, Symbol};

/// Storage key prefix for job records
pub const JOB: Symbol = symbol_short!("JOB");

/// Storage key prefix for reputation records
pub const REP: Symbol = symbol_short!("REP");

/// Dispute time-to-live in seconds (48 hours)
pub const DISPUTE_TTL: u64 = 172_800;

/// Event symbols for contract events
pub mod events {
    use soroban_sdk::{symbol_short, Symbol};

    pub const JOB_CREATED: (Symbol, Symbol) = (symbol_short!("created"), symbol_short!("job"));
    pub const JOB_ACCEPTED: (Symbol, Symbol) = (symbol_short!("accepted"), symbol_short!("job"));
    pub const JOB_COMPLETED: (Symbol, Symbol) = (symbol_short!("completed"), symbol_short!("job"));
    pub const JOB_DISPUTED: (Symbol, Symbol) = (symbol_short!("disputed"), symbol_short!("job"));
    pub const JOB_RESOLVED: (Symbol, Symbol) = (symbol_short!("resolved"), symbol_short!("job"));
}

/// Error messages for contract validation
pub mod errors {
    pub const JOB_EXISTS: &str = "job already exists";
    pub const JOB_NOT_FOUND: &str = "job not found";
    pub const JOB_NOT_OPEN: &str = "job not open";
    pub const JOB_NOT_ACTIVE: &str = "job not active";
    pub const JOB_NOT_DISPUTED: &str = "not disputed";
    pub const WRONG_ARTISAN: &str = "wrong artisan";
    pub const WRONG_CUSTOMER: &str = "wrong customer";
    pub const DISPUTE_WINDOW_NOT_ELAPSED: &str = "48h window not elapsed yet";
}