use zerocopy::{AsBytes, FromBytes, LayoutVerified};
use crate::disk::PageId;

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
    // body: Slotted<B>
}