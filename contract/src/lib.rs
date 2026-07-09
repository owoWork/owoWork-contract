#![no_std]

//! OwoWork Contract - Job Escrow and Reputation System
//!
//! This contract implements an escrow system for artisan services marketplace.
//! It manages job lifecycle from creation to completion or dispute resolution,
//! while tracking artisan reputation based on job outcomes.
//!
//! # Modules
//! - `constants`: Contract configuration and storage keys
//! - `state`: Data structures and enums
//! - `storage`: Persistent storage access patterns
//!
//! # State Machine
//! Open → Active → Completed (happy path)
//! Open → Active → Disputed → Completed/Refunded (dispute path)
//!
//! # Security Features
//! - All state-changing functions require authentication
//! - Role-based access control (customer, artisan, mediator)
//! - Time-based dispute resolution window
//! - No duplicate job creation

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String, Symbol};

// Internal modules
pub mod constants;
pub mod state;
pub mod storage;

// Conditionally include test module
#[cfg(test)]
mod test;

// Re-exports for easier access
pub use constants::*;
pub use state::*;
pub use storage::*;

/// Main contract struct
///
/// This is the entry point for the Soroban contract.
/// All contract functions are implemented in the `JobEscrow` impl block.
#[contract]
pub struct JobEscrow;

/// Contract implementation with all public functions
///
/// # Functions
/// - Task Management:
///   - `create_task`: Create a new work item/task
///   - `get_task`: Retrieve task information by ID
///   - `update_task`: Update task title and description
///   - `change_task_status`: Change task status with validation
///   - `get_tasks`: Get multiple tasks (for testing/querying)
///
/// - User Management:
///   - `register_user`: Register a new user account
///   - `get_user`: Retrieve user profile information
///   - `update_user`: Update user profile information
///   - `deactivate_user`: Deactivate user account
///   - `reactivate_user`: Reactivate user account
///
/// - Job Management:
///   - `create_job`: Customer creates a job and locks payment
///   - `accept_job`: Artisan accepts an open job
///   - `confirm_done`: Customer confirms job completion
///   - `raise_dispute`: Customer raises a dispute on active job
///   - `resolve_dispute`: Mediator resolves dispute
///   - `get_job`: Read job record
///   - `get_reputation`: Read artisan reputation
#[contractimpl]
impl JobEscrow {
    /// Registers a new user account
    ///
    /// # Arguments
    /// - `env`: Contract environment
    /// - `address`: User's wallet address (unique identifier)
    /// - `name`: Display name for the user
    /// - `avatar`: Avatar URL or hash
    ///
    /// # Returns
    /// - User address that was registered
    ///
    /// # Panics
    /// - If user already exists
    /// - If name or avatar exceed maximum length
    ///
    /// # Authentication
    /// - Requires user authentication (must match the address)
    pub fn register_user(env: Env, address: Address, name: String, avatar: String) -> Address {
        address.require_auth();

        let user_storage = UserStorage::new(&env);

        // Validate user doesn't already exist
        if user_storage.exists(&address) {
            panic!("{}", errors::USER_EXISTS);
        }

        // Validate input parameters
        user_storage.validate_input(&name, &avatar)
            .unwrap_or_else(|err| panic!("{}", err));

        // Create user record
        let user = User::new(
            address.clone(),
            name,
            avatar,
            env.ledger().timestamp(),
        );

        // Save user
        user_storage.save(&user);

        // Emit registration event
        env.events().publish(
            (symbol_short!("reg"), symbol_short!("user")),
            address.clone(),
        );

        address
    }

    /// Retrieves user profile information
    ///
    /// # Arguments
    /// - `env`: Contract environment
    /// - `address`: User's wallet address
    ///
    /// # Returns
    /// - User profile information
    ///
    /// # Panics
    /// - If user not found
    pub fn get_user(env: Env, address: Address) -> User {
        let user_storage = UserStorage::new(&env);
        user_storage.get_expect(&address)
    }

    /// Updates user profile information
    ///
    /// # Arguments
    /// - `env`: Contract environment
    /// - `address`: User's wallet address
    /// - `name`: New display name (optional)
    /// - `avatar`: New avatar URL or hash (optional)
    ///
    /// # Panics
    /// - If user not found
    /// - If name or avatar exceed maximum length
    /// - If caller is not the user
    ///
    /// # Authentication
    /// - Requires user authentication (must match the address)
    pub fn update_user(env: Env, address: Address, name: Option<String>, avatar: Option<String>) {
        address.require_auth();

        let user_storage = UserStorage::new(&env);
        let mut user = user_storage.get_expect(&address);

        // Validate input parameters if provided
        if let Some(ref new_name) = name {
            if let Some(ref new_avatar) = avatar {
                user_storage.validate_input(new_name, new_avatar)
                    .unwrap_or_else(|err| panic!("{}", err));
            } else {
                user_storage.validate_input(new_name, &user.avatar)
                    .unwrap_or_else(|err| panic!("{}", err));
            }
        } else if let Some(ref new_avatar) = avatar {
            user_storage.validate_input(&user.name, new_avatar)
                .unwrap_or_else(|err| panic!("{}", err));
        }

        // Update user profile
        user.update(name, avatar, env.ledger().timestamp());
        user_storage.save(&user);

        // Emit update event
        env.events().publish(
            (symbol_short!("updated"), symbol_short!("user")),
            address,
        );
    }

    /// Deactivates a user account
    ///
    /// # Arguments
    /// - `env`: Contract environment
    /// - `address`: User's wallet address
    ///
    /// # Panics
    /// - If user not found
    /// - If caller is not the user
    ///
    /// # Authentication
    /// - Requires user authentication (must match the address)
    pub fn deactivate_user(env: Env, address: Address) {
        address.require_auth();

        let user_storage = UserStorage::new(&env);
        let mut user = user_storage.get_expect(&address);

        // Deactivate user
        user.deactivate(env.ledger().timestamp());
        user_storage.save(&user);

        // Emit update event
        env.events().publish(
            (symbol_short!("updated"), symbol_short!("user")),
            address,
        );
    }

    /// Reactivates a user account
    ///
    /// # Arguments
    /// - `env`: Contract environment
    /// - `address`: User's wallet address
    ///
    /// # Panics
    /// - If user not found
    /// - If caller is not the user
    ///
    /// # Authentication
    /// - Requires user authentication (must match the address)
    pub fn reactivate_user(env: Env, address: Address) {
        address.require_auth();

        let user_storage = UserStorage::new(&env);
        let mut user = user_storage.get_expect(&address);

        // Reactivate user
        user.reactivate(env.ledger().timestamp());
        user_storage.save(&user);

        // Emit update event
        env.events().publish(
            (symbol_short!("updated"), symbol_short!("user")),
            address,
        );
    }

    /// Creates a new task (work item)
    ///
    /// # Arguments
    /// - `env`: Contract environment
    /// - `creator`: User creating the task
    /// - `title`: Task title
    /// - `description`: Task description
    ///
    /// # Returns
    /// - Task ID (auto-incremented)
    ///
    /// # Panics
    /// - If title or description exceed maximum length
    ///
    /// # Authentication
    /// - Requires creator authentication
    pub fn create_task(env: Env, creator: Address, title: String, description: String) -> u64 {
        creator.require_auth();

        let task_storage = TaskStorage::new(&env);

        // Validate input parameters
        task_storage.validate_input(&title, &description)
            .unwrap_or_else(|err| panic!("{}", err));

        // Get next auto-incremented task ID
        let task_id = task_storage.next_id();

        // Create task record
        let task = Task::new(
            task_id,
            creator.clone(),
            title,
            description,
            env.ledger().timestamp(),
        );

        // Save task
        task_storage.save(&task);

        // Emit creation event
        env.events().publish(
            (constants::events::TASK_CREATED.0, constants::events::TASK_CREATED.1),
            (task_id, creator),
        );

        task_id
    }

    /// Retrieves task information by ID
    ///
    /// # Arguments
    /// - `env`: Contract environment
    /// - `task_id`: Task identifier
    ///
    /// # Returns
    /// - Task information
    ///
    /// # Panics
    /// - If task not found
    pub fn get_task(env: Env, task_id: u64) -> Task {
        let task_storage = TaskStorage::new(&env);
        task_storage.get_expect(task_id)
    }

    /// Updates task title and description
    ///
    /// # Arguments
    /// - `env`: Contract environment
    /// - `task_id`: Task identifier
    /// - `creator`: Task creator (must match original creator)
    /// - `title`: New task title (optional)
    /// - `description`: New task description (optional)
    ///
    /// # Panics
    /// - If task not found
    /// - If title or description exceed maximum length
    /// - If caller is not the task creator
    ///
    /// # Authentication
    /// - Requires creator authentication
    pub fn update_task(env: Env, task_id: u64, creator: Address, title: Option<String>, description: Option<String>) {
        creator.require_auth();

        let task_storage = TaskStorage::new(&env);
        let mut task = task_storage.get_expect(task_id);

        // Validate caller is the task creator
        assert!(task.creator == creator, "{}", constants::errors::UNAUTHORIZED_TASK_UPDATE);

        // Validate input parameters if provided
        let validate_title = title.as_ref().unwrap_or(&task.title);
        let validate_description = description.as_ref().unwrap_or(&task.description);
        
        task_storage.validate_input(validate_title, validate_description)
            .unwrap_or_else(|err| panic!("{}", err));

        // Update task information
        task.update_info(title, description, env.ledger().timestamp());
        task_storage.save(&task);

        // Emit update event
        env.events().publish(
            (constants::events::TASK_UPDATED.0, constants::events::TASK_UPDATED.1),
            (task_id, creator),
        );
    }

    /// Changes task status with validation
    ///
    /// # Arguments
    /// - `env`: Contract environment
    /// - `task_id`: Task identifier
    /// - `creator`: Task creator (must match original creator)
    /// - `new_status`: New task status
    ///
    /// # Panics
    /// - If task not found
    /// - If status transition is invalid
    /// - If caller is not the task creator
    ///
    /// # Authentication
    /// - Requires creator authentication
    pub fn change_task_status(env: Env, task_id: u64, creator: Address, new_status: TaskStatus) {
        creator.require_auth();

        let task_storage = TaskStorage::new(&env);
        let mut task = task_storage.get_expect(task_id);

        // Validate caller is the task creator
        assert!(task.creator == creator, "{}", constants::errors::UNAUTHORIZED_TASK_UPDATE);

        // Change task status with validation
        task.change_status(new_status.clone(), env.ledger().timestamp())
            .unwrap_or_else(|err| panic!("{}", err));

        // Save updated task
        task_storage.save(&task);

        // Emit status change event
        env.events().publish(
            (constants::events::TASK_STATUS_CHANGED.0, constants::events::TASK_STATUS_CHANGED.1),
            (task_id, creator, new_status),
        );
    }

    /// Gets multiple tasks (for testing/querying)
    ///
    /// # Arguments
    /// - `env`: Contract environment
    /// - `limit`: Maximum number of tasks to return (optional, default: 100)
    ///
    /// # Returns
    /// - Vector of tasks
    ///
    /// # Note
    /// - This is a simplified implementation suitable for testing
    /// - For production use, consider pagination or indexed queries
    pub fn get_tasks(env: Env, limit: Option<u32>) -> soroban_sdk::Vec<Task> {
        let task_storage = TaskStorage::new(&env);
        task_storage.get_all(limit)
    }

    /// Creates a new job and stores escrow metadata
    ///
    /// # Arguments
    /// - `env`: Contract environment
    /// - `job_id`: Unique identifier for the job
    /// - `customer`: Customer creating the job
    /// - `artisan`: Artisan assigned to the job
    /// - `amount`: Payment amount in stroops
    /// - `job_hash`: Hash of job description/metadata
    /// - `trade`: Trade category (e.g., "plumber")
    ///
    /// # Returns
    /// - Job ID string
    ///
    /// # Panics
    /// - If job already exists
    ///
    /// # Authentication
    /// - Requires customer authentication
    pub fn create_job(
        env: Env,
        job_id: String,
        customer: Address,
        artisan: Address,
        amount: i128,
        job_hash: String,
        trade: Symbol,
    ) -> String {
        customer.require_auth();

        let job_storage = JobStorage::new(&env);

        // Validate job doesn't already exist
        if job_storage.exists(&job_id) {
            panic!("{}", errors::JOB_EXISTS);
        }

        // Create job record
        let job = Job {
            customer: customer.clone(),
            artisan: artisan.clone(),
            amount,
            state: JobState::Open,
            created_at: env.ledger().timestamp(),
            dispute_at: 0,
            job_hash,
            trade,
        };

        // Save job
        job_storage.save(&job_id, &job);

        // Emit creation event
        env.events().publish(
            (symbol_short!("created"), job_id.clone()),
            (customer, artisan, amount),
        );

        job_id
    }

    /// Artisan accepts an open job
    ///
    /// # Arguments
    /// - `env`: Contract environment
    /// - `job_id`: Job identifier
    /// - `artisan`: Artisan accepting the job
    ///
    /// # Panics
    /// - If job not found
    /// - If job not in Open state
    /// - If wrong artisan
    ///
    /// # Authentication
    /// - Requires artisan authentication
    pub fn accept_job(env: Env, job_id: String, artisan: Address) {
        artisan.require_auth();

        let job_storage = JobStorage::new(&env);
        let mut job = job_storage.get_expect(&job_id);

        // Validate job state and artisan
        assert!(job.state == JobState::Open, "{}", errors::JOB_NOT_OPEN);
        assert!(job.artisan == artisan, "{}", errors::WRONG_ARTISAN);

        // Update state
        job.state = JobState::Active;
        job_storage.save(&job_id, &job);

        // Emit acceptance event
        env.events()
            .publish((symbol_short!("accepted"), job_id), artisan);
    }

    /// Customer confirms job completion
    ///
    /// # Arguments
    /// - `env`: Contract environment
    /// - `job_id`: Job identifier
    /// - `customer`: Customer confirming completion
    ///
    /// # Panics
    /// - If job not found
    /// - If job not in Active state
    /// - If wrong customer
    ///
    /// # Authentication
    /// - Requires customer authentication
    pub fn confirm_done(env: Env, job_id: String, customer: Address) {
        customer.require_auth();

        let job_storage = JobStorage::new(&env);
        let mut job = job_storage.get_expect(&job_id);

        // Validate job state and customer
        assert!(job.state == JobState::Active, "{}", errors::JOB_NOT_ACTIVE);
        assert!(job.customer == customer, "{}", errors::WRONG_CUSTOMER);

        // Update state
        job.state = JobState::Completed;
        job_storage.save(&job_id, &job);

        // Update reputation
        let rep_storage = ReputationStorage::new(&env);
        rep_storage.update(&job.artisan, job.amount, true);

        // Emit completion event
        env.events().publish(
            (symbol_short!("completed"), job_id),
            (job.artisan.clone(), job.amount),
        );
    }

    /// Customer raises a dispute on active job
    ///
    /// # Arguments
    /// - `env`: Contract environment
    /// - `job_id`: Job identifier
    /// - `customer`: Customer raising dispute
    ///
    /// # Panics
    /// - If job not found
    /// - If job not in Active state
    /// - If wrong customer
    ///
    /// # Authentication
    /// - Requires customer authentication
    pub fn raise_dispute(env: Env, job_id: String, customer: Address) {
        customer.require_auth();

        let job_storage = JobStorage::new(&env);
        let mut job = job_storage.get_expect(&job_id);

        // Validate job state and customer
        assert!(job.state == JobState::Active, "{}", errors::JOB_NOT_ACTIVE);
        assert!(job.customer == customer, "{}", errors::WRONG_CUSTOMER);

        // Update state and record dispute timestamp
        job.state = JobState::Disputed;
        job.dispute_at = env.ledger().timestamp();
        job_storage.save(&job_id, &job);

        // Emit dispute event
        env.events()
            .publish((symbol_short!("disputed"), job_id), customer);
    }

    /// Mediator resolves a disputed job
    ///
    /// # Arguments
    /// - `env`: Contract environment
    /// - `job_id`: Job identifier
    /// - `mediator`: Mediator resolving the dispute
    /// - `favour`: Resolution in favour of "artisan" or "customer"
    ///
    /// # Panics
    /// - If job not found
    /// - If job not in Disputed state
    /// - If favouring artisan before 48h window elapsed
    ///
    /// # Authentication
    /// - Requires mediator authentication
    pub fn resolve_dispute(env: Env, job_id: String, mediator: Address, favour: Symbol) {
        mediator.require_auth();

        let job_storage = JobStorage::new(&env);
        let mut job = job_storage.get_expect(&job_id);

        // Validate job state
        assert!(job.state == JobState::Disputed, "{}", errors::JOB_NOT_DISPUTED);

        let elapsed = env.ledger().timestamp() - job.dispute_at;

        if favour == symbol_short!("artisan") {
            // Artisan can only be favored after 48h dispute window
            assert!(elapsed >= DISPUTE_TTL, "{}", errors::DISPUTE_WINDOW_NOT_ELAPSED);
            job.state = JobState::Completed;
            
            // Update reputation for successful completion
            let rep_storage = ReputationStorage::new(&env);
            rep_storage.update(&job.artisan, job.amount, true);
        } else {
            // Customer gets refund
            job.state = JobState::Refunded;
            
            // Update reputation for disputed outcome
            let rep_storage = ReputationStorage::new(&env);
            rep_storage.update(&job.artisan, 0, false);
        }

        job_storage.save(&job_id, &job);

        // Emit resolution event
        env.events()
            .publish((symbol_short!("resolved"), job_id), (favour, job.amount));
    }

    /// Reads a job record
    ///
    /// # Arguments
    /// - `env`: Contract environment
    /// - `job_id`: Job identifier
    ///
    /// # Returns
    /// - Job record
    ///
    /// # Panics
    /// - If job not found
    pub fn get_job(env: Env, job_id: String) -> Job {
        let job_storage = JobStorage::new(&env);
        job_storage.get_expect(&job_id)
    }

    /// Reads artisan reputation
    ///
    /// # Arguments
    /// - `env`: Contract environment
    /// - `artisan`: Artisan address
    ///
    /// # Returns
    /// - Reputation record (default values if not found)
    pub fn get_reputation(env: Env, artisan: Address) -> Reputation {
        let rep_storage = ReputationStorage::new(&env);
        rep_storage.get(&artisan)
    }
}