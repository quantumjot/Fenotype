pub struct Permutations {
    count: usize,
    max_count: usize,
    current: [u64; 2],
    n: u64,
}

impl Iterator for Permutations {
    type Item = [u64; 2];

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.max_count {
            return None;
        }

        self.count += 1;

        match (self.current[1] + 1) % self.n {
            0 => {
                self.current[0] += 1;
                self.current[1] = self.current[0] + 1;
            }
            _ => self.current[1] += 1,
        };

        assert!(self.current[0] < self.current[1]);

        return Some(self.current);
    }
}

impl Permutations {
    pub fn new(n: u64) -> Self {
        let max_count: usize = ((n * (n - 1)) / 2) as usize;
        Permutations {
            count: 0,
            max_count: max_count,
            current: [0, 0],
            n: n,
        }
    }

    pub fn len(&self) -> usize {
        return self.max_count;
    }
}

pub fn unique_permutations(n: u64) -> Vec<[u64; 2]> {
    let _perm = Permutations::new(n);
    let mut _permutations: Vec<[u64; 2]> = Vec::with_capacity(_perm.len());
    for p in _perm {
        _permutations.push(p);
    }
    // println!("{} -> {} unique permutations", n, _perm.len());
    return _permutations;
}
