rust:
    cargo build --release --target x86_64-pc-windows-gnu

copy:
    cp target/x86_64-pc-windows-gnu/release/showtoon.exe .

build: rust copy
