use zerocopy::{AsBytes, ByteSlice, FromBytes, LayoutVerified};
use crate::disk::PageId;
use crate::slotted::Slotted;

#[derive(Debug, FromBytes, AsBytes)]
#[repr(C)]
pub struct Header {
    right_child: PageId,
}

pub struct Branch<B> {
    header: LayoutVerified<B, Header>,
    body: Slotted<B>,
}

impl<B: ByteSlice> Branch<B> {
    pub fn new(bytes: B) -> Self {
        let (header, body) = LayoutVerified::new_from_prefix(bytes).expect("branch header must be aligned");
        let body = Slotted::new(body);
        Self { header, body }
    }
}