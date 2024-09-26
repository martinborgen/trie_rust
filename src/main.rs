use std::rc::Rc;

struct TrieNode<String> {
    children: std::array::Array<Rc<TrieNode<String>>>,
    isterm: bool,
    value: String,
}

impl TrieNode<String> {
    fn new() -> TrieNode<String> {
        todo!()
    }
}

struct Trie<String> {
    root: Option<TrieNode<String>>,
}

impl Trie<String> {
    fn new() -> Trie<String> {
        Trie { root: None }
    }

    fn isempty(&self) -> bool {
        self.root.is_some()
    }

    fn insert(&mut self, data: String) {
        if self.isempty() {
            self.root = Some(TrieNode {
                children: Vec::new(),
                isterm: true,
                value: data,
            });
        } else {
            todo!()
        }
    }

    fn find(&self, key: String) {}
}

fn main() {
    println!("Hello, world!");
}
