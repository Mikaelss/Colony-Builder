use Colony_Builder::core::identity::{EntityMap, Id, IdAllocator, IdComponent};
use bevy::prelude::*;

#[derive(Debug)]
struct TestMarker;

fn make_id(value: u64) -> Id<TestMarker> {
    Id::new(value)
}

fn make_entity(index: u32) -> Entity {
    Entity::from_bits(index as u64)
}

#[test]
fn id_new_get() {
    assert_eq!(make_id(42).get(), 42);
}

#[test]
fn id_equality() {
    assert_eq!(make_id(1), make_id(1));
    assert_ne!(make_id(1), make_id(2));
}

#[test]
fn id_clone() {
    let a = make_id(5);
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn id_is_copy() {
    let a = make_id(3);
    let b = a;
    assert_eq!(a, b);
}

#[test]
fn id_type_safety_different_types() {
    struct MarkerA;
    struct MarkerB;
    let a = Id::<MarkerA>::new(1);
    let b = Id::<MarkerB>::new(1);
    assert_eq!(a.get(), b.get());
}

#[test]
fn id_serialize_roundtrip() {
    let id = make_id(42);
    let json = serde_json::to_string(&id).unwrap();
    let deserialized: Id<TestMarker> = serde_json::from_str(&json).unwrap();
    assert_eq!(id, deserialized);
}

#[test]
fn allocator_starts_at_zero() {
    let alloc = IdAllocator::<TestMarker>::default();
    assert_eq!(alloc.current().get(), 0);
}

#[test]
fn allocator_first_next_returns_one() {
    let mut alloc = IdAllocator::<TestMarker>::default();
    assert_eq!(alloc.next().get(), 1);
}

#[test]
fn allocator_increments_sequentially() {
    let mut alloc = IdAllocator::<TestMarker>::default();
    assert_eq!(alloc.next().get(), 1);
    assert_eq!(alloc.next().get(), 2);
    assert_eq!(alloc.next().get(), 3);
}

#[test]
fn allocator_set_next() {
    let mut alloc = IdAllocator::<TestMarker>::default();
    alloc.set_next(100);
    assert_eq!(alloc.next().get(), 101);
}

#[test]
fn allocator_current_after_next() {
    let mut alloc = IdAllocator::<TestMarker>::default();
    alloc.next();
    assert_eq!(alloc.current().get(), 1);
}

#[test]
fn entity_map_empty_get_none() {
    let map = EntityMap::<TestMarker>::default();
    assert!(map.get(make_id(1)).is_none());
}

#[test]
fn entity_map_insert_and_get() {
    let mut map = EntityMap::<TestMarker>::default();
    let entity = make_entity(42);
    map.insert(make_id(1), entity);
    assert_eq!(map.get(make_id(1)), Some(entity));
}

#[test]
fn entity_map_remove() {
    let mut map = EntityMap::<TestMarker>::default();
    map.insert(make_id(1), make_entity(42));
    map.remove(make_id(1));
    assert!(map.get(make_id(1)).is_none());
}

#[test]
fn entity_map_multiple_entries() {
    let mut map = EntityMap::<TestMarker>::default();
    map.insert(make_id(1), make_entity(10));
    map.insert(make_id(2), make_entity(20));
    assert_eq!(map.get(make_id(1)), Some(make_entity(10)));
    assert_eq!(map.get(make_id(2)), Some(make_entity(20)));
}

#[test]
fn entity_map_entity_alias() {
    let mut map = EntityMap::<TestMarker>::default();
    let entity = make_entity(99);
    map.insert(make_id(1), entity);
    assert_eq!(map.entity(make_id(1)), Some(entity));
}

#[test]
fn id_component_struct() {
    let component = IdComponent::<TestMarker> { id: make_id(7) };
    assert_eq!(component.id.get(), 7);
}
