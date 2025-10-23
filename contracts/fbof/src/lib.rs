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

#[derive(Clone)]
#[contracttype]
pub struct WishlistItem {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub requested_by: Address,
    pub request_timestamp: u64,
    pub vote_count: u32
}

#[contracttype]
pub enum DataKey {
    Books,
    LendingHistory,
    Donors,
    Foundation,
    TokenAddress,
    Wishlist,
    WishlistCount,
    ReservationQueue(u32),
    WishlistVotes(u32)
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
        env.storage().instance().set(&DataKey::Wishlist, &Vec::<WishlistItem>::new(&env));
        env.storage().instance().set(&DataKey::WishlistCount, &0u32);

        let mut books = Vec::<Book>::new(&env);

        books.push_back(Book {
            id: 1,
            title: String::from_str(&env, "Laskar Pelangi"),
            author: String::from_str(&env, "Andrea Hirata"),
            is_available: true,
            current_borrower: None
        });

        books.push_back(Book {
            id: 2,
            title: String::from_str(&env, "Bumi Manusia"),
            author: String::from_str(&env, "Pramoedya Ananta Toer"),
            is_available: true,
            current_borrower: None
        });

        books.push_back(Book {
            id: 3,
            title: String::from_str(&env, "Negeri 5 Menara"),
            author: String::from_str(&env, "Ahmad Fuadi"),
            is_available: true,
            current_borrower: None
        });

        books.push_back(Book {
            id: 4,
            title: String::from_str(&env, "Saman"),
            author: String::from_str(&env, "Ayu Utami"),
            is_available: true,
            current_borrower: None
        });

        books.push_back(Book {
            id: 5,
            title: String::from_str(&env, "Ronggeng Dukuh Paruk"),
            author: String::from_str(&env, "Ahmad Tohari"),
            is_available: true,
            current_borrower: None
        });

        books.push_back(Book {
            id: 6,
            title: String::from_str(&env, "Orang-Orang Biasa"),
            author: String::from_str(&env, "Andrea Hirata"),
            is_available: true,
            current_borrower: None
        });

        books.push_back(Book {
            id: 7,
            title: String::from_str(&env, "Cantik Itu Luka"),
            author: String::from_str(&env, "Eka Kurniawan"),
            is_available: true,
            current_borrower: None
        });

        books.push_back(Book {
            id: 8,
            title: String::from_str(&env, "Aroma Karsa"),
            author: String::from_str(&env, "Dee Lestari"),
            is_available: true,
            current_borrower: None
        });

        books.push_back(Book {
            id: 9,
            title: String::from_str(&env, "Di Tanah Lada"),
            author: String::from_str(&env, "Ziggy Zezsyazeoviennazabrizkie"),
            is_available: true,
            current_borrower: None
        });

        books.push_back(Book {
            id: 10,
            title: String::from_str(&env, "Sepotong Senja untuk Pacarku"),
            author: String::from_str(&env, "Seno Gumira Ajidarma"),
            is_available: true,
            current_borrower: None
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

                let mut queue: Vec<Address> = env.storage().instance()
                    .get(&DataKey::ReservationQueue(book_id))
                    .unwrap_or(Vec::new(&env));

                if queue.len() > 0 {
                    let first_in_queue = queue.get(0).unwrap();
                    if first_in_queue != borrower {
                        panic!("Book is reserved by someone else. Join the reservation queue.");
                    }

                    queue.remove(0);
                    env.storage().instance().set(&DataKey::ReservationQueue(book_id), &queue);
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

        history.push_back(LendingRecord {
            book_id,
            borrower: borrower.clone(),
            lend_timestamp: env.ledger().timestamp(),
            return_timestamp: None
        });

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

    pub fn reserve_book(
        env: Env,
        book_id: u32,
        user: Address
    ) {
        user.require_auth();

        let mut books: Vec<Book> = env.storage().instance().get(&DataKey::Books).unwrap();
        let mut book_exists = false;
        let mut is_available = false;
        let mut current_borrower: Option<Address> = None;

        for i in 0..books.len() {
            let book = books.get(i).unwrap();
            if book.id == book_id {
                book_exists = true;
                is_available = book.is_available;
                current_borrower = book.current_borrower.clone();
                break;
            }
        }

        if !book_exists {
            panic!("Book not found");
        }

        if is_available {
            panic!("Book is available, borrow it directly");
        }

        if current_borrower == Some(user.clone()) {
            panic!("You are currently borrowing this book");
        }

        let mut queue: Vec<Address> = env.storage().instance()
            .get(&DataKey::ReservationQueue(book_id))
            .unwrap_or(Vec::new(&env));

        for i in 0..queue.len() {
            if queue.get(i).unwrap() == user {
                panic!("Already in reservation queue");
            }
        }

        queue.push_back(user);
        env.storage().instance().set(&DataKey::ReservationQueue(book_id), &queue);
    }

    pub fn cancel_reservation(
        env: Env,
        book_id: u32,
        user: Address
    ) {
        user.require_auth();

        let mut queue: Vec<Address> = env.storage().instance()
            .get(&DataKey::ReservationQueue(book_id))
            .unwrap_or(Vec::new(&env));

        let mut found = false;
        for i in 0..queue.len() {
            if queue.get(i).unwrap() == user {
                queue.remove(i);
                found = true;
                break;
            }
        }

        if !found {
            panic!("Not in reservation queue");
        }

        env.storage().instance().set(&DataKey::ReservationQueue(book_id), &queue);
    }

    pub fn get_reservation_queue(env: Env, book_id: u32) -> Vec<Address> {
        env.storage().instance()
            .get(&DataKey::ReservationQueue(book_id))
            .unwrap_or(Vec::new(&env))
    }

    pub fn add_to_wishlist(
        env: Env,
        title: String,
        author: String,
        requester: Address
    ) {
        requester.require_auth();

        let mut wishlist_count: u32 = env.storage().instance().get(&DataKey::WishlistCount).unwrap_or(0);
        wishlist_count += 1;

        let wishlist_item = WishlistItem {
            id: wishlist_count,
            title,
            author,
            requested_by: requester.clone(),
            request_timestamp: env.ledger().timestamp(),
            vote_count: 1,
        };

        let mut wishlist: Vec<WishlistItem> = env.storage().instance()
            .get(&DataKey::Wishlist)
            .unwrap_or(Vec::new(&env));

        wishlist.push_back(wishlist_item);

        let mut votes = Vec::new(&env);
        votes.push_back(requester);
        env.storage().instance().set(&DataKey::WishlistVotes(wishlist_count), &votes);

        env.storage().instance().set(&DataKey::Wishlist, &wishlist);
        env.storage().instance().set(&DataKey::WishlistCount, &wishlist_count);
    }

    pub fn vote_wishlist(env: Env, wishlist_id: u32, voter: Address) {
        voter.require_auth();

        let mut votes: Vec<Address> = env.storage().instance()
            .get(&DataKey::WishlistVotes(wishlist_id))
            .unwrap_or(Vec::new(&env));

        for i in 0..votes.len() {
            if votes.get(i).unwrap() == voter {
                panic!("Already voted");
            }
        }

        votes.push_back(voter);
        env.storage().instance().set(&DataKey::WishlistVotes(wishlist_id), &votes);

        // Update vote count
        let mut wishlist: Vec<WishlistItem> = env.storage().instance().get(&DataKey::Wishlist).unwrap();
        for i in 0..wishlist.len() {
            let mut item = wishlist.get(i).unwrap();
            if item.id == wishlist_id {
                item.vote_count += 1;
                wishlist.set(i, item);
                break;
            }
        }
        env.storage().instance().set(&DataKey::Wishlist, &wishlist);
    }

    pub fn unvote_wishlist(env: Env, wishlist_id: u32, voter: Address) {
        voter.require_auth();

        let mut votes: Vec<Address> = env.storage().instance()
            .get(&DataKey::WishlistVotes(wishlist_id))
            .unwrap_or(Vec::new(&env));

        let mut found = false;
        for i in 0..votes.len() {
            if votes.get(i).unwrap() == voter {
                votes.remove(i);
                found = true;
                break;
            }
        }

        if !found {
            panic!("Haven't voted");
        }

        env.storage().instance().set(&DataKey::WishlistVotes(wishlist_id), &votes);

        let mut wishlist: Vec<WishlistItem> = env.storage().instance().get(&DataKey::Wishlist).unwrap();
        for i in 0..wishlist.len() {
            let mut item = wishlist.get(i).unwrap();
            if item.id == wishlist_id {
                item.vote_count -= 1;
                wishlist.set(i, item);
                break;
            }
        }
        env.storage().instance().set(&DataKey::Wishlist, &wishlist);
    }

    pub fn get_wishlist(env: Env) -> Vec<WishlistItem> {
        let mut wishlist: Vec<WishlistItem> = env.storage().instance()
            .get(&DataKey::Wishlist)
            .unwrap_or(Vec::new(&env));

        let len = wishlist.len();
        for i in 0..len {
            for j in 0..len - i - 1 {
                let item1 = wishlist.get(j).unwrap();
                let item2 = wishlist.get(j + 1).unwrap();

                if item1.vote_count < item2.vote_count {
                    wishlist.set(j, item2);
                    wishlist.set(j + 1, item1);
                }
            }
        }

        wishlist
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
