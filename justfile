build:
    cd backend && cargo build --bin backend --release && \
    cp ./target/release/backend .

test-preview:
    ./backend/backend "PreviewOption" "" /home/ishaan/fifc/README.md