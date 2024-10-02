#![allow(dead_code)]
use std::{cell::RefCell, rc::Rc};
// This is only for lowercase a-z at the moment.
struct TrieNode<String> {
    children: [Option<Rc<RefCell<TrieNode<String>>>>; 26], // This would have to be a different implementation to generalize from not only lower case a-z strings
    isterm: bool,
    value: String,
}

impl TrieNode<String> {
    fn new(val: String) -> TrieNode<String> {
        const CHILDREN_DEFAULT_VALUE: Option<Rc<RefCell<TrieNode<String>>>> = None;
        TrieNode {
            children: [CHILDREN_DEFAULT_VALUE; 26],
            isterm: false,
            value: val,
        }
    }

    fn has_child(&self, c: char) -> bool {
        self.children[c as usize - 'a' as usize].is_some()
    }

    fn get_child(&self, key: char) -> Option<Rc<RefCell<TrieNode<String>>>> {
        self.children[key as usize - 'a' as usize].clone()
    }
}

pub struct Trie<String> {
    root: Option<Rc<RefCell<TrieNode<String>>>>,
}

impl Trie<String> {
    fn new() -> Trie<String> {
        Trie {
            root: Some(Rc::new(RefCell::new(TrieNode::new(String::new())))),
        }
    }

    fn isempty(&self) -> bool {
        for c in self.root.as_ref().unwrap().borrow().children.as_ref() {
            if c.is_some() {
                return false;
            }
        }
        true
    }

    fn insert(&mut self, data: String) {
        let mut current = self.root.clone().unwrap();
        for c in data.chars() {
            if !current.borrow().has_child(c) {
                let _ = current
                    .borrow_mut()
                    .children
                    .get_mut(c as usize - 'a' as usize)
                    .unwrap()
                    .insert(Rc::new(RefCell::new(TrieNode::new(data.clone()))));
            }

            let nxt = current.borrow().get_child(c).unwrap();
            current = nxt;
        }

        current.borrow_mut().isterm = true;
    }

    fn find(&self, key: String) -> Option<String> {
        let mut current = self.root.clone().unwrap();

        for c in key.chars() {
            if current.borrow().has_child(c) {
                let nxt = current.borrow().get_child(c).unwrap();
                current = nxt;
            } else {
                return None;
            }
        }

        if current.borrow().isterm {
            Some(current.borrow().value.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::Trie;

    #[test]
    fn insert_and_find() {
        let mut trie = Trie::new();

        assert!(trie.isempty());

        trie.insert(String::from("hello"));

        assert!(!trie.isempty());

        trie.insert(String::from("hi"));

        assert_eq!(trie.find(String::from("hi")).unwrap(), String::from("hi"));
        assert_eq!(
            trie.find(String::from("hello")).unwrap(),
            String::from("hello")
        );
        assert_eq!(trie.find(String::from("asdf")), None);

        assert_eq!(trie.find(String::from("hel")), None);

        assert!(trie.root.clone().unwrap().as_ref().borrow().has_child('h'));

        assert!(trie
            .root
            .clone()
            .unwrap()
            .as_ref()
            .borrow()
            .get_child('h')
            .unwrap()
            .as_ref()
            .borrow()
            .has_child('e'));

        assert_eq!(
            trie.root
                .clone()
                .unwrap()
                .as_ref()
                .borrow()
                .get_child('h')
                .unwrap()
                .as_ref()
                .borrow()
                .get_child('e')
                .unwrap()
                .as_ref()
                .borrow()
                .isterm,
            false
        );

        assert!(trie
            .root
            .clone()
            .unwrap()
            .as_ref()
            .borrow()
            .get_child('h')
            .unwrap()
            .as_ref()
            .borrow()
            .has_child('i'));

        assert_eq!(
            trie.root
                .unwrap()
                .as_ref()
                .borrow()
                .get_child('h')
                .unwrap()
                .as_ref()
                .borrow()
                .get_child('i')
                .unwrap()
                .as_ref()
                .borrow()
                .isterm,
            true
        );
    }
}
