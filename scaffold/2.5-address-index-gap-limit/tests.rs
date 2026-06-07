// tests/lesson_2_5_address_index_gap_limit.rs
//
// Expected behavior verification for lesson 2.5 — Address Index Gap Limit.
// Copy this file into your project's tests/ directory and run:
//
//   cargo test --test lesson_2_5_address_index_gap_limit
//
// The tests will fail with a `todo!` panic until you implement
// `WalletState::next_unused_address` in src/wallet.rs.
//
// WalletState fields available from this lesson onward:
//   utxos: Vec<Utxo>, tip_height: u32, checkpoints: Vec<u32>,
//   addresses: Vec<AddressRecord>
//
// AddressRecord { index: u32, address: String, used: bool }
//
// next_unused_address() must:
//   - Return the first AddressRecord where used == false, if one exists
//   - If none exists, derive a new one at the next index and append it
//   - Consecutive calls return the SAME address until it is marked used

// Update this import to match your Cargo.toml package name.
use your_crate_name::{WalletState};

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Verifies the full gap-limit reuse cycle:
///
///  1. First call → derives and returns address at index 0, used=false
///  2. Second call → returns the SAME index-0 address (not yet used)
///  3. Mark addresses[0].used = true
///  4. Third call → derives and returns a new address at index 1
#[test]
fn next_unused_address_reuses_until_marked_used_then_derives_next() {
    let mut wallet = WalletState::new(800_000);

    // ------------------------------------------------------------------
    // 1. First call: wallet has no addresses yet; index 0 is derived.
    // ------------------------------------------------------------------
    let addr0_first = wallet.next_unused_address();

    assert_eq!(
        addr0_first.index, 0,
        "first address derived must have index 0"
    );
    assert!(
        !addr0_first.used,
        "freshly derived address must start as unused"
    );
    assert!(
        !addr0_first.address.is_empty(),
        "derived address string must not be empty"
    );

    // ------------------------------------------------------------------
    // 2. Second call: index 0 is still unused; the same address is reused.
    // ------------------------------------------------------------------
    let addr0_second = wallet.next_unused_address();

    assert_eq!(
        addr0_second.index, 0,
        "second call must return the same index-0 address (not yet used)"
    );
    assert_eq!(
        addr0_second.address, addr0_first.address,
        "address string must be identical on reuse"
    );

    // ------------------------------------------------------------------
    // 3. Mark index-0 address as used.
    // ------------------------------------------------------------------
    wallet.addresses[0].used = true;

    // ------------------------------------------------------------------
    // 4. Third call: index 0 is now used; a new address at index 1 is derived.
    // ------------------------------------------------------------------
    let addr1 = wallet.next_unused_address();

    assert_eq!(
        addr1.index, 1,
        "after marking index 0 used, next call must return index 1"
    );
    assert!(
        !addr1.used,
        "newly derived index-1 address must start as unused"
    );
    assert!(
        !addr1.address.is_empty(),
        "derived index-1 address string must not be empty"
    );
    // The new address should be distinct from the index-0 address.
    assert_ne!(
        addr1.address, addr0_first.address,
        "index-1 address must differ from index-0 address"
    );
}
