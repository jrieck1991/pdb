.PHONY: dal trusted

dal:
	cd dal && cargo +nightly build && cd ..

trusted:
	cd trusted && cargo +nightly build && cd ..
