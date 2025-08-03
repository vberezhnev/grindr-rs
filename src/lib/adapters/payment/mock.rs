use crate::domain::payment::ports::PaymentRepositoryPort;

pub struct MockPaymentRepository;
impl PaymentRepositoryPort for MockPaymentRepository {}
