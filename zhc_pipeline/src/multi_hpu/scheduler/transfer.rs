use serde::Serialize;
use zhc_config::multi_hpu::MultiHpuConfig;
use zhc_ir::{AsOpId, IR, OpId, OpMap, OpRef};
use zhc_langs::hpulang::{HpuId, HpuInstructionSet, HpuLang, NextTransferId, TransferId};
use zhc_utils::{Store, small::SmallSet};

#[derive(Debug, Clone, Serialize)]
struct HpuState {
    next_tid: TransferId,
    recycling: Option<OpId>,
    locked: SmallSet<HpuId>,
    n_hpus: u8
}

impl HpuState {
    pub fn new(n_hpus: u8) -> Self {
        HpuState {
            next_tid: TransferId::FIRST,
            recycling: None,
            locked: SmallSet::new(),
            n_hpus
        }
    }

    pub fn get_tid(&mut self, opid: OpId) -> TransferId {
        let oup = self.next_tid;
        self.next_tid = match self.next_tid.inc() {
            NextTransferId::NewGen(tid) => {
                self.recycling = Some(opid);
                self.locked = (0..self.n_hpus).map(HpuId).collect();
                tid
            },
            NextTransferId::SameGen(tid) => {
                tid
            },
        };
        oup
    }

    pub fn does_lock(&self, hid: HpuId) -> bool {
        self.locked.contains(&hid)
    }

    pub fn unlock(&mut self, hid: HpuId) {
        assert!(self.locked.remove(&hid));
    }

    pub fn last_recycling(&self) -> OpId {
        *self.recycling.as_ref().unwrap()
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum TransferKind {
    Unconstrained { tid: TransferId },
    Locking { tid: TransferId, locks: Vec<HpuId>},
    Locked { tid: TransferId, waits: HpuId },
    LockedLocking { tid: TransferId, waits: HpuId, locks: Vec<HpuId> }
}

impl TransferKind {
    pub fn lock(&mut self, hid: HpuId) {
        match self {
            TransferKind::Locking {locks: locked, ..} | TransferKind::LockedLocking { locks: locked, .. } => {
                assert!(!locked.contains(&hid));
                locked.push(hid);
            }
            _ => unreachable!()
        }
    }

    pub fn get_tid(&self) -> TransferId {
        match self {
            TransferKind::Unconstrained { tid }
            | TransferKind::Locking { tid, .. }
            | TransferKind::Locked { tid, .. }
            | TransferKind::LockedLocking { tid, .. } => *tid,
        }
    }

    pub fn get_wait(&self) -> Option<HpuId> {
        match self {
            TransferKind::Locked { waits, .. } | TransferKind::LockedLocking { waits, .. } => Some(*waits),
            _ => None
        }
    }

    pub fn get_locks(&self) -> Option<Vec<HpuId>> {
        match self {
            TransferKind::Locking { locks, .. } | TransferKind::LockedLocking { locks, .. } => Some(locks.clone()),
            _ => None
        }
    }
}

#[derive(Serialize)]
struct TransferMapper {
    pub transfer_kinds: OpMap<TransferKind>,
    hpu_states: Store<HpuId, HpuState>,
}

impl TransferMapper {
    pub fn new(
        map: OpMap<TransferKind>,
        config: &MultiHpuConfig,
    ) -> Self {
        TransferMapper { transfer_kinds: map, hpu_states: Store::with_value(HpuState::new(config.n_hpus), config.n_hpus as usize) }
    }

    pub fn build<'a>(
        mut self,
        iter: impl Iterator<Item = OpRef<'a, HpuLang>>
    ) -> Self {
        for op in iter {
            let HpuInstructionSet::Transfer { from, to } = op.get_instruction() else {unreachable!()};
            let opid = op.op_id();
            let waits = if self.hpu_states[to].does_lock(*from) {
                let last_recycling = self.hpu_states[to].last_recycling();
                self.transfer_kinds[last_recycling].lock(*from);
                self.hpu_states[to].unlock(*from);
                Some(to)
            } else {
                None
            };
            let tid = self.hpu_states[to].get_tid(opid);
            let tkind = if tid.is_last_of_gen() {
                if let Some(waits) = waits.copied() {
                    TransferKind::LockedLocking { tid, waits, locks: Vec::new() }
                } else {
                    TransferKind::Locking { tid, locks: Vec::new() }
                }
            } else {
                if let Some(waits) = waits.copied() {
                    TransferKind::Locked { tid, waits }
                } else {
                    TransferKind::Unconstrained { tid }
                }
            };
            self.transfer_kinds.insert(opid, tkind);
        }
        self
    }

    pub fn into_map(self) -> OpMap<TransferKind> {
        self.transfer_kinds
    }
}


pub fn build_transfer_map(ir: &IR<HpuLang>, walker: impl Iterator<Item=OpId>, config: &MultiHpuConfig) -> OpMap<TransferKind> {
    let transfer_map = ir.empty_opmap();
    TransferMapper::new(transfer_map, config)
        .build(ir.walk_ops_with(walker))
        .into_map()
}
