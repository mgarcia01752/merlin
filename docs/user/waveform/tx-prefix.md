# TX: cyclic-prefix insertion

TX prepends the selected $C$ trailing samples to each useful symbol:

$$
y=[x[N-C],\ldots,x[N-1],x[0],\ldots,x[N-1]].
$$

```mermaid
flowchart LR
    USEFUL["N useful IFFT samples"] --> COPY["Copy final C samples to the front"]
    COPY --> PREFIXED["N+C samples; useful values unchanged"]
```

Prefix length is configurable before a run and constant in its geometry snapshot.
All five permitted lengths are supported at both spacings. Insertion is a pure
operation with no hidden previous-symbol state. It checks exact useful length
and finite components. The later [window](tx-window.md) owns cyclic suffix and
tapering; the [RX inverse](rx-prefix.md) owns removal. Component tests verify
sample-for-sample copying and useful-data preservation for every prefix.
