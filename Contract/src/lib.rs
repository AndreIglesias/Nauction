use near_sdk::{env, near_bindgen, AccountId, Gas, Promise};
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
    last_bet: u128,
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

#[ext_contract(ext_contract_)]
trait ExtContract {
	fn nft_supply_for_owner(&self, account_id: AccountId);
	fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>);
	fn nft_transfer(
		&mut self,
		receiver_id: AccountId,
		token_id: TokenId,
		approval_id: u64,
		memo: Option<String>,
	);
	fn nft_token(&self, token_id: TokenId);
}

#[ext_contract(ext_self)]
pub trait MyContract {
    fn my_callback(&self) -> String;
    fn on_transfer_locked_nft(&mut self, transaction_id: TransactionId) -> String;
    fn on_is_nft_locked(&self, transaction_id: TransactionId) -> bool;
    fn on_pay_completed(&mut self, transaction_id: TransactionId) -> ();
    fn on_pay_cancelled(&mut self, transaction_id: TransactionId) -> ();
    fn on_set_royalties_data(&mut self, transaction_id: TransactionId) -> bool;
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

		self.users.insert(env::signer_account_id(), &user);
	}

	pub fn set_card(&mut self, nft_id: String, nft_account: AccountId) {
		let card = Cards{nft_id: nft_id, nft_account: nft_account, last_bet: 0, bettor: ""};

		self.cards.insert(&cards_n.copy(), &card);
	}

	#[payable]
	pub fn set_bet(&mut self, card_n: u128) {
		card = self.cards.get(&card_n.clone()).unwrap();
		if (env::attached_deposit() <= card.last_bet) {
			env::panic_str("bet not high enough");
		}
		card.last_bet = env::attached_deposit();
		card.bettor = env::singer_account_id();
	}

	pub fn end_auction(&self, card_n: u128) {
		let card = self.cards.get(&card_n.clone()).unwrap();
		let promise = ext_contract_::nft_transfer(
			card.bettor.clone(),
			card.nft_id.clone(),
			0,
			None,
			card.nft_account(),
			1,
			Gas(11_000_000_000_000);
		);
		promise.then(ext_self::on_transfer_locked_nft(card_n.clone(), env::current_account_id(),0, Gas(5_000_000_000_000)));
	}
}
