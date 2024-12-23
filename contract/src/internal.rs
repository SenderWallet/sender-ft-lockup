use std::collections::HashSet;

use hodl_model::lockup::{Lockup, LockupIndex};

use crate::{AccountId, Contract};

impl Contract {
    pub(crate) fn assert_deposit_whitelist(&self, account_id: &AccountId) {
        assert!(self.deposit_whitelist.contains(account_id), "Not in deposit whitelist");
    }

    pub(crate) fn assert_draft_operators_whitelist(&self, account_id: &AccountId) {
        assert!(
            (self.deposit_whitelist.contains(account_id) || self.draft_operators_whitelist.contains(account_id)),
            "Not in draft operators whitelist"
        );
    }

    pub(crate) fn internal_add_lockup(&mut self, lockup: &Lockup) -> LockupIndex {
        let index = LockupIndex::try_from(self.lockups.len()).unwrap();
        self.lockups.push(lockup);
        let mut indices = self.account_lockups.get(&lockup.account_id).unwrap_or_default();
        indices.insert(index);
        self.internal_save_account_lockups(&lockup.account_id, indices);
        index
    }

    pub(crate) fn internal_save_account_lockups(&mut self, account_id: &AccountId, indices: HashSet<LockupIndex>) {
        if indices.is_empty() {
            self.account_lockups.remove(account_id);
        } else {
            self.account_lockups.insert(account_id, &indices);
        }
    }

    pub(crate) fn internal_get_account_lockups(&self, account_id: &AccountId) -> Vec<(LockupIndex, Lockup)> {
        self.account_lockups
            .get(account_id)
            .unwrap_or_default()
            .into_iter()
            .map(|lockup_index| (lockup_index, self.lockups.get(u64::from(lockup_index)).unwrap()))
            .collect()
    }

    pub(crate) fn internal_get_account_lockups_by_id(
        &self,
        account_id: &AccountId,
        lockup_ids: &HashSet<LockupIndex>,
    ) -> Vec<(LockupIndex, Lockup)> {
        let account_lockup_ids = self.account_lockups.get(account_id).unwrap_or_default();

        lockup_ids
            .iter()
            .map(|&lockup_index| {
                assert!(
                    account_lockup_ids.contains(&lockup_index),
                    "lockup not found for account: {lockup_index}",
                );
                let lockup = self.lockups.get(u64::from(lockup_index)).unwrap();
                (lockup_index, lockup)
            })
            .collect()
    }
}
