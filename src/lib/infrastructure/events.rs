use crate::domain::events::events::DomainEvent;
use tokio::sync::mpsc;

pub struct EventBus {
    tx: mpsc::Sender<DomainEvent>,
}
