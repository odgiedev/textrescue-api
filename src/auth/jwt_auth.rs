use bcrypt::{DEFAULT_COST, hash, verify};

pub fn create_token(user_id: &str) -> String {
    let token = hash(user_id, DEFAULT_COST).unwrap();
    token
}

pub fn verify_token(user_id: &String, token: &String) -> Result<bool, ()> {
    let valid = verify(user_id, &token);
    match valid {
        Ok(v) => Ok(v),
        _ => Err(())
    }
}
