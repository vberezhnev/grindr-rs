pub mod chat {
    pub mod redis;
}
pub mod grpc {
    pub mod server;
    pub mod handlers {
        pub mod chat;
        pub mod matching;
        pub mod notification;
        pub mod payment;
        pub mod profile;
        pub mod responses;
        pub mod user;
    }
}
pub mod matching {
    pub mod distance;
    pub mod interest;
}
pub mod notification {
    pub mod websocket;
}
pub mod payment {
    pub mod mock;
}
pub mod profile {
    pub mod postgres;
}
pub mod user {
    pub mod postgres;
    pub mod redis;
}
