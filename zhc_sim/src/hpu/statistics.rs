use super::*;

/// Tracks completed operations and manages resource cleanup after execution.
#[derive(Debug, Default, Serialize)]
pub struct Statistics {
    #[serde(skip)]
    pub dops: Vec<DOp>,
    pub timeouts: u16,
}

impl Simulatable for Statistics {
    type Event = Events;

    fn handle(
        &mut self,
        _dispatcher: &mut impl Dispatch<Event = Self::Event>,
        trigger: Trigger<Self::Event>,
    ) {
        match trigger.event {
            Events::IscRetireDOp(dop) => {
                self.dops.push(dop);
            }
            Events::NotifyStartOnTimeout { .. } => {
                self.timeouts += 1;
            }
            _ => {}
        }
    }
}
