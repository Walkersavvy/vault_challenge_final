use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_lang::InstructionData;
use solana_program_test::*;
use solana_sdk::{
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};

#[tokio::test]
async fn test_deposit_and_withdraw() {
    // 1. Initialize test environment
    let program_id = blueshift_anchor_vault::id();
    let mut test = ProgramTest::new(
        "blueshift_anchor_vault", // name must match Cargo.toml
        program_id,
        processor!(blueshift_anchor_vault::entry), // entrypoint
    );

    let (mut banks_client, payer, recent_blockhash) = test.start().await;

    // 2. Create signer (user)
    let user = Keypair::new();

    // Fund user with lamports
    let fund_tx = Transaction::new_signed_with_payer(
        &[system_program::transfer(
            &payer.pubkey(),
            &user.pubkey(),
            2_000_000, // enough lamports
        )],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    banks_client.process_transaction(fund_tx).await.unwrap();

    // 3. Derive vault PDA
    let (vault_pda, _bump) =
        Pubkey::find_program_address(&[b"vault", user.pubkey().as_ref()], &program_id);

    // 4. Build deposit instruction
    let deposit_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(user.pubkey(), true),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: blueshift_anchor_vault::instruction::Deposit { amount: 1_000_000 }.data(),
    };

    let deposit_tx = Transaction::new_signed_with_payer(
        &[deposit_ix],
        Some(&user.pubkey()),
        &[&user],
        recent_blockhash,
    );
    banks_client.process_transaction(deposit_tx).await.unwrap();

    // 5. Verify vault balance
    let vault_balance = banks_client.get_balance(vault_pda).await.unwrap();
    assert_eq!(vault_balance, 1_000_000);

    // 6. Build withdraw instruction
    let withdraw_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(user.pubkey(), true),
            AccountMeta::new(vault_pda, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: blueshift_anchor_vault::instruction::Withdraw {}.data(),
    };

    let withdraw_tx = Transaction::new_signed_with_payer(
        &[withdraw_ix],
        Some(&user.pubkey()),
        &[&user],
        recent_blockhash,
    );
    banks_client.process_transaction(withdraw_tx).await.unwrap();

    // 7. Verify vault is empty
    let vault_balance_after = banks_client.get_balance(vault_pda).await.unwrap();
    assert_eq!(vault_balance_after, 0);
}
