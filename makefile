
dev: 
	cargo build --locked
	./target/debug/electrs -vvv \
		--address-search \
		--network mainnet \
		--db-dir ./tests/db \
		--jsonrpc-import \
		--daemon-rpc-addr lab.in.ionance.com:15213 \
		--electrum-rpc-addr 0.0.0.0:50001 \
		--http-addr 0.0.0.0:3000 \
		--cookie "moonuser:passthemoon" \
		--enable-json-rpc-logging \
		--index-unspendables 


test:
	@printf '%s\n' '{"jsonrpc":"2.0","id":1,"method":"blockchain.transaction.get","params":["4d87517238eeb36b2d724c5242b6f1f34f0ed93180f7b3529c8938a4901e2bae",true]}' \
		| nc -w 3 127.0.0.1 50001 \
		| jq .


