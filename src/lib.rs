struct LinkedList {
    head: Link,
}

struct Node {
    element: i32,
    next: Link,
}

type Link = Option<Box<Node>>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let list: Option<Box<Node>> = Some(Box::new(Node { 
            element: (1), next: None 
        }));
    }
}
