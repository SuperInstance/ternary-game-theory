# ternary-game-theory

Game theory with ternary strategies — normal-form games, Nash equilibrium, cooperative games, and mechanism design over {-1, 0, +1}.

## Why This Exists

Classical game theory is built on binary choices: cooperate or defect, bid or pass, attack or defend. Real strategic situations often have a middle ground — abstain, delay, hedge — that binary models can't represent without awkward hacks.

Ternary game theory gives every player three strategies per move: negative, neutral, and positive. This maps naturally to scenarios like markets (sell/hold/buy), voting (against/abstain/for), and military decisions (retreat/hold/advance). The third strategy isn't just noise — it creates new equilibria, changes the price of anarchy, and enables richer cooperative structures.

## Core Concepts

| Type | Meaning |
|---|---|
| `Ternary` | Strategy value: `Neg` (-1), `Zero` (0), `Pos` (+1) |
| `NormalFormGame` | 3×3 payoff matrix for two players with ternary strategies |
| `CooperativeGame` | Coalition game with bitmask values and Shapley allocation |
| `VickreyAuction` | Second-price auction with ternary-structured valuations |

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-game-theory = "0.1"
```

```rust
use ternary_game_theory::*;

fn main() {
    // Build a game with 3×3 payoff matrix
    let game = NormalFormGame::new(
        "Market Entry",
        vec![
            vec![(1.0, 1.0), (3.0, 0.0), (5.0, -1.0)],
            vec![(0.0, 3.0), (2.0, 2.0), (4.0, 1.0)],
            vec![(-1.0, 5.0), (1.0, 4.0), (3.0, 3.0)],
        ],
    );

    // Find all pure-strategy Nash equilibria
    let nash = game.find_pure_nash();
    println!("Nash equilibria: {:?}", nash);

    // Compute best-response dynamics from an initial state
    let result = game.best_response_dynamics((Ternary::Zero, Ternary::Zero), 20);
    println!("Converged to: {:?}", result);

    // Analyze welfare
    let (profile, welfare) = max_social_welfare(&game);
    println!("Optimal welfare: {} at {:?}", welfare, profile);
}
```

## API Overview

### Normal-Form Games
- `new(name, payoffs)` — create a 3×3 two-player game
- `payoff(s1, s2) → (f64, f64)` — lookup payoffs for a strategy pair
- `best_response_row(col_strategy)` / `best_response_col(row_strategy)` — best replies
- `find_pure_nash() → Vec<(Ternary, Ternary)>` — enumerate pure Nash equilibria
- `best_response_dynamics(initial, max_iters)` — iterative convergence
- `minmax_row() → f64` — minmax value for the row player
- `dominated_strategies_row() → Vec<Ternary>` — iterated dominance

### Cooperative Games
- `CooperativeGame::new(players, coalition_values)` — define a characteristic function
- `shapley_values() → Vec<f64>` — fair allocation via Shapley value
- `is_in_core(payoffs) → bool` — check if an allocation is in the core
- `is_superadditive() → bool` — test superadditivity

### Mechanism Design
- `VickreyAuction::new(valuations)` — second-price sealed-bid auction
- `resolve() → (winner, price)` — determine outcome
- `is_truthful() → bool` — verify incentive compatibility

### Welfare Analysis
- `social_welfare(game, s1, s2)` — sum of payoffs
- `max_social_welfare(game)` — find welfare-maximizing profile
- `price_of_anarchy(game)` — ratio of optimal to worst Nash welfare

### Built-in Games
- `prisoners_dilemma_ternary()` — three-strategy Prisoner's Dilemma (Defect/Silent/Cooperate)

## How It Works

**Normal-form games** use a dense 3×3 payoff matrix indexed by ternary strategy values shifted to {0, 1, 2}. Nash equilibria are found by exhaustive best-response checking across all 9 strategy profiles. Best-response dynamics iterates alternating best replies until convergence or a maximum iteration limit.

**Cooperative games** store coalition values in a bitmask-indexed vector (coalition `i` has value `coalition_values[i]`). Shapley values are computed by enumerating all player permutations and averaging marginal contributions — exact for up to ~10 players. Core membership is verified by checking every coalition's feasibility constraint.

**Welfare analysis** computes the price of anarchy as the ratio between the socially optimal outcome and the worst Nash equilibrium welfare, providing a measure of efficiency loss from decentralized decision-making.

## Use Cases

- **Market simulation** — model sell/hold/buy strategies across multiple agents, compute equilibria and welfare
- **Voting system design** — analyze ternary voting (against/abstain/for) for Nash equilibria and price of anarchy
- **Resource allocation** — use Shapley values to fairly divide gains from cooperation in ternary-strategy settings

## Ecosystem

Part of the **SuperInstance** ternary computing ecosystem:

- [`ternary`](https://crates.io/crates/ternary) — core trit types and balanced ternary arithmetic
- [`ternary-game-theory`](https://crates.io/crates/ternary-game-theory) — this crate
- [`ternary-constraint`](https://crates.io/crates/ternary-constraint) — constraint satisfaction for ternary variables
- [`ternary-swarm`](https://crates.io/crates/ternary-swarm) — swarm intelligence with ternary decisions
- [`ternary-control`](https://crates.io/crates/ternary-control) — ternary control theory

## Known Limitations

- **Pure-strategy Nash only**: `find_pure_nash()` only finds pure-strategy Nash equilibria. Games with only mixed-strategy equilibria will return an empty list with no indication that mixed equilibria exist.
- **Best-response dynamics may not converge**: Iterated best-response has no convergence guarantee for general games; it can cycle indefinitely for games without pure Nash equilibria.
- **Small strategy space**: With only 3 pure strategies per player, the space is too coarse for many real-world strategic situations. Games requiring fine-grained strategies are not well-represented.
- **No support for n-player games**: The normal-form game representation assumes exactly 2 players. Extensive-form, repeated, and n-player games are not supported.

## License

MIT

## See Also
- **ternary-games** — related
- **ternary-auction** — related
- **ternary-market** — related
- **ternary-voting** — related
- **ternary-econ** — related

