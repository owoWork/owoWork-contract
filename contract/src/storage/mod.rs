//! Storage access patterns for the OwoWork contract

use soroban_sdk::{Env, Address, String};

use crate::constants::{JOB, REP};
use crate::state::{Job, Reputation};

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