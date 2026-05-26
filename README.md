# P2P Order Book via Gossip Protocol

Livro de ofertas descentralizado — ordens propagadas via libp2p GossipSub, matching ao encontrar pares.

## Stack

- Rust, libp2p, IPFS (opcional para snapshots)

## Topologia P2P

```
     Node A ────── GossipSub mesh ────── Node B
        \                               /
         \──────── Node C ─────────────/
```

## Resolução de conflitos

- Ordem identificada por `(maker_id, nonce)` único
- Timestamp lógico por nó; desempate: hash lexicográfico
- Fork: manter ordem com maior sequência confirmada por quorum parcial

Ver [docs/CONFLICT_RESOLUTION.md](docs/CONFLICT_RESOLUTION.md)

## Mitigação DDoS

- Rate limit por peer
- Proof-of-work leve em join (opcional)
- Banlist de peers maliciosos
- Tamanho máximo de mensagem 64KB

## Run

```bash
cargo run -- --listen /ip4/0.0.0.0/tcp/9000
```
