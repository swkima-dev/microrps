use rand;

pub struct Linux {}

pub fn random16() -> u16 {
    rand::random()
}
