use std::collections::{HashSet, LinkedList};

/// Remove duplicates from an unsorted linked list LIST.
pub fn remove_dups(list: &mut LinkedList<usize>) {
    let mut result = LinkedList::new();
    let mut appeared = HashSet::new();

    while let Some(data) = list.pop_front() {
        if appeared.get(&data).is_none() {
            appeared.insert(data);
            result.push_back(data);
        }
    }

    list.append(&mut result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_dups() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(3);
        list.push_back(3);
        list.push_back(4);
        remove_dups(&mut list);

        let mut list2 = LinkedList::new();
        list2.push_back(1);
        list2.push_back(3);
        list2.push_back(4);

        assert_eq!(list, list2);
    }
}
