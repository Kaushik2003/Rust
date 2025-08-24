// use auth_service::Credentials;
// use auth_service::authenticate;

use auth_service::authenticate;
use auth_service::auth_utils::models::Credentials;


fn main() {
    println!("Hello, world!");
    let cred=Credentials{
        username: String::from("kzark"),
        password: String::from("admin123"),
    };
    authenticate(cred);
}
