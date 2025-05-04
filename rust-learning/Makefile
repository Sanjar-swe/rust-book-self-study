run-hello:
	cargo run --manifest-path 00-getting-started/hello_cargo/Cargo.toml

run-ownership:
	cargo run --manifest-path 02-ownership/ownership_basics/Cargo.toml
	
commit:
	@git add .
	@read -p "Введите сообщение коммита: " msg; \
	git commit -m "$$msg"; \
	git push
