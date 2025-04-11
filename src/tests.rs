use linked_list::dynamic_list::DynamicLinkedList;

#[test]
fn test_insert() {
    let mut list = DynamicLinkedList::new();
    list.insert(10);
    list.insert(20);
    list.insert(30);

    assert_eq!(list.get(0), Some(10));
    assert_eq!(list.get(1), Some(20));
    assert_eq!(list.get(2), Some(30));
}

#[test]
fn test_insert_at_index() {
    let mut list = DynamicLinkedList::new();
    list.insert(1);
    list.insert(3);
    list.insert_at_index(1, 2); // Insert 2 between 1 and 3

    assert_eq!(list.get(0), Some(1));
    assert_eq!(list.get(1), Some(2));
    assert_eq!(list.get(2), Some(3));

    list.insert_at_index(0, 0); // Insert at head
    assert_eq!(list.get(0), Some(0));
}
#[test]
fn test_delete_element() {
    let mut list = DynamicLinkedList::new();
    list.insert(10);
    list.insert(20);
    list.insert(30);

    assert!(list.delete_element(20)); // Should delete the middle
    assert_eq!(list.get(1), Some(30));
    assert_eq!(list.get(2), None);

    assert!(list.delete_element(10)); // Delete head
    assert_eq!(list.get(0), Some(30));

    assert!(!list.delete_element(100)); // Non-existent value
}
#[test]
fn test_delete_at_index() {
    let mut list = DynamicLinkedList::new();
    list.insert(5);
    list.insert(10);
    list.insert(15);

    assert!(list.delete_at_index(1)); // Delete the middle (10)
    assert_eq!(list.get(0), Some(5));
    assert_eq!(list.get(1), Some(15));

    assert!(list.delete_at_index(0)); // Delete head (5)
    assert_eq!(list.get(0), Some(15));

    assert!(!list.delete_at_index(5)); // Index out of bounds
}
#[test]
fn test_update_element() {
    let mut list = DynamicLinkedList::new();
    list.insert(1);
    list.insert(2);
    list.insert(3);

    assert!(list.update_element(2, 20)); // Update middle
    assert_eq!(list.get(1), Some(20));

    assert!(list.update_element(1, 10)); // Update head
    assert_eq!(list.get(0), Some(10));

    assert!(!list.update_element(100, 999)); // No match
}
#[test]
fn test_update_element_at_index() {
    let mut list = DynamicLinkedList::new();
    list.insert(100);
    list.insert(200);
    list.insert(300);

    assert!(list.update_element_at_index(1, 250)); // Update middle
    assert_eq!(list.get(1), Some(250));

    assert!(list.update_element_at_index(0, 150)); // Update head
    assert_eq!(list.get(0), Some(150));

    assert!(!list.update_element_at_index(5, 500)); // Out of bounds
}
