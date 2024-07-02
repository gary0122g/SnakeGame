use uuid::Uuid;

#[derive(Clone)]
pub struct User {
    pub user_id: Uuid,
}

impl User {
    pub fn new() -> Self {
        User {
            user_id: Uuid::new_v4(),
        }
    }
}
