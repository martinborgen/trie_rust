#![allow(dead_code)]
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
// This is only for lowercase a-z at the moment.
struct TrieNode<String> {
    children: [Option<Rc<RefCell<TrieNode<String>>>>; 26], // This would have to be a different implementation to generalize from not only lower case a-z strings
    isterm: bool,
    value: String,
}

impl TrieNode<String> {
    fn new() -> TrieNode<String> {
        const CHILDREN_DEFAULT_VALUE: Option<Rc<RefCell<TrieNode<String>>>> = None;
        TrieNode {
            children: [CHILDREN_DEFAULT_VALUE; 26],
            isterm: false,
            value: String::new(),
        }
    }

    fn has_child(&self, c: char) -> bool {
        self.children[c as usize - 'a' as usize].is_some()
    }

    fn get_child(&self, key: char) -> Option<Rc<RefCell<TrieNode<String>>>> {
        self.children[key as usize - 'a' as usize].clone()
    }

    fn has_children(&self) -> bool {
        for child in self.children.as_ref() {
            if child.is_some() {
                return true;
            }
        }
        false
    }
}

pub struct Trie<String> {
    root: Option<Rc<RefCell<TrieNode<String>>>>,
}

impl Trie<String> {
    fn new() -> Trie<String> {
        Trie {
            root: Some(Rc::new(RefCell::new(TrieNode::new()))),
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
                    .insert(Rc::new(RefCell::new(TrieNode::new())));
            }

            let nxt = current.borrow().get_child(c).unwrap();
            current = nxt;
        }

        current.borrow_mut().value = data.clone();
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

    fn delete(&mut self, key: String) {
        let mut path: VecDeque<Rc<RefCell<TrieNode<String>>>> = VecDeque::new();

        let mut current = self.root.clone().unwrap();

        for c in key.chars() {
            let nxt = current.borrow().get_child(c);

            if nxt.is_none() {
                // means what we're trying to delete is not in trie
                return;
            }

            path.push_back(nxt.clone().unwrap());
            current = nxt.unwrap();
        }

        // entire key should have been pushed on to path
        let mut prev_char_iter = key.chars().into_iter();
        current = path.pop_back().unwrap();

        current.borrow_mut().value = String::new(); // Clear the value. Make method?
        assert!(current.borrow().isterm);
        current.borrow_mut().isterm = false;
        if current.borrow().has_children() {
            return;
        }

        while !path.is_empty() {
            let chr = prev_char_iter.next_back().unwrap();
            current = path.pop_back().unwrap();

            current.borrow_mut().children[chr as usize - 'a' as usize] = None;

            if current.borrow().has_children() || current.borrow().isterm {
                return;
            }
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
                .value
                .is_empty(),
            true
        );

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
                .clone()
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

        trie.delete(String::from("hi"));

        assert_eq!(trie.find(String::from("hi")), None);
        assert!(trie.find(String::from("hello")).is_some());
        assert_eq!(
            trie.find(String::from("hello")).unwrap(),
            String::from("hello")
        );
    }
}
