defmodule MplBubblegum do
  @moduledoc """
  MplBubblegum provides an Elixir interface to the Metaplex Bubblegum compressed NFT protocol.
  This module enables creating and managing compressed NFTs on the Solana blockchain.
  """

  defmodule TreeConfig do
    @moduledoc """
    Struct representing the configuration for creating a new compressed NFT tree.
    """
    defstruct [:max_depth, :max_buffer_size, :authority, :canopy_depth]
  end

  defmodule MintArgs do
    @moduledoc """
    Struct representing the arguments for minting a new compressed NFT.
    """
    defstruct [:tree_authority, :leaf_owner, :metadata_uri, :name, :symbol]
  end

  defmodule TransferArgs do
    @moduledoc """
    Struct representing the arguments for transferring a compressed NFT.
    """
    defstruct [
      :tree_authority,
      :leaf_owner,
      :new_leaf_owner,
      :root,
      :data_hash,
      :creator_hash,
      :nonce,
      :index
    ]
  end

  @doc """
  Creates a new compressed NFT tree configuration.

  ## Parameters

  * `args` - A `TreeConfig` struct containing:
    * `max_depth` - Maximum depth of the merkle tree
    * `max_buffer_size` - Maximum buffer size for the tree
    * `authority` - Base58 encoded public key of the tree authority
    * `canopy_depth` - Depth of the canopy
  * `payer_keypair` - The keypair bytes of the payer account

  ## Returns

  * `{:ok, signature}` - The transaction signature on success
  * `{:error, reason}` - On failure
  """
  def create_tree_config(%TreeConfig{} = args, payer_keypair) do
    case MplBubblegum.Native.create_tree_config(args, payer_keypair) do
      {:ok, signature} -> {:ok, signature}
      {:error, reason} -> {:error, reason}
    end
  end

  @doc """
  Mints a new compressed NFT.

  ## Parameters

  * `args` - A `MintArgs` struct containing:
    * `tree_authority` - Base58 encoded public key of the tree authority
    * `leaf_owner` - Base58 encoded public key of the NFT owner
    * `metadata_uri` - URI pointing to the NFT metadata
    * `name` - Name of the NFT
    * `symbol` - Symbol of the NFT
  * `payer_keypair` - The keypair bytes of the payer account

  ## Returns

  * `{:ok, signature}` - The transaction signature on success
  * `{:error, reason}` - On failure
  """
  def mint_v1(%MintArgs{} = args, payer_keypair) do
    case MplBubblegum.Native.mint_v1(args, payer_keypair) do
      {:ok, signature} -> {:ok, signature}
      {:error, reason} -> {:error, reason}
    end
  end

  @doc """
  Transfers a compressed NFT to a new owner.

  ## Parameters

  * `args` - A `TransferArgs` struct containing:
    * `tree_authority` - Base58 encoded public key of the tree authority
    * `leaf_owner` - Base58 encoded public key of the current NFT owner
    * `new_leaf_owner` - Base58 encoded public key of the new NFT owner
    * `root` - The current root hash of the merkle tree
    * `data_hash` - The hash of the NFT data
    * `creator_hash` - The hash of the NFT creators
    * `nonce` - The nonce of the leaf
    * `index` - The index of the leaf in the tree
  * `payer_keypair` - The keypair bytes of the payer account

  ## Returns

  * `{:ok, signature}` - The transaction signature on success
  * `{:error, reason}` - On failure
  """
  def transfer_v1(%TransferArgs{} = args, payer_keypair) do
    case MplBubblegum.Native.transfer_v1(args, payer_keypair) do
      {:ok, signature} -> {:ok, signature}
      {:error, reason} -> {:error, reason}
    end
  end
end 