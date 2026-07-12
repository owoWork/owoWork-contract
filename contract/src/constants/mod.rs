//! Constants and configuration for the OwoWork contract

use soroban_sdk::{symbol_short, Symbol};

/// Storage key prefix for job records
pub const JOB: Symbol = symbol_short!("JOB");

/// Storage key prefix for reputation records
pub const REP: Symbol = symbol_short!("REP");

/// Storage key prefix for user accounts
pub const USER: Symbol = symbol_short!("USER");

/// Storage key prefix for task records
pub const TASK: Symbol = symbol_short!("TASK");

/// Storage key for task counter
pub const TASK_COUNTER: Symbol = symbol_short!("TASK_CNT");

/// Dispute time-to-live in seconds (48 hours)
pub const DISPUTE_TTL: u64 = 172_800;

/// Maximum length for user name
pub const MAX_NAME_LENGTH: u32 = 100;

/// Maximum length for avatar URL
pub const MAX_AVATAR_LENGTH: u32 = 500;

/// Maximum length for task title
pub const MAX_TITLE_LENGTH: u32 = 200;

/// Maximum length for task description
pub const MAX_DESCRIPTION_LENGTH: u32 = 2000;

/// Event symbols for contract events
pub mod events {
    use soroban_sdk::{symbol_short, Symbol};

    pub const JOB_CREATED: (Symbol, Symbol) = (symbol_short!("created"), symbol_short!("job"));
    pub const JOB_ACCEPTED: (Symbol, Symbol) = (symbol_short!("accepted"), symbol_short!("job"));
    pub const JOB_COMPLETED: (Symbol, Symbol) = (symbol_short!("completed"), symbol_short!("job"));
    pub const JOB_DISPUTED: (Symbol, Symbol) = (symbol_short!("disputed"), symbol_short!("job"));
    pub const JOB_RESOLVED: (Symbol, Symbol) = (symbol_short!("resolved"), symbol_short!("job"));
    
    pub const USER_REGISTERED: (Symbol, Symbol) = (symbol_short!("reg"), symbol_short!("user"));
    pub const USER_UPDATED: (Symbol, Symbol) = (symbol_short!("updated"), symbol_short!("user"));
    
    pub const TASK_CREATED: (Symbol, Symbol) = (symbol_short!("created"), symbol_short!("task"));
    pub const TASK_UPDATED: (Symbol, Symbol) = (symbol_short!("updated"), symbol_short!("task"));
    pub const TASK_STATUS_CHANGED: (Symbol, Symbol) = (symbol_short!("status"), symbol_short!("task"));
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
    
    pub const USER_EXISTS: &str = "user already registered";
    pub const USER_NOT_FOUND: &str = "user not found";
    pub const NAME_TOO_LONG: &str = "name exceeds maximum length";
    pub const AVATAR_TOO_LONG: &str = "avatar URL exceeds maximum length";
    pub const UNAUTHORIZED_UPDATE: &str = "only user can update their own profile";
    
    pub const TASK_NOT_FOUND: &str = "task not found";
    pub const TASK_TITLE_TOO_LONG: &str = "task title exceeds maximum length";
    pub const TASK_DESCRIPTION_TOO_LONG: &str = "task description exceeds maximum length";
    pub const INVALID_STATUS_TRANSITION: &str = "invalid status transition";
    pub const UNAUTHORIZED_TASK_UPDATE: &str = "only task creator can update task";
}