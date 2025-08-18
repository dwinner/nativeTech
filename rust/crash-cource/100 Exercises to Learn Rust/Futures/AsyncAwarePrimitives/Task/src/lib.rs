/// TODO: the code below will deadlock because it's using std's channels,
///  which are not async-aware.
///  Rewrite it to use `tokio`'s channels primitive (you'll have to touch
///  the testing code too, yes).
///
/// Can you understand the sequence of events that can lead to a deadlock?
use std::sync::mpsc; // TODO

pub struct Message {
    payload: String,
    response_channel: mpsc::Sender<Message>,
}

impl Message {
    pub fn new(payload: String, response_channel: mpsc::Sender<Message>) -> Self {
        Message {
            payload: payload.into(),
            response_channel,
        }
    }

    pub fn payload(self) -> String {
        self.payload
    }
}

/// Replies with `pong` to any message it receives, setting up a new
/// channel to continue communicating with the caller.
pub async fn pong(mut receiver: mpsc::Receiver<Message>) {
    loop {
        if let Ok(msg) = receiver.recv() { // TODO
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = mpsc::channel(/* TODO */);
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                /* TODO */
                .unwrap();
            receiver = new_receiver;
        }
    }
}
