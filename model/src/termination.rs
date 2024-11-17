use near_sdk::{json_types::Base58CryptoHash, near, AccountId, CryptoHash};

use crate::{lockup::Lockup, schedule::Schedule, Balance, TimestampSec};

#[near(serializers=[borsh, json])]
#[derive(Clone, Debug, PartialEq)]
pub enum VestingConditions {
    SameAsLockupSchedule,
    //Hash(Base58CryptoHash),
    Schedule(Schedule),
}

#[near(serializers=[borsh, json])]
#[derive(Debug, PartialEq, Clone)]
pub struct TerminationConfig {
    /// The account ID who paid for the lockup creation
    /// and will receive unvested balance upon termination
    pub beneficiary_id: AccountId,
    /// An optional vesting schedule
    pub vesting_schedule: VestingConditions,
}

impl Lockup {
    pub fn terminate(
        &mut self,
        termination_timestamp: TimestampSec,
    ) -> (Balance, AccountId) {
        let termination_config = self.termination_config.take().expect("No termination config");
        let total_balance = self.schedule.total_balance();
        let vested_balance = match &termination_config.vesting_schedule {
            VestingConditions::SameAsLockupSchedule => &self.schedule,
            VestingConditions::Schedule(schedule) => schedule,
        }
        .unlocked_balance(termination_timestamp);
        let unvested_balance = total_balance - vested_balance;
        if unvested_balance > 0 {
            self.schedule.terminate(vested_balance, termination_timestamp);
        }
        (unvested_balance, termination_config.beneficiary_id)
    }
}
