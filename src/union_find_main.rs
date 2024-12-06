use std::fmt::Debug;
use union_find::{UnionFind, UnionFindByHeight, UnionFindBySize, UnionFindBySizeWithCompression};

mod union_find;

fn make_insertions<UF: UnionFind + Debug>(
    uf: &mut UF,
    insertions: &[(usize, usize)],
    algorithm_name: &str,
) {
    println!("Using algorithm: {}", algorithm_name);
    for (a, b) in insertions.iter().copied() {
        uf.union(a, b);
        println!("After inserting ({a}, {b}): {uf:?}");
    }
    println!("Number of roots: {}", uf.roots().count());
    println!("Roots: {:?}", uf.roots().collect::<Vec<_>>());
    println!();
    println!();
}

fn perform_find(uf: &mut UnionFindBySizeWithCompression, x: usize) {
    let root = uf.find(x);
    println!("Root of {x} is {root}");
    println!("After finding root of {x}: {uf:?}");
    println!();
}

fn main() {
    let mut uf = UnionFindBySize::new(9, 1);
    make_insertions(
        &mut uf,
        &[(1, 2), (3, 4), (9, 8), (1, 7), (3, 5), (6, 3), (9, 3)],
        "UnionFindBySize",
    );
    let mut uf = UnionFindByHeight::new(9, 1);
    make_insertions(
        &mut uf,
        &[
            (4, 1),
            (3, 4),
            (8, 9),
            (2, 5),
            (7, 2),
            (8, 2),
            (6, 4),
            (4, 8),
        ],
        "UnionFindByHeight",
    );
    let mut uf = UnionFindBySizeWithCompression::from_nodes_and_starting_index(
        vec![-5, 0, -12, 2, 2, 2, 0, 0, 7, 2, 2, 2, 2, 2, 13, 13, 15].into_boxed_slice(),
        1,
    );
    perform_find(&mut uf, 9);
    perform_find(&mut uf, 13);
    perform_find(&mut uf, 15);
    perform_find(&mut uf, 17);
}
