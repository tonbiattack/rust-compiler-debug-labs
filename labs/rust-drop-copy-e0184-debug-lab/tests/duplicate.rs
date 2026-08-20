use rust_drop_copy_e0184_debug_lab::{duplicate, Ticket};
#[test] fn duplicates_a_ticket_for_two_consumers() { let (first, second) = duplicate(Ticket { id: 7 }); assert_eq!(first.id, 7); assert_eq!(second.id, 7); }
