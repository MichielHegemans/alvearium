use crate::operation::{Custom, CustomJson, Vote};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum Operation {
    #[serde(rename = "vote_operation")]
    Vote(Vote),
    #[serde(rename = "comment_operation")]
    Comment(Value),
    #[serde(rename = "transfer_operation")]
    Transfer(Value),
    #[serde(rename = "transfer_to_vesting_operation")]
    TransferToVesting(Value),
    #[serde(rename = "withdraw_vesting_operation")]
    WithdrawVesting(Value),
    #[serde(rename = "limit_order_create_operation")]
    LimitOrderCreate(Value),
    #[serde(rename = "limit_order_cancel_operation")]
    LimitOrderCancel(Value),
    #[serde(rename = "feed_publish_operation")]
    FeedPublish(Value),
    #[serde(rename = "convert_operation")]
    Convert(Value),
    #[serde(rename = "account_create_operation")]
    AccountCreate(Value),
    #[serde(rename = "account_update_operation")]
    AccountUpdate(Value),
    #[serde(rename = "witness_update_operation")]
    WitnessUpdate(Value),
    #[serde(rename = "account_witness_vote_operation")]
    AccountWitnessVote(Value),
    #[serde(rename = "account_witness_proxy_operation")]
    AccountWitnessProxy(Value),
    #[serde(rename = "pow_operation")]
    Pow(Value),
    #[serde(rename = "custom_operation")]
    Custom(Custom),
    #[serde(rename = "report_over_production_operation")]
    ReportOverProduction(Value),
    #[serde(rename = "delete_comment_operation")]
    DeleteComment(Value),
    #[serde(rename = "custom_json_operation")]
    CustomJson(CustomJson),
    #[serde(rename = "comment_options_operation")]
    CommentOptions(Value),
    #[serde(rename = "set_withdraw_vesting_route_operation")]
    SetWithdrawVestingRoute(Value),
    #[serde(rename = "limit_order_create2_operation")]
    LimitOrderCreate2(Value),
    #[serde(rename = "claim_account_operation")]
    ClaimAccount(Value),
    #[serde(rename = "create_claimed_account_operation")]
    CreateClaimedAccount(Value),
    #[serde(rename = "request_account_recovery_operation")]
    RequestAccountRecovery(Value),
    #[serde(rename = "recovery_account_operation")]
    RecoverAccount(Value),
    #[serde(rename = "change_recovery_account_operation")]
    ChangeRecoveryAccount(Value),
    #[serde(rename = "escrow_transfer_operation")]
    EscrowTransfer(Value),
    #[serde(rename = "escrow_dispute_operation")]
    EscrowDispute(Value),
    #[serde(rename = "escrow_release_operation")]
    EscrowRelease(Value),
    #[serde(rename = "pow2_operation")]
    Pow2(Value),
    #[serde(rename = "escrow_approve_operation")]
    EscrowApprove(Value),
    #[serde(rename = "transfer_to_savings_operation")]
    TransferToSavings(Value),
    #[serde(rename = "transfer_from_savings_operation")]
    TransferFromSavings(Value),
    #[serde(rename = "cancel_transfer_from_savings_operation")]
    CancelTransferFromSavings(Value),
    #[serde(rename = "custom_binary_operation")]
    CustomBinary(Value),
    #[serde(rename = "decline_voting_rights_operation")]
    DeclineVotingRights(Value),
    #[serde(rename = "reset_account_operation")]
    ResetAccount(Value),
    #[serde(rename = "set_reset_account_operation")]
    SetResetAccount(Value),
    #[serde(rename = "claim_reward_balance_operation")]
    ClaimRewardBalance(Value),
    #[serde(rename = "delegate_vesting_shares_operation")]
    DelegateVestingShares(Value),
    #[serde(rename = "account_create_with_delegation_operation")]
    AccountCreateWithDelegation(Value),
    #[serde(rename = "witness_set_properties_operation")]
    WitnessSetProperties(Value),
    #[serde(rename = "account_update2_operation")]
    AccountUpdate2(Value),
    #[serde(rename = "create_proposal_operation")]
    CreateProposal(Value),
    #[serde(rename = "update_proposal_votes_operation")]
    UpdateProposalVotes(Value),
    #[serde(rename = "remove_proposal_operation")]
    RemoveProposal(Value),
    #[serde(rename = "update_proposal_operation")]
    UpdateProposal(Value),
    #[serde(rename = "collateralized_convert_operation")]
    CollateralizedConvert(Value),
    #[serde(rename = "recurrent_transfer_operation")]
    RecurrentTransfer(Value),
}
