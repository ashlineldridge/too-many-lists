#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    // Not part of the 2.4 Push chapter but I initially interpretted "push" as an
    // append operation and tried to implement it without looking at the book's code.
    pub fn append(&mut self, elem: i32) {
        let mut cur_link = &mut self.head;
        while let Link::More(cur_node) = cur_link {
            cur_link = &mut cur_node.next;
        }

        let new_node = Box::new(Node {
            elem,
            next: Link::Empty,
        });
        *cur_link = Link::More(new_node);
    }
}
