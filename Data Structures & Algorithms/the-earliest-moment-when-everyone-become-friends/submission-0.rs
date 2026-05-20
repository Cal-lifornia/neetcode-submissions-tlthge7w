impl Solution {
    pub fn earliest_acq(logs: Vec<Vec<i32>>, n: i32) -> i32 {
        let mut logs = logs;
        logs.sort_by(|a, b| a[0].cmp(&b[0]));

        let n = n as usize;
        let mut union = UnionFind::new(n);
        let mut groups = n;

        for log in logs {
            let time = log[0];
            let p1 = log[1] as usize;
            let p2 = log[2] as usize;

            if union.union(p1, p2) {
                groups -= 1;
            }
            if groups == 1 {
                return time;
            }
        }
        -1
    }
}
struct UnionFind {
    group: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            group: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, person: usize) -> usize {
        if self.group[person] != person {
            self.group[person] = self.find(self.group[person]);
        }
        self.group[person]
    }

    // a=0,b=1
    fn union(&mut self, a: usize, b: usize) -> bool {
        // Will return 0
        let group_a = self.find(a);
        // Will return 1
        let group_b = self.find(b);

        // Won't return
        if group_a == group_b {
            return false;
        }

        // self.rank[0] == self.rank[1];
        // Both will be 0
        if self.rank[group_a] > self.rank[group_b] {
            self.group[group_b] = group_a;
        } else if self.rank[group_a] < self.rank[group_b] {
            self.group[group_a] = group_b;
        } else {
            // self.group[0] = 1
            // self.rank[1] = 1;
            self.group[group_a] = group_b;
            self.rank[group_b] += 1;
        }
        true
    }
}
