#[derive(Debug, Clone)]
pub enum PaymentError {
    InvalidAmount,
    InvalidDetails,
}
