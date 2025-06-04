use serde::{Serialize,Deserialize};

#[derive(Debug,Deserialize)]
pub struct NewTicket{
    pub title: String,
    pub description: String,
    pub assigned_to: Option<i64>
}

#[derive(Debug,Serialize,sqlx::FromRow)]
pub struct Ticket{
    pub id: i32,
    pub title:String,
    pub description: String,
    pub created_by:i64,
    pub assigned_to: Option<i64>,
    pub priority:String,
    pub status: String

}
#[derive(Debug, Deserialize)]
pub struct AssignTicket {
    pub ticket_id: i32,
    pub developer_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatus {
    pub ticket_id: i32,
    pub status: String,
}