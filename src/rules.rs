pub(crate) struct Rule {
    pub(crate) name: &'static str,
    pub(crate) phrase: &'static str,
}

macro_rules! rules {
    ($(($name:literal, [$($phrase:literal),+ $(,)?])),+ $(,)?) => {
        &[
            $(
                $(Rule { name: $name, phrase: $phrase },)+
            )+
        ]
    };
}

pub(crate) static RULES: &[Rule] = rules![
    (
        "significance announcement",
        [
            "That distinction matters",
            "And that's important",
            "This is a crucial point",
        ]
    ),
    (
        "unpacking ritual",
        [
            "Let's unpack that",
            "Let's break this down",
            "Let's take a step back",
        ]
    ),
    (
        "depth teaser",
        [
            "There's a deeper issue here",
            "This raises a fundamental question",
            "Here's where things get interesting",
        ]
    ),
    (
        "compulsory caveat",
        [
            "It's important to note",
            "It's worth emphasizing",
            "One crucial caveat",
        ]
    ),
    (
        "conspicuous declaration of honesty",
        [
            "The honest answer is",
            "Here's the uncomfortable truth",
            "Let me be blunt",
        ]
    ),
    (
        "precision ceremony",
        [
            "To be precise",
            "More precisely",
            "We need to distinguish between",
        ]
    ),
    (
        "manufactured bottom line",
        [
            "At its core",
            "Ultimately",
            "The key takeaway is",
            "What this really means is",
        ]
    ),
    (
        "not X, but Y reversal",
        ["It's not about working harder. It's about working smarter",]
    ),
    (
        "not just X, it's Y escalation",
        ["This isn't just a technical problem—it's a human one",]
    ),
    (
        "replacement question",
        ["The real question isn't whether it works. It's who it works for",]
    ),
    (
        "interpretive takeover",
        ["You're not asking about efficiency. You're asking about meaning",]
    ),
    (
        "balanced duality",
        [
            "Both things can be true",
            "These aren't mutually exclusive",
            "It's not either/or",
        ]
    ),
    (
        "three-part cadence",
        [
            "Clear, concise, and actionable",
            "Thoughtful, intentional, and authentic",
        ]
    ),
    (
        "staccato manifesto",
        ["No fluff. No distractions. Just results",]
    ),
    (
        "question-fragment reveal",
        [
            "The problem? Complexity",
            "The result? A better experience",
            "The catch? Everything",
        ]
    ),
    (
        "isolated profundity",
        [
            "That's the shift",
            "That's the work",
            "And that changes everything",
        ]
    ),
    (
        "symmetrical pseudo-aphorism",
        ["Clarity isn't the absence of complexity. It's the ability to navigate it",]
    ),
    (
        "unearned gold star",
        [
            "That's an excellent question",
            "You've put your finger on something crucial",
            "You're asking exactly the right question",
        ]
    ),
    (
        "flattering reinterpretation",
        ["You're not overthinking it. You're noticing something others miss",]
    ),
    (
        "agreement-then-retreat maneuver",
        [
            "You're absolutely right. However",
            "Exactly—with one important qualification",
        ]
    ),
    (
        "standardized correction apology",
        [
            "You're right—I overstated that",
            "I should have been clearer",
            "Let me correct the framing",
        ]
    ),
    (
        "unsolicited therapist voice",
        [
            "That sounds incredibly difficult",
            "Your feelings are valid",
            "And that's okay",
        ]
    ),
    (
        "personalized flattery wrapper",
        [
            "Given your analytical mindset",
            "For someone who thinks as deeply as you do",
        ]
    ),
    (
        "evasive balance paragraph",
        [
            "It depends on the context",
            "There's no one-size-fits-all answer",
            "Different perspectives offer valuable insights",
        ]
    ),
    (
        "answer about the answer",
        [
            "Here's a clear, practical breakdown",
            "I'll keep this concise and actionable",
        ]
    ),
    (
        "perpetual follow-up offer",
        [
            "Would you like me to turn this into a checklist",
            "Would a side-by-side comparison help",
        ]
    ),
    (
        "abstract-noun fog",
        [
            "nuance",
            "complexity",
            "dynamics",
            "framework",
            "landscape",
            "interplay",
        ]
    ),
    (
        "corporate adjective drawer",
        [
            "robust",
            "seamless",
            "comprehensive",
            "transformative",
            "actionable",
            "innovative",
        ]
    ),
    (
        "ornate verb drawer",
        [
            "delve",
            "navigate",
            "leverage",
            "foster",
            "underscore",
            "illuminate",
        ]
    ),
    (
        "grand metaphor bundle",
        [
            "A rich tapestry",
            "An intricate dance",
            "A symphony of",
            "A testament to",
        ]
    ),
    (
        "em-dash hinge",
        ["The system works—but not in the way you think",]
    ),
];
