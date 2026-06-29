use crate::Dispatch;

use super::*;

#[derive(Debug, Serialize)]
pub struct UCore;

impl Simulatable for UCore {
    type Event = Events;
    fn handle(
        &mut self,
        dispatcher: &mut impl Dispatch<Event = Self::Event>,
        trigger: Trigger<Self::Event>,
    ) {
        match trigger.event {
            Events::UCorePushDOps(dops) => {
                for dop in dops {
                    dispatcher.dispatch_now(Events::IscPushDOp(dop));
                }
            }
            _ => {}
        }
    }

    fn name(&self) -> String {
        "UCore".into()
    }
}
