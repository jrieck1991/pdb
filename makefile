.PHONY: dal trusted client mac

dal:
	cd dal && cargo +nightly build && cd ..

trusted:
	cd trusted && cargo +nightly build && cd ..

client:
	cd client && cargo +nightly build && cd ..

# below targets for local dev
mac:
	cargo build --target x86_64-apple-darwin

mac_dal:
	cargo build --bin dal --target x86_64-apple-darwin

mac_trusted:
	cargo build --bin trusted --target x86_64-apple-darwin

mac_client:
	cargo build --bin client --target x86_64-apple-darwin