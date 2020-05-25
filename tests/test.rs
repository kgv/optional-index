use maplit::hashmap;
use optional_index::{OptionalIndex, OptionalIndexMut};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Map<T>(HashMap<&'static str, T>);

impl OptionalIndex<&'static str> for Map<Map<usize>> {
    type Output = Map<usize>;

    fn optional_index(&self, index: &'static str) -> Option<&Self::Output> {
        self.0.get(index)
    }
}

impl OptionalIndexMut<&'static str> for Map<Map<usize>> {
    fn optional_index_mut(&mut self, index: &'static str) -> Option<&mut Self::Output> {
        self.0.get_mut(index)
    }
}

impl OptionalIndex<&'static str> for Map<usize> {
    type Output = usize;

    fn optional_index(&self, index: &'static str) -> Option<&Self::Output> {
        self.0.get(index)
    }
}

impl OptionalIndexMut<&'static str> for Map<usize> {
    fn optional_index_mut(&mut self, index: &'static str) -> Option<&mut Self::Output> {
        self.0.get_mut(index)
    }
}

#[test]
fn immutable() {
    let map = Map(hashmap! {
        "a" => Map(hashmap! {}),
        "b" => Map(hashmap! {
            "ba" => 0,
            "bb" => 0,
        }),
    });
    assert_eq!(map.optional_index("a"), Some(&Map(hashmap! {})));
    assert_eq!(map.optional_index("a").optional_index("ab"), None);
    assert_eq!(
        map.optional_index("b"),
        Some(&Map(hashmap! { "ba" => 0, "bb" => 0 }))
    );
    assert_eq!(map.optional_index("b").optional_index("ba"), Some(&0));
    assert_eq!(map.optional_index("c"), None);
    assert_eq!(map.optional_index("c").optional_index("ca"), None);
}

#[test]
fn mutable() {
    let mut map = Map(hashmap! {
        "a" => Map(hashmap! {}),
        "b" => Map(hashmap! {
            "ba" => 0,
            "bb" => 0,
        }),
    });
    assert_eq!(map.optional_index_mut("a"), Some(&mut Map(hashmap! {})));
    assert_eq!(map.optional_index_mut("a").optional_index_mut("ab"), None);
    assert_eq!(
        map.optional_index_mut("b"),
        Some(&mut Map(hashmap! { "ba" => 0, "bb" => 0 }))
    );
    assert_eq!(
        map.optional_index_mut("b").optional_index_mut("ba"),
        Some(&mut 0)
    );
    assert_eq!(map.optional_index_mut("c"), None);
    assert_eq!(map.optional_index_mut("c").optional_index_mut("ca"), None);
}
