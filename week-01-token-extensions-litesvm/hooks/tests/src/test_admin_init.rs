use anchor_client::solana_sdk::system_program;
use litesvm::LiteSVM;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use admin;

// #[derive()]
// struct InitArgs { };

fn setup_admin() -> (LiteSVM, Keypair) {
    let mut svm = LiteSVM::new();
    // deploy the program
    let program_id = Pubkey::from_str_const("8UTBWoFJczymeBeJDUPjNajMLSYNhjmf1SkCBU3cBzrh");
    let program_bytes = include_bytes!("../../target/deploy/admin.so");
    svm.add_program(program_id, program_bytes);

    // create and fund test account
    let admin = Keypair::new();
    svm.airdrop(&admin.pubkey(), LAMPORTS_PER_SOL * 10)
        .expect("airdrop failed");
    (svm, admin)
}

#[test]
fn test_admin_init() {
    let (svm, admin) = setup_admin();
    // admin + config + system_program

    let (config_pda, _bump) = Pubkey::find_program_address(&[b"config".as_ref()], admin::ID_CONST);

    let init_accounts = admin::accounts::Init {
        admin: admin.pubkey(),
        system_program: system_program::id(),
        config: config_pda,
    };

    let data = admin::instruction::Initialize {};

    let ix = Instruction {
        program_id: admin::ID_CONST,
        accounts: init_accounts,
        data,
    };

    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[ix],
        Some(&admin.pubkey()),
        &[&admin],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx);
}
