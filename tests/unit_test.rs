use multi_vest_dao::*;
use multiversx_sc::types::BigUint;
use multiversx_sc_scenario::api::SingleTxApi;


#[test]
fn dao_unit_test() {
    let dao = multi_vest_dao::contract_obj::<SingleTxApi>();

dao.stake();
        /*
    adder.init(BigUint::from(5u32));
    assert_eq!(BigUint::from(5u32), adder.sum().get());

    adder.add(BigUint::from(7u32));
    assert_eq!(BigUint::from(12u32), adder.sum().get());

    adder.add(BigUint::from(1u32));
    assert_eq!(BigUint::from(13u32), adder.sum().get());*/
}
