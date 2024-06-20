use crate::attack_types::AttackType;

pub struct Payload {
    data: String,
}

impl Payload {
    pub fn new(attack_type: AttackType) -> Self {
        Payload {
            data: attack_type.generate(),
        }
    }

    pub fn to_base64(&self) -> String {
        base64::encode(&self.data)
    }

    pub fn to_hex(&self) -> String {
        hex::encode(&self.data)
    }

    pub fn raw(&self) -> &str {
        &self.data
    }
}
