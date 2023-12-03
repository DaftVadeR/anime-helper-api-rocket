use crate::ctx::Ctx;
use crate::model::Release;
use crate::utils::RequestGetter;

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone)]
pub struct ReleasesController {
    // tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ReleasesController {
    pub async fn new() -> Result<Self> {
        // Ok(Self {
        // tickets_store: Arc::default(),
        // })

        Ok(Self {})
    }
}

impl ReleasesController {
    // pub async fn create_ticket(&self, ctx: Ctx, ticket_fc: TicketForCreate) -> Result<Ticket> {
    //     let mut store = self.tickets_store.lock().unwrap();

    //     let id = store.len() as u64;
    //     let ticket = Ticket {
    //         id,
    //         cid: ctx.user_id(),
    //         title: ticket_fc.title,
    //     };
    //     store.push(Some(ticket.clone()));

    //     Ok(ticket)
    // }

    pub async fn list_releases_today(&self) -> Result<Vec<Release>> {
        // let store = self.tickets_store.lock().unwrap();

        // let releases = store.iter().filter_map(|t| t.clone()).collect();

        Ok(releases)
    }

    // pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {
    //     let mut store = self.tickets_store.lock().unwrap();

    //     let ticket = store.get_mut(id as usize).and_then(|t| t.take());

    //     ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    // }
}
