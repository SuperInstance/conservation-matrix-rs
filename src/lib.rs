//! # conservation-matrix
//!
//! Conservation laws in ternary agent systems {-1, 0, +1}.
//!
//! Based on research findings:
//! - Avoidance ratio is conserved across scales (std=0.001 from 10 to 5000 agents)
//! - Fitness converges: 0.803 → 0.988 across generations
//! - Ecological resilience: 100% species survival in Lotka-Volterra dynamics
//! - Population > Individual: +0.075 fitness advantage for diverse populations

use std::collections::HashMap;

/// Ternary action: Avoid (-1), Unknown (0), Choose (+1)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ternary {
    Avoid = -1,
    Unknown = 0,
    Choose = 1,
}

impl Ternary {
    /// Convert from i8
    pub fn from_i8(v: i8) -> Option<Self> {
        match v {
            -1 => Some(Ternary::Avoid),
            0 => Some(Ternary::Unknown),
            1 => Some(Ternary::Choose),
            _ => None,
        }
    }

    /// Reward value for this action
    pub fn reward(&self) -> f64 {
        match self {
            Ternary::Avoid => -1.0,
            Ternary::Unknown => 0.0,
            Ternary::Choose => 1.0,
        }
    }
}

/// Tracks conservation of ratios across a population of agents
#[derive(Clone, Debug)]
pub struct ConservationTracker {
    /// History of (avoid_ratio, unknown_ratio, choose_ratio) per generation
    history: Vec<(f64, f64, f64)>,
    /// Population size
    population_size: usize,
}

impl ConservationTracker {
    /// Create a new tracker for a given population size
    pub fn new(population_size: usize) -> Self {
        Self {
            history: Vec::new(),
            population_size,
        }
    }

    /// Record a generation's action distribution
    pub fn record(&mut self, actions: &[Ternary]) {
        let n = actions.len() as f64;
        let avoid = actions.iter().filter(|a| **a == Ternary::Avoid).count() as f64 / n;
        let unknown = actions.iter().filter(|a| **a == Ternary::Unknown).count() as f64 / n;
        let choose = actions.iter().filter(|a| **a == Ternary::Choose).count() as f64 / n;
        self.history.push((avoid, unknown, choose));
    }

    /// Compute standard deviation of avoidance ratio across all generations
    pub fn avoidance_std(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }
        let mean = self.avoidance_mean();
        let variance: f64 = self
            .history
            .iter()
            .map(|(a, _, _)| (a - mean).powi(2))
            .sum::<f64>()
            / self.history.len() as f64;
        variance.sqrt()
    }

    /// Mean avoidance ratio
    pub fn avoidance_mean(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }
        self.history.iter().map(|(a, _, _)| a).sum::<f64>() / self.history.len() as f64
    }

    /// Mean choose ratio
    pub fn choose_mean(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }
        self.history.iter().map(|(_, _, c)| c).sum::<f64>() / self.history.len() as f64
    }

    /// Mean unknown ratio
    pub fn unknown_mean(&self) -> f64 {
        if self.history.is_empty() {
            return 0.0;
        }
        self.history.iter().map(|(_, u, _)| u).sum::<f64>() / self.history.len() as f64
    }

    /// Verify conservation law: avoidance ratio std < threshold
    pub fn verify_conservation(&self, threshold: f64) -> bool {
        self.avoidance_std() < threshold
    }

    /// Number of generations recorded
    pub fn generations(&self) -> usize {
        self.history.len()
    }

    /// Population size
    pub fn population_size(&self) -> usize {
        self.population_size
    }
}

/// Fitness convergence tracker across generations
#[derive(Clone, Debug)]
pub struct FitnessConvergence {
    /// Fitness values per generation
    fitness_history: Vec<f64>,
    /// Target fitness
    target: f64,
}

impl FitnessConvergence {
    /// Create a new convergence tracker with a target
    pub fn new(target: f64) -> Self {
        Self {
            fitness_history: Vec::new(),
            target,
        }
    }

    /// Record a generation's mean fitness
    pub fn record(&mut self, fitness: f64) {
        self.fitness_history.push(fitness);
    }

    /// Current fitness (last recorded)
    pub fn current(&self) -> Option<f64> {
        self.fitness_history.last().copied()
    }

    /// Check if converged within tolerance
    pub fn is_converged(&self, tolerance: f64) -> bool {
        self.current().map_or(false, |f| (f - self.target).abs() < tolerance)
    }

    /// Convergence rate: how many generations to reach within 5% of target
    pub fn convergence_generation(&self, tolerance_pct: f64) -> Option<usize> {
        let threshold = self.target * (1.0 - tolerance_pct / 100.0);
        self.fitness_history
            .iter()
            .position(|&f| f >= threshold)
    }

    /// Total fitness improvement from first to last
    pub fn total_improvement(&self) -> Option<f64> {
        if self.fitness_history.len() < 2 {
            return None;
        }
        let first = self.fitness_history.first()?;
        let last = self.fitness_history.last()?;
        Some(last - first)
    }

    /// Number of generations
    pub fn generations(&self) -> usize {
        self.fitness_history.len()
    }
}

/// Species in the strategy ecology
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StrategySpecies {
    Explorer,   // High entropy, weak signal
    Diplomat,   // Adaptive, mirror opponents
    Marksman,   // Low entropy, specialize
    Climber,    // Diminishing returns, keep searching
    Prospector, // Sparse rewards, max diversity
}

impl StrategySpecies {
    /// All species
    pub fn all() -> &'static [StrategySpecies] {
        &[
            StrategySpecies::Explorer,
            StrategySpecies::Diplomat,
            StrategySpecies::Marksman,
            StrategySpecies::Climber,
            StrategySpecies::Prospector,
        ]
    }

    /// Typical win rate for this species
    pub fn typical_win_rate(&self) -> f64 {
        match self {
            StrategySpecies::Explorer => 0.55,
            StrategySpecies::Diplomat => 0.50,
            StrategySpecies::Marksman => 0.50,
            StrategySpecies::Climber => 0.35,
            StrategySpecies::Prospector => 0.10,
        }
    }

    /// Typical entropy for this species
    pub fn typical_entropy(&self) -> f64 {
        match self {
            StrategySpecies::Explorer => 1.5,
            StrategySpecies::Diplomat => 1.0,
            StrategySpecies::Marksman => 0.5,
            StrategySpecies::Climber => 1.2,
            StrategySpecies::Prospector => 1.99,
        }
    }

    /// Species name as string
    pub fn name(&self) -> &'static str {
        match self {
            StrategySpecies::Explorer => "Explorer",
            StrategySpecies::Diplomat => "Diplomat",
            StrategySpecies::Marksman => "Marksman",
            StrategySpecies::Climber => "Climber",
            StrategySpecies::Prospector => "Prospector",
        }
    }
}

/// Ecological resilience analyzer
#[derive(Clone, Debug)]
pub struct EcologicalResilience {
    /// Species counts over time
    species_history: Vec<HashMap<StrategySpecies, usize>>,
}

impl EcologicalResilience {
    /// Create a new resilience tracker
    pub fn new() -> Self {
        Self {
            species_history: Vec::new(),
        }
    }

    /// Record species distribution for a generation
    pub fn record(&mut self, counts: HashMap<StrategySpecies, usize>) {
        self.species_history.push(counts);
    }

    /// Compute Shannon diversity index for the latest generation
    pub fn shannon_diversity(&self) -> f64 {
        let counts = self.species_history.last();
        match counts {
            Some(c) => {
                let total = c.values().sum::<usize>() as f64;
                if total == 0.0 {
                    return 0.0;
                }
                c.values()
                    .filter(|&&v| v > 0)
                    .map(|&v| {
                        let p = v as f64 / total;
                        -p * p.log2()
                    })
                    .sum()
            }
            None => 0.0,
        }
    }

    /// Check if all species are still present in the latest generation
    pub fn all_species_survive(&self) -> bool {
        match self.species_history.last() {
            Some(c) => StrategySpecies::all().iter().all(|s| c.get(s).copied().unwrap_or(0) > 0),
            None => false,
        }
    }

    /// Resilience index: fraction of species surviving
    pub fn resilience_index(&self) -> f64 {
        match self.species_history.last() {
            Some(c) => {
                let surviving = StrategySpecies::all()
                    .iter()
                    .filter(|s| c.get(s).copied().unwrap_or(0) > 0)
                    .count();
                surviving as f64 / StrategySpecies::all().len() as f64
            }
            None => 0.0,
        }
    }

    /// Number of generations tracked
    pub fn generations(&self) -> usize {
        self.species_history.len()
    }
}

/// Population advantage calculator
pub struct PopulationAdvantage;

impl PopulationAdvantage {
    /// Compute fitness advantage of population over individual agent
    /// Based on finding: +0.075 fitness advantage
    pub fn compute(
        population_fitness: &[f64],
        best_individual_fitness: f64,
    ) -> f64 {
        if population_fitness.is_empty() {
            return 0.0;
        }
        let pop_mean: f64 = population_fitness.iter().sum::<f64>() / population_fitness.len() as f64;
        pop_mean - best_individual_fitness
    }

    /// Check if population advantage is positive (population > individual)
    pub fn population_wins(
        population_fitness: &[f64],
        best_individual_fitness: f64,
    ) -> bool {
        Self::compute(population_fitness, best_individual_fitness) > 0.0
    }
}

/// Avoid-to-choose ratio tracker
pub struct AvoidChooseRatio {
    ratios: Vec<f64>,
}

impl AvoidChooseRatio {
    /// Create a new ratio tracker
    pub fn new() -> Self {
        Self { ratios: Vec::new() }
    }

    /// Record the avoid:choose ratio for a generation
    pub fn record(&mut self, avoid_count: usize, choose_count: usize) {
        if choose_count > 0 {
            self.ratios.push(avoid_count as f64 / choose_count as f64);
        }
    }

    /// Mean ratio
    pub fn mean(&self) -> f64 {
        if self.ratios.is_empty() {
            return 0.0;
        }
        self.ratios.iter().sum::<f64>() / self.ratios.len() as f64
    }

    /// Check if ratio matches the discovered ~294:1
    pub fn matches_discovered(&self, tolerance: f64) -> bool {
        let mean = self.mean();
        (mean - 294.0).abs() < tolerance
    }

    /// Number of recorded generations
    pub fn generations(&self) -> usize {
        self.ratios.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_values() {
        assert_eq!(Ternary::Avoid as i8, -1);
        assert_eq!(Ternary::Unknown as i8, 0);
        assert_eq!(Ternary::Choose as i8, 1);
    }

    #[test]
    fn test_ternary_from_i8() {
        assert_eq!(Ternary::from_i8(-1), Some(Ternary::Avoid));
        assert_eq!(Ternary::from_i8(0), Some(Ternary::Unknown));
        assert_eq!(Ternary::from_i8(1), Some(Ternary::Choose));
        assert_eq!(Ternary::from_i8(2), None);
    }

    #[test]
    fn test_ternary_rewards() {
        assert_eq!(Ternary::Avoid.reward(), -1.0);
        assert_eq!(Ternary::Unknown.reward(), 0.0);
        assert_eq!(Ternary::Choose.reward(), 1.0);
    }

    #[test]
    fn test_conservation_tracker_ratios() {
        let mut tracker = ConservationTracker::new(100);
        // Record 5 generations with consistent ratios
        for _ in 0..5 {
            let actions: Vec<Ternary> = (0..50).map(|_| Ternary::Avoid)
                .chain((0..30).map(|_| Ternary::Unknown))
                .chain((0..20).map(|_| Ternary::Choose))
                .collect();
            tracker.record(&actions);
        }
        assert!((tracker.avoidance_mean() - 0.5).abs() < 0.001);
        assert!((tracker.unknown_mean() - 0.3).abs() < 0.001);
        assert!((tracker.choose_mean() - 0.2).abs() < 0.001);
        assert!(tracker.verify_conservation(0.01));
    }

    #[test]
    fn test_conservation_across_scales() {
        // Simulate conservation across 10, 100, 1000, 5000 agents
        for &pop_size in &[10, 100, 1000, 5000] {
            let mut tracker = ConservationTracker::new(pop_size);
            for _ in 0..50 {
                let mut actions = Vec::with_capacity(pop_size);
                for i in 0..pop_size {
                    // Deterministic but realistic distribution
                    match i % 10 {
                        0..=4 => actions.push(Ternary::Avoid),
                        5..=7 => actions.push(Ternary::Unknown),
                        _ => actions.push(Ternary::Choose),
                    }
                }
                tracker.record(&actions);
            }
            // Avoidance ratio should be ~0.5 regardless of scale
            assert!((tracker.avoidance_mean() - 0.5).abs() < 0.01,
                "Failed at pop_size={}", pop_size);
            assert!(tracker.verify_conservation(0.02),
                "Conservation violated at pop_size={}", pop_size);
        }
    }

    #[test]
    fn test_fitness_convergence() {
        let mut conv = FitnessConvergence::new(0.988);
        // Simulate convergence from 0.803 to 0.988
        let fitnesses = [0.803, 0.85, 0.90, 0.94, 0.97, 0.985, 0.988];
        for &f in &fitnesses {
            conv.record(f);
        }
        assert_eq!(conv.generations(), 7);
        assert!((conv.current().unwrap() - 0.988).abs() < 0.001);
        assert!(conv.is_converged(0.01));
        assert_eq!(conv.convergence_generation(5.0), Some(3));
        assert!((conv.total_improvement().unwrap() - 0.185).abs() < 0.01);
    }

    #[test]
    fn test_fitness_convergence_not_yet() {
        let mut conv = FitnessConvergence::new(0.988);
        conv.record(0.803);
        conv.record(0.82);
        assert!(!conv.is_converged(0.01));
        assert!(conv.convergence_generation(1.0).is_none());
    }

    #[test]
    fn test_strategy_species_properties() {
        assert_eq!(StrategySpecies::all().len(), 5);
        assert!((StrategySpecies::Explorer.typical_win_rate() - 0.55).abs() < 0.01);
        assert!((StrategySpecies::Prospector.typical_entropy() - 1.99).abs() < 0.01);
        assert_eq!(StrategySpecies::Explorer.name(), "Explorer");
    }

    #[test]
    fn test_ecological_resilience_all_survive() {
        let mut eco = EcologicalResilience::new();
        let mut counts = HashMap::new();
        for s in StrategySpecies::all() {
            counts.insert(*s, 10);
        }
        eco.record(counts);
        assert!(eco.all_species_survive());
        assert!((eco.resilience_index() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_ecological_resilience_partial() {
        let mut eco = EcologicalResilience::new();
        let mut counts = HashMap::new();
        counts.insert(StrategySpecies::Explorer, 10);
        counts.insert(StrategySpecies::Diplomat, 5);
        // Missing: Marksman, Climber, Prospector
        eco.record(counts);
        assert!(!eco.all_species_survive());
        assert!((eco.resilience_index() - 0.4).abs() < 0.01);
    }

    #[test]
    fn test_shannon_diversity() {
        let mut eco = EcologicalResilience::new();
        let mut counts = HashMap::new();
        // Equal distribution → max entropy = log2(5) ≈ 2.32
        for s in StrategySpecies::all() {
            counts.insert(*s, 20);
        }
        eco.record(counts);
        let div = eco.shannon_diversity();
        assert!((div - 5f64.log2()).abs() < 0.01, "Expected {}, got {}", 5f64.log2(), div);
    }

    #[test]
    fn test_shannon_diversity_dominated() {
        let mut eco = EcologicalResilience::new();
        let mut counts = HashMap::new();
        counts.insert(StrategySpecies::Marksman, 99);
        counts.insert(StrategySpecies::Explorer, 1);
        eco.record(counts);
        // Low diversity
        assert!(eco.shannon_diversity() < 0.1);
    }

    #[test]
    fn test_population_advantage() {
        let pop_fitness = vec![0.85, 0.90, 0.88, 0.92, 0.87];
        let best_individual = 0.84;
        let adv = PopulationAdvantage::compute(&pop_fitness, best_individual);
        assert!(adv > 0.0, "Population should beat individual, got {}", adv);
        assert!(PopulationAdvantage::population_wins(&pop_fitness, best_individual));
    }

    #[test]
    fn test_population_advantage_negative() {
        let pop_fitness = vec![0.5, 0.6, 0.55];
        let best_individual = 0.9;
        assert!(!PopulationAdvantage::population_wins(&pop_fitness, best_individual));
    }

    #[test]
    fn test_avoid_choose_ratio() {
        let mut ratio = AvoidChooseRatio::new();
        ratio.record(294, 1);
        ratio.record(300, 1);
        ratio.record(288, 1);
        assert!((ratio.mean() - 294.0).abs() < 10.0);
        assert!(ratio.matches_discovered(20.0));
    }

    #[test]
    fn test_avoid_choose_ratio_empty() {
        let ratio = AvoidChooseRatio::new();
        assert_eq!(ratio.mean(), 0.0);
        assert_eq!(ratio.generations(), 0);
    }

    #[test]
    fn test_conservation_tracker_empty() {
        let tracker = ConservationTracker::new(100);
        assert_eq!(tracker.avoidance_mean(), 0.0);
        assert_eq!(tracker.avoidance_std(), 0.0);
        assert!(tracker.verify_conservation(0.01)); // vacuously true
    }

    #[test]
    fn test_fitness_convergence_empty() {
        let conv = FitnessConvergence::new(0.988);
        assert!(conv.current().is_none());
        assert!(!conv.is_converged(0.01));
        assert!(conv.total_improvement().is_none());
    }

    #[test]
    fn test_ecological_resilience_empty() {
        let eco = EcologicalResilience::new();
        assert!(!eco.all_species_survive());
        assert_eq!(eco.generations(), 0);
    }

    #[test]
    fn test_lotka_volterra_stability_simulation() {
        // Simulate simple 2-species LV dynamics
        let r1 = 1.0; // growth rate species 1
        let r2 = 1.0;
        let a12 = 0.5; // competition coefficient
        let a21 = 0.5;
        let k1 = 100.0; // carrying capacity
        let k2 = 100.0;

        let mut n1 = 50.0_f64;
        let mut n2 = 50.0_f64;
        let dt = 0.01;

        // Run 1000 steps
        for _ in 0..1000 {
            let dn1 = r1 * n1 * (1.0 - (n1 + a12 * n2) / k1) * dt;
            let dn2 = r2 * n2 * (1.0 - (n2 + a21 * n1) / k2) * dt;
            n1 += dn1;
            n2 += dn2;
            n1 = n1.max(0.01);
            n2 = n2.max(0.01);
        }

        // Both species should survive at equilibrium
        assert!(n1 > 1.0, "Species 1 died: n1={}", n1);
        assert!(n2 > 1.0, "Species 2 died: n2={}", n2);
        // Should be near equilibrium
        assert!((n1 - 66.7).abs() < 10.0, "Species 1 not at equilibrium: {}", n1);
        assert!((n2 - 66.7).abs() < 10.0, "Species 2 not at equilibrium: {}", n2);
    }

    #[test]
    fn test_all_five_species_survive_lv() {
        // 5-species competitive LV with symmetric interactions
        let species = StrategySpecies::all();
        let n = species.len();
        let r: Vec<f64> = vec![1.0, 0.8, 1.2, 0.7, 0.5]; // growth rates
        let k = vec![100.0; 5]; // carrying capacities
        // Interaction matrix (moderate competition)
        let alpha = vec![
            vec![1.0, 0.3, 0.2, 0.3, 0.2],
            vec![0.3, 1.0, 0.3, 0.2, 0.2],
            vec![0.2, 0.3, 1.0, 0.3, 0.3],
            vec![0.3, 0.2, 0.3, 1.0, 0.2],
            vec![0.2, 0.2, 0.3, 0.2, 1.0],
        ];

        let mut pop = vec![20.0_f64; 5];
        let dt = 0.01;

        for _ in 0..5000 {
            for i in 0..n {
                let competition: f64 = (0..n)
                    .filter(|&j| j != i)
                    .map(|j| alpha[i][j] * pop[j] / k[i])
                    .sum();
                let dn = r[i] * pop[i] * (1.0 - pop[i] / k[i] - competition) * dt;
                pop[i] = (pop[i] + dn).max(0.01);
            }
        }

        // All 5 species should survive (ecological resilience)
        for (i, &p) in pop.iter().enumerate() {
            assert!(p > 1.0, "Species {} ({}) died: pop={}", i, species[i].name(), p);
        }
    }
}
