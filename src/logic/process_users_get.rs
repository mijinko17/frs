use crate::db::UsersDb;

pub fn process_users_get(name: &str) -> Option<String> {
    let db = UsersDb::default();
    db.find_user(name).map(|user| user.emotion)
}
