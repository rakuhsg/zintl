use ztloop::{EventPump, EventPumpImpl};

fn main() {
    let mut e = EventPumpImpl::new();
    e.run();
}
