## Verfier contracts

```shell
$ forge script script/Verifier.s.sol --chain-id=17000 --rpc-url=https://holesky.drpc.org --broadcast --private-key=0xxxxxxxxxxxxxxxxxxxxxxxxx 
##### holesky
[⠊] Compiling...
No files changed, compilation skipped
Script ran successfully.


##### holesky
✅  [Success]Hash: 0x81b5abbd5d8b1f97b718635cf7c645231e65e975523148c4d415f0e0d4686afa
Contract Address: 0x433cDcF08F8AD1652cdEA5B719D98F365b14355c
Block: 2612691
Paid: 0.00021084212304178 ETH (2173630 gas * 0.097000006 gwei)

✅ Sequence #1 on holesky | Total Paid: 0.00021084212304178 ETH (2173630 gas * avg 0.097000006 gwei)

==========================

ONCHAIN EXECUTION COMPLETE & SUCCESSFUL.

Transactions saved to: /xxxxxxxxxx/aligned/leaderboard-contract/broadcast/Verifier.s.sol/17000/run-latest.json

Sensitive values saved to: /xxxxxxxxxx/aligned/leaderboard-contract/cache/Verifier.s.sol/17000/run-latest.json


$ forge verify-contract 0x433cDcF08F8AD1652cdEA5B719D98F365b14355c --rpc-url=https://holesky.drpc.org --constructor-args 0x00000000000000000000000058f280bebe9b34c9939c3c39e0890c81f163b623000000000000000000000000815aeca64a974297942d2bbf034abee22a38a003 --etherscan-api-key=xxxx
[⠊] Compiling...
No files changed, compilation skipped
Start verifying contract `0x433cDcF08F8AD1652cdEA5B719D98F365b14355c` deployed on holesky

Submitting verification for [src/Verifier.sol:Verifier] 0x433cDcF08F8AD1652cdEA5B719D98F365b14355c.
Submitted contract for verification:
        Response: `OK`
        GUID: `zbcyxuxuh7nf6kz48qvtfg9ckx6x4jwyjyxmeeszeiy2m9r2at`
        URL: https://holesky.etherscan.io/address/0x433cdcf08f8ad1652cdea5b719d98f365b14355c
```

