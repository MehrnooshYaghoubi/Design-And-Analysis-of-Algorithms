use crate::types::Edge;
// Disjoint Set Union (Union-Find) with path compression and union by rank
struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, u: usize) -> usize {
        if self.parent[u] != u {
            self.parent[u] = self.find(self.parent[u]); // path compression
        }
        self.parent[u]
    }

    fn union(&mut self, u: usize, v: usize) {
        let root_u = self.find(u);
        let root_v = self.find(v);

        if root_u != root_v {
            if self.rank[root_u] < self.rank[root_v] {
                self.parent[root_u] = root_v;
            } else if self.rank[root_u] > self.rank[root_v] {
                self.parent[root_v] = root_u;
            } else {
                self.parent[root_v] = root_u;
                self.rank[root_u] += 1;
            }
        }
    }
}

pub fn kruskal(num_vertices: usize, mut edges: Vec<Edge>) -> Vec<Edge> {
    // Sort edges by weight
    edges.sort_by_key(|e| e.weight);

    let mut ds = DisjointSet::new(num_vertices);
    let mut mst = Vec::new();

    for edge in edges {
        let root1 = ds.find(edge.src);
        let root2 = ds.find(edge.dest);

        if root1 != root2 {
            mst.push(edge.clone());
            ds.union(root1, root2);
        }
    }

    mst
}
