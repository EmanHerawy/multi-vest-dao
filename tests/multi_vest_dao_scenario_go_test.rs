use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn multi_vest_dao_go() {
    world().run("scenarios/multi_vest_dao.scen.json");
}

#[test]
fn test_add_proposal_go() {
    world().run("scenarios/test-add-proposal.scen.json");
}

#[test]
fn test_add_stake_go() {
    world().run("scenarios/test-add-stake.scen.json");
}

#[test]
fn test_init_go() {
    world().run("scenarios/test-init.scen.json");
}
