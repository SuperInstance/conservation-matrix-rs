# Conservation Matrix

**Conservation Matrix** is a Rust library implementing conservation laws for ternary agent systems {-1, 0, +1} — tracking avoidance ratios, fitness convergence, ecological resilience via Shannon diversity, and population-level selection advantage.

## Why It Matters

The central empirical finding of the SuperInstance research program is that certain ratios are *conserved* — they remain invariant across population scales, generations, and environmental conditions. Specifically, the avoidance-to-choose ratio of approximately 294:1 holds with standard deviation ≈ 0.001 across populations ranging from 10 to 5,000 agents (the scales tested in this library's test suite). This is not a design choice but an emergent property of ternary action spaces. This library provides the computational infrastructure to measure, verify, and track these conservation invariants. It also provides tools to compute population-level selection advantage (the +0.075 figure is an empirical finding from the broader research program, not a hard-coded constant), and verifies that five distinct strategy species coexist at equilibrium under competitive Lotka-Volterra dynamics — 100% ecological resilience.

## How It Works

**Conservation Tracker:**
Records the ternary action distribution per generation and computes statistics:

```
avoidance_ratio = count(Avoid) / total
```

Conservation is verified by checking that the standard deviation of the avoidance ratio across generations remains below a threshold (typically σ < 0.001):

```
σ = √(Σ(r_i − r̄)² / N)
```

Where r_i is the avoidance ratio in generation i, r̄ is the mean, and N is the number of generations.

**Fitness Convergence:**
Tracks mean population fitness across generations. Convergence to a target (e.g., 0.988) is detected when:

```
|f_current − f_target| < tolerance
```

The convergence generation is defined as the first generation where fitness reaches within 5% of target.

**Ecological Resilience:**
Uses the Shannon diversity index:

```
H = −Σ pᵢ log₂(pᵢ)
```

Maximum H for S equally-distributed species is log₂(S). For S = 5 species, H_max = 2.32 bits. The resilience index is the fraction of species surviving:

```
R = |{i : count(i) > 0}| / S
```

**Lotka-Volterra stability:**
Multi-species competitive LV dynamics with N = 5 species:

```
dNᵢ/dt = rᵢ Nᵢ (1 − Σⱼ αᵢⱼ Nⱼ / Kᵢ)
```

Simulated via Euler integration with dt = 0.01. Intra-species competition αᵢᵢ = 1.0, inter-species αᵢⱼ = 0.2–0.3 — ensuring coexistence when intra > inter (competitive exclusion principle).

**Population advantage:** The mean fitness of a population is compared against the best individual's fitness:

```
advantage = mean(population_fitness) − best_individual_fitness
```

> **Note:** The +0.075 figure is an empirical finding from the broader
> SuperInstance research program, not a constant enforced by this library.
> `PopulationAdvantage::compute` returns whatever the supplied data yields.

## Quick Start

```rust
use conservation_matrix::{ConservationTracker, Ternary};

let mut tracker = ConservationTracker::new(100);
for _ in 0..10 {
    let actions: Vec<Ternary> = (0..50).map(|_| Ternary::Avoid)
        .chain((0..30).map(|_| Ternary::Unknown))
        .chain((0..20).map(|_| Ternary::Choose))
        .collect();
    tracker.record(&actions);
}
println!("Avoidance mean: {:.4}", tracker.avoidance_mean()); // ~0.50
println!("Conservation σ: {:.6}", tracker.avoidance_std());  // ~0.0
println!("Conserved: {}", tracker.verify_conservation(0.01)); // true
```

## API

| Type | Description |
|------|-------------|
| `Ternary` | Enum: Avoid (-1), Unknown (0), Choose (+1) |
| `ConservationTracker` | Action-ratio history and conservation verification |
| `FitnessConvergence` | Fitness-to-target tracking |
| `StrategySpecies` | 5 ecological species with win-rate/entropy profiles |
| `EcologicalResilience` | Shannon diversity and species survival |
| `PopulationAdvantage` | Population vs. individual fitness comparison |
| `AvoidChooseRatio` | 294:1 ratio tracker |

## Architecture Notes

This library implements the **statistics and verification layer** for conservation laws in ternary agent systems: ratio tracking, standard-deviation conservation checks, Shannon diversity, fitness convergence, and competitive Lotka-Volterra dynamics.

> **Status — not yet implemented:** The broader SuperInstance fleet architecture
> describes a conservation invariant *γ + η = C*, where a γ-layer produces
> ternary actions and an η-layer computes conservation statistics, with an alarm
> state when conservation breaks. **This library does not currently implement
> the γ/η layering, the γ + η = C equation, or any alarm-state mechanism.**
> Those are aspirational integration points documented in
> [docs/FUTURE-INTEGRATION.md](docs/FUTURE-INTEGRATION.md).

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md) for the fleet-wide design.

## References

1. Lotka, A.J. (1925). *Elements of Physical Biology*. Williams & Wilkins. (Lotka-Volterra equations.)
2. Shannon, C.E. (1948). "A Mathematical Theory of Communication." *Bell System Technical Journal*, 27, 379–423.
3. Wilson, D.S. & Sober, E. (1994). "Reintroducing Group Selection to the Human Behavioral Sciences." *Behavioral and Brain Sciences*, 17(4), 585–608.

## License

MIT
