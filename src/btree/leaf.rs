
use zerocopy::{AsBytes, ByteSlice, FromBytes, LayoutVerified};
use crate::disk::PageId;
use crate::slotted::Slotted;

/// リーフノードのヘッダ
#[derive(Debug, FromBytes, AsBytes)]
#[repr(C)]
pub struct Header {
    /// 次のページID
    prev_page_id: PageId,
    /// 前のページID
    next_page_id: PageId
}

pub struct Leaf<B> {
    header: LayoutVerified<B, Header>,
    body: Slotted<B>
}

impl<B: ByteSlice> Leaf<B> {
    pub fn new(bytes: B) -> Self {
        let (header, body) = LayoutVerified::new_from_prefix(bytes).expect("leaf header must be aligned");
        let body = Slotted::new(body);
        Self { header, body }
    }
}