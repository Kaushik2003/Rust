use crate::database; 
    pub fn login(cred: models::Credentials) {
        //try to login the user
        database::get_user();
    }
    pub mod models;