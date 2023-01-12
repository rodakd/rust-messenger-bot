pub struct Env {
    pub email: String,
    pub password: String,
}

pub fn get_env() -> Env {
    let email = std::env::var("EMAIL").expect("No email provided");
    let password = std::env::var("PASSWORD").expect("No password provided");

    return Env { email, password };
}
