.PHONY: sync_tests
sync_tests:
	if [ ! -d tests ]; then \
  		git clone https://github.com/ethereum/tests; \
  	else \
  		cd tests; git pull; git checkout 1c23e3c27ac53b794de0844d2d5e19cd2495b9d8; \
  	fi

.PHONY: run_general_state_tests
run_general_state_tests:
	cargo run --color=always --bin fluentbase-revm-e2e statetest tests/GeneralStateTests -s

.PHONY: help
help:
	cargo run --color=always --bin fluentbase-revm-e2e -h
