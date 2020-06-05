.PHONY: dal trusted

dal:
	pushd dal && cargo build && popd

trusted:
	pushd trusted && cargo build && popd
