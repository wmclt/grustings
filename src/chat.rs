use rocket::form::Form;
use rocket::http::CookieJar;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{error::RecvError, Sender};
use rocket::{Shutdown, State};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct Message {
    // #[field(validate = len(..30))]
    pub room: String,
    // #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.
#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

#[derive(Debug, Clone, FromForm)]
pub struct CreateMessage {
    pub room: String,
    pub message: String,
}

/// Receive a message from a form submission and broadcast it to any receivers.
#[post("/message", data = "<form>")]
fn post(jar: &CookieJar<'_>, form: Form<CreateMessage>, queue: &State<Sender<Message>>) {
    // A send 'fails' if there are no active subscribers. That's okay.
    let username = jar
        .get_private("username")
        .and_then(|cookie| cookie.value().parse().ok())
        .unwrap();
    let form = form.into_inner();
    let _res = queue.send(Message {
        message: form.message,
        username,
        room: form.room,
    });
}

pub fn routes() -> Vec<rocket::Route> {
    routes![events, post]
}
