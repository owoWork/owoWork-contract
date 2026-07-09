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

#[test]
fn test_register_user() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test address
    let user = Address::generate(&env);
    
    // Set authentication for user
    env.mock_all_auths();

    // Register user
    let name = String::from_str(&env, "Test User");
    let avatar = String::from_str(&env, "https://example.com/avatar.jpg");
    
    let result = client.register_user(&user, &name, &avatar);

    assert_eq!(result, user);

    // Verify user was created
    let user_profile = client.get_user(&user);
    assert_eq!(user_profile.address, user);
    assert_eq!(user_profile.name, name);
    assert_eq!(user_profile.avatar, avatar);
    assert!(user_profile.is_active);
}

#[test]
fn test_get_user() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test address
    let user = Address::generate(&env);
    
    // Set authentication for user
    env.mock_all_auths();

    // Register user first
    let name = String::from_str(&env, "Test User");
    let avatar = String::from_str(&env, "https://example.com/avatar.jpg");
    
    client.register_user(&user, &name, &avatar);

    // Retrieve user
    let user_profile = client.get_user(&user);
    
    assert_eq!(user_profile.address, user);
    assert_eq!(user_profile.name, name);
    assert_eq!(user_profile.avatar, avatar);
    assert!(user_profile.is_active);
    // Timestamps should be set (may be 0 in test environment)
    assert_eq!(user_profile.updated_at, user_profile.created_at);
}

#[test]
fn test_update_user() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test address
    let user = Address::generate(&env);
    
    // Set authentication for user
    env.mock_all_auths();

    // Register user first
    let initial_name = String::from_str(&env, "Initial Name");
    let initial_avatar = String::from_str(&env, "https://example.com/avatar1.jpg");
    
    client.register_user(&user, &initial_name, &initial_avatar);

    // Update user profile
    let new_name = String::from_str(&env, "Updated Name");
    let new_avatar = String::from_str(&env, "https://example.com/avatar2.jpg");
    
    client.update_user(&user, &Some(new_name.clone()), &Some(new_avatar.clone()));

    // Retrieve updated user
    let user_profile = client.get_user(&user);
    
    assert_eq!(user_profile.name, new_name);
    assert_eq!(user_profile.avatar, new_avatar);
    // updated_at should be set (may equal created_at in test environment)
}

#[test]
fn test_update_user_partial() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test address
    let user = Address::generate(&env);
    
    // Set authentication for user
    env.mock_all_auths();

    // Register user first
    let initial_name = String::from_str(&env, "Initial Name");
    let initial_avatar = String::from_str(&env, "https://example.com/avatar1.jpg");
    
    client.register_user(&user, &initial_name, &initial_avatar);

    // Update only name
    let new_name = String::from_str(&env, "Updated Name");
    client.update_user(&user, &Some(new_name.clone()), &None);

    // Retrieve updated user
    let user_profile = client.get_user(&user);
    
    assert_eq!(user_profile.name, new_name);
    assert_eq!(user_profile.avatar, initial_avatar); // Avatar unchanged
    // updated_at should be set

    // Update only avatar
    let new_avatar = String::from_str(&env, "https://example.com/avatar3.jpg");
    client.update_user(&user, &None, &Some(new_avatar.clone()));

    // Retrieve updated user again
    let user_profile2 = client.get_user(&user);
    
    assert_eq!(user_profile2.name, new_name); // Name unchanged
    assert_eq!(user_profile2.avatar, new_avatar);
    // updated_at should be set
}

#[test]
fn test_deactivate_reactivate_user() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test address
    let user = Address::generate(&env);
    
    // Set authentication for user
    env.mock_all_auths();

    // Register user
    let name = String::from_str(&env, "Test User");
    let avatar = String::from_str(&env, "https://example.com/avatar.jpg");
    
    client.register_user(&user, &name, &avatar);

    // Deactivate user
    client.deactivate_user(&user);
    
    let deactivated_user = client.get_user(&user);
    assert!(!deactivated_user.is_active);

    // Reactivate user
    client.reactivate_user(&user);
    
    let reactivated_user = client.get_user(&user);
    assert!(reactivated_user.is_active);
}

#[test]
#[should_panic(expected = "user already registered")]
fn test_duplicate_user_registration() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test address
    let user = Address::generate(&env);
    
    // Set authentication for user
    env.mock_all_auths();

    // Register user twice
    let name = String::from_str(&env, "Test User");
    let avatar = String::from_str(&env, "https://example.com/avatar.jpg");
    
    client.register_user(&user, &name, &avatar);
    
    // This should panic
    client.register_user(&user, &name, &avatar);
}

#[test]
#[should_panic(expected = "name exceeds maximum length")]
fn test_user_name_too_long() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test address
    let user = Address::generate(&env);
    
    // Set authentication for user
    env.mock_all_auths();

    // Try to register with name that's too long
    let long_name = String::from_str(&env, &"a".repeat(150)); // Exceeds MAX_NAME_LENGTH
    let avatar = String::from_str(&env, "https://example.com/avatar.jpg");
    
    // This should panic
    client.register_user(&user, &long_name, &avatar);
}

#[test]
fn test_user_struct_methods() {
    let env = Env::default();
    
    // Test User struct methods
    let address = Address::generate(&env);
    let name = String::from_str(&env, "Test User");
    let avatar = String::from_str(&env, "https://example.com/avatar.jpg");
    let created_at = 1234567890;
    
    let mut user = crate::state::User::new(address.clone(), name.clone(), avatar.clone(), created_at);
    
    // Initial state
    assert_eq!(user.address, address);
    assert_eq!(user.name, name);
    assert_eq!(user.avatar, avatar);
    assert_eq!(user.created_at, created_at);
    assert_eq!(user.updated_at, created_at);
    assert!(user.is_active);
    
    // Update profile
    let new_name = String::from_str(&env, "Updated Name");
    let new_avatar = String::from_str(&env, "https://example.com/new-avatar.jpg");
    let updated_at = created_at + 1000;
    
    user.update(Some(new_name.clone()), Some(new_avatar.clone()), updated_at);
    
    assert_eq!(user.name, new_name);
    assert_eq!(user.avatar, new_avatar);
    assert_eq!(user.updated_at, updated_at);
    
    // Deactivate
    user.deactivate(updated_at + 1000);
    assert!(!user.is_active);
    assert_eq!(user.updated_at, updated_at + 1000);
    
    // Reactivate
    user.reactivate(updated_at + 2000);
    assert!(user.is_active);
    assert_eq!(user.updated_at, updated_at + 2000);
}
#[test]
fn test_create_task() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test address
    let creator = Address::generate(&env);
    
    // Set authentication for creator
    env.mock_all_auths();

    // Create task
    let title = String::from_str(&env, "Test Task");
    let description = String::from_str(&env, "This is a test task description");
    
    let task_id = client.create_task(&creator, &title, &description);

    // Verify task ID is 1 (first task)
    assert_eq!(task_id, 1);

    // Verify task was created
    let task = client.get_task(&task_id);
    assert_eq!(task.id, 1);
    assert_eq!(task.creator, creator);
    assert_eq!(task.title, title);
    assert_eq!(task.description, description);
    assert!(matches!(task.status, crate::state::TaskStatus::Pending));
    assert_eq!(task.updated_at, task.created_at);
}

#[test]
fn test_get_task() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test address
    let creator = Address::generate(&env);
    
    // Set authentication for creator
    env.mock_all_auths();

    // Create task first
    let title = String::from_str(&env, "Test Task");
    let description = String::from_str(&env, "This is a test task description");
    
    let task_id = client.create_task(&creator, &title, &description);

    // Retrieve task
    let task = client.get_task(&task_id);
    
    assert_eq!(task.id, task_id);
    assert_eq!(task.creator, creator);
    assert_eq!(task.title, title);
    assert_eq!(task.description, description);
    assert!(matches!(task.status, crate::state::TaskStatus::Pending));
}

#[test]
fn test_update_task() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test address
    let creator = Address::generate(&env);
    
    // Set authentication for creator
    env.mock_all_auths();

    // Create task first
    let initial_title = String::from_str(&env, "Initial Task");
    let initial_description = String::from_str(&env, "Initial description");
    
    let task_id = client.create_task(&creator, &initial_title, &initial_description);

    // Update task
    let new_title = String::from_str(&env, "Updated Task");
    let new_description = String::from_str(&env, "Updated description");
    
    client.update_task(&task_id, &creator, &Some(new_title.clone()), &Some(new_description.clone()));

    // Retrieve updated task
    let task = client.get_task(&task_id);
    
    assert_eq!(task.title, new_title);
    assert_eq!(task.description, new_description);
    // Note: In test environment, timestamps may not increment
}

#[test]
fn test_update_task_partial() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test address
    let creator = Address::generate(&env);
    
    // Set authentication for creator
    env.mock_all_auths();

    // Create task first
    let initial_title = String::from_str(&env, "Initial Task");
    let initial_description = String::from_str(&env, "Initial description");
    
    let task_id = client.create_task(&creator, &initial_title, &initial_description);

    // Update only title
    let new_title = String::from_str(&env, "Updated Task");
    client.update_task(&task_id, &creator, &Some(new_title.clone()), &None);

    let task1 = client.get_task(&task_id);
    assert_eq!(task1.title, new_title);
    assert_eq!(task1.description, initial_description);

    // Update only description
    let new_description = String::from_str(&env, "Updated description");
    client.update_task(&task_id, &creator, &None, &Some(new_description.clone()));

    let task2 = client.get_task(&task_id);
    assert_eq!(task2.title, new_title);
    assert_eq!(task2.description, new_description);
}

#[test]
fn test_change_task_status() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test address
    let creator = Address::generate(&env);
    
    // Set authentication for creator
    env.mock_all_auths();

    // Create task first
    let title = String::from_str(&env, "Test Task");
    let description = String::from_str(&env, "Test description");
    
    let task_id = client.create_task(&creator, &title, &description);

    // Change status from Pending to InProgress
    client.change_task_status(&task_id, &creator, &crate::state::TaskStatus::InProgress);
    
    let task1 = client.get_task(&task_id);
    assert!(matches!(task1.status, crate::state::TaskStatus::InProgress));
    // Note: In test environment, timestamps may not increment

    // Change status from InProgress to Completed
    client.change_task_status(&task_id, &creator, &crate::state::TaskStatus::Completed);
    
    let task2 = client.get_task(&task_id);
    assert!(matches!(task2.status, crate::state::TaskStatus::Completed));
    // Note: In test environment, timestamps may not increment
}

#[test]
fn test_task_status_transition_cancelled() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test address
    let creator = Address::generate(&env);
    
    // Set authentication for creator
    env.mock_all_auths();

    // Create task first
    let title = String::from_str(&env, "Test Task");
    let description = String::from_str(&env, "Test description");
    
    let task_id = client.create_task(&creator, &title, &description);

    // Cancel task from Pending
    client.change_task_status(&task_id, &creator, &crate::state::TaskStatus::Cancelled);
    
    let task1 = client.get_task(&task_id);
    assert!(matches!(task1.status, crate::state::TaskStatus::Cancelled));

    // Reopen cancelled task to Pending
    client.change_task_status(&task_id, &creator, &crate::state::TaskStatus::Pending);
    
    let task2 = client.get_task(&task_id);
    assert!(matches!(task2.status, crate::state::TaskStatus::Pending));
}

#[test]
fn test_get_tasks() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test address
    let creator = Address::generate(&env);
    
    // Set authentication for creator
    env.mock_all_auths();

    // Create multiple tasks
    let title1 = String::from_str(&env, "Task 1");
    let description1 = String::from_str(&env, "Description 1");
    client.create_task(&creator, &title1, &description1);
    
    let title2 = String::from_str(&env, "Task 2");
    let description2 = String::from_str(&env, "Description 2");
    client.create_task(&creator, &title2, &description2);
    
    let title3 = String::from_str(&env, "Task 3");
    let description3 = String::from_str(&env, "Description 3");
    client.create_task(&creator, &title3, &description3);

    // Get all tasks
    let tasks = client.get_tasks(&None);
    
    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks.get(0).unwrap().id, 1);
    assert_eq!(tasks.get(1).unwrap().id, 2);
    assert_eq!(tasks.get(2).unwrap().id, 3);
}

#[test]
fn test_task_struct_methods() {
    let env = Env::default();
    
    // Test Task struct methods
    let address = Address::generate(&env);
    let title = String::from_str(&env, "Test Task");
    let description = String::from_str(&env, "Test description");
    let created_at = 1234567890;
    
    let mut task = crate::state::Task::new(1, address.clone(), title.clone(), description.clone(), created_at);
    
    // Initial state
    assert_eq!(task.id, 1);
    assert_eq!(task.creator, address);
    assert_eq!(task.title, title);
    assert_eq!(task.description, description);
    assert!(matches!(task.status, crate::state::TaskStatus::Pending));
    assert_eq!(task.created_at, created_at);
    assert_eq!(task.updated_at, created_at);
    
    // Update information
    let new_title = String::from_str(&env, "Updated Task");
    let new_description = String::from_str(&env, "Updated description");
    let updated_at = created_at + 1000;
    
    task.update_info(Some(new_title.clone()), Some(new_description.clone()), updated_at);
    
    assert_eq!(task.title, new_title);
    assert_eq!(task.description, new_description);
    assert_eq!(task.updated_at, updated_at);
    
    // Test status transitions
    assert!(task.can_transition_to(&crate::state::TaskStatus::InProgress));
    assert!(task.can_transition_to(&crate::state::TaskStatus::Cancelled));
    assert!(!task.can_transition_to(&crate::state::TaskStatus::Completed));
    
    // Change status to InProgress
    let result = task.change_status(crate::state::TaskStatus::InProgress, updated_at + 1000);
    assert!(result.is_ok());
    assert!(matches!(task.status, crate::state::TaskStatus::InProgress));
    assert_eq!(task.updated_at, updated_at + 1000);
    
    // Now can transition to Completed or Cancelled
    assert!(task.can_transition_to(&crate::state::TaskStatus::Completed));
    assert!(task.can_transition_to(&crate::state::TaskStatus::Cancelled));
    assert!(!task.can_transition_to(&crate::state::TaskStatus::Pending));
    
    // Change status to Completed
    let result = task.change_status(crate::state::TaskStatus::Completed, updated_at + 2000);
    assert!(result.is_ok());
    assert!(matches!(task.status, crate::state::TaskStatus::Completed));
    assert_eq!(task.updated_at, updated_at + 2000);
    
    // Test invalid transition (Completed back to InProgress should fail)
    assert!(!task.can_transition_to(&crate::state::TaskStatus::InProgress));
    let result = task.change_status(crate::state::TaskStatus::InProgress, updated_at + 3000);
    assert!(result.is_err());
}

#[test]
#[should_panic(expected = "task title exceeds maximum length")]
fn test_task_title_too_long() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test address
    let creator = Address::generate(&env);
    
    // Set authentication for creator
    env.mock_all_auths();

    // Try to create task with title that's too long
    let long_title = String::from_str(&env, &"a".repeat(250)); // Exceeds MAX_TITLE_LENGTH
    let description = String::from_str(&env, "Test description");
    
    // This should panic
    client.create_task(&creator, &long_title, &description);
}

#[test]
#[should_panic(expected = "task description exceeds maximum length")]
fn test_task_description_too_long() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test address
    let creator = Address::generate(&env);
    
    // Set authentication for creator
    env.mock_all_auths();

    // Try to create task with description that's too long
    let title = String::from_str(&env, "Test Task");
    let long_description = String::from_str(&env, &"a".repeat(2500)); // Exceeds MAX_DESCRIPTION_LENGTH
    
    // This should panic
    client.create_task(&creator, &title, &long_description);
}

#[test]
#[should_panic(expected = "invalid status transition")]
fn test_invalid_task_status_transition() {
    let env = Env::default();
    let contract_id = env.register_contract(None, JobEscrow);
    let client = JobEscrowClient::new(&env, &contract_id);

    // Create test address
    let creator = Address::generate(&env);
    
    // Set authentication for creator
    env.mock_all_auths();

    // Create task first
    let title = String::from_str(&env, "Test Task");
    let description = String::from_str(&env, "Test description");
    
    let task_id = client.create_task(&creator, &title, &description);

    // Try invalid transition (Pending -> Completed, skipping InProgress)
    // This should panic
    client.change_task_status(&task_id, &creator, &crate::state::TaskStatus::Completed);
}