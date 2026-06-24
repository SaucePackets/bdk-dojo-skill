// lesson_5_1_bdk_project_orientation.rs — Lesson 5.1: BDK Project Orientation
//
// This file contains pre-written tests for the BDK project orientation bridge-note kata.
// Run them with:
//
//   cargo test
//
// Update this import to match your Cargo.toml package name.
use your_crate_name::{balance_bridge_note, BdkBridgeNote};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bdk_bridge_notes_name_what_the_toy_model_hides() {
        let note: BdkBridgeNote = balance_bridge_note();

        assert!(!note.toy_concept.is_empty());
        assert!(!note.bdk_concept.is_empty());
        assert!(!note.what_toy_hides.is_empty());

        let hidden = note.what_toy_hides.to_lowercase();
        let mentions_real_distinction = hidden.contains("descriptor")
            || hidden.contains("script")
            || hidden.contains("chain")
            || hidden.contains("transaction")
            || hidden.contains("persistence")
            || hidden.contains("indexed")
            || hidden.contains("immature");

        assert!(
            mentions_real_distinction,
            "what_toy_hides should mention at least one real BDK/wallet distinction. Got: {:?}",
            note.what_toy_hides
        );
    }
}
