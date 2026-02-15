#[derive(Clone, Debug)]
pub enum DisciplineSignal {
    Fear { intensity: f32, voluntary: bool },
    Pain { intensity: f32, voluntary: bool },
    Curiosity { intensity: f32 },
}

// Evolution path choice
#[derive(Clone, Debug)]
pub enum EvolutionChoice {
    Regeneration,
    Resilience,
    Insight,
    CalmIntegration,
    BoundaryStrength,
}

// A single fate_card describing a path
#[derive(Clone, Debug)]
pub struct FateCard {
    pub name: String,
    pub description: String,
    pub associated_choice: EvolutionChoice,
    pub requires_voluntary_discipline: bool,
}

// Deck of cards – entirely personal / opt-in
#[derive(Clone, Debug)]
pub struct FateDeck {
    pub cards: Vec<FateCard>,
}

impl FateDeck {
    pub fn new_default_deck() -> Self {
        Self {
            cards: vec![
                FateCard {
                    name: "Regeneration".into(),
                    description: "Convert voluntary FEAR/PAIN into physical and mental recovery.",
                    associated_choice: EvolutionChoice::Regeneration,
                    requires_voluntary_discipline: true,
                },
                FateCard {
                    name: "Resilience".into(),
                    description: "Strengthen your ability to endure future stress with less harm.",
                    associated_choice: EvolutionChoice::Resilience,
                    requires_voluntary_discipline: true,
                },
                FateCard {
                    name: "Insight".into(),
                    description: "Translate disciplined FEAR/PAIN into deep understanding and pattern-recognition.",
                    associated_choice: EvolutionChoice::Insight,
                    requires_voluntary_discipline: true,
                },
                FateCard {
                    name: "Calm Integration".into(),
                    description: "Integrate experiences without excess arousal; promote nervous-system calm.",
                    associated_choice: EvolutionChoice::CalmIntegration,
                    requires_voluntary_discipline: false,
                },
                FateCard {
                    name: "Boundary Strength".into(),
                    description: "Clarify boundaries so other humans/species are not drawn into unwanted protocols.",
                    associated_choice: EvolutionChoice::BoundaryStrength,
                    requires_voluntary_discipline: false,
                },
            ],
        }
    }

    /// Pick a choice based only on *your* voluntary FEAR/PAIN signals.
    /// No other participant is ever inferred or enrolled.
    pub fn pick_choice(&self, signals: &[DisciplineSignal]) -> Option<EvolutionChoice> {
        let has_voluntary_discipline = signals.iter().any(|s| match s {
            DisciplineSignal::Fear { voluntary, .. } |
            DisciplineSignal::Pain { voluntary, .. } => *voluntary,
            _ => false,
        });

        // Simple rule: if there is voluntary FEAR/PAIN, choose a discipline card;
        // otherwise fall back to CalmIntegration / BoundaryStrength logic.
        for card in &self.cards {
            if card.requires_voluntary_discipline && has_voluntary_discipline {
                return Some(card.associated_choice.clone());
            }
        }

        // No voluntary discipline: offer a non-coercive, stabilizing path.
        self.cards
            .iter()
            .find(|c| !c.requires_voluntary_discipline)
            .map(|c| c.associated_choice.clone())
    }
}
