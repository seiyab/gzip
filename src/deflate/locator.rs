use std::collections::LinkedList;

#[derive(Default)]
pub struct Locator {
    locations: Vec<Option<LinkedList<usize>>>,
    queue: LinkedList<usize>,
    hash: usize,
}

const LOCATIONS_SIZE: usize = u16::MAX as usize;
const HASH_SLIDE: usize = 5;
const HASH_MASK: usize = (u16::MAX >> 1) as usize;

impl Locator {
    pub fn new() -> Self {
        Self {
            locations: vec![None; LOCATIONS_SIZE],
            queue: LinkedList::new(),
            hash: 0,
        }
    }

    pub fn slide_hash(&mut self, byte: u8) -> usize {
        self.hash = (self.hash << HASH_SLIDE) ^ (byte as usize);
        self.hash = self.hash & HASH_MASK;
        return self.hash;
    }

    pub fn register(&mut self, hash: usize, location: usize) {
        if self.locations[hash] == None {
            self.locations[hash] = Some(LinkedList::new());
        }
        let locs = self.locations[hash].as_mut().unwrap();
        locs.push_front(location);
        self.queue.push_back(hash);
        while self.queue.len() > 20_000 {
            if let Some(old_hash) = self.queue.pop_front() {
                if let Some(old_locs) = self.locations[old_hash].as_mut() {
                    old_locs.pop_back();
                };
            };
        }
    }

    pub fn locate(&self, hash: usize) -> Option<&LinkedList<usize>> {
        self.locations[hash].as_ref()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Locator;

    #[test]
    fn test_hash() {
        let mut hashes = Vec::new();
        let mut locator = Locator::new();
        for b in 0..15 {
            hashes.push(locator.slide_hash(b));
        }
        for b in 0..15 {
            let hash = locator.slide_hash(b);
            if b > 2 {
                assert_eq!(hash, hashes[b as usize]);
            }
        }
        assert_eq!(HashSet::<usize>::from_iter(hashes.into_iter()).len(), 15);
    }
}
