use crate::StmtAst;
use cool_resolve::FrameId;
use rustc_hash::FxHashMap;
use std::sync::Arc;

#[derive(Clone, Default, Debug)]
pub struct DeferStmtMap {
    defer_stmts: FxHashMap<FrameId, Arc<StmtAst>>,
}

impl DeferStmtMap {
    #[inline]
    pub fn insert(&mut self, frame_id: FrameId, code: Arc<StmtAst>) {
        self.defer_stmts.insert(frame_id, code);
    }

    #[inline]
    pub fn get(&self, frame_id: FrameId) -> Option<&StmtAst> {
        self.defer_stmts.get(&frame_id).map(Arc::as_ref)
    }
}
