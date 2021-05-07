cargo build --all \
  && RUST_BACKTRACE=1 cargo watch -i "*/game/**" -i -x "run"
