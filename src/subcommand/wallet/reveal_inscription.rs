// use bitcoin::key::UntweakedKeyPair;
// use bitcoin::secp256k1::{Secp256k1, XOnlyPublicKey};
// use bitcoin::secp256k1::rand::{self};
// use bitcoin::taproot::{LeafVersion, TaprootBuilder};
// use super::*;
// use bitcoin::ScriptBuf;
// use bitcoin::Address;
// use anyhow::Result;
// use bitcoin::address::NetworkUnchecked;
// use serde::{Serialize, Deserialize};
// use bitcoin::consensus;
// use bitcoin::util::bip32::ChildNumber;
// 
// #[derive(Debug, Parser)]
// pub struct RevealInscription {
//     #[arg(short, long, help = "Commit transaction ID")]
//     commit_tx_id: String,
//     #[arg(short, long, help = "Taproot Address")]
//     taproot_address: String,
// }
// 
// impl RevealInscription {
//     pub(crate) fn run(self, wallet: Wallet) -> SubcommandResult {
//         let commit_tx_id = self.commit_tx_id.parse().expect("Invalid transaction ID");
//         let taproot_address = self.taproot_address.parse().expect("Invalid taproot address");
// 
//         // Generate Taproot key pair and reveal script
//         let secp256k1 = Secp256k1::new();
//         let key_pair = UntweakedKeyPair::new(&secp256k1, &mut rand::thread_rng());
//         let (public_key, _parity) = XOnlyPublicKey::from_keypair(&key_pair);
// 
//         let reveal_script = ScriptBuf::builder()
//             .push_slice(public_key.serialize())
//             .push_opcode(bitcoin::opcodes::all::OP_CHECKSIG)
//             .into_script();
// 
//         let taproot_spend_info = TaprootBuilder::new()
//             .add_leaf(0, reveal_script.clone())
//             .expect("adding leaf should work")
//             .finalize(&secp256k1, public_key)
//             .expect("finalizing taproot builder should work");
// 
//         let control_block = taproot_spend_info
//             .control_block(&(reveal_script.clone(), LeafVersion::TapScript))
//             .expect("should compute control block");
// 
//         // Create reveal transaction
//         let reveal_tx = self.create_reveal_transaction(
//             commit_tx_id,
//             taproot_address,
//             reveal_script,
//             control_block,
//             &wallet,
//         )?;
// 
//         // Sign and broadcast the reveal transaction
//         let signed_reveal_tx = wallet
//             .bitcoin_client()
//             .sign_raw_transaction_with_wallet(&reveal_tx, None, None)?
//             .hex;
// 
//         let reveal_txid = wallet
//             .bitcoin_client()
//             .send_raw_transaction(&signed_reveal_tx)?;
// 
//         println!("Reveal transaction sent successfully. TxID: {:?}", reveal_txid);
// 
//         Ok(Some(Box::new(Output { reveal_txid })))
//     }
// 
//     fn create_reveal_transaction(
//         &self,
//         commit_tx_id: Txid,
//         taproot_address: Address,
//         reveal_script: Script,
//         control_block: ControlBlock,
//         wallet: &Wallet,
//     ) -> Result<Transaction> {
//         // Build reveal transaction using the provided commit_tx_id and taproot_address
//         let mut tx_builder = TransactionBuilder::new();
//         tx_builder.input(commit_tx_id, 0);  // Assuming vout 0 for simplicity
//         tx_builder.output(taproot_address, 10000);  // Set appropriate amount
//         tx_builder.reveal_script(reveal_script);
//         tx_builder.control_block(control_block);
// 
//         Ok(tx_builder.build())
//     }
// }
