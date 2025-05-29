use litesvm::LiteSVM;
use litesvm_client::SVMClient;
use program::state::hello::Message;
use program_loader::TemplateProgramLoader;
use putils::account::AccountDeserialize;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

pub mod program_loader;

pub struct TestClient {
    pub svm_client: SVMClient,
}

impl Default for TestClient {
    fn default() -> Self {
        Self::new()
    }
}

impl TestClient {
    pub fn new() -> Self {
        let mut svm_client = SVMClient::new();

        svm_client.load_programs(vec![Box::new(TemplateProgramLoader {})]);

        Self { svm_client }
    }
    pub fn hello_tx(&mut self, payer: &Keypair) {
        let msg = Keypair::new();
        let mut tx = Transaction::new_with_payer(
            &[sdk::hello_instruction(
                payer.pubkey(),
                msg.pubkey(),
                b"hello world".to_vec(),
            )],
            Some(&payer.pubkey()),
        );

        let svm: &mut LiteSVM = self.svm_client.as_mut();

        tx.sign(&vec![payer, &msg], svm.latest_blockhash());

        let res = svm.send_transaction(tx).unwrap();
        println!("{res:#?}");

        let msg_acct: Message =
            Message::try_from_bytes(&svm.get_account(&msg.pubkey()).unwrap().data).unwrap();

        assert_eq!(
            String::from_utf8(
                // need to trim the first 8 bytes
                // we have to allocate some extra space in the account
                // to store the length of the vector which is 4 bytes
                // then there is the vector length info itself which is 4 bytes
                msg_acct.msg.to_vec()
            )
            .unwrap()
            .replace('\0', ""),
            "hello world"
        );
    }
    pub fn noop_tx(&mut self, payer: &Keypair) {
        let mut tx = Transaction::new_with_payer(
            &[sdk::noop_instruction(b"hello world".to_vec())],
            Some(&payer.pubkey()),
        );

        let svm: &mut LiteSVM = self.svm_client.as_mut();

        tx.sign(&vec![payer], svm.latest_blockhash());

        let res = svm.send_transaction(tx).unwrap();
        println!("{res:#?}");
    }
}
