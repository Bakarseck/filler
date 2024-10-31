run: build
	docker run -v "$(pwd)/solution":/filler/solution -it filler

build:
	docker build -t filler .

exec:
	cd solution && cargo build
	./linux_game_engine -f maps/map01 -p1 solution/target/debug/solution -p2 linux_robots/$(robot)

audit:
	wget https://assets.01-edu.org/filler/filler.zip
	unzip filler.zip