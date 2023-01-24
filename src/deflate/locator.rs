use std::collections::LinkedList;

pub struct Locator {
    windows: LinkedList<WindowLocator>,
    hash: usize,
}

const LOCATIONS_SIZE: usize = u16::MAX as usize;
const HASH_SLIDE: usize = 5;
const HASH_MASK: usize = (u16::MAX >> 1) as usize;

const WINDOW_SIZE: usize = 20_000;

impl Locator {
    pub fn new() -> Self {
        Self {
            windows: LinkedList::from_iter([WindowLocator::new(0), WindowLocator::new(0)]),
            hash: 0,
        }
    }

    pub fn slide_hash(&mut self, byte: u8) -> usize {
        self.hash = (self.hash << HASH_SLIDE) ^ (byte as usize);
        self.hash = self.hash & HASH_MASK;
        return self.hash;
    }

    pub fn register(&mut self, hash: usize, location: usize) {
        let r = self
            .windows
            .front_mut()
            .expect("locator should have 2 windows")
            .register(hash, location);
        if r.is_ok() {
            return;
        }
        self.windows.pop_back();
        self.windows.push_front(WindowLocator::new(location));
        self.windows
            .front_mut()
            .expect("locator should have 2 windows")
            .register(hash, location)
            .expect("new WindowLocator should accept register");
    }

    pub fn locate(&self, hash: usize) -> Box<dyn Iterator<Item = usize> + '_> {
        Box::new(
            self.windows
                .iter()
                .flat_map(move |it| it.locate(hash.clone())),
        )
    }
}

struct WindowLocator {
    heads: Box<[Option<usize>; LOCATIONS_SIZE]>,
    tail_links: Box<[Option<usize>; WINDOW_SIZE]>,
    offset: usize,
}

impl WindowLocator {
    fn new(offset: usize) -> Self {
        Self {
            heads: Box::new([None; LOCATIONS_SIZE]),
            tail_links: Box::new([None; WINDOW_SIZE]),
            offset,
        }
    }

    pub fn register(&mut self, hash: usize, location: usize) -> Result<(), ()> {
        let local_location = location - self.offset;
        if !(local_location < WINDOW_SIZE) {
            return Err(());
        }
        if let Some(head) = self.heads[hash] {
            self.tail_links[local_location] = Some(head);
        }
        self.heads[hash] = Some(local_location);
        Ok(())
    }

    pub fn locate(&self, hash: usize) -> LocationIter {
        LocationIter {
            locator: &self,
            pending: self.heads[hash],
        }
    }
}

struct LocationIter<'a> {
    locator: &'a WindowLocator,
    pending: Option<usize>,
}

impl<'a> Iterator for LocationIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.pending {
            self.pending = self.locator.tail_links[c].clone();
            Some(c + self.locator.offset)
        } else {
            None
        }
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
