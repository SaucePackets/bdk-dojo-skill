// lesson_5_5_bdk_sync_example.rs — Lesson 5.5: BDK Sync Example
//
// This file contains pre-written tests for the BDK sync bridge-note kata.
// Run them with:
//
//   cargo test
//
// Update this import to match your Cargo.toml package name.
use your_crate_name::{sync_bridge_note, BdkBridgeNote};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sync_bridge_note_names_real_bdk_sync_concepts() {
        let note: BdkBridgeNote = sync_bridge_note();

        assert!(!note.toy_concept.is_empty());
        assert!(!note.bdk_concept.is_empty());
        assert!(!note.what_toy_hides.is_empty());

        let bdk_concept = note.bdk_concept.to_lowercase();
        assert!(
            bdk_concept.contains("sync") || bdk_concept.contains("full_scan"),
            "bdk_concept should mention sync or full_scan. Got: {:?}",
            note.bdk_concept
        );

        let hidden = note.what_toy_hides.to_lowercase();
        let mentions_real_sync_detail = hidden.contains("chain")
            || hidden.contains("script")
            || hidden.contains("anchor")
            || hidden.contains("reorg")
            || hidden.contains("persistence")
            || hidden.contains("source");

        assert!(
            mentions_real_sync_detail,
            "what_toy_hides should mention at least one real sync detail. Got: {:?}",
            note.what_toy_hides
        );
    }
}
