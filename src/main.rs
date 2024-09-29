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
            isterm: true,
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

struct Trie<String> {
    root: Option<Rc<RefCell<TrieNode<String>>>>,
}

impl Trie<String> {
    fn new() -> Trie<String> {
        Trie {
            root: Some(Rc::new(RefCell::new(TrieNode::new(String::new())))),
        }
    }

    fn isempty(&self) -> bool {
        self.root.as_ref().unwrap().borrow().isterm
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

            if current.borrow().isterm {
                current.borrow_mut().isterm = false;
            }

            let nxt = current.borrow().get_child(c).unwrap();
            current = nxt;
        }
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
        return Some(current.borrow().value.clone());
    }
}

fn main() {
    let mut mytrie = Trie::new();
    mytrie.insert(String::from("h"));
    mytrie.insert(String::from("e"));
    mytrie.insert(String::from("j"));
    mytrie.insert(String::from("he"));
    mytrie.insert(String::from("hej"));

    let tst1 = mytrie.find(String::from("he"));
    if tst1.is_some() {
        println!("{}", tst1.unwrap());
    } else {
        println!(".find did not find!")
    }

    let tst2 =
        &mytrie.root.clone().unwrap().borrow().children['h' as usize - 'a' as usize].is_some();
    if *tst2 {
        println!("root HAS child at index 'h'");
    } else {
        println!("root has NO child at index 'h'");
    }

    let tst3 = &mytrie.root.unwrap().borrow().has_child('h');
    if *tst3 {
        println!(".has_child found child at root index 'h'")
    } else {
        println!(".has_child did not find it")
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

        assert!(trie
            .root
            .unwrap()
            .as_ref()
            .borrow()
            .get_child('h')
            .unwrap()
            .as_ref()
            .borrow()
            .has_child('i'));
    }
}
