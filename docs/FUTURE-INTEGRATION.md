# Future Integration: conservation-matrix-rs

## Current State
Conservation laws for ternary agent systems with `ConservationTracker` (tracking avoid/unknown/choose ratios across generations), `Ternary` action types with reward values, and empirical constants: avoidance ratio conserved with std=0.001, fitness converges from 0.803→0.988, ecological resilience at 100% species survival.

## Integration Opportunities

### With World Physics (ternary-world)
`ConservationTracker` extends `WorldPhysics` beyond simple sum conservation. `WorldPhysics` enforces grid sum = target; `ConservationTracker` adds the empirical ratio conservation (avoid:unknown:choose ratios stable across scales). The world's physics engine runs multiple conservation laws simultaneously — sum conservation AND ratio conservation AND population fitness conservation.

### With ternary-cell (Conservation as Cell Invariant)
The cell tick cycle's conservation phase (`apply_conservation()`) uses `ConservationTracker` to verify the tissue's action ratios remain invariant. If a tissue's avoidance ratio drifts beyond std=0.001, it triggers cascade detection. This is the cell-level conservation law that prevents tissue-level collapse.

### With negative-space-core (Empirical Validation)
The 294:1 ratio from negative-space-core is one of the conservation laws tracked here. `ConservationTracker::avoidance_std()` should return values ≤ 0.001 when the negative-space-core's avoidance ratio holds. This crate provides the mathematical verification layer for negative-space-core's empirical observations.

## Potential in Mature Systems
Conservation laws become the physics of the ternary universe. Every system invariant — resource budgets, communication ratios, population dynamics, fitness convergence — is a conservation law enforced by this crate. Like physical laws (conservation of energy, momentum), these are inviolable constraints that all rooms, agents, and cells must respect.

## Cross-Pollination Ideas
- The fitness convergence data (0.803→0.988) connects to `ternary-fitness` for empirical landscape validation
- Population advantage (+0.075 for diverse populations) connects to `strategy-ecology` for diversity-fitness correlation
- `ConservationTracker` history feeds into `ternary-entropy` for measuring conservation law entropy over time
- The std=0.001 threshold could be a tunable parameter in `ternary-world`'s `WorldPhysics`

## Dependencies for Next Steps
- Integration with ternary-cell's conservation phase
- Real-time conservation monitoring for live fleet data
- Extension to N-dimensional conservation (not just ratio, but multi-variable invariants)
- Connect to ternary-science for continuous experimental validation
