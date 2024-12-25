trait DisjointSetUnion {
    fn make_set(&mut self, v: i32);
    fn find_set(&mut self, v: i32) -> i32;
    fn union(&mut self, u: i32, v: i32);
}

struct DSU {
    parent: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: Vec::with_capacity(n),
        }
    }
}

impl DisjointSetUnion for DSU {
    fn make_set(&mut self, v: i32) {
        self.parent.push(v);
    }

    fn find_set(&mut self, v: i32) -> i32 {
        if self.parent[v as usize] == v {
            return v;
        }
        self.parent[v as usize] = self.find_set(self.parent[v as usize]);
        self.parent[v as usize]
    }

    fn union(&mut self, u: i32, v: i32) {
        let a = self.find_set(u);
        let b = self.find_set(v);
        if a != b {
            self.parent[b as usize] = a;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsu() {
        let mut dsu = DSU::new(6);
        for i in 0..6 {
            dsu.make_set(i);
        }
        dsu.union(0, 1);
        dsu.union(1, 2);
        dsu.union(4, 5);
        dsu.union(3, 4);
        dsu.union(0, 3);
        assert_eq!(dsu.find_set(0), dsu.find_set(5));
    }
}
