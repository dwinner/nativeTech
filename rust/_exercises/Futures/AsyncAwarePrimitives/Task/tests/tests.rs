#[cfg(test)]
mod tests {
    use task_async_aware_primitives::{pong, Message};
    use std::sync::mpsc; // TODO

    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = mpsc::channel(); // TODO
        let (response_sender, mut response_receiver) = mpsc::channel(); // TODO

        sender
            .send(Message::new("pong".into(), response_sender))
            /* TODO */
            .unwrap();

        tokio::spawn(pong(receiver));

        let answer = response_receiver.recv()./* TODO */unwrap().payload();
        assert_eq!(answer, "pong");
    }
}
