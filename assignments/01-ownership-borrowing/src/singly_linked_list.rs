struct Node {
    val: i32,
    next: Link,
}

type Link = Option<Box<Node>>;
// A stack is first in last out. So we need to add elements to the head of the list.
pub struct LinkedStack {
    head: Link,
}

impl LinkedStack {
    fn new() -> Self {
        LinkedStack { head: None }
    }

    fn push(&mut self, val: i32) {
        let temp: Link = self.head.take();
        self.head = Some(Box::new(Node {
            val: val,
            next: temp,
        }))
    }

    fn pop(&mut self) -> Option<i32> {
        // Take the current head, leaving `self.head` as None
        if let Some(node) = self.head.take() {
            // Move self.head to point to the next node
            self.head = node.next;
            // Return the value of the node we just popped
            Some(node.val)
        } else {
            // If the stack was empty, return None
            None
        }
    }
}

impl Drop for LinkedStack {
    fn drop(&mut self) {
        // Continually pop elements from the stack until it's empty
        while let Some(_) = self.pop() {}
    }
}

// DO NOT MODIFY BELOW THIS LINE

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack() {
        let mut stack = LinkedStack::new();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_linked_stack() {
        let mut stack = LinkedStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        stack.push(4);

        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_big_stack() {
        let mut stack = LinkedStack::new();
        for i in 0..1_000_000 {
            stack.push(i);
        }

        for i in (0..1_000_000).rev() {
            assert_eq!(stack.pop(), Some(i));
        }

        assert_eq!(stack.pop(), None);
    }
}
