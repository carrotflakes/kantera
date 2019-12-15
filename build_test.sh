for f in $(ls examples/*.rs); do
    cargo build --release --example $(basename $f | sed 's/\.[^\.]*$//')
done
