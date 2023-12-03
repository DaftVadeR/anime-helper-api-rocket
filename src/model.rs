//! Simplistic Model Layer
//! (with mock-store layer)

use crate::ctx::Ctx;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// region:    --- Ticket Types
#[derive(Clone, Debug, Serialize)]
pub struct Release {
    pub id: u64,
    pub cid: u64, // creator user_id
    pub title: String,
}

#[derive(Deserialize)]
pub struct ReleaseForCreate {
    pub title: String,
}
// endregion: --- Ticket Types

// endregion: --- Model Controller
