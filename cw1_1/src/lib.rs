/// This is the CIS1112 Coursework 1.1 assignment answer file.
///
///
/// File name: lib.rs
/// Package: default
/// Created: Febuary 2nd, 2022
///

pub struct BankAccount {
    // Rust defaults to private
    account_number: u32,
    balance: f32,
    sortcode: String,
}

impl Default for BankAccount {
    fn default() -> BankAccount {
        BankAccount {
            account_number: 0,
            balance: 0f32,
            sortcode: "".to_string(),
        }
    }
}

impl BankAccount {
    pub fn deposit(&mut self, f: f32) {
        // Adds amount to the class variable balance.
        self.balance += f;
    }
    pub fn withdraw(&mut self, f: f32) {
        // Subtracts amount from the class variable balance.
        self.balance -= f;
    }
    pub fn summarise(&mut self) -> String {
        /*
         * Returns sortcode variable as a string.
         * {2:.2} represents the balance variable to two decial places.
         */

        format!(
            "Acc. Num: {0} Sort:{1} Balance: ${2:.2}",
            self.get_account_number(),
            self.get_sortcode(),
            self.get_balance(),
        )
    }
    pub fn set_balance(&mut self, f: f32) {
        // Sets the value of the class variable balance to b <float>.
        self.balance = f;
    }
    pub fn set_account_number(&mut self, n: u32) {
        // Sets the value of the class variable accountNumber to n <int>.
        self.account_number = n;
    }
    pub fn set_sortcode(&mut self, s: String) {
        // Sets the value of the class variable sortcode to s <string>.
        self.sortcode = s;
    }
    pub fn get_balance(&mut self) -> f32 {
        // Returns current account balance as an string to two decimal places
        // using class variable balance.
        self.balance
    }
    pub fn get_account_number(&mut self) -> u32 {
        // Returns the account number as a float using the class variable accountNumber.
        self.account_number
    }
    pub fn get_sortcode(&mut self) -> String {
        // Returns the sort code as a sting using the class variable sortcode.
        self.sortcode.to_owned()
    }
}
