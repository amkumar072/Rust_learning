// struct

fn main() {
    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 100.0,
    };

    account.check_balance();

    account.withdraw(50.05);

    account.check_balance();

    struct_new();
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

fn struct_new() {
    struct Author {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User {
        name: String,
        email: String,
        active: bool,
        sign_in_count: u64,
    }

    let mut user1 = User {
        name: String::from("Alice"),
        email: String::from("test@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("alice@gmail.com");
    println!("User1 name: {}", user1.name);

    // tuple struct

    struct Color(u8, u8, u8);
    let black = Color(0, 0, 0);

    // unit-like struct
    struct UnitLike;
    let unit = UnitLike;
}
