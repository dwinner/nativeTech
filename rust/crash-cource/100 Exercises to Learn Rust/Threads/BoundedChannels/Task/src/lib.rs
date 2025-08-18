// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: /* TODO */,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> /* TODO */, Overloaded/* TODO */.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result/* TODO */, Overloaded/* TODO */.recv().un/* TODO */ OverloadedError;

pub fn launch(capacity: usize) -> TicketS/* TODO */hannel(capacity);
    std::thread::spawn(move || server/* TODO */Client { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        r/* TODO */cSender<TicketId>,
    },
    Get {
        id: TicketId,
        r/* TODO */r<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
        /* TODO */end(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                l/* TODO */            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
