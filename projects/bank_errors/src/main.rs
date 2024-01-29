use chrono::Datelike;
use thiserror::Error;

// DSM Banks errors system :)

struct Card {
    balance: f64,
    pin: i32,
    expiration_date: (i32, i32)
}

#[derive(Debug, Error)]
enum CardError {
    #[error("You need more money to pay for it: current balance ${0}, balance must be ${1}")]
    InvalidBalance(f64, f64),
    #[error("Invalid card PIN. Contact your bank branch to restore it if you have forgotten it")]
    InvalidPIN,
    #[error("Card expiration date: {0}")]
    CardExpired(String)
}

impl Card {
    fn purchase(&mut self, cost: f64, pin: i32) -> Result<(), CardError> {
        if self.is_card_expired() {
            return Err(CardError::CardExpired(format!("{}/{}", self.expiration_date.0, self.expiration_date.1)));
        }
        else if self.pin != pin {
            return Err(CardError::InvalidPIN)
        }
        else if self.is_invalid_balance(cost) {
            return Err(CardError::InvalidBalance(self.balance, cost));
        }

        self.balance -= cost; // Для этого и нужен мутабельная ссылка &mut

        Ok(())
    }

    fn is_card_expired(&self) -> bool {
        let current_date = chrono::Utc::now();
        let expire = self.expiration_date;

        expire.1 < current_date.year() || (expire.0 < current_date.month() as i32 && expire.1 == current_date.year())
    }

    fn is_invalid_balance(&self, cost: f64) -> bool {
        self.balance - cost < (0f64)
    }
}

fn main() {
    // EXPIRATION DATE
    // let mut card = Card {
    //     balance: 1000.0,
    //     pin: 7534,
    //     expiration_date: (9, 2022) // Текущий реальный год
    // };

    // PIN
    // let mut card = Card {
    //     balance: 1000.0,
    //     pin: 4829,
    //     expiration_date: (9, 2024) // Текущий реальный год
    // };

    // BALANCE
    let mut card = Card {
        balance: 1300.0,
        pin: 7534,
        expiration_date: (9, 2024) // Текущий реальный год
    };

    // match card.purchase(1301.0, 7534) {
    //     Ok(()) => println!("Purchase successful!"),
    //     Err(err) => println!("Transaction failed: {}", err)
    // }

    // Тройная покупка
    for _ in 0..3 {
        match card.purchase(500.0, 7534) {
            Ok(()) => println!("Purchase successful!"),
            Err(err) => println!("Transaction failed: {}", err)
        }
    }

}
