use std::fmt::Debug;

#[derive(Debug)]
struct Ring<T: Debug> {
    buffer: Vec<Option<T>>,
    write: usize,
    read: usize,
}

impl<T: Clone + Debug> Ring<T> {
    fn with_size(len: usize) -> Self {
        Self {
            buffer: vec![None; len],
            write: 0,
            read: 0,
        }
    }

    fn len(&self) -> usize {
        if self.write < self.read {
            (std::usize::MAX - (self.read.wrapping_sub(1))) + self.write
        } else {
            self.write - self.read
        }
    }

    fn push(&mut self, item: T) -> Option<T> {
        let buffer_len = self.buffer.len();
        if self.len() < buffer_len {
            self.buffer[self.write % buffer_len] = Some(item);
            self.write = self.write.wrapping_add(1);
            return None;
        }
        Some(item)
    }

    fn read(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        }

        let buffer_len = self.buffer.len();
        let res = self.buffer[self.read % buffer_len].take();
        if res.is_some() {
            self.read = self.read.wrapping_add(1);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let len = 10;
        let mut ring: Ring<usize> = Ring::with_size(len);

        for i in 0..len {
            assert_eq!(None, ring.push(i));
        }

        assert_eq!(Some(3), ring.push(3));

        for i in 0..len {
            assert_eq!(Some(i), ring.read());
        }
        assert_eq!(None, ring.read());
        assert_eq!(None, ring.push(len));
        assert_eq!(Some(len), ring.read());
    }

    #[test]
    fn wrap_at_end() {
        let mut ring: Ring<usize> = Ring {
            buffer: vec![None; 3],
            write: std::usize::MAX,
            read: std::usize::MAX,
        };
        assert_eq!(0, ring.len());
        ring.push(std::usize::MAX);
        assert_eq!(1, ring.len());
        ring.push(0);
        assert_eq!(2, ring.len());
        ring.push(1);
        assert_eq!(3, ring.len());
    }
}
