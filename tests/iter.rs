#[macro_use]
extern crate ego_tree;

#[test]
fn values() {
    let tree = tree!('a' => { 'b', 'c', 'd' });
    assert_eq!(
        vec![&'a', &'b', &'c', &'d'],
        tree.values().collect::<Vec<_>>()
    );
}

#[test]
fn values_mut() {
    use std::ascii::AsciiExt;

    let mut tree = tree!('a' => { 'b', 'c', 'd' });

    for c in tree.values_mut() {
        *c = c.to_ascii_uppercase();
    }

    assert_eq!(
        vec![&'A', &'B', &'C', &'D'],
        tree.values().collect::<Vec<_>>()
    );
}

#[test]
fn into_values() {
    let tree = tree!('a' => { 'b', 'c', 'd' });
    assert_eq!(
        vec!['a', 'b', 'c', 'd'],
        tree.into_values().collect::<Vec<_>>()
    );
}

#[test]
fn nodes() {
    let mut tree = tree!('a' => { 'b' => { 'c' }, 'd' });
    tree.orphan('e').append('f');
    tree.root_mut().append('g');
    assert_eq!(
        vec![&'a', &'b', &'c', &'d', &'e', &'f', &'g'],
        tree.nodes().map(|n| n.value()).collect::<Vec<_>>()
    );
}

#[test]
fn ancestors() {
    let tree = tree!('a' => { 'b' => { 'c' => { 'd' } } });
    let d = tree.root()
        .last_child().unwrap()
        .last_child().unwrap()
        .last_child().unwrap();
    assert_eq!(
        vec![&'c', &'b', &'a'],
        d.ancestors().map(|n| n.value()).collect::<Vec<_>>()
    );
}

#[test]
fn prev_siblings() {
    let tree = tree!('a' => { 'b', 'c', 'd' });
    assert_eq!(
        vec![&'c', &'b'],
        tree.root()
            .last_child()
            .unwrap()
            .prev_siblings()
            .map(|n| n.value())
            .collect::<Vec<_>>()
    );
}

#[test]
fn next_siblings() {
    let tree = tree!('a' => { 'b', 'c', 'd' });
    assert_eq!(
        vec![&'c', &'d'],
        tree.root()
            .first_child()
            .unwrap()
            .next_siblings()
            .map(|n| n.value())
            .collect::<Vec<_>>()
    );
}

#[test]
fn children() {
    let tree = tree!('a' => { 'b', 'c', 'd' });
    assert_eq!(
        vec![&'b', &'c', &'d'],
        tree.root().children().map(|n| n.value()).collect::<Vec<_>>()
    );
}

#[test]
fn children_rev() {
    let tree = tree!('a' => { 'b', 'c', 'd' });
    assert_eq!(
        vec![&'d', &'c', &'b'],
        tree.root().children().rev().map(|n| n.value()).collect::<Vec<_>>()
    );
}

#[test]
fn first_children() {
    let tree = tree!('a' => { 'b' => { 'd', 'e' }, 'c' });
    assert_eq!(
        vec![&'b', &'d'],
        tree.root().first_children().map(|n| n.value()).collect::<Vec<_>>()
    );
}

#[test]
fn last_children() {
    let tree = tree!('a' => { 'b', 'c' => { 'd', 'e' } });
    assert_eq!(
        vec![&'c', &'e'],
        tree.root().last_children().map(|n| n.value()).collect::<Vec<_>>()
    );
}

#[test]
fn filter_deep_nodes() {
    let tree = tree!('a' => { 
        'b' => { 'd', 'e' },
        'f' => { 'g' => { 'h' => { 'i' => {'j'} } } },
    }); 
    let matched_nodes: Vec<_> = tree.root().filter_deep_nodes(|node|{
        let node_values = ['a','b','c','d','e','f','g','h','i','j'];
        return node_values.contains(node.value());
    }).collect();
    assert_eq!(matched_nodes.len(), 3);
    assert_eq!(*matched_nodes.get(0).unwrap().value(), 'd');
    assert_eq!(*matched_nodes.get(1).unwrap().value(), 'e');
    assert_eq!(*matched_nodes.get(2).unwrap().value(), 'j');
}

#[test]
fn traverse() {
    use ego_tree::iter::Edge;

    #[derive(Debug, PartialEq, Eq)]
    enum Value<'a> {
        Open(&'a char),
        Close(&'a char),
    }

    let tree = tree!('a' => { 'b' => { 'd', 'e' }, 'c' });

    let traversal = tree.root().traverse().map(|edge| {
        match edge {
            Edge::Open(node) => Value::Open(node.value()),
            Edge::Close(node) => Value::Close(node.value()),
        }
    }).collect::<Vec<_>>();

    assert_eq!(
        &[
            Value::Open(&'a'),
            Value::Open(&'b'),
            Value::Open(&'d'),
            Value::Close(&'d'),
            Value::Open(&'e'),
            Value::Close(&'e'),
            Value::Close(&'b'),
            Value::Open(&'c'),
            Value::Close(&'c'),
            Value::Close(&'a'),
        ],
        &traversal[..]
    );
}
