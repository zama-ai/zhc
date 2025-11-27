use hpuc_langs::doplang::Affinity;

use super::*;

#[derive(Debug, Serialize)]
pub struct PeCtl;

impl Simulatable for PeCtl {
    type Event = Events;

    fn handle(&mut self, dispatcher: &mut Dispatcher<Self::Event>, trigger: Trigger<Self::Event>) {
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
