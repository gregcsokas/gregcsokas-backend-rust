mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use crate::handlers::{newsletter_home, subscribe_handler};

pub fn newsletter_routes() -> Router {
    Router::new()
        .route("/", get(newsletter_home))
        .route("/subscribe", post(subscribe_handler))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        todo!()
    }
}
