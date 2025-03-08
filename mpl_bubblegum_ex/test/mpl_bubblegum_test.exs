defmodule MplBubblegumTest do
  use ExUnit.Case
  doctest MplBubblegum

  # These tests are examples and will need real keypairs and proper setup to run
  
  @test_keypair <<0::256>> # Replace with a real keypair for testing
  @test_pubkey "11111111111111111111111111111111" # Replace with a real public key

  describe "create_tree_config/2" do
    test "creates a new tree configuration" do
      args = %MplBubblegum.TreeConfig{
        max_depth: 14,
        max_buffer_size: 64,
        authority: @test_pubkey,
        canopy_depth: 5
      }

      assert {:ok, signature} = MplBubblegum.create_tree_config(args, @test_keypair)
      assert is_binary(signature)
    end
  end

  describe "mint_v1/2" do
    test "mints a new compressed NFT" do
      args = %MplBubblegum.MintArgs{
        tree_authority: @test_pubkey,
        leaf_owner: @test_pubkey,
        metadata_uri: "https://example.com/metadata.json",
        name: "Test NFT",
        symbol: "TEST"
      }

      assert {:ok, signature} = MplBubblegum.mint_v1(args, @test_keypair)
      assert is_binary(signature)
    end
  end

  describe "transfer_v1/2" do
    test "transfers a compressed NFT" do
      args = %MplBubblegum.TransferArgs{
        tree_authority: @test_pubkey,
        leaf_owner: @test_pubkey,
        new_leaf_owner: @test_pubkey,
        root: <<0::256>>,
        data_hash: <<0::256>>,
        creator_hash: <<0::256>>,
        nonce: 0,
        index: 0
      }

      assert {:ok, signature} = MplBubblegum.transfer_v1(args, @test_keypair)
      assert is_binary(signature)
    end
  end
end 