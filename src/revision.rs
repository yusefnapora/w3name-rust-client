use crate::name::Name;

pub struct Revision {
  name: Name,
  value: String,
  seq_no: u128,
  validity: String,
}

impl Revision {
  pub fn v0(name: Name, value: String, validity: String) -> Revision {
    Revision { name, value, validity, seq_no: 0 }
  }

  pub fn increment(&self, value: String) -> Revision {
    let seq_no = self.seq_no + 1;
    Revision { name: self.name, value, seq_no: (), validity: self.validity }
  }
}