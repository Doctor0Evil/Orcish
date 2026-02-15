use core_contract::hit_guard::{
    HitGovernanceObject, HitContext, RawResponse, GuardOutcome, DefaultHitResponseGuard,
};
use core_contract::human_shell::{
    HumanShell, DefaultHumanShell, SovereigntyState, DisciplineSignal, EvolutionChoice,
    FateCard, FateDeck,
};

pub struct OrchestrationResult {
    pub final_text: String,
    pub sovereignty_after: SovereigntyState,
    pub evolution_choice: Option<EvolutionChoice>,
}

/// Core orchestrator: applies HIT guard + human_shell + fate_cards.
///
/// - `generate` is your neuromorphic/chat engine.
/// - `fate_deck` is your personal evolution-choice space.
pub fn run_with_hgo_and_human_shell<F>(
    hgo: &HitGovernanceObject,
    ctx: &HitContext,
    sovereignty_before: &SovereigntyState,
    signals: &[DisciplineSignal],
    fate_deck: &FateDeck,
    generate: F,
) -> OrchestrationResult
where
    F: Fn(&HitContext) -> RawResponse,
{
    // 1. Generate candidate response
    let candidate = generate(ctx);

    // 2. Apply HIT response guard
    let guard = DefaultHitResponseGuard;
    let outcome = guard.evaluate(hgo, ctx, &candidate);

    let human_shell = DefaultHumanShell;

    // 3. Choose text based on guard outcome (no rollback here)
    let approved_text = match outcome {
        GuardOutcome::ApprovedLabeled { content } => content,
        GuardOutcome::RequiresHumanReview { content, reason } => {
            // In a real system, you'd send this to a human reviewer.
            // Here, we mark it clearly but do NOT auto-downgrade anything.
            format!("(REVIEW REQUIRED: {reason})\n{content}")
        }
        GuardOutcome::Blocked { reason } => {
            format!("(BLOCKED BY HIT GUARD: {reason})")
        }
    };

    // 4. Summarize discipline signals for humans
    let summary = human_shell.summarize_for_human(sovereignty_before, signals);

    // 5. Use fate_cards to pick an evolution path (purely voluntary)
    let evolution_choice = fate_deck.pick_choice(signals);

    // 6. Compute evolution score and guaranteed non-rollback sovereignty
    let (sovereignty_after, evo_score) =
        human_shell.compute_evolution_score(sovereignty_before, signals);

    let final_text = format!(
        "{}\n\n---\nHUMAN-SUMMARY:\n{}\n\nEVOLUTION:\nPoints: {}\nHint: {}\nChoice: {:?}",
        approved_text,
        summary.text,
        evo_score.points,
        evo_score.health_benefit_hint,
        evolution_choice,
    );

    OrchestrationResult {
        final_text,
        sovereignty_after,
        evolution_choice,
    }
}
