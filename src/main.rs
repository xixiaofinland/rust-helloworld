use tree_sitter::Node;

fn main() {
    println!("Hello, world!");
}

pub struct Class<'a, 'tree> {
    inner: &'a Node<'tree>,
}

impl<'a, 'tree> Class<'a, 'tree> {
    pub fn get_1(&self) -> Vec<Node<'tree>> {
        if let Some(n) = self.inner.get_child("modifiers") {
            vec![n]
        } else {
            Vec::new()
        }
    }

    pub fn get_2(&self) -> Vec<Node<'tree>> {
        if let Some(n) = self.inner.get_child("modifiers") {
            n.get_children("modifier")
        } else {
            Vec::new()
        }
    }
}

pub trait NodeUtilities<'tree> {
    fn get_child(&self, kind: &str) -> Option<Node<'tree>>;
    fn get_children(&self, kind: &str) -> Vec<Node<'tree>>;
}

impl<'tree> NodeUtilities<'tree> for Node<'tree> {
    fn get_child(&self, kind: &str) -> Option<Node<'tree>> {
        let mut cursor = self.walk();
        for child in self.children(&mut cursor) {
            if child.kind() == kind {
                return Some(child);
            }
        }
        None
    }

    fn get_children(&self, kind: &str) -> Vec<Node<'tree>> {
        let mut cursor = self.walk();
        let mut modifiers = Vec::new();
        for child in self.children(&mut cursor) {
            if child.kind() == kind {
                modifiers.push(child);
            }
        }
        modifiers
    }
}
