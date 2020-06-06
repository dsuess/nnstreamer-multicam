## How to run?

- start Docker-container using docker-compose (there are currently some hard-coded
  path, which will not work notside the container)

```
docker-compose -f docker/docker-compose.dev.yml run nnstreamer-dev
```

- build the GStreamer plugin
```
cd /workspace
cargo build
```

- run the example-app
```
cd /workspace/examples
cargo run
```


## What's happening?

- example-app sets up pipeline `videotestsrc | rsstreamid | appsink`
- the `rsstreamid` plugin attaches custom metadata to each buffer
- the metadata is read by the appsink and printed