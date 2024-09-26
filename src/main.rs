use std::{default, process::Child, rc::Rc};

// This is only for lowercase a-z at the moment.
struct TrieNode<String> {
    children: [Option<Rc<TrieNode<String>>>; 26], // This would have to be a different implementation to generalize from lower case a-z strings
    isterm: bool,
    value: String,
}

impl TrieNode<String> {
    fn new() -> TrieNode<String> {
        const CHILDREN_DEFAULT_VALUE: Option<Rc<TrieNode<String>>> = None;
        TrieNode {
            children: [CHILDREN_DEFAULT_VALUE; 26],
            isterm: true, // Not sure what is the logical default here
            value: String::from(""),
        }
    }

    fn haschild(&self, c: char) -> bool {
        self.children[c as usize - 'a' as usize].is_some()
    }

    fn get_child(&self, key: char) -> Option<Rc<TrieNode<String>>> {
        self.children[key as usize - 'a' as usize].clone()
    }
}

struct Trie<String> {
    root: Option<Rc<TrieNode<String>>>,
}

impl Trie<String> {
    // fn new() -> Trie<String> {
    //     Trie { root: None }
    // }

    fn isempty(&self) -> bool {
        self.root.is_some()
    }

    fn insert(&mut self, data: String) {
        if self.isempty() {
            let mut new_root = TrieNode::new();
            new_root.value = data;
            self.root = Some(Rc::new(new_root));
        } else {
            let mut current = self.root.clone().unwrap();
            for c in data.chars() {
                // Just iterating over chars here, though actual intention would be grapheme clusters
                if current.haschild(c) {
                    current = current.get_child(c).clone().unwrap();
                } else {
                }
            }
        }
    }

    fn find(&self, key: String) {}
}

fn main() {
    println!("Hello, world!");
}
