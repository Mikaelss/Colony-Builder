use Colony_Builder::world::hash2d;

fn pick_variant(x: u32, y: u32, variant_count: u32) -> usize {
    (hash2d(x, y, 0) % variant_count) as usize
}

#[test]
fn single_variant_always_zero() {
    for x in 0..100 {
        for y in 0..100 {
            assert_eq!(pick_variant(x, y, 1), 0);
        }
    }
}

#[test]
fn two_variants_both_appear() {
    let mut seen = std::collections::HashSet::new();
    for i in 0..200 {
        seen.insert(pick_variant(i * 7, i * 13, 2));
    }
    assert!(seen.len() >= 2, "both variants should appear");
}

#[test]
fn three_variants_distributed() {
    let mut counts = [0usize; 3];
    for i in 0..1000 {
        let v = pick_variant(i, i * 3, 3);
        assert!(v < 3);
        counts[v] += 1;
    }
    for &count in &counts {
        assert!(count > 0, "each variant should appear at least once");
    }
}

#[test]
fn variant_deterministic() {
    assert_eq!(pick_variant(5, 10, 4), pick_variant(5, 10, 4));
}
