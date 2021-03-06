use crate::{Address, U128};
use ontio_std::abi::EventBuilder;

pub fn new_pending_admin_event(new_pending_admin: &Address) {
    EventBuilder::new()
        .string("setPendingAdmin")
        .address(new_pending_admin)
        .notify();
}

pub fn new_admin_event(old_admin: &Address, new_pending_admin: &Address) {
    EventBuilder::new()
        .string("acceptAdmin")
        .address(old_admin)
        .address(new_pending_admin)
        .notify();
}

pub fn transfer_token_pair_owner_evt(old_owner: &Address, new_owner: &Address) {
    EventBuilder::new()
        .string("transferTokenPairOwner")
        .address(old_owner)
        .address(new_owner)
        .notify();
}

pub fn oep4_to_orc20_event(
    ont_acct: &Address,
    eth_acct: &Address,
    amount: U128,
    erc20_amt: U128,
    oep4_addr: &Address,
    erc20_addr: &Address,
) {
    EventBuilder::new()
        .string("oep4ToOrc20")
        .address(ont_acct)
        .address(eth_acct)
        .number(amount)
        .number(erc20_amt)
        .address(oep4_addr)
        .address(erc20_addr)
        .notify();
}

pub fn orc20_to_oep4_event(
    ont_acct: &Address,
    eth_acct: &Address,
    amount: U128,
    oep4_amt: U128,
    oep4_addr: &Address,
    erc20_addr: &Address,
) {
    EventBuilder::new()
        .string("orc20ToOep4")
        .address(ont_acct)
        .address(eth_acct)
        .number(amount)
        .number(oep4_amt)
        .address(oep4_addr)
        .address(erc20_addr)
        .notify();
}
