Init CodeId: 2138
Migrated CodeId: 2612 2652

```shell
seid tx wasm store artifacts/zkme_sbt.wasm -y \
    --from [account name] \
    --chain-id atlantic-2 \
    --node https://rpc.atlantic-2.seinetwork.io \
    --gas auto \
    --gas-prices 0.25usei \
    --gas-adjustment 1.3 \
    --broadcast-mode block
```

```shell
seid tx wasm instantiate 2138 '{}' \
    --chain-id atlantic-2 \
    --node https://rpc.atlantic-2.seinetwork.io \
    --from [account name] \
    --gas 1000000 \
    --broadcast-mode=block \
    --label "zkMeSBT" \
    --admin sei1u6rxx79qc7snmmtesvp3zr9wx2shff4gkyypew \
    --fees 100000usei
```

Contract Address: sei1dmwr4e6k4n0dlwtkh598sxp2al3wvkvwew658r3cqx98648uqhcs7sd38d
