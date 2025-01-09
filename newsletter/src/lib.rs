use axum::{
    routing::{get, post},
    Router,
};


async fn newsletter_home() -> &'static str {
    "Welcome to the newsletter!"
}

async fn subscribe_handler() -> &'static str {
    "You have subscribed!"
}


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
