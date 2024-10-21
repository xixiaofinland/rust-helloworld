use pretty::Arena;
use pretty::{BoxAllocator, DocAllocator, DocBuilder};
use std::io;
use std::str;

#[derive(Clone, Debug)]
pub struct Forest<'a>(&'a [Tree<'a>]);

impl<'a> Forest<'a> {
    fn forest(forest: &'a [Tree<'a>]) -> Forest<'a> {
        Forest(forest)
    }

    fn nil() -> Forest<'a> {
        Forest(&[])
    }

    fn bracket<'b, D, A>(&'b self, allocator: &'b D) -> DocBuilder<'b, D, A>
    where
        D: DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        if (self.0).is_empty() {
            allocator.nil()
        } else {
            allocator
                .text("[")
                .append(allocator.hardline().append(self.pretty(allocator)).nest(2))
                .append(allocator.hardline())
                .append(allocator.text("]"))
        }
    }

    fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> DocBuilder<'b, D, A>
    where
        D: DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        let forest = self.0;
        let separator = allocator.text(",").append(allocator.hardline());
        allocator.intersperse(forest.iter().map(|tree| tree.pretty(allocator)), separator)
    }
}

#[derive(Clone, Debug)]
pub struct Tree<'a> {
    node: String,
    forest: Forest<'a>,
}

impl<'a> Tree<'a> {
    pub fn node(node: &str) -> Tree<'a> {
        Tree {
            node: node.to_string(),
            forest: Forest::nil(),
        }
    }

    pub fn node_with_forest(node: &str, forest: &'a [Tree<'a>]) -> Tree<'a> {
        Tree {
            node: node.to_string(),
            forest: Forest::forest(forest),
        }
    }

    pub fn pretty<'b, D, A>(&'b self, allocator: &'b D) -> DocBuilder<'b, D, A>
    where
        D: DocAllocator<'b, A>,
        D::Doc: Clone,
        A: Clone,
    {
        allocator
            .text(&self.node[..])
            .append((self.forest).bracket(allocator))
            .group()
    }
}

#[allow(dead_code)]
pub fn main() {
    let arena: Arena<'_, ()> = pretty::Arena::new();

    // Function example
    // Function example
    let doc = arena
        .text("function")
        .append(arena.space())
        .append(arena.text("example("))
        .append(
            arena
                .softline() // Add softline before param1
                .append(arena.text("param1,"))
                .append(arena.line())
                .append(arena.text("param2,"))
                .append(arena.line())
                .append(arena.text("param3"))
                .append(arena.softline())
                .nest(2)
                .group(),
        )
        .append(
            arena
                .text(")")
                .append(arena.space())
                .append(arena.text("{"))
                .append(
                    arena
                        .line()
                        .append(arena.text("const x = 1;"))
                        .append(arena.softline())
                        .append(arena.text("const y = 2;"))
                        .nest(4)
                        .group(),
                )
                .append(arena.line())
                .append(arena.text("}")),
        )
        .group();
    // Array example
    let list_doc = arena
        .text("const items = [")
        .append(
            arena
                .line_()
                .append(arena.text("'first item',"))
                .append(arena.line())
                .append(arena.text("'second item',"))
                .append(arena.line())
                .append(arena.text("'third item'"))
                .nest(2) // Use nest for array items
                .group(),
        )
        .append(arena.line_())
        .append(arena.text("];"))
        .group();

    println!("Width 80:");
    println!("\n\nArray example:");
    list_doc.render(80, &mut io::stdout()).unwrap();
    println!("\n\nWidth 20:");
    println!("\n\nArray example:");
    list_doc.render(20, &mut io::stdout()).unwrap();

    //println!("Width 80:");
    //println!("Function example:");
    //doc.render(80, &mut io::stdout()).unwrap();
    //println!("\n\nWidth 20:");
    //println!("Function example:");
    //doc.render(20, &mut io::stdout()).unwrap();
}
