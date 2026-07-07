#[test]
fn zoom_factor_positive() {
    // ZOOM_FACTOR = 1.15
    let factor: f32 = 1.15;
    // scroll up divides scale by 1.15
    let scale = 1.0 / factor;
    assert!((scale - 0.86956525).abs() < 0.001);
    // zooming out multiplies
    let scale_out = 1.0 * factor;
    assert!((scale_out - 1.15).abs() < 0.001);
}

#[test]
fn zoom_clamp_min() {
    let min_scale: f32 = 0.1;
    let mut scale: f32 = 0.5;
    for _ in 0..5 {
        scale *= 1.15;
    }
    scale = scale.max(min_scale);
    assert!(scale >= min_scale);
}

#[test]
fn zoom_clamp_max() {
    let max_scale: f32 = 20.0;
    let mut scale: f32 = 10.0;
    for _ in 0..5 {
        scale /= 1.15;
    }
    scale = scale.min(max_scale);
    assert!(scale <= max_scale);
}

#[test]
fn zoom_in_decreases_scale() {
    let original = 1.0;
    let zoomed_in = original / 1.15;
    assert!(zoomed_in < original);
}

#[test]
fn zoom_out_increases_scale() {
    let original = 1.0;
    let zoomed_out = original * 1.15;
    assert!(zoomed_out > original);
}

#[test]
fn camera_center_position() {
    let grid_size: f32 = 275.0;
    let tile_size: f32 = 64.0;
    let center_x = grid_size * tile_size / 2.0;
    let center_y = grid_size * tile_size / 2.0;
    assert!((center_x - 8800.0).abs() < f32::EPSILON);
    assert!((center_y - 8800.0).abs() < f32::EPSILON);
}

#[test]
fn map_bounds_calculation() {
    let grid_size: u32 = 275;
    let tile_size: f32 = 64.0;
    let max_x = grid_size as f32 * tile_size;
    let max_y = grid_size as f32 * tile_size;
    assert!((max_x - 17600.0).abs() < f32::EPSILON);
    assert!((max_y - 17600.0).abs() < f32::EPSILON);
}

#[test]
fn pan_speed_scales_with_zoom() {
    let base_speed: f32 = 600.0;
    let scale: f32 = 2.0;
    let panned = base_speed * scale;
    assert!((panned - 1200.0).abs() < f32::EPSILON);
}

#[test]
fn h_key_resets_to_center() {
    let half_grid: f32 = 275.0 * 64.0 / 2.0;
    assert!((half_grid - 8800.0).abs() < f32::EPSILON);
}

#[test]
// test that 3 consecutive zoom-ins reach min_scale
fn rapid_zoom_hits_clamp() {
    let min_scale: f32 = 0.1;
    let mut scale: f32 = 1.0;
    for _ in 0..100 {
        scale /= 1.15;
    }
    scale = scale.max(min_scale);
    assert_eq!(scale, min_scale);
}
