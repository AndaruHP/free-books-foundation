#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, String, Vec};

#[derive(Clone)]
#[contracttype]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub is_available: bool,
    pub current_borrower: Option<Address>
}

#[derive(Clone)]
#[contracttype]
pub struct LendingRecord {
    pub book_id: u32,
    pub borrower: Address,
    pub lend_timestamp: u64,
    pub return_timestamp: Option<u64>
}

#[derive(Clone)]
#[contracttype]
pub struct Donor {
    pub address: Address,
    pub total_donated: i128
}

#[contracttype]
pub enum DataKey {
    Books,
    LendingHistory,
    Donors,
    Foundation,
    TokenAddress
}

#[contract]
pub struct FreeBooks;

#[contractimpl]
impl FreeBooks {
    pub fn initialize(
        env: Env,
        foundation: Address,
        token_address: Address

    ) {
        if env.storage().instance().has(&DataKey::Foundation) {
            panic!("Already initialized")
        }

        env.storage().instance().set(&DataKey::Foundation, &foundation);
        env.storage().instance().set(&DataKey::TokenAddress, &token_address);
        env.storage().instance().set(&DataKey::LendingHistory, &Vec::<LendingRecord>::new(&env));
        env.storage().instance().set(&DataKey::Donors, &Vec::<Donor>::new(&env));

        let mut books = Vec::<Book>::new(&env);

        books.push_back(Book {
            id: 1,
            title: String::from_str(&env, "The Great Gatsby"),
            author: String::from_str(&env, "F. Scott Fitzgerald"),
            is_available: true,
            current_borrower: None,
        });

        books.push_back(Book {
            id: 2,
            title: String::from_str(&env, "To Kill a Mockingbird"),
            author: String::from_str(&env, "Harper Lee"),
            is_available: true,
            current_borrower: None,
        });

        books.push_back(Book {
            id: 3,
            title: String::from_str(&env, "1984"),
            author: String::from_str(&env, "George Orwell"),
            is_available: true,
            current_borrower: None,
        });

        books.push_back(Book {
            id: 4,
            title: String::from_str(&env, "Pride and Prejudice"),
            author: String::from_str(&env, "Jane Austen"),
            is_available: true,
            current_borrower: None,
        });

        books.push_back(Book {
            id: 5,
            title: String::from_str(&env, "The Catcher in the Rye"),
            author: String::from_str(&env, "J.D. Salinger"),
            is_available: true,
            current_borrower: None,
        });

        env.storage().instance().set(&DataKey::Books, &books);
    }

    pub fn lend_book(
        env: Env,
        book_id: u32,
        borrower: Address
    ) {
        borrower.require_auth();

        let mut books: Vec<Book> = env.storage().instance().get(&DataKey::Books).unwrap();
        let mut book_found = false;

        for i in 0..books.len() {
            let mut book = books.get(i).unwrap();
            if book.id == book_id {
                if !book.is_available {
                    panic!("Book is currently borrowed");
                }

                book.is_available = false;
                book.current_borrower = Some(borrower.clone());
                books.set(i, book);
                book_found = true;
                break;
            }
        }

        if !book_found {
            panic!("Book not found");
        }

        env.storage().instance().set(&DataKey::Books, &books);

        let mut history: Vec<LendingRecord> = env.storage().instance()
            .get(&DataKey::LendingHistory)
            .unwrap_or(Vec::new(&env));

        let record = LendingRecord {
            book_id,
            borrower: borrower.clone(),
            lend_timestamp: env.ledger().timestamp(),
            return_timestamp: None
        };

        history.push_back(record);
        env.storage().instance().set(&DataKey::LendingHistory, &history);
    }

    pub fn return_book(
        env: Env,
        book_id: u32,
        borrower: Address
    ) {
        borrower.require_auth();

        let mut books: Vec<Book> = env.storage().instance().get(&DataKey::Books).unwrap();
        let mut book_found = false;

        for i in 0..books.len() {
            let mut book = books.get(i).unwrap();
            if book.id == book_id {
                if book.is_available {
                    panic!("Book is not currently borrowed");
                }

                if book.current_borrower != Some(borrower.clone()) {
                    panic!("You are not the borrower of this book");
                }

                book.is_available = true;
                book.current_borrower = None;
                books.set(i, book);
                book_found = true;
                break;
            }
        }

        if !book_found {
            panic!("Book not found");
        }

        env.storage().instance().set(&DataKey::Books, &books);

        let mut history: Vec<LendingRecord> = env.storage().instance()
            .get(&DataKey::LendingHistory).unwrap();

        for i in (0..history.len()).rev() {
            let mut record = history.get(i).unwrap();
            if record.book_id == book_id && record.borrower == borrower && record.return_timestamp.is_none() {
                record.return_timestamp = Some(env.ledger().timestamp());
                history.set(i, record);
                break;
            }
        }

        env.storage().instance().set(&DataKey::LendingHistory, &history);
    }

    pub fn donate(
        env: Env,
        donor: Address,
        amount: i128
    ) {
        donor.require_auth();

        if amount <= 0 {
            panic!("Donation amount must be positive");
        }

        let foundation: Address = env.storage().instance().get(&DataKey::Foundation).unwrap();
        let token_address: Address = env.storage().instance().get(&DataKey::TokenAddress).unwrap();

        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(&donor, &foundation, &amount);

        let mut donors: Vec<Donor> = env.storage().instance()
            .get(&DataKey::Donors)
            .unwrap_or(Vec::new(&env));

        let mut donor_found = false;
        for i in 0..donors.len() {
            let mut donor_entry = donors.get(i).unwrap();
            if donor_entry.address == donor {
                donor_entry.total_donated += amount;
                donors.set(i, donor_entry);
                donor_found = true;
                break;
            }
        }

        if !donor_found {
            donors.push_back(Donor {
                address: donor,
                total_donated: amount,
            });
        }

        env.storage().instance().set(&DataKey::Donors, &donors);
    }

    pub fn get_books(env: Env) -> Vec<Book> {
        env.storage().instance().get(&DataKey::Books).unwrap_or(Vec::new(&env))
    }

    pub fn get_lending_history(env: Env) -> Vec<LendingRecord> {
        env.storage().instance().get(&DataKey::LendingHistory).unwrap_or(Vec::new(&env))
    }

    pub fn get_donor_leaderboard(env: Env) -> Vec<Donor> {
        let mut donors: Vec<Donor> = env.storage().instance()
            .get(&DataKey::Donors)
            .unwrap_or(Vec::new(&env));

        let len = donors.len();
        for i in 0..len {
            for j in 0..len - i - 1 {
                let donor1 = donors.get(j).unwrap();
                let donor2 = donors.get(j + 1).unwrap();

                if donor1.total_donated < donor2.total_donated {
                    donors.set(j, donor2.clone());
                    donors.set(j + 1, donor1);
                }
            }
        }

        donors
    }

    pub fn get_book(env: Env, book_id: u32) -> Option<Book> {
        let books: Vec<Book> = env.storage().instance().get(&DataKey::Books).unwrap();

        for i in 0..books.len() {
            let book = books.get(i).unwrap();
            if book.id == book_id {
                return Some(book);
            }
        }

        None
    }
}

mod test;
