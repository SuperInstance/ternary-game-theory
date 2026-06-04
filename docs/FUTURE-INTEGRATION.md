# Future Integration: ternary-game-theory

## Current State
Provides normal-form games with ternary strategies, Nash equilibrium finding, prisoner's dilemma variants, cooperative games with Shapley values, and mechanism design for ternary agents.

## Integration Opportunities

### With ternary-voting (Strategic Voting)
Voting IS a game. Agents may vote strategically rather than truthfully. `ternary-game-theory` models the strategic behavior; `ternary-voting` implements the voting mechanism. Nash equilibrium of the voting game predicts how agents will actually vote, not just how they should vote. Mechanism design ensures the voting rules incentivize truth-telling.

### With ternary-room (Resource Competition)
Rooms competing for resources IS a game. `NormalFormGame` where strategies are resource request levels (-1 = release, 0 = maintain, +1 = acquire more). Nash equilibrium finds the stable allocation. Cooperative game theory with Shapley values determines each room's fair share.

### With ternary-econ
Market equilibrium IS game-theoretic equilibrium. `ternary-econ` models the market; `ternary-game-theory` provides the equilibrium analysis. The prisoner's dilemma variant with ternary strategies models the fundamental tension: cooperate (share resources) vs. defect (hoard resources) vs. abstain (don't participate).

## Potential in Mature Systems
In room-as-codespace, every multi-agent interaction is a game. Nash equilibrium predicts stable operating points. Shapley values ensure fair resource allocation. Mechanism design creates incentive-compatible room rules — agents benefit from honest behavior. The ternary strategy space {-1, 0, +1} naturally captures decrease/maintain/increase decisions.

## Cross-Pollination Ideas
- Shapley values for attributing fleet performance to individual rooms
- Prisoner's dilemma as the model for room cooperation — when should rooms share ensigns?
- Mechanism design for room admission control — design rules that make agents self-select into appropriate rooms

## Dependencies for Next Steps
- Integration with ternary-voting for strategic voting analysis
- Integration with ternary-econ for game-theoretic market analysis
- ternary-room needs game-theoretic resource allocation
