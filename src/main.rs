use fxhash::FxHashMap;
use std::fs;
use std::{cell::RefCell, collections::*};

fn main() {
    let en_lng = TrieNode::<char>::init_english();

    // TEST BUILD
    for key in en_lng.children.clone().into_keys().collect::<Vec<char>>() {
        println!("{key}", key = key);
    }
    // for keys in en_lng.children.clone().into_values().

    // TEST is_partial
    println!(
        "{txt}",
        txt = en_lng.clone().is_partial("enteri").to_string()
    );

    // TEST is_word
    println!("{txt}", txt = en_lng.clone().is_word("hello").to_string());
}

#[derive(Default, Clone)]
struct TrieNode<'a, T> {
    value: Option<T>,
    terminal: bool,
    children: FxHashMap<T, RefCell<TrieNode<'a, T>>>,
}

impl<'a, T> TrieNode<'a, T> {
    fn new() -> Self {
        TrieNode {
            value: None,
            terminal: false,
            children: HashMap::default(),
        }
    }

    fn new_with_val(val: T, term: bool) -> Self {
        TrieNode {
            value: Some(val),
            terminal: term,
            children: HashMap::default(),
        }
    }
}

impl<'a> TrieNode<'a, char> {
    pub fn init_english() -> Self {
        let mut head: TrieNode<char> = TrieNode::<char>::new_with_val(' ', false);
        let word_str: String = fs::read_to_string("words/english-words/single_word.txt").unwrap();
        let words: Vec<&str> = word_str.lines().collect::<Vec<&str>>();

        for word in words {
            head.append(word);
        }

        head
    }

    fn append(&mut self, val: &str) {
        if val.chars().count() == 0 {
            return;
        }
        println!("{:?} curr val", self.value.unwrap());
        let this_chr: char = val.chars().nth(0).unwrap();
        let children: FxHashMap<char, RefCell<TrieNode<char>>> = self.children;
        match children.get(&this_chr) {
            Some(c) => c.take().append(&val[1..]),
            None => {
                let term: bool = val.chars().count() == 1;
                println!("{:?}", term);
                let next: RefCell<TrieNode<char>> =
                    RefCell::new(TrieNode::new_with_val(this_chr, term));
                children.insert(this_chr, next);
                children
                    .get_mut(&this_chr)
                    .unwrap()
                    .get_mut()
                    .append(&val[1..]);
            }
        }
    }

    /**
     * @search: a string to verify existence of potential words
     *
     * @return:
     *      * true if there are possible words to be made from @search,
     *      * false if the search word is not present in the Trie
     */
    pub fn is_partial(&self, search: &str) -> bool {
        if search.chars().count() == 0 {
            return false;
        }
        match self.children.get(&search.chars().nth(0).unwrap()) {
            Some(n) => {
                if search.chars().count() == 1 {
                    n.take().children.len() != 0
                } else {
                    n.take().is_partial(&search[1..])
                }
            }
            None => false,
        }
    }

    /**
     * @search: a string to search for the terminality of
     *
     * @return:
     *      * true if the search term is a terminal node,
     *      * false if the search term is not a terminal node
     */
    pub fn is_word(&self, search: &str) -> bool {
        println!("{:?} the search", &search);
        for key in self.children.clone().into_keys().collect::<Vec<char>>() {
            print!("key: {:?}!", key);
        }
        if search.chars().count() == 0 {
            println!("Not good to be here");
            return false;
        }
        match self.children.get(&search.chars().nth(0).unwrap()) {
            Some(n) => {
                println!("\n{:?} we got one", n.take().value.unwrap());
                if search.chars().count() == 1 {
                    n.take().terminal
                } else {
                    n.take().is_word(&search[1..])
                }
            }
            None => {
                println!("There were no children!");
                false
            }
        }
    }

    /**
     * @search: a partial string to search for near suggestions.
     * @count: number of suggestions to retrieve.
     *
     * @return: a String vector of size @count, containing the most
     *          similar words to the search
     */
    pub fn suggest_n(&self, search: &str, count: usize) -> Vec<String> {
        unimplemented!();
    }
}
