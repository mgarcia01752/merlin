# RX: centered unitary FFT

RX converts exactly one useful received time window into the original increasing
physical-bin order. Prefix removal and timing acquisition are separate stages.
The analysis transform is the inverse of the [PHY TX definition](tx-ifft.md):

$$
\widehat X[k]=\frac{1}{\sqrt N}\sum_{n=0}^{N-1}r[n]
 e^{-j2\pi n(k-N/2)/N}.
$$

```mermaid
flowchart LR
    USEFUL["N useful received samples"] --> FFT["Centered unitary FFT"]
    FFT --> BINS["RX physical bins in increasing frequency order"]
```

The wrapper owns a reusable plan and returns a new grid without changing input.
Both transforms preserve discrete energy; neither applies arbitrary gain.
The FFT does not remove propagation effects or estimate a channel. Those belong
to [equalization](rx-equalization.md). Independent impulse and quarter-tone tests
check the FFT directly so a reversible TX/RX scaling mistake cannot pass only
because two wrong transforms cancel each other.
