use Colony_Builder::world::hash2d;

#[test]
fn hash_is_deterministic() {
    assert_eq!(hash2d(0, 0, 0), hash2d(0, 0, 0));
}

#[test]
fn hash_changes_with_x() {
    assert!(hash2d(0, 0, 0) != hash2d(1, 0, 0));
}

#[test]
fn hash_changes_with_y() {
    assert!(hash2d(0, 0, 0) != hash2d(0, 1, 0));
}

#[test]
fn hash_changes_with_seed() {
    assert!(hash2d(0, 0, 0) != hash2d(0, 0, 1));
}

#[test]
fn hash_different_xy_produce_different_results() {
    let results: std::collections::HashSet<u32> = (0..100).map(|i| hash2d(i, i * 7, 0)).collect();
    assert!(results.len() > 90, "hash should produce varied outputs");
}
