use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{IsInitialized, Pack, Sealed},
    sysvar::{rent::Rent, Sysvar},
};

#[derive(Debug, PartialEq)]
struct AresToken {
    is_initialized: bool,
    total_supply: u64,
    unlocked_supply: u64,
    locked_supply: u64,
    ares_symbol: [u8; 4], // 4-byte symbol, e.g., "ARES"
}

#[derive(Debug, PartialEq)]
struct LiquidityPool {
    is_initialized: bool,
    reserve: u64,
    last_burn_timestamp: i64,
}

#[derive(Debug, PartialEq)]
struct KingWhale {
    kingwhale_account: Pubkey,
    largest_purchase: u64,
}

#[derive(Debug, PartialEq)]
struct Blacklist {
    is_initialized: bool,
    blacklisted_accounts: Vec<Pubkey>,
}

#[derive(Debug, PartialEq)]
struct Wallets {
    marketing_wallet: Pubkey,
    staff_wallet: Pubkey,
}

impl Sealed for AresToken {}
impl Sealed for LiquidityPool {}
impl Sealed for KingWhale {}
impl Sealed for Blacklist {}
impl Sealed for Wallets {}

impl IsInitialized for AresToken {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl IsInitialized for LiquidityPool {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl IsInitialized for KingWhale {
    fn is_initialized(&self) -> bool {
        self.kingwhale_account != Pubkey::default()
    }
}

impl IsInitialized for Blacklist {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl IsInitialized for Wallets {
    fn is_initialized(&self) -> bool {
        self.marketing_wallet != Pubkey::default() && self.staff_wallet != Pubkey::default()
    }
}

impl Pack for AresToken {
    const LEN: usize = 29; // 1 (is_initialized) + 8 (total_supply) + 8 (unlocked_supply) + 8 (locked_supply) + 4 (ares_symbol)

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let is_initialized = src[0] != 0;
        let total_supply = u64::from_le_bytes(src[1..9].try_into().unwrap());
        let unlocked_supply = u64::from_le_bytes(src[9..17].try_into().unwrap());
        let locked_supply = u64::from_le_bytes(src[17..25].try_into().unwrap());
        let mut ares_symbol = [0u8; 4];
        ares_symbol.copy_from_slice(&src[25..29]);

        Ok(AresToken {
            is_initialized,
            total_supply,
            unlocked_supply,
            locked_supply,
            ares_symbol,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst[0] = self.is_initialized as u8;
        dst[1..9].copy_from_slice(&self.total_supply.to_le_bytes());
        dst[9..17].copy_from_slice(&self.unlocked_supply.to_le_bytes());
        dst[17..25].copy_from_slice(&self.locked_supply.to_le_bytes());
        dst[25..29].copy_from_slice(&self.ares_symbol);
    }
}

impl Pack for LiquidityPool {
    const LEN: usize = 17; // 1 (is_initialized) + 8 (reserve) + 8 (last_burn_timestamp)

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let is_initialized = src[0] != 0;
        let reserve = u64::from_le_bytes(src[1..9].try_into().unwrap());
        let last_burn_timestamp = i64::from_le_bytes(src[9..17].try_into().unwrap());

        Ok(LiquidityPool {
            is_initialized,
            reserve,
            last_burn_timestamp,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst[0] = self.is_initialized as u8;
        dst[1..9].copy_from_slice(&self.reserve.to_le_bytes());
        dst[9..17].copy_from_slice(&self.last_burn_timestamp.to_le_bytes());
    }
}

impl Pack for KingWhale {
    const LEN: usize = 40; // 32-byte kingwhale_account + 8-byte largest_purchase

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let kingwhale_account = Pubkey::new_from_array(src[0..32].try_into().unwrap());
        let largest_purchase = u64::from_le_bytes(src[32..40].try_into().unwrap());

        Ok(KingWhale {
            kingwhale_account,
            largest_purchase,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst[0..32].copy_from_slice(&self.kingwhale_account.to_bytes());
        dst[32..40].copy_from_slice(&self.largest_purchase.to_le_bytes());
    }
}

impl Pack for Blacklist {
    const LEN: usize = 33; // 1 (is_initialized) + (32 * 1) (blacklisted_accounts)

    fn.unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let is_initialized = src[0] != 0;
        let blacklisted_accounts = src[1..33].chunks_exact(32).map(|x| Pubkey::new_from_array(x.try_into().unwrap())).collect();

        Ok(Blacklist {
            is_initialized,
            blacklisted_accounts,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst[0] = self.is_initialized as u8;

        for (i, pubkey) in self.blacklisted_accounts.iter().enumerate() {
            dst[1 + i * 32..33 + i * 32].copy_from_slice(&pubkey.to_bytes());
        }
    }
}

impl Pack for Wallets {
    const LEN: usize = 64; // 32-byte marketing_wallet + 32-byte staff_wallet

    fn.unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let marketing_wallet = Pubkey::new_from_array(src[0..32].try_into().unwrap());
        let staff_wallet = Pubkey::new_from_array(src[32..64].try_into().unwrap());

        Ok(Wallets {
            marketing_wallet,
            staff_wallet,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst[0..32].copy_from_slice(&self.marketing_wallet.to_bytes());
        dst[32..64].copy_from_slice(&self.staff_wallet.to_bytes());
    }
}

// Entry point
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Ares Token program entrypoint");

    let accounts_iter = &mut accounts.iter();

    let ares_account = next_account_info(accounts_iter)?;

    if ares_account.owner != program_id {
        msg!("Ares account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let rent = &Rent::from_account_info(next_account_info(accounts_iter)?)?;
    if !ares_account.is_rent_exempt(rent) {
        msg!("Ares account is not rent exempt");
        return Err(ProgramError::AccountNotRentExempt);
    }

    let mut ares_token_data = AresToken::unpack_from_slice(&ares_account.data.borrow())?;
    if !ares_token_data.is_initialized {
        ares_token_data.is_initialized = true;
        ares_token_data.total_supply = 40_000_000 * 1_000_000; // 40M tokens with 6 decimal places
        ares_token_data.unlocked_supply = 0;
        ares_token_data.locked_supply = ares_token_data.total_supply;
        ares_token_data.ares_symbol = *b"ARES"; // 4-byte symbol
    }

    AresToken::pack_into_slice(&ares_token_data, &mut ares_account.data.borrow_mut());

    // Blacklist mechanism: Check if the account is blacklisted
    let blacklist_account = next_account_info(accounts_iter)?;

    let blacklist_data = Blacklist::unpack_from_slice(&blacklist_account.data.borrow())?;
    if blacklist_data.is_initialized && blacklist_data.blacklisted_accounts.contains(&ares_account.key) {
        msg!("Ares account is blacklisted");
        return Err(ProgramError::InvalidAccountData);
    }

    let pool_account = next_account_info(accounts_iter)?;

    if pool_account.owner != program_id {
        msg!("Pool account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let clock = Clock::from_account_info(next_account_info(accounts_iter)?)?;

    let mut pool_data = LiquidityPool::unpack_from_slice(&pool_account.data.borrow())?;
    if !pool_data.is_initialized {
        pool_data.is_initialized = true;
        pool_data.reserve = (ares_token_data.total_supply as f64 * 0.20) as u64; // 20% of total supply
        pool_data.last_burn_timestamp = clock.unix_timestamp;
    }

    // Calculate the elapsed time since the last burn
    let elapsed_time = clock.unix_timestamp - pool_data.last_burn_timestamp;

    // Burn 0.25% of the liquidity if an hour has passed since the last burn
    if elapsed_time >= 3600 {
        let burn_amount = (pool_data.reserve as f64 * 0.0025) as u64; // 0.25% of the reserve
        pool_data.reserve -= burn_amount;
        pool_data.last_burn_timestamp = clock.unix_timestamp;

        // Perform any additional logic related to burning tokens if needed

        msg!("Burned {} tokens from the liquidity pool", burn_amount);
    }

    LiquidityPool::pack_into_slice(&pool_data, &mut pool_account.data.borrow_mut());

    // Load or create the King Whale account
    let kingwhale_account = next_account_info(accounts_iter)?;

    let mut kingwhale_data = KingWhale::unpack_from_slice(&kingwhale_account.data.borrow())?;
    if !kingwhale_data.is_initialized {
        kingwhale_data.is_initialized = true;
        kingwhale_data.kingwhale_account = *kingwhale_account.key;
        kingwhale_data.largest_purchase = 0;
    }

    // Check if the current purchase is the largest so far
    if let Some(sender_wallet) = accounts_iter.next() {
        if sender_wallet.is_signer && instruction_data.len() >= 8 {
            let transfer_amount = u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());

            if transfer_amount > kingwhale_data.largest_purchase {
                kingwhale_data.largest_purchase = transfer_amount;
                kingwhale_data.kingwhale_account = *sender_wallet.key;

                // Perform any additional logic related to updating the King Whale if needed

                msg!("New King Whale: {}", kingwhale_data.kingwhale_account);
                msg!("Largest Purchase: {}", kingwhale_data.largest_purchase);
            }
        }
    }

    KingWhale::pack_into_slice(&kingwhale_data, &mut kingwhale_account.data.borrow_mut());

    // Buy mechanism: Users can buy tokens and receive them in their wallet
    if let Some(sender_wallet) = accounts_iter.next() {
        if sender_wallet.is_signer && instruction_data.len() >= 8 {
            let buy_amount = u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());

            if buy_amount > pool_data.reserve {
                msg!("Not enough liquidity in the pool");
                return Err(ProgramError::InsufficientFunds);
            }

            // Update the reserve and mint tokens to the user's wallet
            pool_data.reserve -= buy_amount;
            ares_token_data.unlocked_supply += buy_amount;

            // Perform any additional logic related to minting tokens if needed

            msg!("Bought {} tokens from the liquidity pool", buy_amount);
            msg!("New total supply: {}", ares_token_data.total_supply);
            msg!("Unlocked supply: {}", ares_token_data.unlocked_supply);
            msg!("Locked supply: {}", ares_token_data.locked_supply);

            AresToken::pack_into_slice(&ares_token_data, &mut ares_account.data.borrow_mut());
            LiquidityPool::pack_into_slice(&pool_data, &mut pool_account.data.borrow_mut());

            // Transfer the bought tokens to the user's wallet
            let mut user_token_data = AresToken::unpack_from_slice(&sender_wallet.data.borrow())?;
            if !user_token_data.is_initialized {
                user_token_data.is_initialized = true;
                user_token_data.total_supply = 0;
                user_token_data.unlocked_supply = 0;
                user_token_data.locked_supply = 0;
                user_token_data.ares_symbol = *b"ARES";
            }

            user_token_data.unlocked_supply += buy_amount;
            AresToken::pack_into_slice(&user_token_data, &mut sender_wallet.data.borrow_mut());

            msg!("Transferred {} tokens to user's wallet", buy_amount);
        }
    }

    // Transfer mechanism with lock-up period and tax
    if let (Some(sender_wallet), Some(recipient_wallet)) = (accounts_iter.next(), accounts_iter.next()) {
        if sender_wallet.is_signer && recipient_wallet.is_signer && instruction_data.len() >= 24 {
            let transfer_amount = u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());
            let unlock_date = i64::from_le_bytes(instruction_data[8..16].try_into().unwrap());
            let tax_percentage = 5;

            if transfer_amount > ares_token_data.unlocked_supply {
                msg!("Not enough unlocked supply for transfer");
                return Err(ProgramError::InsufficientFunds);
            }

            if unlock_date > clock.unix_timestamp {
                msg!("Tokens are still locked. Unlock date: {}", unlock_date);
                return Err(ProgramError::InvalidInstructionData);
            }

            // Calculate tax
            let tax_amount = (transfer_amount as f64 * (tax_percentage as f64 / 100.0)) as u64;

            // Update the token balances for sender and recipient after tax
            ares_token_data.unlocked_supply -= transfer_amount;
            ares_token_data.unlocked_supply -= tax_amount;

            // Check if the sender is the King Whale and add the tax to the King Whale's holding
            if sender_wallet.key == &kingwhale_data.kingwhale_account {
                kingwhale_data.largest_purchase = 0; // Reset the largest purchase for the next transaction
                kingwhale_data.largest_purchase += transfer_amount;
                msg!("King Whale ARES Holding: {}", kingwhale_data.largest_purchase);
            }

            // Distribute tax to different wallets
            let wallets_account = next_account_info(accounts_iter)?;

            let mut wallets_data = Wallets::unpack_from_slice(&wallets_account.data.borrow())?;
            if !wallets_data.is_initialized {
                wallets_data.is_initialized = true;
                wallets_data.marketing_wallet = *next_account_info(accounts_iter)?.key;
                wallets_data.staff_wallet = *next_account_info(accounts_iter)?.key;
            }

            // Update wallet balances
            wallets_data.marketing_wallet
                .try_borrow_mut_data()?
                .get_mut(0..8)
                .map(|data| {
                    data.copy_from_slice(&(wallets_data.marketing_wallet.data.borrow()[0..8].to_le_bytes()));
                });

            wallets_data.staff_wallet
                .try_borrow_mut_data()?
                .get_mut(0..8)
                .map(|data| {
                    data.copy_from_slice(&(wallets_data.staff_wallet.data.borrow()[0..8].to_le_bytes()));
                });

            msg!(
                "Tax: {} ({}%)",
                tax_amount,
                tax_percentage
            );
            msg!("Distributed to Marketing: {}", tax_amount);

            // Update wallet balances
            wallets_data.marketing_wallet
                .try_borrow_mut_data()?
                .get_mut(0..8)
                .map(|data| {
                    data.copy_from_slice(&(wallets_data.marketing_wallet.data.borrow()[0..8].to_le_bytes()));
                });

            wallets_data.staff_wallet
                .try_borrow_mut_data()?
                .get_mut(0..8)
                .map(|data| {
                    data.copy_from_slice(&(wallets_data.staff_wallet.data.borrow()[0..8].to_le_bytes()));
                });

            // Update the token balances for sender and recipient
            let mut sender_token_data = AresToken::unpack_from_slice(&sender_wallet.data.borrow())?;
            sender_token_data.unlocked_supply -= transfer_amount;
            AresToken::pack_into_slice(&sender_token_data, &mut sender_wallet.data.borrow_mut());

            let mut recipient_token_data = AresToken::unpack_from_slice(&recipient_wallet.data.borrow())?;
            recipient_token_data.unlocked_supply += transfer_amount;
            AresToken::pack_into_slice(&recipient_token_data, &mut recipient_wallet.data.borrow_mut());

            msg!(
                "Transferred {} tokens from sender to recipient after tax",
                transfer_amount
            );
            msg!("New unlocked supply: {}", ares_token_data.unlocked_supply);

            AresToken::pack_into_slice(&ares_token_data, &mut ares_account.data.borrow_mut());
            Wallets::pack_into_slice(&wallets_data, &mut wallets_account.data.borrow_mut());
        }
    }

    Ok(())
}