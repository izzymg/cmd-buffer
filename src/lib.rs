use std::{collections::VecDeque, fmt::Debug};
// Commands that act as the "input" to the game engine.
// FIFO buffer implementation
// Push: A, B, C, D
// Read: A, B, C, D
// If the buffer's capacity is < 4, A is dropped when D is pushed
// Read: B, C, D

pub struct CommandBuffer<T>
where
    T: Sync + Send,
{
    events: VecDeque<T>,
}

#[allow(unused)]
impl<T> CommandBuffer<T>
where
    T: Sync + Send,
{
    /// Creates a new command buffer. If the command buffers length grows to `capacity`, subsequent writes will drop the least-recent command.
    pub fn new(capacity: usize) -> Self {
        Self {
            events: VecDeque::with_capacity(capacity),
        }
    }
    /// Write an event to to the command bufer. Returns true if the buffer overran and the first value was dropped.
    pub fn write_command(&mut self, event: T) -> bool {
        let capped = if self.events.len() >= self.events.capacity() {
            self.events.pop_front();
            true
        } else {
            false
        };
        self.events.push_back(event);
        capped
    }

    /// Read an event from the command buffer, destroying it. Returns `None` if the buffer is empty.
    pub fn read_command(&mut self) -> Option<T> {
        self.events.pop_front()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Returns the number of commands currently held in the buffer.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn has<F>(&self, mut predicate: F) -> bool
    where
        F: FnMut(&T) -> bool,
    {
        self.events.iter().any(predicate)
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Retains elements that do not match the predicate and pops off elements that match.
    /// Returns a vector of popped elements.
    pub fn retain<F>(&mut self, mut predicate: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool,
    {
        let mut popped = Vec::new();
        let mut i = 0;
        while i < self.events.len() {
            if predicate(&self.events[i]) {
                if let Some(val) = self.events.remove(i) {
                    popped.push(val);
                }
            } else {
                i += 1;
            }
        }
        popped
    }
}

impl<T> Debug for CommandBuffer<T>
where
    T: Debug + Sync + Send,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandBuffer")
            .field("events", &self.events)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum Event {
        Up,
        Down,
        Left,
        Right,
    }

    #[test]
    fn test_capacity() {
        let capacity = 3;
        let mut events = CommandBuffer::<Event>::new(capacity);
        assert!(!events.write_command(Event::Up));
        assert!(!events.write_command(Event::Down));
        assert!(!events.write_command(Event::Left));
        assert!(events.write_command(Event::Right));
        assert!(events.len() == capacity);

        // Up discarded
        assert_eq!(events.read_command(), Some(Event::Down));
        assert_eq!(events.read_command(), Some(Event::Left));
        assert_eq!(events.read_command(), Some(Event::Right));
        assert_eq!(events.read_command(), None);
        assert!(events.len() == 0);
    }

    #[test]
    fn test_retain_and_pop() {
        let mut events = CommandBuffer::<Event>::new(10);
        events.write_command(Event::Up);
        events.write_command(Event::Down);
        events.write_command(Event::Left);
        events.write_command(Event::Right);

        let popped = events.retain(|e| *e == Event::Down || *e == Event::Left);
        assert_eq!(popped, vec![Event::Down, Event::Left]);
        assert_eq!(events.len(), 2);
        assert_eq!(events.read_command(), Some(Event::Up));
        assert_eq!(events.read_command(), Some(Event::Right));
    }

    #[test]
    fn test_has() {
        let mut events = CommandBuffer::<Event>::new(10);
        events.write_command(Event::Up);
        events.write_command(Event::Down);
        events.write_command(Event::Left);
        events.write_command(Event::Right);

        assert!(events.has(|e| matches!(e, Event::Up)));
        assert!(events.has(|e| matches!(e, Event::Down)));
    }
}
