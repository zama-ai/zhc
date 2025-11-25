use hpuc_langs::doplang::Affinity;
use crate::Dispatch;

use super::*;

#[derive(Debug, Serialize)]
pub struct PeCtl;

impl<D: Dispatch<Event = Events>> Simulatable<D> for PeCtl {
    fn handle(&mut self, dispatcher: &mut D, trigger: Trigger<D::Event>) {
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
