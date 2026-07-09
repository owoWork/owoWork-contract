//! Test module for OwoWork contract
//! 
//! This module provides test utilities and example test cases
//! for the contract functionality.

#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String, Symbol};

use crate::{JobEscrow, JobEscrowClient};

#[test]
fn test_create_job() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test addresses
    let customer = Address::generate(&env);
    let artisan = Address::generate(&env);
    
    // Set authentication for customer
    env.mock_all_auths();

    // Create job
    let job_id = String::from_str(&env, "job-123");
    let job_hash = String::from_str(&env, "hash123");
    let trade = Symbol::new(&env, "plumber");
    
    let result = client.create_job(
        &job_id,
        &customer,
        &artisan,
        &1000000,
        &job_hash,
        &trade,
    );

    assert_eq!(result, job_id);

    // Verify job was created
    let job = client.get_job(&job_id);
    assert_eq!(job.customer, customer);
    assert_eq!(job.artisan, artisan);
    assert_eq!(job.amount, 1000000);
    assert_eq!(job.trade, trade);
}

#[test]
fn test_accept_job() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test addresses
    let customer = Address::generate(&env);
    let artisan = Address::generate(&env);
    
    // Set authentication for customer and artisan
    env.mock_all_auths();

    // Create job
    let job_id = String::from_str(&env, "job-123");
    let job_hash = String::from_str(&env, "hash123");
    let trade = Symbol::new(&env, "plumber");
    
    client.create_job(
        &job_id,
        &customer,
        &artisan,
        &1000000,
        &job_hash,
        &trade,
    );

    // Accept job
    client.accept_job(&job_id, &artisan);

    // Verify job state is now Active
    let job = client.get_job(&job_id);
    assert!(matches!(job.state, crate::state::JobState::Active));
}

#[test]
fn test_confirm_done() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test addresses
    let customer = Address::generate(&env);
    let artisan = Address::generate(&env);
    
    // Set authentication for customer and artisan
    env.mock_all_auths();

    // Create and accept job
    let job_id = String::from_str(&env, "job-123");
    let job_hash = String::from_str(&env, "hash123");
    let trade = Symbol::new(&env, "plumber");
    
    client.create_job(
        &job_id,
        &customer,
        &artisan,
        &1000000,
        &job_hash,
        &trade,
    );

    client.accept_job(&job_id, &artisan);

    // Confirm completion
    client.confirm_done(&job_id, &customer);

    // Verify job state is now Completed
    let job = client.get_job(&job_id);
    assert!(matches!(job.state, crate::state::JobState::Completed));

    // Verify reputation was updated
    let rep = client.get_reputation(&artisan);
    assert_eq!(rep.completed, 1);
    assert_eq!(rep.total_earned, 1000000);
}

#[test]
#[should_panic(expected = "job already exists")]
fn test_duplicate_job_creation() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test addresses
    let customer = Address::generate(&env);
    let artisan = Address::generate(&env);
    
    // Set authentication for customer
    env.mock_all_auths();

    // Create job twice with same ID
    let job_id = String::from_str(&env, "job-123");
    let job_hash = String::from_str(&env, "hash123");
    let trade = Symbol::new(&env, "plumber");
    
    client.create_job(
        &job_id,
        &customer,
        &artisan,
        &1000000,
        &job_hash,
        &trade,
    );

    // This should panic
    client.create_job(
        &job_id,
        &customer,
        &artisan,
        &2000000,
        &job_hash,
        &trade,
    );
}

#[test]
fn test_reputation_calculation() {
    let _env = Env::default();
    
    // Test reputation struct methods
    let mut rep = crate::state::Reputation::new();
    
    // Initial state
    assert_eq!(rep.completed, 0);
    assert_eq!(rep.disputed, 0);
    assert_eq!(rep.total_earned, 0);
    assert_eq!(rep.score(), 0.0);
    
    // Update with successful job
    rep.update(1000000, true);
    assert_eq!(rep.completed, 1);
    assert_eq!(rep.disputed, 0);
    assert_eq!(rep.total_earned, 1000000);
    assert_eq!(rep.score(), 1.0);
    
    // Update with disputed job
    rep.update(0, false);
    assert_eq!(rep.completed, 1);
    assert_eq!(rep.disputed, 1);
    assert_eq!(rep.total_earned, 1000000);
    assert_eq!(rep.score(), 0.5);
}