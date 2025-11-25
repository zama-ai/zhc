use super::*;

#[derive(Debug, Default, Serialize)]
pub struct Retirement {
    dops: Vec<DOp>,
}

impl Retirement {
    pub fn last_retired(&self) -> Option<&DOp> {
        self.dops.last()
    }
}

impl Simulatable for Retirement {
    type Event = Events;

    fn handle(&mut self, _dispatcher: &mut impl Dispatch<Event = Events>, trigger: Trigger<Events>) {
        match trigger.event {
            Events::IscRetireDOp(dop) => {
                self.dops.push(dop);
            }
            _ => {}
        }
    }
}
