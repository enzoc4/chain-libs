mod builders;

use crate::{
    certificate::{ExternalProposalId, PoolPermissions, Proposal, Proposals, VoteAction, VotePlan},
    header::BlockDate,
    rewards::TaxType,
    value::Value,
    vote::{Options, PayloadType},
};
pub use builders::*;
use chain_crypto::{Ed25519, PublicKey};
use chain_vote::MemberPublicKey;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct WalletTemplate {
    pub alias: String,
    pub stake_pool_delegate_alias: Option<String>,
    pub stake_pool_owner_alias: Option<String>,
    pub initial_value: Value,
    pub committee_member: bool,
}

impl PartialEq for WalletTemplate {
    fn eq(&self, other: &WalletTemplate) -> bool {
        self.alias == other.alias
    }
}

impl Hash for WalletTemplate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.alias.hash(state)
    }
}

impl Eq for WalletTemplate {}

impl WalletTemplate {
    pub fn new(alias: &str, initial_value: Value) -> Self {
        WalletTemplate {
            alias: alias.to_owned(),
            stake_pool_delegate_alias: None,
            stake_pool_owner_alias: None,
            initial_value,
            committee_member: false,
        }
    }

    pub fn delegates_stake_pool(&self) -> Option<String> {
        self.stake_pool_delegate_alias.clone()
    }

    pub fn owns_stake_pool(&self) -> Option<String> {
        self.stake_pool_owner_alias.clone()
    }

    pub fn is_committee_member(&self) -> bool {
        self.committee_member
    }

    pub fn alias(&self) -> String {
        self.alias.clone()
    }
}

#[derive(Clone, Debug)]
pub struct StakePoolTemplate {
    pub alias: String,
    pub owners: Vec<PublicKey<Ed25519>>,
}

impl StakePoolTemplate {
    pub fn alias(&self) -> String {
        self.alias.clone()
    }

    pub fn owners(&self) -> Vec<PublicKey<Ed25519>> {
        self.owners.clone()
    }
}

#[derive(Clone, Debug)]
pub struct StakePoolDef {
    pub alias: String,
    pub permissions_threshold: Option<u8>,
    pub has_reward_account: bool,
    pub tax_type: Option<TaxType>,
}

impl StakePoolDef {
    pub fn pool_permission(&self) -> Option<PoolPermissions> {
        self.permissions_threshold.map(PoolPermissions::new)
    }
}

#[derive(Clone, Debug)]
pub struct VotePlanDef {
    alias: String,
    owner_alias: String,
    payload_type: PayloadType,
    vote_date: BlockDate,
    tally_date: BlockDate,
    end_tally_date: BlockDate,
    committee_keys: Vec<MemberPublicKey>,
    proposals: Vec<ProposalDef>,
}

impl VotePlanDef {
    pub fn alias(&self) -> String {
        self.alias.clone()
    }

    pub fn owner(&self) -> String {
        self.owner_alias.clone()
    }

    pub fn proposals(&self) -> Vec<ProposalDef> {
        self.proposals.iter().cloned().map(Into::into).collect()
    }

    pub fn proposal(&self, index: usize) -> ProposalDef {
        self.proposals.get(index).unwrap().clone()
    }

    pub fn id(&self) -> String {
        let vote_plan: VotePlan = self.clone().into();
        vote_plan.to_id().to_string()
    }

    pub fn committee_keys(&self) -> Vec<MemberPublicKey> {
        self.committee_keys.clone()
    }

    pub fn committee_keys_mut(&mut self) -> &mut Vec<MemberPublicKey> {
        &mut self.committee_keys
    }
}

impl From<VotePlanDef> for VotePlan {
    fn from(dto: VotePlanDef) -> Self {
        let mut proposals = Proposals::new();
        for proposal in dto.proposals.iter().cloned() {
            let _ = proposals.push(proposal.into());
        }

        VotePlan::new(
            dto.vote_date,
            dto.tally_date,
            dto.end_tally_date,
            proposals,
            dto.payload_type,
            dto.committee_keys,
        )
    }
}

#[derive(Clone, Debug)]
pub struct ProposalDef {
    id: ExternalProposalId,
    options: u8,
    action_type: VoteAction,
}

impl ProposalDef {
    pub fn id(&self) -> ExternalProposalId {
        self.id.clone()
    }
}

impl From<ProposalDef> for Proposal {
    fn from(dto: ProposalDef) -> Self {
        Proposal::new(
            dto.id,
            Options::new_length(dto.options).unwrap(),
            dto.action_type,
        )
    }
}
