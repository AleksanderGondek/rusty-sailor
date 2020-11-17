# rusty-sailor
Single-binary k8s installer for bare metal (one-node, multi-node)

## Big caveat

For some unexplicit reason, using rustup provided by `<nixpkgs>`, will always enforce dynamic linking (even in musl target).
