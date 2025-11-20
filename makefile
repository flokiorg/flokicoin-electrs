
dev: 
	cargo build --locked
	./target/debug/electrs -vvv \
		--address-search \
		--network testnet \
		--db-dir ./tests/db \
		--jsonrpc-import \
		--daemon-rpc-addr lab.in.ionance.com:35213 \
		--electrum-rpc-addr 0.0.0.0:50001 \
		--http-addr 0.0.0.0:3000 \
		--cookie "moonuser:passthemoon" \
		--enable-json-rpc-logging \
		--index-unspendables 
