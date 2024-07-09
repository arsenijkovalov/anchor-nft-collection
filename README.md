# anchor-nft-collection

## Getting Started

To use this repository, you need to have [yarn](https://yarnpkg.com/getting-started/install), [Anchor](https://www.anchor-lang.com/docs/installation) and the [Solana cli suite](https://solana.com/developers/guides/getstarted/setup-local-development) installed on your machine.

It is highly recommended that you start this project from scratch, following along with the tutorial.

Follow the steps outlined below:

1. Connect to the devnet Cluster using the Solana CLI:

```bash
solana config set --url https://api.devnet.solana.com
```

2. Create a file system wallet.

```bash
solana-keygen new
```

By default, the `solana-keygen` command will create a new file system wallet located at `~/.config/solana/id.json`. You can manually specify the output file location using the `--outfile /path` option.

3. Set your new wallet as the default, this is your deployer's address.

```bash
solana config set -k ~/.config/solana/id.json
```

4. Airdrop SOL tokens to your wallet.

Once your new wallet is set as the default, you can request a free airdrop of SOL tokens to it:

```bash
solana airdrop 2
```

or using Web Faucet: https://faucet.solana.com/.

You can check your current wallet's address or SOL balance any time:

```bash
solana address
```

```bash
solana balance
```

Then request a free airdrop of SOL tokens to it by using Web Faucet: https://faucet.solana.com/.

5. In the root of the directory:

```bash
yarn install
```

**NOTE:** You must use yarn to install the dependencies. If you use a different package manager, you will run into issues minting the NFT.

6. Build your anchor project.

```bash
anchor build
```

7. List the project deployment keys and copy the address to a clipboard.

```bash
anchor keys list
```

8. Update your [`Anchor.toml`](Anchor.toml) file, by using the address generated in the previous step.

```toml
[programs.localnet]
anchor_nft_collection = "<REPLACE WITH YOUR ADDRESS HERE>"
```

9. Update your [`lib.rs`](programs/anchor-nft-collection/src/lib.rs) file by adding the the address generated in step 4 to the `declare_id!()` macro.

```rust
    // snip
declare_id!("<REPLACE WITH YOUR ADDRESS HERE>");

#[constant]
pub const SEED: &str = "Collection";

#[program]
pub mod anchor_nft_collection {
```

10. Build your anchor project.

```bash
anchor build
```

11. Deploy the program.

```bash
anchor deploy
```

12. Create NFT Collection and mint two NFTs.

```bash
anchor run test
```
