#[derive(Clone)]
pub struct Ticket {
    pub id: u32,
}

impl Drop for Ticket {
    fn drop(&mut self) {}
}

pub fn duplicate(ticket: Ticket) -> (Ticket, Ticket) {
    (ticket.clone(), ticket)
}
