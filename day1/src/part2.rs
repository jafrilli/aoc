use std::collections::HashMap;

struct Node {
    children: HashMap<char, Node>,
    leaf: Option<u32>,
}

impl Node {
    fn new() -> Self {
        Node {
            children: HashMap::new(),
            leaf: None,
        }
    }

    fn insert(&mut self, word: & str, val: u32) {
        if let Some(character) = word.chars().next() {
            let child = self.children.entry(character).or_insert_with(Node::new);
            child.insert(&word[1..], val);
        } else {
            self.leaf = Some(val);
        }
    }

    fn contains(&self, word: &str) -> Option<u32> {
        if let Some(character) = word.chars().next() {
            match self.children.get(&character) {
                Some(child) => {
                   if child.leaf.is_some() {
                       Some(child.leaf.unwrap())
                   } else {
                       child.contains(&word[1..])
                   }
                },
                None => None
            }
        } else {
            None
        }
    }

    fn print(&self, depth: usize) {
        for (char, node) in &self.children {
            println!("{}- '{}': {}", " ".repeat(depth * 2), char, node.leaf.is_some());
            node.print(depth + 1);
        }
    }
}

pub fn exec(input: &str) -> String {
    let numbers = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];
    let numbers_rev = numbers.map(|v: &str| v.chars().rev().collect::<String>());
    let (mut root, mut root_rev) = (Node::new(), Node::new());

    for i in 0..numbers.len() {
        root.insert(numbers[i], (i % 10) as u32);
        root_rev.insert(&numbers_rev[i], (i % 10) as u32);
    }

    // root.print(0);
    // root_rev.print(0);

    let mut sum = 0;

    for line in input.lines() {
        let line_rev = line.chars().rev().collect::<String>();

        let mut num = 0;

        for i in 0..line.len() {
            if let Some(val) = root.contains(&line[i..]) {
                num = val * 10;
                break
            }
        }
        for i in 0..line.len() {
            if let Some(val) = root_rev.contains(&line_rev[i..]) {
                num += val;
                break
            }
        }

        sum += num;
    }
    sum.to_string()
}