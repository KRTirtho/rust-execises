use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&Node<T>),
    {
        let mut head = &self.head;
        while let Some(ref node) = *head {
            f(node);
            head = &node.next;
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        match self.head {
            Some(_) => false,
            _ => true,
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        self.for_each(|_| count += 1);
        count
    }

    pub fn push(&mut self, _element: T) {
        let mut node = Node::new(_element);
        match self.head {
            Some(_) => {
                let head = self.head.take().unwrap();
                node.next = Some(head);
                self.head = Some(Box::new(node));
            }
            _ => self.head = Some(Box::new(node)),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|box_node| {
            let node = *box_node;
            self.head = node.next;
            node.data
        })
    }
    // optionally returns head's data if list isn't empty
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|ref node| &node.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();
        self.for_each(|ref node| reversed.push(node.data.clone()));
        reversed
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for i in _iter {
            list.push(i);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vector = Vec::new();
        let mut head = self.head;
        while let Some(node) = head {
            vector.insert(0, node.data);
            head = node.next;
        }
        vector
    }
}
