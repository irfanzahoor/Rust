mod errors;
mod history;
mod payment;

use history::PaymentHistory;
use payment::{Payment, PaymentMethod};

fn main() {
    println!("=== Multi-file Payment System with History ===");

    let mut history = PaymentHistory::new();

    // Example 1: Cash
    let cash_payment = Payment {
        amount: 100.0,
        method: PaymentMethod::Cash,
    };
    history.add(cash_payment);

    // Example 2: Credit Card
    let card_payment = Payment {
        amount: 250.0,
        method: PaymentMethod::CreditCard("1234-5678-9012".to_string()),
    };
    history.add(card_payment);

    // Example 3: PayPal
    let paypal_payment = Payment {
        amount: 500.0,
        method: PaymentMethod::Paypal("boss@example.com".to_string()),
    };
    history.add(paypal_payment);

    // Show full history
    history.show();
}
