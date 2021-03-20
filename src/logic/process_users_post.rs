use crate::{
    db::{User, UsersDb},
    routing::users_post::UserPostBody,
};

pub fn process_users_post(body: UserPostBody) {
    let db = UsersDb::default();
    let user = User {
        name: body.name,
        emotion: body.emotion,
    };
    db.find_user(&user.name)
        .map(|_| db.update_user(&user))
        .unwrap_or_else(|| db.register_user(&user))
}
