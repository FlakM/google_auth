use std::env;

use google_authenticator::GoogleAuthenticator;

fn main() {
    let secret = env::var("GOOGLE_SECRET").expect("Env variable GOOGLE_SECRET not set");
    let auth = GoogleAuthenticator::new();

    let result = auth.get_code(&secret, 0).unwrap();

    println!("{}", result);
}
