# ωφ

```
                    APIs (sources.φ)    Relay (Nostr)
                         │                   │
    IO (Web-API) ──── CPU (Rust) ────────────┘
     │  pushInput          │  φ(x,y,z,t)
     │  φ(x,y,z,t)         │
     └────────────── CPU (JS) / GPU (WGSL)
                         ω
```

Protocol: φ(x,y,z,t), 32 bytes  
Certainty: exp(-Δt_eff · g) · exp(-v_c / (g + ε)) · c_q · decay

[omegaflow.space](https://omegaflow.space)

CC BY-NC-SA 4.0
