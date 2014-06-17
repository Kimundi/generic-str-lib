pub struct OffsetSlice<'a> {
    slice: &'a str,
    start: uint,
    end: uint,
}

impl<'a> OffsetSlice<'a> {
    #[inline]
    pub fn new(s: &'a str) -> OffsetSlice<'a> {
        OffsetSlice {
            slice: s,
            start: 0,
            end: s.len()
        }
    }

    #[inline]
    pub fn find_front(&mut self, buf: &[u8]) -> Option<(uint, uint)> {
        while self.start < self.end {
            let start = self.start;
            self.start += 1;

            if self.slice.as_bytes().slice_from(start).starts_with(buf) {
                return Some((start, start + buf.len()));
            }
        }
        None
    }

    #[inline]
    pub fn find_back(&mut self, buf: &[u8]) -> Option<(uint, uint)> {
        while self.start < self.end {
            let end = self.end;
            self.end -= 1;

            if self.slice.as_bytes().slice_to(end).ends_with(buf) {
                return Some((end - buf.len(), end));
            }
        }
        None
    }

    #[inline]
    pub fn original_str(self) -> &'a str {
        self.slice
    }
}
