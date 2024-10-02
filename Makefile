folder_number ?= 1

folder:
	chmod +x scripts/create
	scripts/create $(folder_number)

dev:
	cd problem_set_$(folder_number) && cargo watch -x run