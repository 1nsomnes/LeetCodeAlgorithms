// #13
pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
     }
   }
}
pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    if head.is_none() { return true; }

    let mut items:Vec<i32> = Vec::new();
    let mut currentNode = head.unwrap();

    loop {
        items.push(currentNode.val);

        match currentNode.next {
            Some(newNode) => currentNode = newNode,
            None => break
        }
    }

    if items.clone().into_iter().rev().collect::<Vec<i32>>() == items { return true; }
    false
}

