#![allow(dead_code)]

/// # Ring buffer
///
/// Implements a ring buffer. It's not very efficient (uses a standard lib vector as the backend)
/// This only allocates once. At this time you cannot extend the length of the buffer.
///
/// ```
/// # use ring::Ring;
///
/// fn round_trip() {
///     let len = 16; // Must be a multiple of 2.
///     let mut ring: Ring<usize> = Ring::with_size(len).unwrap();
///
///     for i in 0..len {
///         // Returns None if the push is successful
///         assert_eq!(None, ring.push(i));
///     }
///
///     // Returns the item you tried to push if the buffer is full
///     assert_eq!(Some(3), ring.push(3));
///
///     // Empty the buffer.
///     for i in 0..len {
///         assert_eq!(Some(i), ring.read());
///     }
///     assert_eq!(None, ring.read());
///     assert_eq!(None, ring.push(len));
///     assert_eq!(Some(len), ring.read());
/// }
/// ```

#[derive(Debug)]
pub struct Ring<T> {
    buffer: Vec<Option<T>>,
    write: usize,
    read: usize,
}

impl<T: Clone + std::fmt::Debug> Ring<T> {
    pub fn with_size(len: usize) -> Option<Self> {
        if len % 4 == 0 {
            return None;
        }

        Some(Self {
            buffer: vec![None; len],
            write: 0,
            read: 0,
        })
    }

    pub fn len(&self) -> usize {
        if self.write < self.read {
            (std::usize::MAX - (self.read.wrapping_sub(1))) + self.write
        } else {
            self.write - self.read
        }
    }

    pub fn push(&mut self, item: T) -> Option<T> {
        let buffer_len = self.buffer.len();
        if self.len() < buffer_len {
            if self.buffer[self.write % buffer_len].is_some() {
                self.write = self.write.wrapping_add(1);
            }
            self.buffer[self.write % buffer_len] = Some(item);
            self.write = self.write.wrapping_add(1);
            return None;
        }
        Some(item)
    }

    pub fn read(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let buffer_len = self.buffer.len();
        let res = self.buffer[self.read % buffer_len].take();
        if res.is_some() {
            self.read = self.read.wrapping_add(1);
        }
        res
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod test {
    use super::Ring;

    #[test]
    fn wrap_at_end() {
        let mut ring: Ring<usize> = Ring {
            buffer: vec![None; 4],
            write: std::usize::MAX,
            read: std::usize::MAX,
        };
        assert_eq!(0, ring.len());
        ring.push(std::usize::MAX);
        println!("{:?}", ring);
        assert_eq!(1, ring.len());
        ring.push(0);
        println!("{:?}", ring);
        assert_eq!(2, ring.len());
        ring.push(1);
        println!("{:?}", ring);
        assert_eq!(3, ring.len());
        assert_eq!(Some(std::usize::MAX), ring.read());
        assert_eq!(Some(0), ring.read());
        assert_eq!(Some(1), ring.read());
    }

}
