use crate::domain::notification::ports::NotificationPort;

pub struct WebSocketNotificationAdapter;
impl NotificationPort for WebSocketNotificationAdapter {}
