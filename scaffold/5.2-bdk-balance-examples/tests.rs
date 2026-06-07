// =============================================================================
// tests.rs — Lesson 5.2: BDK Balance Examples
// =============================================================================
//
// This file contains pre-written tests for the BDK balance bridge-note kata.
// Your job is to implement the stubs in lib.rs (or main.rs) so all tests pass.
//
// HOW TO USE:
//   1. Update the `use` import below to match your Cargo.toml [package] name.
//   2. Implement the following in your crate:
//        - struct BdkBridgeNote { toy_concept, bdk_concept, what_toy_hides }
//        - fn balance_bridge_note() -> BdkBridgeNote
//   3. Run: cargo test
//
// The test verifies that your bridge note captures what the toy "simple balance"
// model omits about BDK's real Balance type (immature coinbase UTXOs, chain
// vs. mempool split, indexed vs. trusted state, etc.).
// =============================================================================

// Update this import to match your Cargo.toml package name.
use your_crate_name::{balance_bridge_note, BdkBridgeNote};

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that `balance_bridge_note()` returns a fully-populated
    /// BdkBridgeNote and that `what_toy_hides` explicitly calls out at least
    /// one detail that a naïve "single u64 balance" model omits.
    ///
    /// Acceptable keywords in `what_toy_hides`: "immature", "chain",
    /// "indexed", "trusted", "untrusted", "pending", "confirmed".
    /// The test checks for "immature" — if you'd rather use a different term,
    /// adjust the assert below and note your reasoning in a comment.
    #[test]
    fn bdk_bridge_notes_name_what_the_toy_model_hides() {
        let note: BdkBridgeNote = balance_bridge_note();

        // All three fields must be non-empty strings.
        assert!(
            !note.toy_concept.is_empty(),
            "toy_concept should describe what the toy model calls 'balance'"
        );
        assert!(
            !note.bdk_concept.is_empty(),
            "bdk_concept should name the BDK type or field (e.g. 'Balance' struct)"
        );
        assert!(
            !note.what_toy_hides.is_empty(),
            "what_toy_hides must explain what the toy model omits"
        );

        // The explanation must mention at least one of the real distinctions
        // BDK's Balance type makes that a simple u64 does not.
        // "immature" refers to coinbase outputs that are not yet spendable;
        // this is a concrete, factually correct omission of the toy model.
        let hidden = note.what_toy_hides.to_lowercase();
        let mentions_real_distinction = hidden.contains("immature")
            || hidden.contains("chain")
            || hidden.contains("indexed")
            || hidden.contains("trusted")
            || hidden.contains("untrusted")
            || hidden.contains("pending")
            || hidden.contains("confirmed");

        assert!(
            mentions_real_distinction,
            "what_toy_hides should mention at least one BDK-specific balance \
             distinction (immature, chain, indexed, trusted, untrusted, pending, \
             or confirmed). Got: {:?}",
            note.what_toy_hides
        );
    }
}
