use std::collections::VecDeque;

/// FIFO buffer implementation
/// Push: A, B, C, D
/// Read: A, B, C, D
/// If the buffer's capacity is < 4, A is dropped when D is pushed
/// Read: B, C, D

pub struct CommandBuffer<T>
where
    T: Sync + Send + Copy,
{
    events: VecDeque<T>,
}

impl<T> CommandBuffer<T>
where
    T: Sync + Send + Copy,
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

    /// Returns the number of commands currently held in the buffer.
    pub fn len(&self) -> usize {
        self.events.len()
    }
}

#[cfg(test)]
mod test {
    use crate::CommandBuffer;

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
}
