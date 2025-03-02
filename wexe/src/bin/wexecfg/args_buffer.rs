#![allow(dead_code)]

pub struct ArgumentsBuffer {
    all_args: Vec<String>,
    current_arg: usize,
}

impl ArgumentsBuffer {
    pub fn new(args: Vec<String>) -> ArgumentsBuffer {
        ArgumentsBuffer {
            all_args: args,
            current_arg: 0,
        }
    }

    // pub fn next(&mut self) -> Option<&str> {
    //     if self.current_arg < self.all_args.len() {
    //         let arg = &self.all_args[self.current_arg];
    //         self.current_arg += 1;
    //         Some(arg)
    //     } else {
    //         None
    //     }
    // }

    /// Peek at the next argument without consuming it.
    pub fn peek(&self) -> Option<&str> {
        if self.current_arg < self.all_args.len() {
            Some(&self.all_args[self.current_arg])
        } else {
            None
        }
    }

    /// Peek at an argument at a specific offset from the current position
    /// without consuming it. x.peek_at(0) is equivalent to x.peek().
    pub fn peek_at(&self, index: usize) -> Option<&str> {
        if index + self.current_arg < self.all_args.len() {
            Some(&self.all_args[index + self.current_arg])
        } else {
            None
        }
    }

    /// Consume the next `count` arguments without returning them.
    pub fn skip(&mut self, count: usize) {
        if self.current_arg + count > self.all_args.len() {
            panic!("Tried to skip past the end of the arguments buffer.");
        }
        self.current_arg += count;
    }

    /// Return the number of arguments remaining in the buffer.
    pub fn remaining(&self) -> usize {
        self.all_args.len() - self.current_arg
    }

    /// Return true if there are no more arguments in the buffer.
    pub fn is_empty(&self) -> bool {
        self.current_arg >= self.all_args.len()
    }
}
