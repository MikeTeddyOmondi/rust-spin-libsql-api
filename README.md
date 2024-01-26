# Spin LibSQL API

Docker build:

```shell
docker buildx build --platform wasi/wasm --provenance=false -t docker.io/ranckosolutionsinc/spin-libsql-api:0.1-spin-2.1 .
```

Start the service

```shell
docker run -d -p 33000:3000 --restart always --runtime=io.containerd.spin.v1 --name spin-libsql-api ranckosolutionsinc/spin-libsql-api:0.1-spin-2.1

``` 