 pub struct LinkedList {
    head: Link,
}

impl LinkedList {
    fn empty() -> LinkedList {
        LinkedList {head: None}
    }
}

struct Node {
    element: i32,
    next: Link,
}

type Link = Option<Box<Node>>;

#[cfg(test)]
mod tests {
    use std::io::Empty;

    use super::*;
    #[test]
    fn it_works() {
        let list: LinkedList = LinkedList::empty();
    }
}
