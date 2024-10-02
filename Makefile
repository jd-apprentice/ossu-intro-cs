folder_number ?= 1
is_intro_to_programming ?= yes

folder:
	chmod +x scripts/create
	scripts/create $(folder_number) $(is_intro_to_programming)

dev:
	cd problem_set_$(folder_number) && cargo watch -x run