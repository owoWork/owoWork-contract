//! State definitions and data structures for the OwoWork contract

use soroban_sdk::{contracttype, Address, String, Symbol};

/// Represents the state of a job in the escrow system
#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum JobState {
    /// Job created, payment locked, waiting for artisan acceptance
    Open,
    /// Artisan accepted, work in progress
    Active,
    /// Customer raised a dispute
    Disputed,
    /// Job completed successfully
    Completed,
    /// Job refunded to customer after dispute resolution
    Refunded,
}

/// Job record containing all escrow metadata
#[contracttype]
#[derive(Clone, Debug)]
pub struct Job {
    /// Customer who created and funded the job
    pub customer: Address,
    /// Artisan assigned to complete the work
    pub artisan: Address,
    /// Locked payment amount in stroops
    pub amount: i128,
    /// Current state of the job
    pub state: JobState,
    /// Timestamp when the job was created
    pub created_at: u64,
    /// Timestamp when dispute was raised (0 if not disputed)
    pub dispute_at: u64,
    /// Hash of job description/metadata for verification
    pub job_hash: String,
    /// Trade category (e.g., "plumber", "electrician")
    pub trade: Symbol,
}

/// Artisan reputation tracking
#[contracttype]
#[derive(Clone, Debug)]
pub struct Reputation {
    /// Number of successfully completed jobs
    pub completed: u32,
    /// Number of disputed jobs
    pub disputed: u32,
    /// Total amount earned from completed jobs (in stroops)
    pub total_earned: i128,
}

impl Reputation {
    /// Creates a new reputation record with default values
    pub fn new() -> Self {
        Reputation {
            completed: 0,
            disputed: 0,
            total_earned: 0,
        }
    }

    /// Updates reputation based on job outcome
    pub fn update(&mut self, amount: i128, success: bool) {
        if success {
            self.completed += 1;
            self.total_earned += amount;
        } else {
            self.disputed += 1;
        }
    }

    /// Calculates reputation score (simple formula)
    pub fn score(&self) -> f64 {
        let total = self.completed + self.disputed;
        if total == 0 {
            return 0.0;
        }
        (self.completed as f64) / (total as f64)
    }
}

/// User account information
#[contracttype]
#[derive(Clone, Debug)]
pub struct User {
    /// Wallet address (unique identifier)
    pub address: Address,
    /// Display name
    pub name: String,
    /// Avatar URL or hash
    pub avatar: String,
    /// Account creation timestamp
    pub created_at: u64,
    /// Last update timestamp
    pub updated_at: u64,
    /// Whether the user is active
    pub is_active: bool,
}

impl User {
    /// Creates a new user account
    pub fn new(address: Address, name: String, avatar: String, created_at: u64) -> Self {
        User {
            address,
            name,
            avatar,
            created_at,
            updated_at: created_at,
            is_active: true,
        }
    }

    /// Updates user profile information
    pub fn update(&mut self, name: Option<String>, avatar: Option<String>, updated_at: u64) {
        if let Some(new_name) = name {
            self.name = new_name;
        }
        
        if let Some(new_avatar) = avatar {
            self.avatar = new_avatar;
        }
        
        self.updated_at = updated_at;
    }

    /// Deactivates the user account
    pub fn deactivate(&mut self, updated_at: u64) {
        self.is_active = false;
        self.updated_at = updated_at;
    }

    /// Reactivates the user account
    pub fn reactivate(&mut self, updated_at: u64) {
        self.is_active = true;
        self.updated_at = updated_at;
    }
}

/// Represents the status of a work item/task
#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum TaskStatus {
    /// Task created but not started
    Pending,
    /// Task is currently being worked on
    InProgress,
    /// Task completed successfully
    Completed,
    /// Task cancelled
    Cancelled,
}

/// Work item/task record
#[contracttype]
#[derive(Clone, Debug)]
pub struct Task {
    /// Unique task identifier (auto-incremented)
    pub id: u64,
    /// Address of the task creator
    pub creator: Address,
    /// Task title
    pub title: String,
    /// Task description
    pub description: String,
    /// Current task status
    pub status: TaskStatus,
    /// Timestamp when task was created
    pub created_at: u64,
    /// Timestamp when task was last updated
    pub updated_at: u64,
}

impl Task {
    /// Creates a new task
    pub fn new(id: u64, creator: Address, title: String, description: String, created_at: u64) -> Self {
        Task {
            id,
            creator,
            title,
            description,
            status: TaskStatus::Pending,
            created_at,
            updated_at: created_at,
        }
    }

    /// Updates task information (title and/or description)
    pub fn update_info(&mut self, title: Option<String>, description: Option<String>, updated_at: u64) {
        if let Some(new_title) = title {
            self.title = new_title;
        }
        
        if let Some(new_description) = description {
            self.description = new_description;
        }
        
        self.updated_at = updated_at;
    }

    /// Changes task status with validation
    pub fn change_status(&mut self, new_status: TaskStatus, updated_at: u64) -> Result<(), &'static str> {
        // Validate status transition
        match (&self.status, &new_status) {
            (TaskStatus::Pending, TaskStatus::InProgress) => {},
            (TaskStatus::Pending, TaskStatus::Cancelled) => {},
            (TaskStatus::InProgress, TaskStatus::Completed) => {},
            (TaskStatus::InProgress, TaskStatus::Cancelled) => {},
            (TaskStatus::Cancelled, TaskStatus::Pending) => {}, // Allow reopening cancelled tasks
            (old, new) if old == new => {
                // Same status is allowed (for timestamp updates)
            },
            _ => {
                return Err("invalid status transition");
            }
        }
        
        self.status = new_status;
        self.updated_at = updated_at;
        Ok(())
    }

    /// Checks if the task can transition to a new status
    pub fn can_transition_to(&self, new_status: &TaskStatus) -> bool {
        match (&self.status, new_status) {
            (TaskStatus::Pending, TaskStatus::InProgress) => true,
            (TaskStatus::Pending, TaskStatus::Cancelled) => true,
            (TaskStatus::InProgress, TaskStatus::Completed) => true,
            (TaskStatus::InProgress, TaskStatus::Cancelled) => true,
            (TaskStatus::Cancelled, TaskStatus::Pending) => true,
            (old, new) if old == new => true,
            _ => false,
        }
    }
}