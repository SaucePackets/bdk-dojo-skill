// =============================================================================
// tests.rs — Lesson 5.3: Descriptor Mental Model
// =============================================================================
//
// This file contains pre-written tests for the descriptor classifier kata.
// Your job is to implement the stubs in lib.rs so all tests pass.
//
// HOW TO USE:
//   1. Update the `use` import below to match your Cargo.toml [package] name.
//   2. Implement the following in your crate:
//        - enum DescriptorKind { SingleSig, MultiSig, Unknown }
//        - enum WalletError (or any error type)
//        - fn classify_descriptor(descriptor: &str) -> DescriptorKind
//        - fn validate_toy_descriptor(descriptor: &str)
//              -> Result<DescriptorKind, WalletError>
//   3. Run: cargo test
//
// classify_descriptor never fails — it just returns Unknown for unrecognised
// input. validate_toy_descriptor returns Err for anything it cannot parse into
// a known DescriptorKind.
//
// Tip: a real descriptor begins with a script type keyword like "wpkh(",
// "pkh(", "sh(", "wsh(", "tr(", or a policy keyword like "sortedmulti(".
// Your classifier only needs to distinguish SingleSig from MultiSig at the
// toy level — no need to parse the full BIP-380 grammar.
// =============================================================================

// Update this import to match your Cargo.toml package name.
use your_crate_name::{classify_descriptor, validate_toy_descriptor, DescriptorKind};

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that classify_descriptor maps common descriptor prefixes to
    /// the right DescriptorKind variants, and that validate_toy_descriptor
    /// returns Ok for valid-looking descriptors and Err for garbage.
    #[test]
    fn descriptor_classifier_recognizes_common_policy_shapes() {
        // ── classify_descriptor ──────────────────────────────────────────────

        // A native-segwit single-key descriptor (wpkh) → SingleSig
        assert_eq!(
            classify_descriptor("wpkh(xpub6ERApfzkCKFGhKCCjJVGkb57cNJJDRn4BkMKVfGCfyNW61jTHBMoJrB4e4ybNMF5f4LDWQ7Kbf3cHixfVxC1iorhq9LoBLsTtq4K3Kfbmz/0/*)"),
            DescriptorKind::SingleSig,
            "wpkh(...) should be classified as SingleSig"
        );

        // Short-form input that still starts with "wpkh(" — classifier should
        // match on the prefix, not try to parse a real xpub.
        assert_eq!(
            classify_descriptor("wpkh(xpub...)"),
            DescriptorKind::SingleSig,
            "wpkh(xpub...) prefix should be classified as SingleSig"
        );

        // A sortedmulti policy → MultiSig
        assert_eq!(
            classify_descriptor("sortedmulti(2,xpubA,xpubB,xpubC)"),
            DescriptorKind::MultiSig,
            "sortedmulti(...) should be classified as MultiSig"
        );

        // A wsh-wrapped multi (another common multisig form) → MultiSig
        assert_eq!(
            classify_descriptor("wsh(multi(2,xpubA,xpubB,xpubC))"),
            DescriptorKind::MultiSig,
            "wsh(multi(...)) should be classified as MultiSig"
        );

        // Completely unrecognised input → Unknown
        assert_eq!(
            classify_descriptor("unknown_garbage"),
            DescriptorKind::Unknown,
            "Unrecognised input should classify as Unknown"
        );

        // Empty string → Unknown (edge case)
        assert_eq!(
            classify_descriptor(""),
            DescriptorKind::Unknown,
            "Empty string should classify as Unknown"
        );

        // ── validate_toy_descriptor ──────────────────────────────────────────

        // Valid-looking wpkh descriptor → Ok(SingleSig)
        assert_eq!(
            validate_toy_descriptor("wpkh(xpub...)"),
            Ok(DescriptorKind::SingleSig),
            "validate_toy_descriptor should return Ok(SingleSig) for wpkh(...)"
        );

        // Valid-looking sortedmulti descriptor → Ok(MultiSig)
        assert_eq!(
            validate_toy_descriptor("sortedmulti(2,xpubA,xpubB,xpubC)"),
            Ok(DescriptorKind::MultiSig),
            "validate_toy_descriptor should return Ok(MultiSig) for sortedmulti(...)"
        );

        // Unrecognised input → Err (any error variant is fine)
        assert!(
            validate_toy_descriptor("unknown_garbage").is_err(),
            "validate_toy_descriptor should return Err for unrecognised descriptors"
        );

        // Empty string → Err
        assert!(
            validate_toy_descriptor("").is_err(),
            "validate_toy_descriptor should return Err for empty input"
        );
    }
}
