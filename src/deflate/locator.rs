use std::collections::{HashMap, LinkedList};

#[derive(Default)]
pub struct Locator {
    locations: HashMap<[u8; 3], LinkedList<usize>>,
    queue: LinkedList<[u8; 3]>,
}

pub struct Progress(pub usize);

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

    pub fn scan<F>(&mut self, data: &Vec<u8>, mut f: F)
    where
        F: FnMut(usize, &LinkedList<usize>) -> Progress,
    {
        let mut i = 0;
        while i < data.len() {
            let tuple = match [data.get(i), data.get(i + 1), data.get(i + 2)] {
                [Some(&x), Some(&y), Some(&z)] => Some([x, y, z]),
                _ => None,
            };
            let empty = &EMPTY;
            let locations = tuple.and_then(|t| self.locations.get(&t)).unwrap_or(empty);
            let Progress(p) = f(i, locations);
            let next = i + p;

            while i < next {
                match [data.get(i), data.get(i + 1), data.get(i + 2)] {
                    [Some(&x), Some(&y), Some(&z)] => self.register(&[x, y, z], i),
                    _ => (),
                }
                i += 1;
            }
        }
    }
}

const EMPTY: LinkedList<usize> = LinkedList::new();
