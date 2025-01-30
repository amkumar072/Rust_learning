// struct

fn main() {
    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 100.0,
    };

    account.check_balance();

    account.withdraw(50.05);

    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // muttable reference
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing ${} from account of {}", amount, self.owner);
        self.balance -= amount;
    }
    // immutable reference
    fn check_balance(&self) {
        println!(
            "The balance of account of {} is ${}",
            self.owner, self.balance
        );
    }
}
