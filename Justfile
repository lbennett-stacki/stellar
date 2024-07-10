flash:
  cd firmware && cargo run --release

test:
  cd firmware && cargo test

broker:
  docker run -d --name stellar-broker -p 1883:1883 -p 8083:8083 -p 8084:8084 -p 8883:8883 -p 18083:18083 emqx/emqx:latest
