#[derive(Clone, Debug)]
pub struct Contact {
  pub first_name: String,
  pub last_name: String,
  pub phone_number: String,
  pub is_active: ContactState,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ContactState {
  Active,
  Inactive,
}

impl Contact {
  pub fn new(
    first_name: String,
    last_name: String,
    phone_number: String,
    is_active: ContactState,
  ) -> Self {
    Self {
      first_name,
      last_name,
      phone_number,
      is_active,
    }
  }

  pub fn activate(&mut self) {
    self.is_active = ContactState::Active
  }

  pub fn deactivate(&mut self) {
    self.is_active = ContactState::Inactive
  }
}