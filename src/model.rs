// src/model.rs
use crate::{Result, Error};

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use axum::extract::Path;

#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
    id: u32,
    title: String,
} impl Ticket { pub fn new(id: u32, title: String) -> Self { Self { id, title }} }

#[derive(Debug, Clone, Deserialize)]
pub struct TicketForCreate {
    title: String,
} impl TicketForCreate {
    pub fn title(&self) -> &str { &self.title }
    pub fn title_to_owned(&self) -> String { self.title.clone() }
}

#[derive(Debug, Clone)]
pub struct ModelController {
    ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub fn new() -> Result<Self> {
        Ok(Self {
            ticket_store: Arc::default(),
        })
    }
    /// Create a Ticket
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();
        let id = store.len() as u32;
        let ticket = Ticket::new(id, ticket_fc.title);
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }
    /// List all Tickets
    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.ticket_store.lock().unwrap();
        let tickets = store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }
    /// Delete a Ticket
    pub async fn delete_ticket(&self, id: u32) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or(Error::LoginFail)
    }
}