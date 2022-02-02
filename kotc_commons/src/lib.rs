use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TestEvent {
    pub user_id: i32,
    pub msg: String,
}
