# MplBubblegum

Elixir NIFs for the Metaplex Bubblegum compressed NFT protocol on Solana. This library enables Elixir developers to create, mint, and transfer compressed NFTs using the mpl-bubblegum protocol.

## Features

- Create compressed NFT tree configurations
- Mint compressed NFTs
- Transfer compressed NFTs
- Direct integration with Solana devnet/mainnet
- Proper error handling and type conversion between Rust and Elixir

## Installation

Add `mpl_bubblegum` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:mpl_bubblegum, "~> 0.1.0"}
  ]
end
```

### Prerequisites

- Elixir 1.14 or later
- Rust 1.70 or later
- Solana CLI tools (for testing and development)

## Usage

### Creating a Tree Configuration

```elixir
# Create a new tree configuration
args = %MplBubblegum.TreeConfig{
  max_depth: 14,
  max_buffer_size: 64,
  authority: "YOUR_AUTHORITY_PUBKEY",
  canopy_depth: 5
}

{:ok, signature} = MplBubblegum.create_tree_config(args, payer_keypair)
```

### Minting a Compressed NFT

```elixir
# Mint a new compressed NFT
args = %MplBubblegum.MintArgs{
  tree_authority: "TREE_AUTHORITY_PUBKEY",
  leaf_owner: "LEAF_OWNER_PUBKEY",
  metadata_uri: "https://example.com/metadata.json",
  name: "My NFT",
  symbol: "MNFT"
}

{:ok, signature} = MplBubblegum.mint_v1(args, payer_keypair)
```

### Transferring a Compressed NFT

```elixir
# Transfer a compressed NFT
args = %MplBubblegum.TransferArgs{
  tree_authority: "TREE_AUTHORITY_PUBKEY",
  leaf_owner: "CURRENT_OWNER_PUBKEY",
  new_leaf_owner: "NEW_OWNER_PUBKEY",
  root: root_hash,
  data_hash: data_hash,
  creator_hash: creator_hash,
  nonce: nonce,
  index: index
}

{:ok, signature} = MplBubblegum.transfer_v1(args, payer_keypair)
```

## Error Handling

The library returns tagged tuples for all operations:

- `{:ok, signature}` - Operation successful, returns the transaction signature
- `{:error, reason}` - Operation failed, returns the error reason

## Development

### Building

```bash
mix deps.get
mix compile
```

### Testing

```bash
mix test
```

Note: Tests require a running Solana validator (local or devnet) and proper keypair setup.

## Contributing

1. Fork it
2. Create your feature branch (`git checkout -b feature/my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin feature/my-new-feature`)
5. Create new Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Metaplex Foundation](https://www.metaplex.com/) for the mpl-bubblegum protocol
- [Solana Foundation](https://solana.com/) for the Solana blockchain
- [Rustler](https://github.com/rusterlium/rustler) for Rust NIFs in Elixir 