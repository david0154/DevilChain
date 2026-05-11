# DevilChain REST API Reference

Base URL: `http://localhost:8545`

## Endpoints

| Method | Path | Description |
|---|---|---|
| GET | /api/status | Network status |
| GET | /api/block/latest | Latest block |
| GET | /api/block/:height | Block by height |
| GET | /api/tx/:hash | Transaction by hash |
| GET | /api/wallet/:address | Wallet info |
| GET | /api/validators | Active validators |
| GET | /api/dao/proposals | DAO proposals |
| POST | /api/send | Send DVC |
| POST | /api/stake | Stake DVC |
| POST | /api/unstake | Unstake DVC |
| POST | /api/vote | DAO vote |

## POST /api/send
```json
{
  "from": "db1xabc...",
  "to": "db1xxyz...",
  "amount": 10.0,
  "gas_fee": 0.01,
  "signature": "ed25519_sig"
}
```

## POST /api/stake
```json
{
  "address": "db1xabc...",
  "amount": 100.0,
  "signature": "ed25519_sig"
}
```

## GraphQL (Port 8546)

```graphql
type Query {
  latestBlock: Block!
  block(height: Int!): Block
  transaction(hash: String!): Transaction
  wallet(address: String!): WalletInfo
  validators: [Validator!]!
  daoProposals: [Proposal!]!
}
```
