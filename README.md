# ternary-game-theory

**Game theory with ternary strategies — normal-form games, Nash equilibrium, cooperative games, Shapley values, and mechanism design.**

## Background

Classical game theory assumes players choose from binary strategies (cooperate/defect) or continuous action spaces. But many real-world decisions naturally fall into three categories: aggressive, neutral, and defensive; invest, hold, or divest; attack, observe, or retreat. The SuperInstance ecosystem models these as ternary strategies: −1 (negative/aggressive), 0 (neutral), +1 (positive/cooperative).

`ternary-game-theory` provides a complete game-theoretic toolkit for ternary strategy spaces:

- **Normal-form games** with 3×3 payoff matrices and ternary best-response dynamics
- **Nash equilibrium** computation (pure-strategy)
- **Prisoner's Dilemma** variant with ternary strategies
- **Cooperative games** with Shapley value computation and core membership
- **Mechanism design** via Vickrey (second-price) auctions
- **Welfare analysis** — social welfare maximization and price of anarchy

## How It Works

### Normal-Form Games

`NormalFormGame` stores a 3×3 payoff matrix indexed by `(Ternary, Ternary)` strategy pairs. Core methods:

- **`best_response_row(col_strategy)`** — row player's optimal response
- **`best_response_col(row_strategy)`** — column player's optimal response
- **`find_pure_nash()`** — enumerate all pure-strategy Nash equilibria (strategy pairs where neither player can unilaterally improve)
- **`best_response_dynamics(initial, max_iters)`** — iterate best responses until convergence or timeout
- **`minmax_row()`** — compute the minmax value (row player's guaranteed payoff under optimal play)
- **`dominated_strategies_row()`** — identify strictly dominated strategies

### Ternary Prisoner's Dilemma

`prisoners_dilemma_ternary()` constructs a three-strategy variant:

| | Defect (−1) | Silent (0) | Cooperate (+1) |
|---|---|---|---|
| **Defect** | (1, 1) | (3, 0) | (5, −1) |
| **Silent** | (0, 3) | (2, 2) | (4, 1) |
| **Cooperate** | (−1, 5) | (1, 4) | (3, 3) |

Defect remains the dominant strategy (highest payoff against every column), but mutual cooperation (3, 3) Pareto-dominates mutual defection (1, 1). The "Silent" middle option adds strategic richness absent from the binary version.

### Cooperative Games

`CooperativeGame` models n-player games with characteristic function values (bitmask-indexed). Methods:

- **`shapley_values()`** — compute Shapley values by enumerating all player permutations, measuring each player's marginal contribution across all orderings
- **`is_in_core(payoffs)`** — check if a payoff vector is in the core (group rationality + individual rationality)
- **`is_superadditive()`** — verify that coalition merging never reduces value

### Mechanism Design

`VickreyAuction` implements a second-price sealed-bid auction where the highest bidder wins but pays the second-highest bid. This mechanism is **incentive-compatible** — truthful bidding is a dominant strategy.

### Welfare Analysis

- **`social_welfare(game, s1, s2)`** — sum of payoffs for a strategy profile
- **`max_social_welfare(game)`** — find the welfare-maximizing profile
- **`price_of_anarchy(game)`** — ratio of optimal welfare to worst Nash equilibrium welfare

## Experimental Results

The test suite (20+ tests) validates:

- **Payoff lookup** — correct indexing by ternary strategies
- **Best responses** — row and column players identify optimal strategies
- **Nash equilibria** — found correctly for identity game and Prisoner's Dilemma
- **Best-response dynamics** — terminates (converges or reaches max iterations)
- **Minmax** — correct value for zero-sum games
- **Dominated strategies** — correctly identified (or correctly found absent)
- **Shapley values** — exact computation matches theoretical values (e.g., player 0: 4.0, player 1: 6.0 for a 2-player game with coalition values [0, 3, 5, 10])
- **Core membership** — (4, 6) is in core; (2, 2) is not
- **Superadditivity** — correctly verified
- **Vickrey auction** — winner and second-price computed correctly; single-bidder edge case handled
- **Price of anarchy** — finite or NaN (no pure Nash) as expected

## Impact

The ternary strategy space has richer structure than binary games. The third option (Silent/Neutral) introduces:

- **Partial cooperation** — a middle ground between full defection and full cooperation
- **More equilibria** — the 3×3 payoff matrix can support up to 3 pure Nash equilibria
- **Nuanced welfare analysis** — the price of anarchy can distinguish between equilibria that the binary model collapses

The Shapley value implementation enables fair value allocation in cooperative ternary systems — assigning credit for coalition outcomes based on marginal contributions.

## Use Cases

1. **Multi-agent resource allocation** — Rooms in a ternary fleet compete for shared resources. Each room's strategy (−1: hoard, 0: fair share, +1: share generously) determines payoffs. `find_pure_nash()` identifies stable allocation profiles.

2. **Fleet coordination games** — Rooms must coordinate actions (e.g., all upgrade simultaneously). The ternary Prisoner's Dilemma models the temptation to delay (defect), the safety of waiting (silent), and the benefit of coordination (cooperate). `price_of_anarchy()` quantifies the cost of miscoordination.

3. **Fair credit distribution** — A group of rooms achieves a collective outcome. `shapley_values()` allocates credit fairly based on each room's marginal contribution across all possible coalition orderings.

4. **Incentive-compatible auctions** — Fleet resources (compute time, bandwidth) are allocated via `VickreyAuction`. Truthful bidding is optimal, so rooms have no incentive to misrepresent their valuations.

5. **Coalition formation** — Rooms form coalitions to achieve goals. `is_superadditive()` checks whether merging coalitions always helps, and `is_in_core()` verifies that no subgroup has incentive to break away.

## Open Questions

- **Mixed-strategy equilibria:** The current implementation finds only pure-strategy Nash equilibria. Should a future version support mixed strategies (probability distributions over ternary actions) and compute Nash equilibria via linear complementarity?
- **Repeated games:** The one-shot game model doesn't capture reputation, retaliation, or learning. Should the crate support iterated games with history-dependent strategies (tit-for-tat on ternary actions)?
- **N-player generalization:** Normal-form games are limited to 2 players. Can the framework extend to n-player games with ternary strategy spaces?

## Connection to Oxide Stack

`ternary-game-theory` provides the strategic reasoning layer:

- **`ternary-voting`** — voting mechanisms can be analyzed as games (strategic voting, manipulation resistance)
- **`ternary-blockchain`** — consensus protocols are games (miners choose strategies: honest mining, selfish mining, or withholding)
- **`ternary-chaos`** — game dynamics (best-response iteration) can exhibit chaotic behavior, analyzable with chaos detection tools
- **`ternary-channel`** — communication channels transport strategic decisions between players
- **`ternary-event`** — game outcomes are published as events for observability

The ternary strategy space (−1, 0, +1) aligns with the ecosystem's core representation, ensuring that game-theoretic analysis and fleet operations share a common language.
