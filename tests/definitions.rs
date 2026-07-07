use Colony_Builder::core::definitions::item::{ItemCategory, ItemDef, ItemRegistry};

fn make_item(id: &str) -> ItemDef {
    ItemDef {
        id: id.to_string(),
        name: format!("Item {}", id),
        category: ItemCategory::Raw,
        stack_size: 10,
    }
}

#[test]
fn registry_starts_empty() {
    let reg = ItemRegistry::default();
    assert_eq!(reg.count(), 0);
}

#[test]
fn register_new_item_succeeds() {
    let mut reg = ItemRegistry::default();
    let result = reg.register(make_item("stone"));
    assert!(result.is_ok());
    assert_eq!(reg.count(), 1);
}

#[test]
fn register_duplicate_fails() {
    let mut reg = ItemRegistry::default();
    reg.register(make_item("stone")).unwrap();
    let result = reg.register(make_item("stone"));
    assert!(result.is_err());
    assert_eq!(reg.count(), 1);
}

#[test]
fn get_returns_none_for_unknown() {
    let reg = ItemRegistry::default();
    assert!(reg.get("unknown").is_none());
}

#[test]
fn get_returns_registered_item() {
    let mut reg = ItemRegistry::default();
    reg.register(make_item("stone")).unwrap();
    let item = reg.get("stone");
    assert!(item.is_some());
    assert_eq!(item.unwrap().id, "stone");
}

#[test]
fn get_preserves_all_fields() {
    let mut reg = ItemRegistry::default();
    let def = ItemDef {
        id: "iron_ore".into(),
        name: "Iron Ore".into(),
        category: ItemCategory::Raw,
        stack_size: 50,
    };
    reg.register(def.clone()).unwrap();
    let retrieved = reg.get("iron_ore").unwrap();
    assert_eq!(retrieved.id, "iron_ore");
    assert_eq!(retrieved.name, "Iron Ore");
    assert_eq!(retrieved.category, ItemCategory::Raw);
    assert_eq!(retrieved.stack_size, 50);
}

#[test]
fn count_increments_per_item() {
    let mut reg = ItemRegistry::default();
    reg.register(make_item("a")).unwrap();
    reg.register(make_item("b")).unwrap();
    reg.register(make_item("c")).unwrap();
    assert_eq!(reg.count(), 3);
}

#[test]
fn count_does_not_increment_on_duplicate() {
    let mut reg = ItemRegistry::default();
    reg.register(make_item("a")).unwrap();
    reg.register(make_item("a")).err();
    assert_eq!(reg.count(), 1);
}

#[test]
fn all_returns_all_items() {
    let mut reg = ItemRegistry::default();
    reg.register(make_item("a")).unwrap();
    reg.register(make_item("b")).unwrap();
    let items: Vec<_> = reg.all().collect();
    assert_eq!(items.len(), 2);
}

#[test]
fn all_returns_empty_for_empty_registry() {
    let reg = ItemRegistry::default();
    assert_eq!(reg.all().count(), 0);
}

#[test]
fn item_category_partial_eq() {
    assert_eq!(ItemCategory::Raw, ItemCategory::Raw);
    assert_ne!(ItemCategory::Raw, ItemCategory::Refined);
}
