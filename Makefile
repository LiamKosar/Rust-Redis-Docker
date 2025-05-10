# Start the application with Docker Compose
up:
	docker-compose up --build -d

# Start the application in detached mode
up-non-detatched:
	docker-compose up --build

down:
	docker-compose down

redis-cli:
	docker-compose exec redis redis-cli

clean:
	docker-compose down -v --remove-orphans

pre-commit:
	cd rust-app && cargo fmt
	cd rust-app && cargo clippy -- -D warnings