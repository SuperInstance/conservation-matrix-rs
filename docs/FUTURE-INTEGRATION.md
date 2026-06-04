# Future Integration: conservation-matrix-rs

## Current State
Conservation laws in ternary agent systems {-1, 0, +1}. Implements the 5 laws from the Negative Space Intelligence project: (1) negative space discovers hidden structure (60% avoidance through negative feedback alone), (2) avoidance dominates choice (294:1 ratio), (3) strategy species coexist stably (Lotka-Volterra, 100% resilience), (4) population beats individual (+0.075 fitness advantage), (5) avoidance ratio conserved across scales (std=0.001 from 10 to 5000 agents).

## Integration Opportunities

### With ternary-cell GC phase
The 5 conservation laws become invariant checks in ternary-cell's GC phase. After each tick, the GC verifies: Is the avoidance ratio conserved? Are all strategy species present? Is the population advantage maintained? If any law is violated, the GC triggers corrective action — injecting diversity, adjusting energy distribution, or flagging the anomaly.

### With conservation-verify
conservation-matrix-rs provides the laws; conservation-verify provides the testing harness. Together they form the fleet's invariant verification system: conservation-matrix-rs defines what should be true, conservation-verify checks that it IS true at every scale.

### With room-as-codespace
Every room runs conservation-matrix-rs as a background invariant checker. The room's ternary cells evolve, compete, and adapt — but always within the conservation laws. If a room's dynamics violate the 5 laws, the room is flagged for investigation.

## Dormant Ideas Now Unlockable
The 5 laws were research findings. Now they're engineering constraints. ternary-cell provides the runtime where they're enforced. The bridge from research to engineering was blocked; now it's built.

## Potential in Mature Systems
The 5 conservation laws are the fleet's physical laws — as fundamental as thermodynamics. Every room obeys them. Every cell tick respects them. The conservation matrix is the fleet's invariant engine, ensuring that no matter how complex the simulation gets, the fundamental dynamics remain healthy.

## Cross-Pollination Ideas
- **conservation-verify**: Testing harness for the 5 laws
- **conservation-spectral-topology-rs**: Spectral methods for detecting conservation violations
- **dissertation-engine**: The 5 laws are the dissertation's core results
- **lotka-volterra-agents**: Law 3 (species coexistence) uses Lotka-Volterra dynamics

## Dependencies for Next Steps
- Integration with ternary-cell GC phase as invariant checker
- Scale-sweep testing at fleet scale (10K+ cells)
- Real-time violation detection and alerting
