pub struct Bcrypt {}

impl Bcrypt {
    pub fn hash(password: &String) -> String {
        bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
    }

    pub fn verify(password: &String, hash: &String) -> bool {
        bcrypt::verify(password, hash).unwrap()
    }
}
