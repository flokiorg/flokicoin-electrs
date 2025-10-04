
dev: 
	cargo build --locked
	./target/debug/electrs -vvv \
		--address-search \
		--network mainnet \
		--db-dir ./tests/db \
		--jsonrpc-import \
		--daemon-rpc-addr localhost:15213 \
		--electrum-rpc-addr 0.0.0.0:50001 \
		--http-addr 0.0.0.0:3000 \
		--cookie "user:pass" \
		--enable-json-rpc-logging \
		--index-unspendables 
