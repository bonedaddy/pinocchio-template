use litesvm::LiteSVM;
use litesvm_client::SVMClient;
use program_loader::TemplateProgramLoader;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

pub mod program_loader;

pub struct TestClient {
    pub svm_client: SVMClient,
}

impl TestClient {
    pub fn new() -> Self {
        let mut svm_client = SVMClient::new();

        svm_client.load_programs(vec![Box::new(TemplateProgramLoader {})]);

        Self { svm_client }
    }
    pub fn hello_tx(&mut self, payer: &Keypair) {
        let mut tx = Transaction::new_with_payer(
            &[sdk::hello_instruction(b"hello world".to_vec())],
            Some(&payer.pubkey()),
        );

        let svm: &mut LiteSVM = self.svm_client.as_mut();

        tx.sign(&vec![payer], svm.latest_blockhash());

        let res = svm.send_transaction(tx).unwrap();
        println!("{res:#?}");
    }
}
