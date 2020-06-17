.PHONY: dal trusted client

dal:
	cd dal && cargo +nightly build && cd ..

trusted:
	cd trusted && cargo +nightly build && cd ..

client:
	cd client && cargo +nightly build && cd ..
