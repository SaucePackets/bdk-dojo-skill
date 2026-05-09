use bdk_dojo_wallet::{balance_bridge_note, sync_bridge_note};

#[test]
fn bdk_bridge_notes_name_what_the_toy_model_hides() {
    let balance = balance_bridge_note();
    let sync = sync_bridge_note();

    assert!(balance.bdk_concept.contains("bdk_chain::Balance"));
    assert!(balance.what_toy_hides.contains("immature coinbase"));
    assert!(sync.bdk_concept.contains("full_scan/sync"));
    assert!(sync.what_toy_hides.contains("chain source"));
}
