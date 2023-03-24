pub mod user;
pub mod sign_in;

pub use user::*;
pub use sign_in::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmptyDTO {}

/// IdDTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IdDTO {
    pub id: Option<String>,
}
