# WASI Word Generator Server

A simple Warp server that generates random words with an optional given start letter. Built with Rust for WebAssembly System Interface (WASI). Meant to be run using [WasmEdge](https://wasmedge.org/docs/start/install) on Kubernetes.

## Create a Rust WASM app

```bash
rustup target add wasm32-wasi
cargo new word_generator
```

## Add dependencies

```toml
warp_wasi = "0.3"
tokio_wasi = { version = "1.21", features = ["full"] }
random_word = { version = "0.4.3", features = ["en"] }
serde_derive = "1.0"
serde = "1.0"
```

## Add code

Refer to the `src/main.rs` file in this repository.

## Build for WASI

```bash
cargo build --target wasm32-wasi --release
```

## Test locally using WasmEdge

```bash
# Install WasmEdge -> https://wasmedge.org/docs/start/install
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
source $HOME/.wasmedge/env

# Run the server
wasmedge target/wasm32-wasi/release/word_generator.wasm


# Open another terminal and test the server -> You should see the weather forecast as JSON
curl "http://localhost:8090?starts_with=d"
```

## Run on Docker

```bash
# Build the Docker image
docker buildx build --provenance=false --platform wasi/wasm -t deepu105/word_generator_wasi .
docker push deepu105/word_generator_wasi

# Requires Docker Desktop with the containerd image store feature in your Docker Desktop settings enabled.
docker run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm deepu105/word_generator_wasi:latest
```

## Run on Kubernetes

### Make an OCI image

```bash
# Install buildah for creating OCI images -> https://github.com/containers/buildah/blob/main/install.md
# Build OIC image
buildah build --annotation "module.wasm.image/variant=compat-smart" -t word_generator_wasi Dockerfile_OCI
# Push the image to Docker Hub
buildah push --authfile ~/.docker/config.json word_generator_wasi docker://docker.io/deepu105/word_generator_wasi:latest
```

### Setup KinD cluster

```bash
# Create a "KinD" Cluster
kind create cluster
# Enable WasmEdge support using KWasm -> https://kwasm.sh/quickstart/
kubectl apply -f https://raw.githubusercontent.com/KWasm/kwasm-node-installer/main/example/daemonset.yaml
```

### Deploy the app on Kubernetes

```bash
# Deploy the app
kubectl apply -f k8s-manifest.yaml

# Open a tunnel to the service
kubectl port-forward service/word-generator-service 8080

# Test the server
curl "http://localhost:8080" && curl "http://localhost:8080?starts_with=d"
```
