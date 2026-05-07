use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    WaitingPaymentVerification,
    Confirmed,
    Rejected,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GlobalOrderStatus {
    Open,
    Confirmed,
    InProgress,
    PartiallyShipped,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FulfillmentStatus {
    NotReady,
    ReadyToProcess,
    Processing,
    Shipped,
    Completed,
    Cancelled,
}

pub fn status_after_proof_upload(current: PaymentStatus) -> Result<PaymentStatus, OrderError> {
    match current {
        PaymentStatus::Pending | PaymentStatus::Rejected => {
            Ok(PaymentStatus::WaitingPaymentVerification)
        }
        PaymentStatus::Confirmed
        | PaymentStatus::Cancelled
        | PaymentStatus::WaitingPaymentVerification => Err(OrderError::InvalidPaymentTransition),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OrderError {
    #[error("invalid payment status transition")]
    InvalidPaymentTransition,
}
