#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct DynamicLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: PartialEq + Copy> DynamicLinkedList<T> {
    pub fn new() -> Self {
        DynamicLinkedList { head: None }
    }

    pub fn insert(&mut self, data: T) {
        let mut new_node = Box::new(Node { data, next: None });

        match self.head.as_mut() {
            None => self.head = Some(new_node),
            Some(mut current) => {
                while let Some(next) = current.next.as_mut() {
                    current = next;
                }
                current.next = Some(new_node);
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<T> {
        let mut current = self.head.as_ref();
        let mut count = 0;

        while let Some(node) = current {
            if count == index {
                return Some(node.data);
            }
            current = node.next.as_ref();
            count += 1;
        }

        None
    }
}
impl<T: PartialEq + Copy> DynamicLinkedList<T> {
    // ... existing methods

    pub fn insert_at_index(&mut self, index: usize, data: T) {
        let mut new_node = Box::new(Node { data, next: None });

        if index == 0 {
            new_node.next = self.head.take();
            self.head = Some(new_node);
            return;
        }

        let mut current = self.head.as_mut();
        let mut i = 0;

        while let Some(node) = current {
            if i + 1 == index {
                new_node.next = node.next.take();
                node.next = Some(new_node);
                return;
            }
            current = node.next.as_mut();
            i += 1;
        }

        // Index out of bounds: do nothing or handle explicitly if needed
    }
}
impl<T: PartialEq + Copy> DynamicLinkedList<T> {
    // ... existing methods

    pub fn delete_element(&mut self, data: T) -> bool {
        let mut current = self.head.as_mut();

        // Check if head needs to be deleted
        if let Some(node) = self.head.as_ref() {
            if node.data == data {
                self.head = self.head.take().unwrap().next;
                return true;
            }
        }

        while let Some(node) = current {
            if let Some(ref mut next_node) = node.next {
                if next_node.data == data {
                    node.next = next_node.next.take();
                    return true;
                }
            }
            current = node.next.as_mut();
        }

        false
    }
}
impl<T: PartialEq + Copy> DynamicLinkedList<T> {
    // ... existing methods

    pub fn delete_at_index(&mut self, index: usize) -> bool {
        if index == 0 {
            if self.head.is_some() {
                self.head = self.head.take().unwrap().next;
                return true;
            } else {
                return false;
            }
        }

        let mut current = self.head.as_mut();
        let mut i = 0;

        while let Some(node) = current {
            if i + 1 == index {
                if let Some(ref mut next_node) = node.next {
                    node.next = next_node.next.take();
                    return true;
                } else {
                    return false;
                }
            }
            current = node.next.as_mut();
            i += 1;
        }

        false
    }
}
impl<T: PartialEq + Copy> DynamicLinkedList<T> {
    // ... existing methods

    pub fn update_element(&mut self, old_data: T, new_data: T) -> bool {
        let mut current = self.head.as_mut();

        while let Some(node) = current {
            if node.data == old_data {
                node.data = new_data;
                return true;
            }
            current = node.next.as_mut();
        }

        false
    }
}
impl<T: PartialEq + Copy> DynamicLinkedList<T> {
    // ... existing methods

    pub fn update_element_at_index(&mut self, index: usize, data: T) -> bool {
        let mut current = self.head.as_mut();
        let mut i = 0;

        while let Some(node) = current {
            if i == index {
                node.data = data;
                return true;
            }
            current = node.next.as_mut();
            i += 1;
        }

        false
    }
}
