use std::ops::Index;

struct Mem<T> {
    mem: [T]
}

impl Index<u16, u16> for Mem<u16> {
    fn index<'a> (&'a self, index: &u16) -> &'a u16 {
        &self.mem[*index as uint]
    }
}

impl Index<u16, u8> for Mem<u16> {
    fn index<'a> (&'a self, index: &u16) -> &'a u16 {
        &self.mem[*index as uint]
    }
}
