#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol,
};

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum JobState {
    Open,
    Active,
    Disputed,
    Completed,
    Refunded,
}

#[contracttype]
#[derive(Clone)]
pub struct Job {
    pub customer: Address,
    pub artisan: Address,
    pub amount: i128,
    pub state: JobState,
    pub created_at: u64,
    pub dispute_at: u64,
    pub job_hash: String,
    pub trade: Symbol,
}

#[contracttype]
#[derive(Clone)]
pub struct Reputation {
    pub completed: u32,
    pub disputed: u32,
    pub total_earned: i128,
}

const JOB: Symbol = symbol_short!("JOB");
const REP: Symbol = symbol_short!("REP");
const DISPUTE_TTL: u64 = 172_800;

#[contract]
pub struct JobEscrow;

#[contractimpl]
impl JobEscrow {
    /// Customer creates a job and locks payment in the contract.
    /// Production escrow should also perform or verify the asset transfer.
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

        let key = (JOB.clone(), job_id.clone());
        if env.storage().persistent().has(&key) {
            panic!("job already exists");
        }

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

        env.storage().persistent().set(&key, &job);

        env.events().publish(
            (symbol_short!("created"), job_id.clone()),
            (customer, artisan, amount),
        );

        job_id
    }

    /// Artisan accepts the job and the state moves to Active.
    pub fn accept_job(env: Env, job_id: String, artisan: Address) {
        artisan.require_auth();

        let key = (JOB.clone(), job_id.clone());
        let mut job: Job = env.storage().persistent().get(&key).expect("job not found");

        assert!(job.state == JobState::Open, "job not open");
        assert!(job.artisan == artisan, "wrong artisan");

        job.state = JobState::Active;
        env.storage().persistent().set(&key, &job);

        env.events()
            .publish((symbol_short!("accepted"), job_id), artisan);
    }

    /// Customer confirms completion. Backend can listen for this event and settle funds.
    pub fn confirm_done(env: Env, job_id: String, customer: Address) {
        customer.require_auth();

        let key = (JOB.clone(), job_id.clone());
        let mut job: Job = env.storage().persistent().get(&key).expect("job not found");

        assert!(job.state == JobState::Active, "job not active");
        assert!(job.customer == customer, "wrong customer");

        job.state = JobState::Completed;
        env.storage().persistent().set(&key, &job);

        Self::_increment_rep(&env, &job.artisan, job.amount, true);

        env.events().publish(
            (symbol_short!("completed"), job_id),
            (job.artisan.clone(), job.amount),
        );
    }

    /// Customer raises a dispute and opens the resolution window.
    pub fn raise_dispute(env: Env, job_id: String, customer: Address) {
        customer.require_auth();

        let key = (JOB.clone(), job_id.clone());
        let mut job: Job = env.storage().persistent().get(&key).expect("job not found");

        assert!(job.state == JobState::Active, "job not active");
        assert!(job.customer == customer, "wrong customer");

        job.state = JobState::Disputed;
        job.dispute_at = env.ledger().timestamp();
        env.storage().persistent().set(&key, &job);

        env.events()
            .publish((symbol_short!("disputed"), job_id), customer);
    }

    /// Resolve dispute: backend mediates or artisan claims after 48 hours.
    pub fn resolve_dispute(env: Env, job_id: String, mediator: Address, favour: Symbol) {
        mediator.require_auth();

        let key = (JOB.clone(), job_id.clone());
        let mut job: Job = env.storage().persistent().get(&key).expect("job not found");

        assert!(job.state == JobState::Disputed, "not disputed");

        let elapsed = env.ledger().timestamp() - job.dispute_at;

        if favour == symbol_short!("artisan") {
            assert!(elapsed >= DISPUTE_TTL, "48h window not elapsed yet");
            job.state = JobState::Completed;
            Self::_increment_rep(&env, &job.artisan, job.amount, true);
        } else {
            job.state = JobState::Refunded;
            Self::_increment_rep(&env, &job.artisan, 0, false);
        }

        env.storage().persistent().set(&key, &job);

        env.events()
            .publish((symbol_short!("resolved"), job_id), (favour, job.amount));
    }

    /// Read a job record.
    pub fn get_job(env: Env, job_id: String) -> Job {
        let key = (JOB.clone(), job_id);
        env.storage().persistent().get(&key).expect("job not found")
    }

    /// Read artisan reputation.
    pub fn get_reputation(env: Env, artisan: Address) -> Reputation {
        let key = (REP.clone(), artisan);
        env.storage().persistent().get(&key).unwrap_or(Reputation {
            completed: 0,
            disputed: 0,
            total_earned: 0,
        })
    }

    fn _increment_rep(env: &Env, artisan: &Address, amount: i128, success: bool) {
        let key = (REP.clone(), artisan.clone());
        let mut rep: Reputation = env.storage().persistent().get(&key).unwrap_or(Reputation {
            completed: 0,
            disputed: 0,
            total_earned: 0,
        });

        if success {
            rep.completed += 1;
            rep.total_earned += amount;
        } else {
            rep.disputed += 1;
        }

        env.storage().persistent().set(&key, &rep);
    }
}
