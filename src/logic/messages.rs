use std::collections::VecDeque;

pub struct MessageLog {
    max_memory: usize,
    messages: VecDeque<(String, u64)>,
    curr_turn: u64,
}

impl MessageLog {
    pub fn new(max_memory: usize) -> Self {
        Self {
            max_memory,
            messages: VecDeque::with_capacity(max_memory),
            curr_turn: 0,
        }
    }

    pub fn append<S: Into<String>>(&mut self, msg: S) {
        self.messages.push_back((msg.into(), self.curr_turn));
        if self.messages.len() > self.max_memory {
            self.messages.pop_front();
        }
    }

    pub fn start_turn(&mut self) {
        self.curr_turn += 1;
    }

    pub fn latest(&self, count: usize) -> impl Iterator<Item = (&str, u64)> {
        self.messages
            .iter()
            .rev()
            .take(count)
            .rev()
            .map(|(msg, turn)| (msg.as_str(), self.curr_turn - *turn))
    }

    pub fn peek(&self, start: usize, count: usize) -> impl Iterator<Item = &str> {
        self.messages
            .iter()
            .skip(start)
            .take(count)
            .map(|(msg, _)| msg.as_str())
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }
}
