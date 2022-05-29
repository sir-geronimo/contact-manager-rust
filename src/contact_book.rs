use crate::contact::Contact;
use crate::ContactState;

pub struct ContactBook {
    contacts: Vec<Contact>,
}

impl ContactBook {
    pub fn new() -> Self {
        Self {
            contacts: Vec::new(),
        }
    }

    pub fn add_contact(&mut self, contact: Contact) -> Contact {
        self.contacts.push(contact.clone());

        contact
    }

    pub fn search_contact(&self, name: String) -> Option<&Contact> {
        let name_to_find = name.to_lowercase();

        self.contacts.iter().find(|&contact| {
            contact.first_name.to_lowercase().contains(&name_to_find.clone())
            || contact.last_name.to_lowercase().contains(&name_to_find.clone())
        })
    }

    pub fn get_contacts(&mut self, active_only: bool) -> Vec<&mut Contact> {
        if active_only {
            return self.contacts
                .iter_mut()
                .filter(|contact| contact.is_active == ContactState::Active)
                .collect()
        }

        self.contacts.iter_mut().collect()
    }

    pub fn remove_contact(&mut self, contact_to_remove: &Contact) {
        println!(
            "{0} {1}, was removed from the contact book.", 
            contact_to_remove.first_name, contact_to_remove.last_name
        );

        self.contacts
            .retain(|contact| {
                contact.first_name != contact_to_remove.first_name
                && contact.last_name != contact_to_remove.last_name
            });
    }
}