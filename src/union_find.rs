use std::fmt::{Debug, Formatter, Result as FmtResult};

fn debug_union_find(nodes: &[isize], starting_index: isize, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_list()
        .entries(
            nodes
                .iter()
                .copied()
                .map(|v| if v < 0 { v } else { v + starting_index }),
        )
        .finish()
}

pub(crate) struct Roots<'a> {
    buffer: &'a [isize],
    starting_index: usize,
    index: usize,
}

impl<'a> Roots<'a> {
    pub(crate) fn new(buffer: &'a [isize], starting_index: usize) -> Self {
        Self {
            buffer,
            starting_index,
            index: 0,
        }
    }
}

impl<'a> Iterator for Roots<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.buffer.len() {
            let index = self.index;
            self.index += 1;
            if self.buffer[index] < 0 {
                return Some(index + self.starting_index);
            }
        }
        None
    }
}

pub(crate) trait UnionFind {
    fn new(n: usize, starting_index: usize) -> Self;
    fn find(&mut self, x: usize) -> usize;
    fn union(&mut self, root_1: usize, root_2: usize);
    fn roots(&self) -> Roots;
    fn from_nodes_and_starting_index(nodes: Box<[isize]>, starting_index: usize) -> Self;
}

#[derive(PartialEq, Eq, Default)]
pub(crate) struct UnionFindBySize {
    nodes: Box<[isize]>,
    starting_index: usize,
}

impl Debug for UnionFindBySize {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        debug_union_find(&self.nodes, self.starting_index as isize, f)
    }
}

impl UnionFind for UnionFindBySize {
    fn new(n: usize, starting_index: usize) -> Self {
        Self {
            nodes: vec![-1; n].into_boxed_slice(),
            starting_index,
        }
    }
    fn find(&mut self, mut x: usize) -> usize {
        assert!(
            x >= self.starting_index && x - self.starting_index < self.nodes.len(),
            "out of bounds"
        );
        x -= self.starting_index;
        while self.nodes[x] >= 0 {
            x = self.nodes[x] as usize;
        }
        x + self.starting_index
    }
    fn union(&mut self, root_1: usize, root_2: usize) {
        assert!(
            root_1 >= self.starting_index && root_1 - self.starting_index < self.nodes.len(),
            "out of bounds"
        );
        assert!(
            root_2 >= self.starting_index && root_2 - self.starting_index < self.nodes.len(),
            "out of bounds"
        );
        let root_1 = self.find(root_1) - self.starting_index;
        let root_2 = self.find(root_2) - self.starting_index;
        let (bigger, smaller) = if self.nodes[root_2] < self.nodes[root_1] {
            (root_2, root_1)
        } else {
            (root_1, root_2)
        };
        self.nodes[bigger] += self.nodes[smaller];
        self.nodes[smaller] = bigger as isize;
    }
    fn roots(&self) -> Roots {
        Roots::new(&self.nodes, self.starting_index)
    }
    fn from_nodes_and_starting_index(nodes: Box<[isize]>, starting_index: usize) -> Self {
        Self {
            nodes,
            starting_index,
        }
    }
}

#[derive(PartialEq, Eq, Default)]
pub(crate) struct UnionFindByHeight {
    nodes: Box<[isize]>,
    starting_index: usize,
}

impl Debug for UnionFindByHeight {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        debug_union_find(&self.nodes, self.starting_index as isize, f)
    }
}

impl UnionFind for UnionFindByHeight {
    fn new(n: usize, starting_index: usize) -> Self {
        Self {
            nodes: vec![-1; n].into_boxed_slice(),
            starting_index,
        }
    }
    fn find(&mut self, mut x: usize) -> usize {
        assert!(
            x >= self.starting_index && x - self.starting_index < self.nodes.len(),
            "out of bounds"
        );
        x -= self.starting_index;
        while self.nodes[x] >= 0 {
            x = self.nodes[x] as usize;
        }
        x + self.starting_index
    }
    fn union(&mut self, root_1: usize, root_2: usize) {
        assert!(
            root_1 >= self.starting_index && root_1 - self.starting_index < self.nodes.len(),
            "out of bounds"
        );
        assert!(
            root_2 >= self.starting_index && root_2 - self.starting_index < self.nodes.len(),
            "out of bounds"
        );
        let root_1 = self.find(root_1) - self.starting_index;
        let root_2 = self.find(root_2) - self.starting_index;
        let (bigger, smaller) = if self.nodes[root_2] < self.nodes[root_1] {
            (root_2, root_1)
        } else {
            (root_1, root_2)
        };
        if self.nodes[bigger] == self.nodes[smaller] {
            self.nodes[bigger] -= 1;
        }
        self.nodes[smaller] = bigger as isize;
    }
    fn roots(&self) -> Roots {
        Roots::new(&self.nodes, self.starting_index)
    }
    fn from_nodes_and_starting_index(nodes: Box<[isize]>, starting_index: usize) -> Self {
        Self {
            nodes,
            starting_index,
        }
    }
}

#[derive(PartialEq, Eq, Default)]
pub(crate) struct UnionFindBySizeWithCompression {
    nodes: Box<[isize]>,
    starting_index: usize,
}

impl Debug for UnionFindBySizeWithCompression {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        debug_union_find(&self.nodes, self.starting_index as isize, f)
    }
}

impl UnionFind for UnionFindBySizeWithCompression {
    fn new(n: usize, starting_index: usize) -> Self {
        Self {
            nodes: vec![-1; n].into_boxed_slice(),
            starting_index,
        }
    }
    fn find(&mut self, x: usize) -> usize {
        assert!(
            x >= self.starting_index && x - self.starting_index < self.nodes.len(),
            "out of bounds"
        );
        fn recurse(nodes: &mut [isize], x: usize) -> usize {
            if nodes[x] >= 0 {
                let ret = recurse(nodes, nodes[x] as usize);
                nodes[x] = ret as isize;
                ret
            } else {
                x
            }
        }
        recurse(&mut self.nodes, x - self.starting_index) + self.starting_index
    }
    fn union(&mut self, root_1: usize, root_2: usize) {
        assert!(
            root_1 >= self.starting_index && root_1 - self.starting_index < self.nodes.len(),
            "out of bounds"
        );
        assert!(
            root_2 >= self.starting_index && root_2 - self.starting_index < self.nodes.len(),
            "out of bounds"
        );
        let root_1 = self.find(root_1) - self.starting_index;
        let root_2 = self.find(root_2) - self.starting_index;
        let (bigger, smaller) = if self.nodes[root_2] < self.nodes[root_1] {
            (root_2, root_1)
        } else {
            (root_1, root_2)
        };
        self.nodes[bigger] += self.nodes[smaller];
        self.nodes[smaller] = bigger as isize;
    }
    fn roots(&self) -> Roots {
        Roots::new(&self.nodes, self.starting_index)
    }
    fn from_nodes_and_starting_index(nodes: Box<[isize]>, starting_index: usize) -> Self {
        Self {
            nodes,
            starting_index,
        }
    }
}
