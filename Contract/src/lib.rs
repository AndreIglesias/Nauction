use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use serde::{Deserialize, Serialize};

pub struct User {
    name: String,
    cards: Vec<String>,
}

pub struct Card {
    nft_id: String,
    nft_account: AccountId,
    auction: u128,
    bettor: AccountId,
}

pub struct Contract {
    users: LookupMap<AccountId, User>,
    cards: LookupMap<u128, Card>,
	cards_n: u128,
    pool: u128,
    earnings: u128,
    fee: u128,
}

impl Default for Contract {
    fn default() ->Self {
        Self {
            users: LookupMap::new(b"a"),
            cards: LookupMap::new(b"a"),
			cards_n: 0,
            pool: 0,
            earnings: 0,
            fee: 10,
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
	pub fn new(fee: u128) ->Self {
        Self {
            users: LookupMap::new(b"a"),
            cards: LookupMap::new(b"a"),
			cards_n: 0,
            pool: 0,
            earnings: 0,
            fee: fee,
        }
    }

	pub fn set_user(&mut self, name: String) {
		let mut user = User{name: name, cards: Vec::new()};

		self.users.add(env::signer_account_id(), &user);
	}

	pub fn set_card(&mut self, nft_id: String, nft_account: AccountId) {
		let card = Cards{nft_id: nft_id, nft_account: nft_account};

		self.cards.add(&cards_n.copy(), &card);
	}
}
