# DevilChain Burn & Liquidity Automation

## Total Supply
- 1,000,000,000 DVC

## Burn Rule
- Auto burn remains active until total burned reaches **200,000,000 DVC** (20% of total supply).
- After the burn cap is reached, burn stops automatically.
- Burn allocation is then redirected to keep the economy sustainable.

### Default fee split
| Recipient | % |
|---|---:|
| Validators | 55% |
| Development | 18% |
| Burn | 10% |
| Liquidity | 10% |
| Marketing | 7% |

### After burn cap reached
| Recipient | % |
|---|---:|
| Validators | 55% |
| Development | 23% |
| Burn | 0% |
| Liquidity | 15% |
| Marketing | 7% |

## Liquidity Pool Rule
- When liquidity reserve reaches **200,000,000 DVC**, the protocol auto-locks **5%** of liquidity.
- That means **10,000,000 DVC** gets locked.
- The lock duration is **5 years**.
- Unlock happens automatically after the timelock expires.

## Implemented constants
- `BURN_CAP = 200_000_000 DVC`
- `LIQUIDITY_TARGET = 200_000_000 DVC`
- `LIQUIDITY_AUTO_LOCK_BP = 500` (5%)
- `FIVE_YEARS_SECS = 157_680_000`
