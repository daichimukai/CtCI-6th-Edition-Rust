/// Stack with min value information.
///
/// # Examples
///
/// ```
/// use ctci_6th_edition::ch3::StackWithMin;
///
/// let mut stack = StackWithMin::new();
/// stack.push(2);
/// stack.push(3);
/// stack.push(1);
/// stack.push(1);
/// stack.push(1);
///
/// assert_eq!(stack.min(), Some(&1));
/// ```
pub struct StackWithMin {
    data_stack: Vec<usize>,
    min_stack: Vec<usize>,
}

impl StackWithMin {
    pub fn new() -> Self {
        StackWithMin {
            data_stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, data: usize) {
        match self.data_stack.len() {
            0 => {
                self.data_stack.push(data);
                self.min_stack.push(data);
            }
            _ => {
                if data <= *self.min_stack.last().unwrap() {
                    self.min_stack.push(data)
                }
                self.data_stack.push(data)
            }
        }
    }

    pub fn pop(&mut self) -> Option<usize> {
        match self.data_stack.len() {
            0 => None,
            _ => {
                let pop = self.data_stack.pop().unwrap();
                if *self.min_stack.last().unwrap() == pop {
                    self.min_stack.pop();
                }
                Some(pop)
            }
        }
    }

    pub fn min(&self) -> Option<&usize> {
        self.min_stack.last()
    }
}
