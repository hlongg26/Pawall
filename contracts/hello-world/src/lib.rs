#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Price,
    Unlocked(Address),
}

#[contract]
pub struct PaywallContract;

#[contractimpl]
impl PaywallContract {
    // 1. Khởi tạo hợp đồng
    pub fn init(env: Env, admin: Address, token: Address, price: i128) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Price, &price);
    }

    // 2. Giao dịch thanh toán để mở khóa
    pub fn unlock(env: Env, user: Address) {
        user.require_auth();

        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        let token_id: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let price: i128 = env.storage().instance().get(&DataKey::Price).unwrap();

        let token_client = token::Client::new(&env, &token_id);
        token_client.transfer(&user, &admin, &price);

        env.storage().persistent().set(&DataKey::Unlocked(user.clone()), &true);
    }

    // 3. Kiểm tra xem user đã thanh toán chưa
    pub fn check_access(env: Env, user: Address) -> bool {
        env.storage().persistent().get(&DataKey::Unlocked(user)).unwrap_or(false)
    }
}
