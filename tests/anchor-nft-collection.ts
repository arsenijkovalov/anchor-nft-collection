import * as anchor from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import { Program } from "@coral-xyz/anchor";
import { AnchorNftCollection } from "../target/types/anchor_nft_collection";
import { PROGRAM_ID as METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
import { Metaplex } from "@metaplex-foundation/js";

// anchor run test
describe("anchor-nft-collection", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .AnchorNftCollection as Program<AnchorNftCollection>;

  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  const metaplex = Metaplex.make(connection);

  // URI:
  // 1. https://ipfs.io/ipfs/QmRoKMWjMScL6eeVYg6wGSJ3m91UCUwoA6URnNTk3xHVQS
  // 2. https://ipfs.io/ipfs/QmQfsDBjhgkVMhnk2jbVA4NL5CEUGiuSuKMYVBNGpspr5S
  // 3. https://ipfs.io/ipfs/QmR1NHPAt72JMQyUVgNEUgZYxETtjR6TF1ppKXjuexRQji
  // 4. https://ipfs.io/ipfs/QmWezZ6L4EmPYMpJkCzUyejxLb85kVfDkvaVqKivMoXywH
  // 5. https://ipfs.io/ipfs/QmUpwEjthtks1e7au1aU7saPP3E9YNYckPi662Zh6BhBbE

  const testMetadata = {
    name: "Decimated NFT-1",
    symbol: "DIO",
    uri: "https://ipfs.io/ipfs/QmRoKMWjMScL6eeVYg6wGSJ3m91UCUwoA6URnNTk3xHVQS",
  };

  const [collectionPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("Collection")],
    program.programId
  );

  it("create collection nft", async () => {
    const collectionMetadataPDA = await metaplex
      .nfts()
      .pdas()
      .metadata({ mint: collectionPDA });

    const collectionMasterEditionPDA = await metaplex
      .nfts()
      .pdas()
      .masterEdition({ mint: collectionPDA });

    const collectionTokenAccount = await spl.getAssociatedTokenAddress(
      collectionPDA,
      wallet.publicKey
    );

    const modifyComputeUnits =
      anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
        units: 300_000,
      });

    try {
      const tx = await program.methods
        .createCollectionNft(
          testMetadata.uri,
          testMetadata.name,
          testMetadata.symbol
        )
        .accounts({
          authority: wallet.publicKey,
          collectionMint: collectionPDA,
          metadataAccount: collectionMetadataPDA,
          masterEdition: collectionMasterEditionPDA,
          tokenAccount: collectionTokenAccount,
          tokenMetadataProgram: METADATA_PROGRAM_ID,
        })
        .postInstructions([modifyComputeUnits])
        .signers([wallet.payer])
        .rpc();

      console.log("txSig:", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("create nft1 in collection", async () => {
    const mint = anchor.web3.Keypair.generate();

    const metadataPDA = await metaplex
      .nfts()
      .pdas()
      .metadata({ mint: mint.publicKey });

    const masterEditionPDA = await metaplex
      .nfts()
      .pdas()
      .masterEdition({ mint: mint.publicKey });

    const tokenAccount = await spl.getAssociatedTokenAddress(
      mint.publicKey,
      wallet.publicKey
    );

    const collectionMetadataPDA = await metaplex
      .nfts()
      .pdas()
      .metadata({ mint: collectionPDA });

    const collectionMasterEditionPDA = await metaplex
      .nfts()
      .pdas()
      .masterEdition({ mint: collectionPDA });

    const modifyComputeUnits =
      anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
        units: 300_000,
      });

    try {
      const tx = await program.methods
        .createNftInCollection(
          testMetadata.uri,
          testMetadata.name,
          testMetadata.symbol
        )
        .accounts({
          user: wallet.publicKey,
          collectionMint: collectionPDA,
          collectionMetadataAccount: collectionMetadataPDA,
          collectionMasterEdition: collectionMasterEditionPDA,
          nftMint: mint.publicKey,
          metadataAccount: metadataPDA,
          masterEdition: masterEditionPDA,
          tokenAccount: tokenAccount,
          tokenMetadataProgram: METADATA_PROGRAM_ID,
        })
        .postInstructions([modifyComputeUnits])
        .signers([wallet.payer, mint])
        .rpc();
      console.log("txSig:", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("create nft2 in collection", async () => {
    const mint = anchor.web3.Keypair.generate();

    const metadataPDA = await metaplex
      .nfts()
      .pdas()
      .metadata({ mint: mint.publicKey });

    const masterEditionPDA = await metaplex
      .nfts()
      .pdas()
      .masterEdition({ mint: mint.publicKey });

    const tokenAccount = await spl.getAssociatedTokenAddress(
      mint.publicKey,
      wallet.publicKey
    );

    const collectionMetadataPDA = await metaplex
      .nfts()
      .pdas()
      .metadata({ mint: collectionPDA });

    const collectionMasterEditionPDA = await metaplex
      .nfts()
      .pdas()
      .masterEdition({ mint: collectionPDA });

    const modifyComputeUnits =
      anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
        units: 300_000,
      });

    try {
      const tx = await program.methods
        .createNftInCollection(
          testMetadata.uri,
          testMetadata.name,
          testMetadata.symbol
        )
        .accounts({
          user: wallet.publicKey,
          collectionMint: collectionPDA,
          collectionMetadataAccount: collectionMetadataPDA,
          collectionMasterEdition: collectionMasterEditionPDA,
          nftMint: mint.publicKey,
          metadataAccount: metadataPDA,
          masterEdition: masterEditionPDA,
          tokenAccount: tokenAccount,
          tokenMetadataProgram: METADATA_PROGRAM_ID,
        })
        .postInstructions([modifyComputeUnits])
        .signers([wallet.payer, mint])
        .rpc();
      console.log("txSig:", tx);
    } catch (error) {
      console.log(error);
    }
  });
});
