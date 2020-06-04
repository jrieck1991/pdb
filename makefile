.PHONY: fix mac_fix mac_dal mac_trusted

fix:
	cargo fix

dal:
	cargo build --bin dal --target x86_64-unknown-linux-gnu

trusted:
	cargo build --bin trusted --target x86_64-fortanix-unknown-sgx

mac_fix:
	cargo fix --target x86_64-apple-darwin

mac_dal:
	rm -rf data db.sock
	cargo run --bin dal --target x86_64-apple-darwin

mac_trusted:
	cargo run --bin trusted --target x86_64-apple-darwin
