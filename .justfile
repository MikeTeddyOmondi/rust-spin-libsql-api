# Default
default:
  just --list

# Run spin build 
run-spin-build:
  spin build

# Run spin build and up
run-spin-buildup:
  spin build --up

# Run spin up 
run-spin-up:
  spin up

# Run spin watch
run-spin-watch:
  spin watch  

# Build Docker Image
build-image:
  docker buildx build --platform wasi/wasm --provenance=false -t docker.io/ranckosolutionsinc/spin-libsql-api:0.1-spin-2.1 .

# Run Docker Container
run-container:
  docker run -d -p 33000:3000 --restart always --runtime=io.containerd.spin.v1 --name spin-libsql-api ranckosolutionsinc/spin-libsql-api:0.1-spin-2.1

# Docker compose 
run-compose:
  docker compose up -d -V

# Docker compose down
run-compose-down:
  docker compose down