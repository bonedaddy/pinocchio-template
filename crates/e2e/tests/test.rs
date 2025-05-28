use e2e::TestClient;
use litesvm::LiteSVM;
use solana_sdk::{signature::Keypair, signer::Signer};

#[test]
fn test_program_basic() {
    let mut test_client = TestClient::new();

    let test_key = Keypair::new();

    {
        let svm: &mut LiteSVM = test_client.svm_client.as_mut();

        svm.airdrop(&test_key.pubkey(), 10_000_000_000).unwrap();
    }

    test_client.hello_tx(&test_key);
}
