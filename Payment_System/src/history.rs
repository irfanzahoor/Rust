use crate::payment::Payment;

pub struct PaymentHistory {
    records: Vec<Payment>,
}

impl PaymentHistory {
    pub fn new() -> Self {
        PaymentHistory { records: Vec::new() }
    }

    pub fn add(&mut self, payment: Payment) {
        match payment.process() {
            Ok(success_msg) => {
                println!("✅ {}", success_msg);
                self.records.push(payment);
            }
            Err(e) => {
                println!("❌ Error: {:?}", e);
            }
        }
    }

    pub fn show(&self) {
        println!("\n=== Payment History ===");
        if self.records.is_empty() {
            println!("(No transactions found)");
        } else {
            for (i, record) in self.records.iter().enumerate() {
                println!("{}. {:?} - ${}", i + 1, record.method, record.amount);
            }
        }
    }
}
