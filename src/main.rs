use crate::utility::NodeUtilities;
use tree_sitter::Node;

mod utility;

fn main() {
    println!("Hello, world!");
}

pub struct Class<'a> {
    inner: &'a Node<'a>,
}

impl<'a> Class<'a> {
    pub fn get_1(&self) -> Vec<Node> {
        if let Some(n) = self.inner.get_child_by_kind("modifiers") {
            vec![n]
        } else {
            Vec::new()
        }
    }

    pub fn get_2(&self) -> Vec<Node> {
        if let Some(n) = self.inner.get_child_by_kind("modifiers") {
            n.get_children_by_kind("modifier")
        } else {
            Vec::new()
        }
    }
}
