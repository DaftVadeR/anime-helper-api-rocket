use crate::ctx::Ctx;
use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;
use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

pub fn routes(mc: ReleasesController) -> Router {
    Router::new()
        .route("/releases", get(list))
        // .route("/releases", post(create_ticket).get(list))
        // .route("/releases/:id", delete(delete_ticket))
        .with_state(mc)
}

// // region:    --- REST Handlers
// async fn create_ticket(
//     State(mc): State<ModelController>,
//     ctx: Ctx,
//     Json(ticket_fc): Json<TicketForCreate>,
// ) -> Result<Json<Ticket>> {
//     println!("->> {:<12} - create_ticket", "HANDLER");

//     let ticket = mc.create_ticket(ctx, ticket_fc).await?;

//     Ok(Json(ticket))
// }

async fn list(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");

    let tickets = mc.list_tickets(ctx).await?;

    Ok(Json(tickets))
}

// async fn delete_ticket(
//     State(mc): State<ModelController>,
//     ctx: Ctx,
//     Path(id): Path<u64>,
// ) -> Result<Json<Ticket>> {
//     println!(">>> {:<12} - delete_ticket", "HANDLER");

//     let ticket = mc.delete_ticket(ctx, id).await?;

//     Ok(Json(ticket))
// }
// // endregion: --- REST Handlers
