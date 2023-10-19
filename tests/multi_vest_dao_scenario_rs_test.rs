use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");

    blockchain.register_contract("file:output/multi_vest_dao.wasm", multi_vest_dao::ContractBuilder);
    blockchain
}

#[test]
fn multi_vest_dao_rs() {
    world().run("scenarios/multi_vest_dao.scen.json");
}

#[test]
fn test_add_proposal_rs() {
    world().run("scenarios/test-add-proposal.scen.json");
}

#[test]
fn test_add_stake_rs() {
    world().run("scenarios/test-add-stake.scen.json");
}

// #[test]
#[test]
fn test_init_rs() {
    world().run("scenarios/test-init.scen.json");
}
