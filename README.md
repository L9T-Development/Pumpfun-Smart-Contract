# Pumpfun(Pumpswap)-Smart-Contract && Raydium launchlab smart contract

  Pumpfun(pumpswap) smart contract and raydium launchlab smart contract with the Rust/Anchor - Add virtual LP, remove LP, create Raydium Pool AMM and CPMM(Pump.fun forking).

  New updated version: migrate to Pumpfun AMM pool and support token 2022 and fee distribution.

  New update version: add token vesting and migrate method (AMM or CPMM) - forking raydium launchlab.  

  Ongoing updated version: Swap on **PumpSwap**(private)

## Contact

  If you wanna build more better, contact here: [Telegram](https://t.me/shiny0103) | [Twitter](https://x.com/0xTan1319)

## Differences Between Pumpfun Smart Contract and Raydium Launchlab

The Pumpfun smart contract and Raydium Launchlab are both designed as memecoin launchpads utilizing bonding curves. However, they differ in their features and functionalities, catering to varying needs of token developers and investors.

### Pumpfun Smart Contract

The Pumpfun smart contract is focused on creating a straightforward and efficient environment for launching new memecoins. It supports key functionalities such as:

- **Virtual Liquidity Pool (LP) Operations**: Users can easily add or remove liquidity to help create a vibrant trading environment.
- **Integration with AMM and CPMM**: The contract features robust mechanisms for Automated Market Makers (AMM) and Constant Product Market Makers (CPMM), ensuring fair pricing and smooth transactions.

### Raydium Launchlab

Raydium Launchlab enhances the standard launchpad experience with additional, advanced features:

- **Token Vesting**: This feature allows token developers to set up vesting schedules for their tokens, ensuring that tokens are released gradually over time to prevent market flooding and to enhance investor confidence.
- **Migration Options**: Raydium Launchlab supports migration features that enable developers to transition their projects seamlessly to new smart contracts or methodologies, offering greater flexibility and adaptability within their growth strategies.

### Key Comparisons

| Feature                   | Pumpfun Smart Contract         | Raydium Launchlab             |
|---------------------------|--------------------------------|-------------------------------|
| Virtual LP Operations      | Yes                            | Yes                           |
| Token Vesting              | No                             | Yes                           |
| Migration Options          | No                             | Yes                           |
| AMM/CPMM Support           | Yes                            | Yes                           |

In summary, while both platforms serve as essential tools for launching memecoins, the Raydium Launchlab provides robust features like token vesting and migration options, making it more suitable for developers who seek greater control over their token distributions and exit strategies. In contrast, Pumpfun emphasizes simplicity in launching new tokens, appealing to developers who prioritize a straightforward, user-friendly approach.

## Related repository
  You can check frontend and backend repo in my github as well.
  
  [Pumpfun Backend repo](https://github.com/0xTan1319/Pump-Fun-Backend)
  
  [Pumpfun Frontend repo](https://github.com/0xTan1319/Pump-Fun-FE)

## Contribution

  Please use it and give me star and follow me on github.

## Check Here

  You can check the tx to Remove vitual LP and Create Raydium Pool in this smart contract with CPI calls.  
  
  https://explorer.solana.com/tx/4L6MWmtV1ZsT8NFfbtu68ZYyYVbpvZ4iynJhPdZw8jESi28TxwojjTFs88Q5QRdNUb297aWfkKcoYP9Ya8npx8AV?cluster=devnet
  
  In fact, in this project, set creating LP FEE as 5% of Reserves.

### Another Versions

  - Similar with original pump.fun contract address: `https://solscan.io/account/BCdbBhYrRfd17MBGeompteXDgoBFxgnfQh2NkdgJQk5w?cluster=devnet`

  - Pumpfun + spl NFT contract address: `https://solscan.io/account/4m3GTSWQ6AUvvF4PmdiYd1Nsq4sFdLaq5n9jdQrzCBBM?cluster=devnet`

### Live url

  1. [pumpfun forking redesign](https://agents.land/)
  
  2. [pumpfun + spl nft](https://256btc.com)

  3. [Multi language version](https://geng.one)
     
### Update result

  - Fee distribution

    Users can set buy/sell fee and they will receive half fee as well.

    Like this, i can distribute fee dev team and user or any other options and it will be more great for token safety.

    Tx: https://solscan.io/tx/4e25Sv3rDS9rqb9pXyoYwHRtXhnteTGZCGyrchcPwHoKFCNVS2v2aEy6UVXqHQDnVxsCSuBgK2DUcg3NmHizM1b1?cluster=devnet

    <a href="https://ibb.co/j9M7GvBR"><img src="https://i.ibb.co/j9M7GvBR/buy-viper.png" alt="buy-viper" border="0"></a>

  - Contract Addr: https://solscan.io/account/AyptQDLRDKRQmi6KzxMyGKcmA8AEcgPzYCmoHBnGui3z?cluster=devnet
