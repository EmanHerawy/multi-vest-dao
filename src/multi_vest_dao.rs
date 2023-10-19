#![no_std]
use multiversx_sc_modules::pause;
use multiversx_sc::types::{ BigUint};
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

/// A DAO to fund promising projects at multiversx ecosystem
#[multiversx_sc::contract]
pub trait MultiVestDao: pause::PauseModule{
    #[init]
    fn init(&self, minimum_stake_value:BigUint  ) {
        // Set the minimum stake value
        self.minimum_stake().set(minimum_stake_value);
    }

   #[endpoint]
 
fn vote(&self, proposal_id: u64, vote: bool) {
    // Get the caller's address
    let caller = self.blockchain().get_caller();

    // Get the proposal by ID
    let mut proposal = self.proposal_details(proposal_id).get();
    
    // Ensure that the caller hasn't voted before
    require!(!proposal.voters.contains(&caller), "Caller has already voted on this proposal.");

    // Check that the proposal is not expired or executed
    require!(proposal.proposal_deadline > self.blockchain().get_block_timestamp(), "Proposal is expired.");
    // Check that the caller is eligible to vote (add your eligibility criteria here)
    require!(self.is_eligible_to_vote(&caller), "Caller is not eligible to vote.");

    // Update the proposal vote counts
    if vote {
        proposal.yes_vote_count += 1;
    } else {
        proposal.no_vote_count += 1;
    }
    proposal.total_votes_count += 1;

    // Add the caller to the list of voters
    proposal.voters.push(caller);

    // Update the proposal in storage
    self.proposal_details(proposal_id).set(&proposal);
}

   #[endpoint]
fn execute(&self, proposal_id: u64) {
    // Get the proposal by ID
    let mut proposal = self.proposal_details(proposal_id).get();
    
    // Check if the proposal in execution time
    require!(proposal.proposal_deadline < self.blockchain().get_block_timestamp(), "Proposal is in voting time.");
    // Check if the proposal is not executed, and the yes vote count is greater than the no vote count
    require!(!proposal.is_excuted, "Proposal has already been executed.");
    require!(proposal.yes_vote_count > proposal.no_vote_count, "Proposal did not receive enough yes votes.");
    
    // Check if there's enough balance in the treasury to execute the proposal
    let treasury_balance = self.balance().get();
    require!(treasury_balance >= proposal.requested_amount, "Insufficient funds in the treasury.");
    
    // Update the proposal status to "Executed"
    proposal.is_excuted = true;
    
    // Deduct the requested amount from the treasury
    self.balance().set(treasury_balance - &proposal.requested_amount);
    
    // Update the proposal in storage
    self.proposal_details(proposal_id).set(&proposal);
    
    // Perform the actions associated with executing the proposal
       self.send().direct_egld(&proposal.proposer, &proposal.requested_amount);
}
    

#[endpoint]
fn propose(&self, proposal_title: ManagedBuffer, proposal_off_chain_link: ManagedBuffer, requested_amount: BigUint) {
    // Get the caller's address
    let caller = self.blockchain().get_caller();

    // Only token holders can propose (add your eligibility criteria here)
     require!(self.is_eligible_to_propose(&caller) , "Caller is not eligible to propose.");
    // Create a new proposal with the provided details
    let proposal = Proposal {
        
        proposal_title,
        proposal_off_chain_link,
        requested_amount,
        proposer: caller,
        yes_vote_count: 0,
        no_vote_count: 0,
        total_votes_count: 0,
        proposal_deadline: self.blockchain().get_block_timestamp() + 10000000, // Set the proposal deadline
        voters: ManagedVec::new(), // Initialize voters as an empty array
        is_excuted: false //
    };

    // Set the proposal in storage
    self.proposal_details(self.total_proposals().get()).set(&proposal);

    // Increment the total number of proposals
    self.total_proposals().set(self.total_proposals().get() + 1);
}

    // any toen holder can stake 
    #[payable("EGLD")]
    #[endpoint]
fn stake(&self) {
    // Get the staking amount from the call value (EGLD)
    let staking_amount = self.call_value().egld_value();
    // Get the caller's address
    let caller = self.blockchain().get_caller();
    // Get the caller's current balance
    let mut caller_balance = self.user_balance(&caller).get();

    // Calculate the new balance by adding the staking amount
    caller_balance = caller_balance.add(staking_amount.to_u64().unwrap()); // we need to check the data type , we might have overflow issue

    // Update the caller's balance in storage
    self.user_balance(&caller).set(caller_balance);
}
    




// user to stake balance 
    #[view(lockedTokens)]
    #[storage_mapper("user_balance")]
    fn user_balance(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;
    #[view(minimumStake)]
    #[storage_mapper("minimum_stake")]
    fn minimum_stake(&self) -> SingleValueMapper<BigUint>;
    // #[view(proposals)]
    // #[storage_mapper("proposals")]
    // fn proposals(&self) -> UnorderedSetMapper<Proposal<Self::Api>>;

    #[view(totalProposals)]
    #[storage_mapper("total_proposals")]
    fn total_proposals(&self) -> SingleValueMapper<u64>;

    #[view(ProposalDetails)]
    #[storage_mapper("proposal_details")]
    fn proposal_details(&self,index :u64 ) -> SingleValueMapper<Proposal<Self::Api>>;

    #[view(totalMembers)]
    #[storage_mapper("total_members")]
    fn total_members(&self) -> SingleValueMapper<u64>;
 
    /// total tokens locked as DAO treasury
    #[view(treasuryBalance)]
    #[storage_mapper("balance")]
    fn balance(&self) -> SingleValueMapper<BigUint>;



    // Implement your eligibility criteria for voting and proposing
fn is_eligible_to_vote(&self, caller: &ManagedAddress) -> bool {
     self.user_balance(&caller).get()>=self.minimum_stake().get() // Example: Allow only users with at least minimum_stake to vote
}

fn is_eligible_to_propose(&self, caller: &ManagedAddress) -> bool {
   
    self.blockchain().get_balance(caller)> 1000000000000000001u64 // Example: Allow only users with at least 1 EGLD to vote
}
}
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum ProposalStatus {
    Proposed,
    Approved,
    Rejected,
    Executed,
    Cancelled,
    Expired,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Proposal<M: ManagedTypeApi>{
// pub index: u64,
pub proposal_title: ManagedBuffer<M>,
// the proposal details will be stored off chain, we need to get the fingerprint of the proposal details
pub proposal_off_chain_link: ManagedBuffer<M>,
pub requested_amount: BigUint<M>,
pub proposer: ManagedAddress<M>,
pub yes_vote_count: u64,
pub no_vote_count: u64,
pub total_votes_count: u64,
pub proposal_deadline: u64,
pub voters :  ManagedVec<M, ManagedAddress<M>>,
pub is_excuted: bool,

}
// struct Member{

// }