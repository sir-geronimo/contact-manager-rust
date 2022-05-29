use crate::contact_book::ContactBook;

pub fn execute(contact_book: &mut ContactBook) {
    let contacts = contact_book.get_contacts(true);

    contacts
    .into_iter()
    .for_each(|contact| {
      println!(
          "Active user: {0} {1}",
          contact.first_name, contact.last_name
      );

      contact.deactivate()
    })
}