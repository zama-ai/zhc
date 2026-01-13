use crate::Dispatch;
use hc_langs::doplang::Affinity;

use super::*;

/// Virtual Control Processing Element.
#[derive(Debug, Serialize)]
pub struct PeCtl;

impl Simulatable for PeCtl {
    type Event = Events;
    fn handle(
        &mut self,
        dispatcher: &mut impl Dispatch<Event = Self::Event>,
        trigger: Trigger<Self::Event>,
    ) {
        match trigger.event {
            Events::IscIssueDOp(dop) if dop.raw.affinity() == Affinity::Ctl => {
                dispatcher.dispatch_now(Events::IscUnlockIssue(dop.id));
                dispatcher.dispatch_now(Events::IscUnlockRead(dop.id));
                dispatcher.dispatch_now(Events::IscUnlockWrite(dop.id));
            }
            _ => {}
        }
    }

    fn name(&self) -> String {
        "PeCtl".into()
    }
}
