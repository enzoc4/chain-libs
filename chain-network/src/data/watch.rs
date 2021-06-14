use super::{BlockId, FragmentId};

#[derive(Debug)]
pub struct MempoolUpdate {
    pub fragment_id: FragmentId,
    pub event: MempoolEvent,
}

#[derive(Debug)]
pub enum MempoolEvent {
    FragmentInserted,
    FragmentRejected { reason: String },
    FragmentInABlock { block_id: BlockId },
}
