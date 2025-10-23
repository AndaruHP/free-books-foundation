# 🌍 Free Books Foundation 📚

<p align="center">
  <img width="400" src="/assets/fbof.png">
</p>

Welcome to **Free Books Foundation**, a blockchain-powered initiative that lets anyone in the world **borrow books for free** — all powered by **Stellar Soroban smart contracts**.

Everything from lending, reserving, wishlists, and donations is stored **on-chain**, keeping everything transparent and fair.

---

## 💡 What Is Free Books Foundation?

**Free Books Foundation** is a decentralized platform where users can borrow, reserve, and vote on books — all recorded immutably on the **Stellar blockchain**.  
We believe access to books should be **a right, not a privilege**.

---

## ⚙️ Key Features

### 📘 Book Management
- `get_books` → View all available books
- `lend_book` → Borrow a book
- `return_book` → Return a borrowed book
- `get_lending_history` → Check full lending history

### 🔒 Reservations
- `reserve_book` → Reserve a book that’s currently borrowed
- `get_reservation_queue` → See the reservation queue
- `cancel_reservation` → Cancel your reservation

### 💭 Wishlist System
- `add_to_wishlist` → Add a new wishlist book
- `get_wishlist` → View all wishlist entries
- `vote_wishlist` / `unvote_wishlist` → Vote or unvote a wishlist item

### 💸 Donations
- `donate` → Donate to keep the foundation alive
- `get_donor_leaderboard` → See who the top donors are 💖

---

## 🧠 Smart Contract Info

Contract ID:
```text
CBK32OZXIH6MDYTFKB2FSZKVP6BZ4XR3HEOHTJWXYRP5EGUKCG6UA26D
```


---

## 👩‍💻 Main Actors

Meet our two on-chain explorers 👇

| Name | Public Key |
|------|-------------|
| 🧑 Alice | `GBFPKEBX7H4WM6AFQSPM7MOAUPYVWKOXAORGXWVYW3BAVQFTSIHNWU3C` |
| 👩‍🚀 Andarian | `GD75PHBZSQMKZSA4DA2ZTMDIO5BZL5YPGOJL6YIW463B3ZCGR3BIYAWK` |

Here's Alice seed phrase XD, ofc not for andarian:
```text
romance stumble surface output pill settle coach office message knock nerve air lizard check economy soccer slender crash public conduct copper century lounge unaware
```

---

## 🚀 Let’s Go On-Chain

Let’s see how Alice and Andarian interact with the contract!

---

### 📚 Step 1 — View All Books

Alice wants to check out the library:

```text
stellar contract invoke \
--id CBK32OZXIH6MDYTFKB2FSZKVP6BZ4XR3HEOHTJWXYRP5EGUKCG6UA26D \
--source alice \
--network testnet \
-- get_books
```

---

🧾 Step 2 — Check Lending History

```text
stellar contract invoke \
--id CBK32OZXIH6MDYTFKB2FSZKVP6BZ4XR3HEOHTJWXYRP5EGUKCG6UA26D \
--source alice \
--network testnet \
-- get_lending_history
```

---

💌 Step 3 — Add a Wishlist Item

Alice adds her favorite book to the wishlist:

```text
stellar contract invoke \
--id CBK32OZXIH6MDYTFKB2FSZKVP6BZ4XR3HEOHTJWXYRP5EGUKCG6UA26D \
--source alice \
--network testnet \
-- add_to_wishlist \
--title "Cantik Itu Luka" \
--author "Eka Kurniawan" \
--requester GBFPKEBX7H4WM6AFQSPM7MOAUPYVWKOXAORGXWVYW3BAVQFTSIHNWU3C
```

Then check the wishlist:

```text
stellar contract invoke \
--id CBK32OZXIH6MDYTFKB2FSZKVP6BZ4XR3HEOHTJWXYRP5EGUKCG6UA26D \
--source alice \
--network testnet \
-- get_wishlist
```

---

🗳️ Step 4 — Voting on Wishlist Items

Cuz Alice give a book a wishlist, Andarian can vote the wishlist 🥰

```text
# Andarian joins in!
stellar contract invoke \
--id CBK32OZXIH6MDYTFKB2FSZKVP6BZ4XR3HEOHTJWXYRP5EGUKCG6UA26D \
--source andarian \
--network testnet \
-- vote_wishlist \
--wishlist_id 1 \
--voter GD75PHBZSQMKZSA4DA2ZTMDIO5BZL5YPGOJL6YIW463B3ZCGR3BIYAWK
```

But if Andarian changes his mind 👀:

```text
stellar contract invoke \
--id CBK32OZXIH6MDYTFKB2FSZKVP6BZ4XR3HEOHTJWXYRP5EGUKCG6UA26D \
--source andarian \
--network testnet \
-- unvote_wishlist \
--wishlist_id 1 \
--voter GD75PHBZSQMKZSA4DA2ZTMDIO5BZL5YPGOJL6YIW463B3ZCGR3BIYAWK
```

---

📖 Step 5 — Borrowing and Returning Books

Alice borrows a book:

```text
stellar contract invoke \
--id CBK32OZXIH6MDYTFKB2FSZKVP6BZ4XR3HEOHTJWXYRP5EGUKCG6UA26D \
--source alice \
--network testnet \
-- lend_book \
--book_id 1 \
--borrower GBFPKEBX7H4WM6AFQSPM7MOAUPYVWKOXAORGXWVYW3BAVQFTSIHNWU3C
```

Andarian borrows one too:

```text
stellar contract invoke \
--id CBK32OZXIH6MDYTFKB2FSZKVP6BZ4XR3HEOHTJWXYRP5EGUKCG6UA26D \
--source andarian \
--network testnet \
-- lend_book \
--book_id 1 \
--borrower GD75PHBZSQMKZSA4DA2ZTMDIO5BZL5YPGOJL6YIW463B3ZCGR3BIYAWK
```

But i will cause an error, so Alice have to return the book first:

```text
stellar contract invoke \
--id CBK32OZXIH6MDYTFKB2FSZKVP6BZ4XR3HEOHTJWXYRP5EGUKCG6UA26D \
--source alice \
--network testnet \
-- return_book \
--book_id 1 \
--borrower GBFPKEBX7H4WM6AFQSPM7MOAUPYVWKOXAORGXWVYW3BAVQFTSIHNWU3C
```

---

⏳ Step 6 — Reserving Books

If a book is taken, Alice can reserve it:
```text
stellar contract invoke \
--id CBK32OZXIH6MDYTFKB2FSZKVP6BZ4XR3HEOHTJWXYRP5EGUKCG6UA26D \
--source alice \
--network testnet \
-- reserve_book \
--book_id 1 \
--user GBFPKEBX7H4WM6AFQSPM7MOAUPYVWKOXAORGXWVYW3BAVQFTSIHNWU3C
```

Check the reservation queue:

```text
stellar contract invoke \
--id CBK32OZXIH6MDYTFKB2FSZKVP6BZ4XR3HEOHTJWXYRP5EGUKCG6UA26D \
--source alice \
--network testnet \
-- get_reservation_queue \
--book_id 1
```

---

💰 Step 7 — Donating to the Cause

Want to help the foundation thrive?
You can send a donation directly on-chain 💖

```text
stellar contract invoke \
--id CBK32OZXIH6MDYTFKB2FSZKVP6BZ4XR3HEOHTJWXYRP5EGUKCG6UA26D \
--source-account alice \
--network testnet \
-- donate \
--donor GBFPKEBX7H4WM6AFQSPM7MOAUPYVWKOXAORGXWVYW3BAVQFTSIHNWU3C \
--amount 100000000
```

Check the leaderboard to see top donors:

```text
stellar contract invoke \
--id CBK32OZXIH6MDYTFKB2FSZKVP6BZ4XR3HEOHTJWXYRP5EGUKCG6UA26D \
--source alice \
--network testnet \
-- get_donor_leaderboard
```

## 😅 About the UI

> “Is there a frontend?” — Nope.  
> “Why not?” — *Skill issue.* 😭

There’s no UI (yet) because the author proudly admits to being more of a backend enjoyer.  
However, the smart contract logic is fully functional and was co-developed with the help of **Claude AI** and for the logo was created by **ChatGPT**.

I love my fellow AI 🤖✨

---

## 💬 Closing Thoughts

📚 **Free Books Foundation** is a small step toward a world where everyone can access knowledge freely — without barriers, borders, or fees.

If you love this idea, you can support the project by:
- ⭐ Starring the repo
- 💖 Donating on-chain to help the foundation stay alive

Every small action helps grow the ecosystem and keeps free knowledge flowing for everyone 🌱

