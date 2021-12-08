 pub struct LinkedList {
    head: Link,
}

impl LinkedList {
    fn empty() -> LinkedList {
        LinkedList {head: None}
    }

    fn push(&mut self, element: i32) {
        let old_head: Option<Box<Node>> = self.head.take();
        let new_head: Box<Node> = Box::new(Node { 
            element, 
            next: old_head, 
        });

        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.element
        }) 
    }

    fn peek(&mut self) -> Option<&i32> {
        match &self.head {
            Some(n) => Some(&n.element),
            None => None,
        }
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
        let mut list: LinkedList = LinkedList::empty();
        list.push(1);
        list.push(2);
        
    }
}
