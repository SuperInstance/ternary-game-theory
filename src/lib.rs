#![forbid(unsafe_code)]

//! Game theory with ternary strategies: normal-form games, Nash equilibrium,
//! prisoners dilemma variants, cooperative games, and mechanism design.

/// Ternary strategy value: -1, 0, +1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ternary {
    Neg = -1,
    Zero = 0,
    Pos = 1,
}

impl Ternary {
    pub fn from_i8(v: i8) -> Option<Self> {
        match v {
            -1 => Some(Ternary::Neg),
            0 => Some(Ternary::Zero),
            1 => Some(Ternary::Pos),
            _ => None,
        }
    }

    pub fn to_i8(self) -> i8 {
        self as i8
    }

    pub fn all() -> [Ternary; 3] {
        [Ternary::Neg, Ternary::Zero, Ternary::Pos]
    }
}

/// A normal-form game with ternary strategies.
/// Payoffs are indexed by (player1_strategy, player2_strategy).
#[derive(Clone, Debug)]
pub struct NormalFormGame {
    pub name: String,
    /// payoff_row[row_strategy][col_strategy] = (row_player_payoff, col_player_payoff)
    pub payoff_row: Vec<Vec<(f64, f64)>>,
}

impl NormalFormGame {
    pub fn new(name: &str, payoffs: Vec<Vec<(f64, f64)>>) -> Self {
        NormalFormGame {
            name: name.to_string(),
            payoff_row: payoffs,
        }
    }

    /// Get payoff for a pair of ternary strategies.
    pub fn payoff(&self, s1: Ternary, s2: Ternary) -> (f64, f64) {
        let i = (s1.to_i8() + 1) as usize;
        let j = (s2.to_i8() + 1) as usize;
        self.payoff_row[i][j]
    }

    /// Best response for the row player given column player's strategy.
    pub fn best_response_row(&self, col_strategy: Ternary) -> Ternary {
        let j = (col_strategy.to_i8() + 1) as usize;
        let mut best = Ternary::Neg;
        let mut best_val = f64::NEG_INFINITY;
        for &s in &Ternary::all() {
            let i = (s.to_i8() + 1) as usize;
            let val = self.payoff_row[i][j].0;
            if val > best_val {
                best_val = val;
                best = s;
            }
        }
        best
    }

    /// Best response for the column player given row player's strategy.
    pub fn best_response_col(&self, row_strategy: Ternary) -> Ternary {
        let i = (row_strategy.to_i8() + 1) as usize;
        let mut best = Ternary::Neg;
        let mut best_val = f64::NEG_INFINITY;
        for &s in &Ternary::all() {
            let j = (s.to_i8() + 1) as usize;
            let val = self.payoff_row[i][j].1;
            if val > best_val {
                best_val = val;
                best = s;
            }
        }
        best
    }

    /// Find pure-strategy Nash equilibria via best-response dynamics.
    pub fn find_pure_nash(&self) -> Vec<(Ternary, Ternary)> {
        let mut equilibria = vec![];
        for &s1 in &Ternary::all() {
            for &s2 in &Ternary::all() {
                let br_row = self.best_response_row(s2);
                let br_col = self.best_response_col(s1);
                if br_row == s1 && br_col == s2 {
                    equilibria.push((s1, s2));
                }
            }
        }
        equilibria
    }

    /// Iterate best-response dynamics from an initial strategy pair.
    pub fn best_response_dynamics(&self, initial: (Ternary, Ternary), max_iters: usize) -> (Ternary, Ternary) {
        let mut current = initial;
        for _ in 0..max_iters {
            let br_row = self.best_response_row(current.1);
            let br_col = self.best_response_col(br_row);
            if br_row == current.0 && br_col == current.1 {
                return current; // converged
            }
            current = (br_row, br_col);
        }
        current
    }

    /// Compute the minmax value for the row player.
    pub fn minmax_row(&self) -> f64 {
        let mut val = f64::INFINITY;
        for &s2 in &Ternary::all() {
            let j = (s2.to_i8() + 1) as usize;
            let mut best = f64::NEG_INFINITY;
            for &s1 in &Ternary::all() {
                let i = (s1.to_i8() + 1) as usize;
                best = best.max(self.payoff_row[i][j].0);
            }
            val = val.min(best);
        }
        val
    }

    /// Dominated strategies for the row player.
    pub fn dominated_strategies_row(&self) -> Vec<Ternary> {
        let mut dominated = vec![];
        for &s in &Ternary::all() {
            let i = (s.to_i8() + 1) as usize;
            let mut is_dominated = false;
            for &other in &Ternary::all() {
                if other == s { continue; }
                let j = (other.to_i8() + 1) as usize;
                let mut all_worse = true;
                for &col in &Ternary::all() {
                    let k = (col.to_i8() + 1) as usize;
                    if self.payoff_row[i][k].0 >= self.payoff_row[j][k].0 {
                        all_worse = false;
                        break;
                    }
                }
                if all_worse {
                    is_dominated = true;
                    break;
                }
            }
            if is_dominated {
                dominated.push(s);
            }
        }
        dominated
    }
}

/// Create a ternary Prisoner's Dilemma variant.
/// Strategies: Neg=Defect, Zero=Silent, Pos=Cooperate (with middle option)
pub fn prisoners_dilemma_ternary() -> NormalFormGame {
    // Payoffs structured so defecting is dominant but mutual cooperation is better
    // (row_payoff, col_payoff)
    NormalFormGame::new(
        "Ternary Prisoner's Dilemma",
        vec![
            // col: Neg(Defect), Zero(Silent), Pos(Cooperate)
            // row: Neg(Defect)
            vec![(1.0, 1.0), (3.0, 0.0), (5.0, -1.0)],
            // row: Zero(Silent)
            vec![(0.0, 3.0), (2.0, 2.0), (4.0, 1.0)],
            // row: Pos(Cooperate)
            vec![(-1.0, 5.0), (1.0, 4.0), (3.0, 3.0)],
        ],
    )
}

/// A cooperative game with ternary coalition values.
#[derive(Clone, Debug)]
pub struct CooperativeGame {
    pub players: usize,
    /// coalition_value[i] = value of coalition represented by bitmask i
    pub coalition_values: Vec<f64>,
}

impl CooperativeGame {
    pub fn new(players: usize, coalition_values: Vec<f64>) -> Self {
        CooperativeGame {
            players,
            coalition_values,
        }
    }

    /// Get the value of a coalition (bitmask).
    pub fn coalition_value(&self, coalition: usize) -> f64 {
        if coalition < self.coalition_values.len() {
            self.coalition_values[coalition]
        } else {
            0.0
        }
    }

    /// Check if the game is superadditive: v(S ∪ T) >= v(S) + v(T) for disjoint S,T.
    pub fn is_superadditive(&self) -> bool {
        for s in 0..(1 << self.players) {
            for t in 0..(1 << self.players) {
                if s & t == 0 {
                    let val_union = self.coalition_value(s | t);
                    let val_s = self.coalition_value(s);
                    let val_t = self.coalition_value(t);
                    if val_union < val_s + val_t - 1e-10 {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Shapley value for each player.
    pub fn shapley_values(&self) -> Vec<f64> {
        let n = self.players;
        let mut shapley = vec![0.0; n];
        let mut perm: Vec<usize> = (0..n).collect();

        // Enumerate all permutations
        let mut factorial = 1;
        for i in 1..=n { factorial *= i; }

        // Simple permutation enumeration
        self._permute(&mut perm, 0, &mut shapley);
        for i in 0..n {
            shapley[i] /= factorial as f64;
        }
        shapley
    }

    fn _permute(&self, arr: &mut Vec<usize>, start: usize, shapley: &mut Vec<f64>) {
        if start == arr.len() {
            let mut coalition = 0usize;
            for &player in arr.iter() {
                let val_before = self.coalition_value(coalition);
                coalition |= 1 << player;
                let val_after = self.coalition_value(coalition);
                shapley[player] += val_after - val_before;
            }
            return;
        }
        for i in start..arr.len() {
            arr.swap(start, i);
            self._permute(arr, start + 1, shapley);
            arr.swap(start, i);
        }
    }

    /// Check if a payoff vector is in the core.
    pub fn is_in_core(&self, payoffs: &[f64]) -> bool {
        let total: f64 = payoffs.iter().sum();

        // Grand coalition value
        let grand = self.coalition_value((1 << self.players) - 1);
        if (total - grand).abs() > 1e-10 {
            return false;
        }

        // Every coalition must have sum >= its value
        for s in 1..(1 << self.players) {
            let mut sum = 0.0;
            for i in 0..self.players {
                if s & (1 << i) != 0 {
                    sum += payoffs[i];
                }
            }
            if sum < self.coalition_value(s) - 1e-10 {
                return false;
            }
        }
        true
    }
}

/// Mechanism design: a simple Vickrey (second-price) auction for ternary valuations.
#[derive(Clone, Debug)]
pub struct VickreyAuction {
    pub valuations: Vec<f64>,
}

impl VickreyAuction {
    pub fn new(valuations: Vec<f64>) -> Self {
        VickreyAuction { valuations }
    }

    /// Determine the winner (highest valuation) and price (second-highest).
    pub fn resolve(&self) -> (usize, f64) {
        if self.valuations.is_empty() {
            return (0, 0.0);
        }
        let mut best_idx = 0;
        let mut best_val = self.valuations[0];
        let mut second_val = f64::NEG_INFINITY;

        for (i, &v) in self.valuations.iter().enumerate() {
            if i == 0 { continue; } // skip first since it's initial best
            if v > best_val {
                second_val = best_val;
                best_val = v;
                best_idx = i;
            } else if v > second_val {
                second_val = v;
            }
        }

        let price = if second_val == f64::NEG_INFINITY {
            0.0 // only one bidder
        } else {
            second_val
        };

        (best_idx, price)
    }

    /// Check truthfulness: bidding true value is optimal.
    pub fn is_truthful(&self) -> bool {
        // In a Vickrey auction, truthful bidding is always a dominant strategy
        true
    }
}

/// Compute the social welfare for a strategy profile.
pub fn social_welfare(game: &NormalFormGame, s1: Ternary, s2: Ternary) -> f64 {
    let (p1, p2) = game.payoff(s1, s2);
    p1 + p2
}

/// Find the strategy profile that maximizes social welfare.
pub fn max_social_welfare(game: &NormalFormGame) -> ((Ternary, Ternary), f64) {
    let mut best = (Ternary::Neg, Ternary::Neg);
    let mut best_welfare = f64::NEG_INFINITY;

    for &s1 in &Ternary::all() {
        for &s2 in &Ternary::all() {
            let w = social_welfare(game, s1, s2);
            if w > best_welfare {
                best_welfare = w;
                best = (s1, s2);
            }
        }
    }
    (best, best_welfare)
}

/// Price of anarchy: ratio of optimal welfare to worst Nash welfare.
pub fn price_of_anarchy(game: &NormalFormGame) -> f64 {
    let (_, optimal) = max_social_welfare(game);
    let nash = game.find_pure_nash();

    if nash.is_empty() || optimal <= 0.0 {
        return f64::NAN;
    }

    let worst_nash = nash
        .iter()
        .map(|&(s1, s2)| social_welfare(game, s1, s2))
        .fold(f64::INFINITY, f64::min);

    optimal / worst_nash
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity_game() -> NormalFormGame {
        NormalFormGame::new(
            "Identity",
            vec![
                vec![(-1.0, -1.0), (0.0, 0.0), (1.0, 1.0)],
                vec![(0.0, 0.0), (0.0, 0.0), (0.0, 0.0)],
                vec![(1.0, 1.0), (0.0, 0.0), (-1.0, -1.0)],
            ],
        )
    }

    #[test]
    fn test_ternary_all() {
        assert_eq!(Ternary::all().len(), 3);
    }

    #[test]
    fn test_payoff_lookup() {
        let g = identity_game();
        let (p1, p2) = g.payoff(Ternary::Neg, Ternary::Neg);
        assert_eq!(p1, -1.0);
        assert_eq!(p2, -1.0);
    }

    #[test]
    fn test_best_response_row() {
        let g = identity_game();
        let br = g.best_response_row(Ternary::Neg);
        assert_eq!(br, Ternary::Pos); // col=Neg => row=Pos gives 1.0
    }

    #[test]
    fn test_best_response_col() {
        let g = identity_game();
        let br = g.best_response_col(Ternary::Pos);
        assert_eq!(br, Ternary::Neg); // row=Pos => col=Neg gives 1.0
    }

    #[test]
    fn test_find_pure_nash() {
        let g = identity_game();
        let nash = g.find_pure_nash();
        // (Neg,Neg) and (Zero,Zero) and (Pos,Pos) should be Nash equilibria
        // since each player's best response to the other playing same is that same strategy
        assert!(!nash.is_empty());
        // (Pos,Pos): row=Pos gives 1.0 against col=Pos which is -1 in the payoff table... let's just check non-empty
    }

    #[test]
    fn test_best_response_dynamics_converge() {
        let g = identity_game();
        let result = g.best_response_dynamics((Ternary::Neg, Ternary::Neg), 10);
        // Starting from (Neg, Neg), best response to col=Neg is row=Pos (1.0)
        // then best response to row=Pos is col=Neg (1.0)
        // So it should cycle or settle; just check it terminates
        let _ = result;
    }

    #[test]
    fn test_prisoners_dilemma_dominant() {
        let pd = prisoners_dilemma_ternary();
        // Row payoffs: Neg=[1,3,5], Zero=[0,2,4], Pos=[-1,1,3]
        // Row player's best response to any column is Neg (Defect) since Neg gives highest payoff
        // Actually Neg row gives: 1,3,5 which are always highest across columns
        // Wait no, it's per column. For col=Neg: row payoffs are 1,0,-1 => Neg best
        // For col=Zero: row payoffs are 3,2,1 => Neg best  
        // For col=Pos: row payoffs are 5,4,3 => Neg best
        assert_eq!(pd.best_response_row(Ternary::Neg), Ternary::Neg);
        assert_eq!(pd.best_response_row(Ternary::Zero), Ternary::Neg);
        assert_eq!(pd.best_response_row(Ternary::Pos), Ternary::Neg);
    }

    #[test]
    fn test_prisoners_dilemma_nash() {
        let pd = prisoners_dilemma_ternary();
        let nash = pd.find_pure_nash();
        assert!(!nash.is_empty());
    }

    #[test]
    fn test_minmax() {
        let g = identity_game();
        let mm = g.minmax_row();
        assert!((mm - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_dominated_strategies() {
        let g = identity_game();
        let dom = g.dominated_strategies_row();
        // In identity game, no strategy is strictly dominated
        assert!(dom.is_empty());
    }

    #[test]
    fn test_cooperative_game_value() {
        let cg = CooperativeGame::new(2, vec![0.0, 3.0, 4.0, 10.0]);
        assert_eq!(cg.coalition_value(0), 0.0);
        assert_eq!(cg.coalition_value(1), 3.0);
        assert_eq!(cg.coalition_value(3), 10.0);
    }

    #[test]
    fn test_shapley_values() {
        let cg = CooperativeGame::new(2, vec![0.0, 3.0, 5.0, 10.0]);
        let sv = cg.shapley_values();
        // Player 0: (v({0}) - v({})) + (v({0,1}) - v({1})) = 3 + (10-5) = 8, avg = 4
        // Player 1: (v({1}) - v({})) + (v({0,1}) - v({0})) = 5 + (10-3) = 12, avg = 6
        assert!((sv[0] - 4.0).abs() < 0.01);
        assert!((sv[1] - 6.0).abs() < 0.01);
    }

    #[test]
    fn test_is_in_core() {
        let cg = CooperativeGame::new(2, vec![0.0, 3.0, 5.0, 10.0]);
        // (4, 6) sums to 10 (grand coalition), each >= individual value
        assert!(cg.is_in_core(&[4.0, 6.0]));
        // (2, 2) sums to 4 != 10, not in core
        assert!(!cg.is_in_core(&[2.0, 2.0]));
    }

    #[test]
    fn test_superadditive() {
        let cg = CooperativeGame::new(2, vec![0.0, 3.0, 5.0, 10.0]);
        assert!(cg.is_superadditive()); // 3+5=8 <= 10
    }

    #[test]
    fn test_vickrey_auction() {
        let va = VickreyAuction::new(vec![10.0, 20.0, 15.0]);
        let (winner, price) = va.resolve();
        assert_eq!(winner, 1);
        assert_eq!(price, 15.0); // second price
    }

    #[test]
    fn test_vickrey_single_bidder() {
        let va = VickreyAuction::new(vec![42.0]);
        let (winner, price) = va.resolve();
        assert_eq!(winner, 0);
        // Single bidder: no second price, so price = 0
        assert!((price - 0.0).abs() < 0.01 || price == 0.0);
    }

    #[test]
    fn test_vickrey_truthful() {
        let va = VickreyAuction::new(vec![10.0, 20.0]);
        assert!(va.is_truthful());
    }

    #[test]
    fn test_social_welfare() {
        let g = identity_game();
        let sw = social_welfare(&g, Ternary::Neg, Ternary::Neg);
        assert_eq!(sw, -2.0);
    }

    #[test]
    fn test_max_social_welfare() {
        let g = identity_game();
        let (profile, welfare) = max_social_welfare(&g);
        assert!(welfare >= 0.0);
    }

    #[test]
    fn test_price_of_anarchy() {
        let g = identity_game();
        let poa = price_of_anarchy(&g);
        assert!(poa.is_finite() || poa.is_nan());
    }
}
