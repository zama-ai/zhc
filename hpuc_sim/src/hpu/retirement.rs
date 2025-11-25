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

impl<D: Dispatch<Event = Events>> Simulatable<D> for Retirement {
    fn handle(&mut self, _dispatcher: &mut D, trigger: Trigger<D::Event>) {
        match trigger.event {
            Events::IscRetireDOp(dop) => {
                self.dops.push(dop);
            }
            _ => {}
        }
    }
}
