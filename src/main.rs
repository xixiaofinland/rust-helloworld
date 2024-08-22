use tree_sitter::Node;

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

pub trait NodeUtilities {
    fn get_child_by_kind(&self, kind: &str) -> Option<Node>;
    fn get_children_by_kind(&self, kind: &str) -> Vec<Node>;
}

impl NodeUtilities for Node<'_> {
    fn get_child_by_kind(&self, kind: &str) -> Option<Node> {
        let mut cursor = self.walk();
        for child in self.children(&mut cursor) {
            if child.kind() == kind {
                return Some(child);
            }
        }
        None
    }

    fn get_children_by_kind(&self, kind: &str) -> Vec<Node> {
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
