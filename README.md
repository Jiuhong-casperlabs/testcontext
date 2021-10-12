# testcontext
## install
```
casper-client put-deploy \
--chain-name casper-test \
--node-address http://16.162.124.124:7777 \
--payment-amount 15000000000 \
--session-path /home/jiuhong/rust/context/contract/target/wasm32-unknown-unknown/release/contract.wasm \
--secret-key /home/jiuhong/keys/test99/secret_key.pem
```

## invoke entrypoint
```
casper-client put-deploy --chain-name casper-test \
--node-address http://16.162.124.124:7777 \
--secret-key /home/jiuhong/keys/test99/secret_key.pem \
--payment-amount 100000000 \
--session-name "contexttestcontract"   \
--session-entry-point "sessioncontext"
```
