use crate::name::Name;

pub struct Revision {
  name: Name,
  value: String,
  sequence: u64,
  validity: String,
}

impl Revision {
  pub fn v0(name: Name, value: String, validity: String) -> Revision {
    Revision { name, value, validity, sequence: 0 }
  }

  pub fn increment(&self, value: String) -> Revision {
    let sequence = self.sequence + 1;
    Revision { name: self.name.clone(), value, sequence, validity: self.validity.clone() }
  }

  pub fn name(&self) -> &Name {
    &self.name
  }

  pub fn value(&self) -> &str {
    &self.value
  }

  pub fn sequence(&self) -> u64 {
    self.sequence
  }

  pub fn validity(&self) -> &str {
    &self.validity
  }


}