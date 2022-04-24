#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]

extern crate ontio_std as ostd;

use ostd::abi::{Sink, Source};
use ostd::prelude::*;
use ostd::runtime::{input, ret};

mod erc20;
mod events;
mod oep4;
mod bridge;

use bridge::*;
use crate::erc20::mock_run;

#[no_mangle]
pub fn invoke() {
    let input = input();
    let mut source = Source::new(&input);
    let action = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        "init" => {
            let admin = source.read().unwrap();
            sink.write(initialize(admin))
        }
        "getAdmin" => {
            sink.write(get_admin());
        }
        "setPendingAdmin" => {
            let new_admin = source.read().unwrap();
            sink.write(set_pending_admin(new_admin));
        }
        "getPendingAdmin" => {
            sink.write(get_pending_admin());
        }
        "acceptAdmin" => {
            sink.write(accept_admin());
        }
        "registerTokenPair" => {
            let (token_pair_name, oep4_addr, oep4_decimals, erc20_addr, erc20_decimals) =
                source.read().unwrap();
            sink.write(register_token_pair(
                token_pair_name,
                oep4_addr,
                oep4_decimals,
                erc20_addr,
                erc20_decimals,
            ))
        }
        "transferTokenPairOwner" => {
            let (token_pair_name, new_owner) = source.read().unwrap();
            sink.write(transfer_token_pair_owner(token_pair_name, new_owner))
        }
        "updateTokenPair" => {
            let (
                token_pair_name,
                oep4_addr,
                oep4_decimals,
                erc20_addr,
                erc20_decimals,
                eth_acct,
                ont_acct,
            ) = source.read().unwrap();
            sink.write(update_token_pair(
                token_pair_name,
                oep4_addr,
                oep4_decimals,
                erc20_addr,
                erc20_decimals,
                eth_acct,
                ont_acct,
            ))
        }
        "getAllTokenPairName" => {
            sink.write(get_all_token_pair_name());
        }
        "getTokenPair" => {
            let token_pair_name = source.read().unwrap();
            sink.write(get_token_pair(token_pair_name));
        }
        "migrate" => {
            let (code, vm_type, name, version, author, email, desc) = source.read().unwrap();
            let vm_type: U128 = vm_type;
            sink.write(migrate(
                code,
                vm_type.raw() as u32,
                name,
                version,
                author,
                email,
                desc,
            ));
        }
        "oep4ToOrc20" => {
            let (ont_acct, eth_acct, amount, token_pair_name) = source.read().unwrap();
            sink.write(oep4_to_orc20(ont_acct, eth_acct, amount, token_pair_name));
        }
        "orc20ToOep4" => {
            let (ont_acct, eth_acct, amount, token_pair_name) = source.read().unwrap();
            sink.write(orc20_to_oep4(ont_acct, eth_acct, amount, token_pair_name));
        }
        _ => panic!("unsupported action mainnet!"),
    }

    ret(sink.bytes())
}
