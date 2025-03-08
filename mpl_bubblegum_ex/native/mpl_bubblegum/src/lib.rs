use rustler::{Encoder, Env, NifResult, NifStruct, Term};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use mpl_bubblegum::{
    instructions::{CreateTreeConfig, MintV1, TransferV1},
    state::{metaplex_adapter::MetadataArgs, TreeConfig},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BubblegumError {
    #[error("Solana client error: {0}")]
    SolanaClientError(#[from] solana_client::client_error::ClientError),
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),
    #[error("Transaction error: {0}")]
    TransactionError(String),
}

#[derive(NifStruct)]
#[module = "MplBubblegum.TreeConfig"]
pub struct TreeConfigArgs {
    pub max_depth: u32,
    pub max_buffer_size: u32,
    pub authority: String,
    pub canopy_depth: u32,
}

#[derive(NifStruct)]
#[module = "MplBubblegum.MintArgs"]
pub struct MintArgs {
    pub tree_authority: String,
    pub leaf_owner: String,
    pub metadata_uri: String,
    pub name: String,
    pub symbol: String,
}

#[derive(NifStruct)]
#[module = "MplBubblegum.TransferArgs"]
pub struct TransferArgs {
    pub tree_authority: String,
    pub leaf_owner: String,
    pub new_leaf_owner: String,
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
}

#[rustler::nif]
pub fn create_tree_config(
    env: Env,
    args: TreeConfigArgs,
    payer_keypair: Vec<u8>,
) -> NifResult<String> {
    let payer = Keypair::from_bytes(&payer_keypair)
        .map_err(|e| rustler::Error::Term(Box::new(format!("Invalid keypair: {}", e))))?;
    
    let authority = Pubkey::try_from(bs58::decode(args.authority).into_vec().unwrap().as_slice())
        .map_err(|e| rustler::Error::Term(Box::new(format!("Invalid authority pubkey: {}", e))))?;

    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    
    let merkle_tree = Keypair::new();
    
    let create_tree_config_ix = CreateTreeConfig {
        max_depth: args.max_depth,
        max_buffer_size: args.max_buffer_size,
        authority,
        merkle_tree: merkle_tree.pubkey(),
        payer: payer.pubkey(),
        tree_creator: payer.pubkey(),
        log_wrapper: Pubkey::default(), // You might want to make this configurable
        compression_program: Pubkey::default(), // You might want to make this configurable
        canopy_depth: args.canopy_depth,
    };

    let recent_blockhash = client
        .get_latest_blockhash()
        .map_err(|e| rustler::Error::Term(Box::new(format!("Failed to get blockhash: {}", e))))?;

    let transaction = Transaction::new_signed_with_payer(
        &[create_tree_config_ix.instruction()],
        Some(&payer.pubkey()),
        &[&payer, &merkle_tree],
        recent_blockhash,
    );

    let signature = client
        .send_and_confirm_transaction(&transaction)
        .map_err(|e| rustler::Error::Term(Box::new(format!("Failed to send transaction: {}", e))))?;

    Ok(signature.to_string())
}

#[rustler::nif]
pub fn mint_v1(
    env: Env,
    args: MintArgs,
    payer_keypair: Vec<u8>,
) -> NifResult<String> {
    let payer = Keypair::from_bytes(&payer_keypair)
        .map_err(|e| rustler::Error::Term(Box::new(format!("Invalid keypair: {}", e))))?;

    let tree_authority = Pubkey::try_from(bs58::decode(args.tree_authority).into_vec().unwrap().as_slice())
        .map_err(|e| rustler::Error::Term(Box::new(format!("Invalid tree authority pubkey: {}", e))))?;

    let leaf_owner = Pubkey::try_from(bs58::decode(args.leaf_owner).into_vec().unwrap().as_slice())
        .map_err(|e| rustler::Error::Term(Box::new(format!("Invalid leaf owner pubkey: {}", e))))?;

    let client = RpcClient::new("https://api.devnet.solana.com".to_string());

    let metadata_args = MetadataArgs {
        name: args.name,
        symbol: args.symbol,
        uri: args.metadata_uri,
        seller_fee_basis_points: 0,
        primary_sale_happened: false,
        is_mutable: true,
        edition_nonce: None,
        token_standard: None,
        collection: None,
        uses: None,
        token_program_version: None,
        creators: vec![],
    };

    let mint_ix = MintV1 {
        tree_authority,
        leaf_owner,
        tree_delegate: payer.pubkey(),
        merkle_tree: payer.pubkey(), // You should get this from the tree creation
        payer: payer.pubkey(),
        tree_creator: payer.pubkey(),
        log_wrapper: Pubkey::default(),
        compression_program: Pubkey::default(),
        metadata_args,
    };

    let recent_blockhash = client
        .get_latest_blockhash()
        .map_err(|e| rustler::Error::Term(Box::new(format!("Failed to get blockhash: {}", e))))?;

    let transaction = Transaction::new_signed_with_payer(
        &[mint_ix.instruction()],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let signature = client
        .send_and_confirm_transaction(&transaction)
        .map_err(|e| rustler::Error::Term(Box::new(format!("Failed to send transaction: {}", e))))?;

    Ok(signature.to_string())
}

#[rustler::nif]
pub fn transfer_v1(
    env: Env,
    args: TransferArgs,
    payer_keypair: Vec<u8>,
) -> NifResult<String> {
    let payer = Keypair::from_bytes(&payer_keypair)
        .map_err(|e| rustler::Error::Term(Box::new(format!("Invalid keypair: {}", e))))?;

    let tree_authority = Pubkey::try_from(bs58::decode(args.tree_authority).into_vec().unwrap().as_slice())
        .map_err(|e| rustler::Error::Term(Box::new(format!("Invalid tree authority pubkey: {}", e))))?;

    let leaf_owner = Pubkey::try_from(bs58::decode(args.leaf_owner).into_vec().unwrap().as_slice())
        .map_err(|e| rustler::Error::Term(Box::new(format!("Invalid leaf owner pubkey: {}", e))))?;

    let new_leaf_owner = Pubkey::try_from(bs58::decode(args.new_leaf_owner).into_vec().unwrap().as_slice())
        .map_err(|e| rustler::Error::Term(Box::new(format!("Invalid new leaf owner pubkey: {}", e))))?;

    let client = RpcClient::new("https://api.devnet.solana.com".to_string());

    let transfer_ix = TransferV1 {
        tree_authority,
        leaf_owner,
        new_leaf_owner,
        merkle_tree: payer.pubkey(), // You should get this from the tree creation
        log_wrapper: Pubkey::default(),
        compression_program: Pubkey::default(),
        root: args.root,
        data_hash: args.data_hash,
        creator_hash: args.creator_hash,
        nonce: args.nonce,
        index: args.index,
    };

    let recent_blockhash = client
        .get_latest_blockhash()
        .map_err(|e| rustler::Error::Term(Box::new(format!("Failed to get blockhash: {}", e))))?;

    let transaction = Transaction::new_signed_with_payer(
        &[transfer_ix.instruction()],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let signature = client
        .send_and_confirm_transaction(&transaction)
        .map_err(|e| rustler::Error::Term(Box::new(format!("Failed to send transaction: {}", e))))?;

    Ok(signature.to_string())
}

rustler::init!("Elixir.MplBubblegum.Native", [
    create_tree_config,
    mint_v1,
    transfer_v1
]); 