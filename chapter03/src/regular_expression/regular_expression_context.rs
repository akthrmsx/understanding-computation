#[derive(Debug, Clone, Default)]
pub struct RegularExpressionContext {
    current_state: usize,
}

impl RegularExpressionContext {
    pub fn next_state(&mut self) -> usize {
        let current_state = self.current_state;
        self.current_state += 1;
        current_state
    }
}
