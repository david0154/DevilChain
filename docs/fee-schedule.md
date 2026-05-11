# DevilChain Fee Schedule

> Developed by Nexuzy Lab | nexuzy.tech | Powered by Devil One | devilone.in

## Transaction Fee Rates

| Transaction Type       | Fee Rate | Basis Points |
|------------------------|----------|--------------|
| Standard Transfer      | 0.3%     | 30 bp        |
| DEX Swap               | 1.5%     | 150 bp       |
| NFT Marketplace Sale   | 2.0%     | 200 bp       |
| Smart Contract Deploy  | 5.0%+    | 500 bp (min) |

*Minimum fee floor: 0.01 DVC (10,000 µDVC) regardless of amount.*

---

## Fee Distribution (of collected fee)

| Recipient             | Percentage | Example (1.5 DVC fee) |
|-----------------------|------------|------------------------|
| Miner / Validator     | 55%        | 0.825 DVC              |
| Development Fund      | 18%        | 0.270 DVC              |
| Burn Wallet (🔥)      | 10%        | 0.150 DVC              |
| Liquidity Pool        | 10%        | 0.150 DVC              |
| Marketing Fund        | 7%         | 0.105 DVC              |
| **Total**             | **100%**   | **1.500 DVC**          |

---

## Example Transaction Walkthrough

```
Sender:   Alice  →  Recipient: Bob
Amount:   100 DVC
TX Type:  DEX Swap (1.5% fee)

Fee = 100 DVC × 1.5% = 1.5 DVC

Distribution:
  Miner      → 0.825 DVC  (55%)
  Dev Fund   → 0.270 DVC  (18%)
  Burn 🔥    → 0.150 DVC  (10%)  ← destroyed forever
  Liquidity  → 0.150 DVC  (10%)  ← DAO time-locked
  Marketing  → 0.105 DVC  ( 7%)

Bob receives: 100 DVC exactly
Alice pays:   101.5 DVC total
```

---

## Addresses

| Wallet               | Address                                 |
|----------------------|-----------------------------------------|
| Dev Fund             | `db1xdev_nexuzy_lab_david0154_00000000` |
| Mining Pool          | `db1xmining_pool_devilchain_000000000`  |
| Burn Address 🔥      | `db1x000000000000000000000000000burn`   |
| Liquidity Vault      | `db1xliquidity_lock_vault_dao_00000000` |
| Marketing Fund       | `db1xmarketing_fund_devilchain_000000`  |

---

## Coin Info

| Property            | Value                   |
|---------------------|-------------------------|
| Name                | DevilCoin               |
| Symbol              | DVC / DVL               |
| Max Supply          | 1,000,000,000 DVC       |
| Decimals            | 6 (1 DVC = 1,000,000 µ) |
| Block Reward        | 50 DVC (halves every 2.1M blocks) |
| Min Gas Fee         | 0.01 DVC                |
