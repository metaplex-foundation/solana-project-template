#![cfg(feature = "test-bpf")]

use mpl_holla::{instruction::CreateArgs, state::MyAccount};
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

#[tokio::test]
async fn create() {
    let mut context = ProgramTest::new("mpl_holla", mpl_holla::ID, None)
        .start_with_context()
        .await;

    let address = Keypair::new();
    let create_args = CreateArgs { foo: 1, bar: 2 };

    let ix = mpl_holla::instruction::create(
        &address.pubkey(),
        &context.payer.pubkey(),
        &context.payer.pubkey(),
        create_args,
    );

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &address],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    let account = context
        .banks_client
        .get_account(address.pubkey())
        .await
        .unwrap();

    assert!(account.is_some());
    assert_eq!(account.unwrap().data.len(), MyAccount::LEN);
}
