.PHONY: fix dal trusted

fix:
	cargo fix --target x86_64-apple-darwin

dal:
	rm -rf data db.sock
	cargo run --bin dal --target x86_64-apple-darwin

trusted:
	cargo run --bin trusted --target x86_64-apple-darwin
