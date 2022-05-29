use contact::Contact;
use contact::ContactState;
use contact_book::ContactBook;
use use_cases::{
    activate_inactive_contacts,
    deactivate_active_contacts,
};

mod contact;
mod contact_book;
mod use_cases;

fn main() {
    let mut contact_book = ContactBook::new();

    let enger = contact_book.add_contact(
        Contact::new(
            "Enger".to_owned(),
            "Jiménez".to_owned(),
            "809-555-5555".to_owned(),
            ContactState::Active,
        )
    );
    contact_book.add_contact(
        Contact::new(
            "Pepe".to_owned(),
            "Agallas".to_owned(),
            "809-555-6666".to_owned(),
            ContactState::Active,
        )
    );
    contact_book.add_contact(
        Contact::new(
            "Miguel".to_owned(),
            "Orcado".to_owned(),
            "809-555-7777".to_owned(),
            ContactState::Inactive,
        )
    );

    if let Some(contact) = contact_book.search_contact("Eng".to_owned()) {
        println!("{0} {1}", contact.first_name, contact.last_name)
    } else {
        println!("Sorry no contact found")
    }

    deactivate_active_contacts::execute(&mut contact_book);
    contact_book.remove_contact(&enger);
    activate_inactive_contacts::execute(&mut contact_book);
}
