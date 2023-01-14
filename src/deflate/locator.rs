use std::collections::{HashMap, LinkedList};

#[derive(Default)]
pub struct Locator {
    locations: HashMap<[u8; 3], LinkedList<usize>>,
    queue: LinkedList<[u8; 3]>,
}

impl Locator {
    pub fn new() -> Self {
        Self {
            locations: HashMap::new(),
            queue: LinkedList::new(),
        }
    }

    pub fn register(&mut self, triple: &[u8; 3], location: usize) {
        self.locations
            .entry(*triple)
            .or_insert_with(LinkedList::new)
            .push_back(location);
        self.queue.push_back(*triple);
        while self.queue.len() > 20_000 {
            if let Some(triple) = self.queue.pop_front() {
                self.locations.entry(triple).and_modify(|l| {
                    l.pop_front();
                });
            };
        }
    }

    pub fn locate(&self, triple: &[u8; 3]) -> Option<&LinkedList<usize>> {
        self.locations.get(triple)
    }
}
