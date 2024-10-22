use super::*;
#[derive(Debug)]
pub(crate) struct RevealTransaction {

    pub(crate) reveal_tx: Transaction,
    pub(crate) key_pair: UntweakedKeyPair,

}