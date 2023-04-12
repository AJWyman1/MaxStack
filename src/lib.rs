use std::cell::RefCell;

pub struct MaxStack<N> {
    head: RefCell<Option<Box<MaxStackElem<N>>>>,
    max: Option<Box<MaxStackElem<N>>>,
}
struct MaxStackElem<N> {
    value: N,
    next: Option<Box<MaxStackElem<N>>>,
}

impl<N> MaxStack<N> 
where N: Ord + Copy
{
    pub fn new() -> Self {
        return MaxStack { head: RefCell::new(None), max: None };
    }
    pub fn push(&mut self, val: N) {
        match &mut self.max {
            Some(max_elem) => {
                if val >= max_elem.value {
                    self.max = Some(Box::new(MaxStackElem { value: val, next: self.max.take() }))
                }
            },
            None => self.max = Some(Box::new(MaxStackElem { value: val, next: None }))
        }
        if self.head.borrow().is_some(){
            self.head = RefCell::new(Some( Box::new(MaxStackElem{ value: val, next: self.head.take()} )));
        }
        else{
            self.head = RefCell::new(Some( Box::new(MaxStackElem { value: val, next: None})));
        }
    }
    pub fn pop(&mut self) -> Option<N> {
        if let Some(top_elem) = self.head.take() {
            let return_val = top_elem.value;

            if let Some(max_elem) = &mut self.max {
                    if return_val >= max_elem.value {
                        self.max = max_elem.next.take();
                    }
                }

            self.head.swap(&RefCell::new(top_elem.next));
            return Some(return_val);
        }
        return None;
    }
    pub fn peek(&mut self) -> Option<&N> {
        if let Some(top_elem) = self.head.get_mut() {
            return Some(&top_elem.value);
        }
        None
    }
    pub fn get_max(&self) -> Option<&N> {
        match &self.max {
            Some(max_elem) => 
                return Some(&max_elem.value),
            None => return None
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_peek_pop_test() {
        let mut stack = MaxStack::new();
        stack.push(4);
        stack.push(7);
        assert_eq!(stack.peek(), Some(&7));
        assert_eq!(stack.pop(), Some(7));
    }
    #[test]
    fn get_max_test() {
        let mut stack = MaxStack::new();
        stack.push(4);
        assert_eq!(stack.get_max(), Some(&4));
        stack.push(7);
        assert_eq!(stack.get_max(), Some(&7));
        stack.push(2);
        stack.push(15);
        stack.pop();
        assert_eq!(stack.get_max(), Some(&7));
        stack.pop();
        stack.pop();
        assert_eq!(stack.get_max(), Some(&4));
        stack.push(9);
        assert_eq!(stack.get_max(), Some(&9));
    }
}
