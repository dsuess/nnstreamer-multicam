## How to run?

- start Docker-container using docker-compose (there are currently some hard-coded
  path, which will not work notside the container)

```
docker-compose -f docker/docker-compose.dev.yml run nnstreamer-dev
```

- run the example

```
cd /workspace
cargo run
```


## What's happening?

- example-app sets up pipeline `videotestsrc | rsstreamid stream_id=??? | appsink`
- the `rsstreamid` plugin attaches custom metadata to each buffer
- value set through rsstreamid plugin settings
- the metadata is read by the appsink and printed