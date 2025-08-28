use crate::errors::PaymentError;

#[derive(Debug, Clone)]
pub enum PaymentMethod {
    Cash,
    CreditCard(String),
    Paypal(String),
}

#[derive(Debug, Clone)]
pub struct Payment {
    pub amount: f64,
    pub method: PaymentMethod,
}

impl Payment {
    pub fn process(&self) -> Result<String, PaymentError> {
        if self.amount <= 0.0 {
            return Err(PaymentError::InvalidAmount);
        }

        match &self.method {
            PaymentMethod::Cash => Ok(format!("💵 Paid ${} in Cash", self.amount)),
            PaymentMethod::CreditCard(card) => {
                if card.is_empty() {
                    Err(PaymentError::InvalidDetails)
                } else {
                    Ok(format!("💳 Paid ${} via Credit Card ({})", self.amount, card))
                }
            }
            PaymentMethod::Paypal(email) => {
                if email.is_empty() {
                    Err(PaymentError::InvalidDetails)
                } else {
                    Ok(format!("🌐 Paid ${} via PayPal ({})", self.amount, email))
                }
            }
        }
    }
}
