.PHONY: dal trusted

dal:
	cd dal && cargo build && cd ..

trusted:
	cd trusted && cargo build && cd ..
