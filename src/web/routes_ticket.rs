// src/web/routes_ticket.rs

use crate::{
    error::{Result, Error},
    model::{Ticket, TicketForCreate, ModelController},
};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::{get, post};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/ticket", post(create_ticket).delete(delete_ticket))
        .route("/tickets", get(list_all_tickets))
        .with_state(mc)
}

pub async fn create_ticket(State(mc): State<ModelController>, Json(ticket_fc): Json<TicketForCreate>) -> Result<Json<Ticket>> {
    println!("->> {:<12} — create_ticket", "HANDLER");
    let ticket = mc.create_ticket(ticket_fc).await?;
    Ok(Json(ticket))
}

pub async fn list_all_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} — list_all_tickets", "HANDLER");
    let tickets = mc.list_tickets().await?;
    Ok(Json(tickets))
}

pub async fn delete_ticket(State(mc): State<ModelController>, Path(id): Path<u32>) -> Result<Json<Ticket>> {
    println!("->> {:<12} — delete_ticket", "HANDLER");
    let ticket = mc.delete_ticket(id).await?;
    Ok(Json(ticket))
}