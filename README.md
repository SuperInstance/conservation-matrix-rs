# conservation-matrix

Conservation laws in ternary agent systems {-1, 0, +1}.

Based on research findings from the Negative Space Intelligence project:

## The 5 Laws

1. **Negative space discovers hidden structure**: Act0 (bad every 5th env) → 60% avoidance DISCOVERED through negative feedback alone
2. **Avoidance dominates choice**: 294:1 avoid:choose ratio. Populations learn what NOT to do far faster than what TO do
3. **Strategy species coexist stably**: Lotka-Volterra shows all 5 species survive, Marksman dominates (27%), 100% ecological resilience
4. **Population > Individual**: +0.075 fitness advantage, finds truth faster via diverse exploration
5. **Avoidance ratio CONSERVED across scales**: std=0.001 from 10 to 5000 agents. Conservation law.

## API

- `ConservationTracker` — tracks avoidance/unknown/choose ratios across generations, verifies conservation law
- `FitnessConvergence` — tracks fitness convergence toward target (0.803 → 0.988)
- `EcologicalResilience` — Shannon diversity, species survival, resilience index
- `PopulationAdvantage` — computes population vs individual fitness advantage
- `AvoidChooseRatio` — tracks the 294:1 avoid:choose ratio
- `StrategySpecies` — 5 species: Explorer, Diplomat, Marksman, Climber, Prospector

## 5 Strategy Species

| Species | Win Rate | Entropy | Strategy |
|---------|----------|---------|----------|
| 🌊 Explorer | 55% | High | Weak signal, keep options open |
| ⚖️ Diplomat | 50% | Medium | Adaptive opponents, mirror them |
| 🎯 Marksman | 50% | Low | Clear feedback, specialize |
| 📈 Climber | 35% | Medium | Diminishing returns, keep searching |
| 🏜️ Prospector | 10% | Max (1.99) | Sparse rewards, never commit |

## Usage

```rust
use conservation_matrix::*;

// Track conservation across scales
let mut tracker = ConservationTracker::new(1000);
for _ in 0..100 {
    let actions = simulate_population(1000);
    tracker.record(&actions);
}
println!("Avoidance std: {:.4}", tracker.avoidance_std());
println!("Conservation verified: {}", tracker.verify_conservation(0.02));

// Track fitness convergence
let mut conv = FitnessConvergence::new(0.988);
conv.record(0.803);
// ... after many generations
println!("Converged: {}", conv.is_converged(0.01));
```

## License

MIT
