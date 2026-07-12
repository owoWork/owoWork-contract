//! Storage access patterns for the OwoWork contract

use soroban_sdk::{Env, Address, String};

use crate::constants::{JOB, REP, USER, TASK, TASK_COUNTER, MAX_NAME_LENGTH, MAX_AVATAR_LENGTH, MAX_TITLE_LENGTH, MAX_DESCRIPTION_LENGTH};
use crate::state::{Job, Reputation, User, Task};

/// Job storage helper
pub struct JobStorage<'a> {
    env: &'a Env,
}

impl<'a> JobStorage<'a> {
    /// Creates a new JobStorage instance
    pub fn new(env: &'a Env) -> Self {
        JobStorage { env }
    }

    /// Gets a job by ID
    pub fn get(&self, job_id: &String) -> Option<Job> {
        let key = (JOB.clone(), job_id.clone());
        self.env.storage().persistent().get(&key)
    }

    /// Gets a job by ID, panics if not found
    pub fn get_expect(&self, job_id: &String) -> Job {
        self.get(job_id).expect("job not found")
    }

    /// Checks if a job exists
    pub fn exists(&self, job_id: &String) -> bool {
        let key = (JOB.clone(), job_id.clone());
        self.env.storage().persistent().has(&key)
    }

    /// Saves a job
    pub fn save(&self, job_id: &String, job: &Job) {
        let key = (JOB.clone(), job_id.clone());
        self.env.storage().persistent().set(&key, job);
    }

    /// Deletes a job (for cleanup if needed)
    pub fn delete(&self, job_id: &String) {
        let key = (JOB.clone(), job_id.clone());
        self.env.storage().persistent().remove(&key);
    }
}

/// Reputation storage helper
pub struct ReputationStorage<'a> {
    env: &'a Env,
}

impl<'a> ReputationStorage<'a> {
    /// Creates a new ReputationStorage instance
    pub fn new(env: &'a Env) -> Self {
        ReputationStorage { env }
    }

    /// Gets reputation for an artisan
    pub fn get(&self, artisan: &Address) -> Reputation {
        let key = (REP.clone(), artisan.clone());
        self.env.storage().persistent().get(&key).unwrap_or(Reputation::new())
    }

    /// Saves reputation for an artisan
    pub fn save(&self, artisan: &Address, reputation: &Reputation) {
        let key = (REP.clone(), artisan.clone());
        self.env.storage().persistent().set(&key, reputation);
    }

    /// Updates reputation for an artisan
    pub fn update(&self, artisan: &Address, amount: i128, success: bool) {
        let mut rep = self.get(artisan);
        rep.update(amount, success);
        self.save(artisan, &rep);
    }
}

/// User storage helper
pub struct UserStorage<'a> {
    env: &'a Env,
}

impl<'a> UserStorage<'a> {
    /// Creates a new UserStorage instance
    pub fn new(env: &'a Env) -> Self {
        UserStorage { env }
    }

    /// Gets a user by address
    pub fn get(&self, address: &Address) -> Option<User> {
        let key = (USER.clone(), address.clone());
        self.env.storage().persistent().get(&key)
    }

    /// Gets a user by address, panics if not found
    pub fn get_expect(&self, address: &Address) -> User {
        self.get(address).expect("user not found")
    }

    /// Checks if a user exists
    pub fn exists(&self, address: &Address) -> bool {
        let key = (USER.clone(), address.clone());
        self.env.storage().persistent().has(&key)
    }

    /// Saves a user
    pub fn save(&self, user: &User) {
        let key = (USER.clone(), user.address.clone());
        self.env.storage().persistent().set(&key, user);
    }

    /// Deletes a user (for cleanup if needed)
    pub fn delete(&self, address: &Address) {
        let key = (USER.clone(), address.clone());
        self.env.storage().persistent().remove(&key);
    }

    /// Validates user input parameters
    pub fn validate_input(&self, name: &String, avatar: &String) -> Result<(), &'static str> {
        if name.len() > MAX_NAME_LENGTH {
            return Err("name exceeds maximum length");
        }
        
        if avatar.len() > MAX_AVATAR_LENGTH {
            return Err("avatar URL exceeds maximum length");
        }
        
        Ok(())
    }
}

/// Task storage helper
pub struct TaskStorage<'a> {
    env: &'a Env,
}

impl<'a> TaskStorage<'a> {
    /// Creates a new TaskStorage instance
    pub fn new(env: &'a Env) -> Self {
        TaskStorage { env }
    }

    /// Gets the next task ID (auto-incremented)
    pub fn next_id(&self) -> u64 {
        let current_id: u64 = self.env.storage().persistent().get(&TASK_COUNTER).unwrap_or(0);
        let next_id = current_id + 1;
        self.env.storage().persistent().set(&TASK_COUNTER, &next_id);
        next_id
    }

    /// Gets a task by ID
    pub fn get(&self, task_id: u64) -> Option<Task> {
        let key = (TASK.clone(), task_id);
        self.env.storage().persistent().get(&key)
    }

    /// Gets a task by ID, panics if not found
    pub fn get_expect(&self, task_id: u64) -> Task {
        self.get(task_id).expect("task not found")
    }

    /// Checks if a task exists
    pub fn exists(&self, task_id: u64) -> bool {
        let key = (TASK.clone(), task_id);
        self.env.storage().persistent().has(&key)
    }

    /// Saves a task
    pub fn save(&self, task: &Task) {
        let key = (TASK.clone(), task.id);
        self.env.storage().persistent().set(&key, task);
    }

    /// Deletes a task (for cleanup if needed)
    pub fn delete(&self, task_id: u64) {
        let key = (TASK.clone(), task_id);
        self.env.storage().persistent().remove(&key);
    }

    /// Validates task input parameters
    pub fn validate_input(&self, title: &String, description: &String) -> Result<(), &'static str> {
        if title.len() > MAX_TITLE_LENGTH {
            return Err("task title exceeds maximum length");
        }
        
        if description.len() > MAX_DESCRIPTION_LENGTH {
            return Err("task description exceeds maximum length");
        }
        
        Ok(())
    }

    /// Gets all tasks (for testing/querying - use with caution in production)
    pub fn get_all(&self, limit: Option<u32>) -> soroban_sdk::Vec<Task> {
        let mut tasks = soroban_sdk::Vec::new(self.env);
        let max_limit = limit.unwrap_or(100);
        
        // This is a simplified implementation - in production you might want
        // a different approach for enumerating all tasks
        for i in 1..=max_limit {
            if let Some(task) = self.get(i as u64) {
                tasks.push_back(task);
            }
        }
        
        tasks
    }
}