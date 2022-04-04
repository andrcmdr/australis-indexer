The `id_ed25519` file must contain an openssh private key that has read access to the required github repos.

Afterwards, build with:
`docker build -t dockerreg.internal.aurora.dev/borealis-indexer:latest -f Dockerfile .`
