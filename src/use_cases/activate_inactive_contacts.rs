use crate::{contact::ContactState, ContactBook};

pub fn execute(contact_book: &mut ContactBook) -> () {
    let contacts = contact_book.get_contacts(false);

    contacts
        .into_iter()
        .filter(|contact| contact.is_active == ContactState::Inactive)
        .for_each(|contact| {
            println!(
                "Inactive user: {0} {1}",
                contact.first_name, contact.last_name
            );

            contact.activate()
        })
}
