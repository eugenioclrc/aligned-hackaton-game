## Verfier contracts

```shell
$ forge script script/Verifier.s.sol --chain-id=17000 --rpc-url=https://holesky.drpc.org --broadcast --private-key=0xxxxxxxxxxxxxxxxxxxxxxxxx 
##### holesky
[⠊] Compiling...
No files changed, compilation skipped
Script ran successfully.

$ forge verify-contract 0x37E84746A4631f80d279bcB410dd320f57C1B842 --rpc-url=https://holesky.drpc.org --constructor-args 0x00000000000000000000000058f280bebe9b34c9939c3c39e0890c81f163b623000000000000000000000000815aeca64a974297942d2bbf034abee22a38a003 --etherscan-api-key=xxxx
[⠊] Compiling...
No files changed, compilation skipped
Start verifying contract `0x37E84746A4631f80d279bcB410dd320f57C1B842` deployed on holesky

Submitting verification for .........
$ forge verify-contract 0x9ca9463389888B8a69755650467EE11437BEe3eE LeaderBoard --rpc-url=https://holesky.drpc.org --etherscan-api-key=XXXXXXXXX
```

