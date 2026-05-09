use bdk_dojo_wallet::{balance_bridge_note, sync_bridge_note};

#[test]
fn bdk_bridge_notes_name_what_the_toy_model_hides() {
    let balance = balance_bridge_note();
    let sync = sync_bridge_note();

    assert!(balance.bdk_concept.contains("BDK"));
    assert!(balance.what_toy_hides.contains("Descriptors"));
    assert!(sync.bdk_concept.contains("sync"));
    assert!(sync.what_toy_hides.contains("chain source"));
}
