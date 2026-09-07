# LLMism Linter

## Context

We are building a custom writing linter in Rust for flagging LLMisms. We will use the following stock phrases below to flag in this linter. 
Our linter will read Markdown and Text files. 

It will be a CLI that uses clap. It can ingest a path or a single file. 

## Usage

`llmisms <path>`

## LLMisms

**Llmisms**: stock phrases, sentence patterns, and conversational habits that make prose sound preassembled. None is exclusive to LLMs; the effect comes from how frequently—and predictably—they appear.

## Stock phrases

1. **The significance announcement**
   “That distinction matters.” “And that’s important.” “This is a crucial point.”
   Announces that something is significant, sometimes instead of explaining its consequences.

2. **The unpacking ritual**
   “Let’s unpack that.” “Let’s break this down.” “Let’s take a step back.”
   Routine stage directions before an explanation that could simply begin.

3. **The depth teaser**
   “There’s a deeper issue here.” “This raises a fundamental question.” “Here’s where things get interesting.”
   Promises an intellectual revelation, regardless of what follows.

4. **The compulsory caveat**
   “It’s important to note…” “It’s worth emphasizing…” “One crucial caveat…”
   Attaches a warning label to information that often needs no introduction.

5. **The conspicuous declaration of honesty**
   “The honest answer is…” “Here’s the uncomfortable truth…” “Let me be blunt…”
   Advertises candor as though the preceding sentences operated under different rules.

6. **The precision ceremony**
   “To be precise…” “More precisely…” “We need to distinguish between…”
   Particularly noticeable when the supposedly more precise version just substitutes different abstractions.

7. **The manufactured bottom line**
   “At its core…” “Ultimately…” “The key takeaway is…” “What this really means is…”
   Repeated attempts to identify the essence of something already adequately explained.

## Sentence templates and cadences

8. **The “not X, but Y” reversal**
   “It’s not about working harder. It’s about working smarter.”
   Turns an ordinary observation into a correction of an assumed misconception.

9. **The “not just X—it’s Y” escalation**
   “This isn’t just a technical problem—it’s a human one.”
   Inflates the scope or emotional importance of the subject.

10. **The replacement question**
    “The real question isn’t whether it works. It’s who it works for.”
    Quietly substitutes a different question for the one actually asked.

11. **The interpretive takeover**
    “You’re not asking about efficiency. You’re asking about meaning.”
    Tells the user what they are *really* asking, sometimes against their explicit wording.

12. **The balanced duality**
    “Both things can be true.” “These aren’t mutually exclusive.” “It’s not either/or.”
    Supplies reconciliation even when the task is to determine which competing claim is correct.

13. **The three-part cadence**
    “Clear, concise, and actionable.” “Thoughtful, intentional, and authentic.”
    Three pleasantly compatible adjectives, often doing the work of one.

14. **The staccato manifesto**
    “No fluff. No distractions. Just results.”
    Short fragments arranged to resemble conviction.

15. **The question-fragment reveal**
    “The problem? Complexity.” “The result? A better experience.” “The catch? Everything.”
    A self-interview used to manufacture momentum.

16. **The isolated profundity line**
    “That’s the shift.”
    “That’s the work.”
    “And that changes everything.”
    An ordinary conclusion gets its own paragraph and the cadence of a revelation.

17. **The symmetrical pseudo-aphorism**
    “Clarity isn’t the absence of complexity. It’s the ability to navigate it.”
    Two balanced clauses that sound quotable before anyone checks whether they say much.

## Conversational mannerisms

18. **The unearned gold star**
    “That’s an excellent question.” “You’ve put your finger on something crucial.” “You’re asking exactly the right question.”
    Praise delivered before—or instead of—substantive engagement.

19. **The flattering reinterpretation**
    “You’re not overthinking it. You’re noticing something others miss.”
    Converts uncertainty, frustration, or disagreement into evidence of exceptional insight.

20. **The agreement-then-retreat maneuver**
    “You’re absolutely right. However…” “Exactly—with one important qualification.”
    Starts with stronger agreement than the remainder of the answer supports.

21. **The standardized correction apology**
    “You’re right—I overstated that.” “I should have been clearer.” “Let me correct the framing.”
    Becomes an llmism when the admission is formulaic and the underlying mistake survives the correction.

22. **The unsolicited therapist voice**
    “That sounds incredibly difficult.” “Your feelings are valid.” “And that’s okay.”
    Applies emotional-support language to requests for explanation, criticism, or practical action.

23. **The personalized flattery wrapper**
    “Given your analytical mindset…” “For someone who thinks as deeply as you do…”
    Makes a generic answer appear personally tailored through compliments.

24. **The evasive balance paragraph**
    “It depends on the context.” “There’s no one-size-fits-all answer.” “Different perspectives offer valuable insights.”
    Names the existence of complexity without identifying the variables that would resolve the question.

25. **The answer about the answer**
    “Here’s a clear, practical breakdown.” “I’ll keep this concise and actionable.”
    Describes the intended quality of the response instead of demonstrating it.

26. **The perpetual follow-up offer**
    “Would you like me to turn this into a checklist?” “Would a side-by-side comparison help?”
    Treats every completed answer as the opening of another workflow.

## Vocabulary and presentation habits

27. **The abstract-noun fog**
    “Nuance,” “complexity,” “dynamics,” “framework,” “landscape,” “interplay.”
    Most conspicuous when these words replace concrete actors, actions, or mechanisms.

28. **The corporate adjective drawer**
    “Robust,” “seamless,” “comprehensive,” “transformative,” “actionable,” “innovative.”
    Positive-sounding attributes offered without specifying what makes the thing good.

29. **The ornate verb drawer**
    “Delve,” “navigate,” “leverage,” “foster,” “underscore,” “illuminate.”
    Elevated or managerial substitutes for “examine,” “use,” “help,” and “show.”

30. **The grand metaphor bundle**
    “A rich tapestry…” “An intricate dance…” “A symphony of…” “A testament to…”
    Decorative imagery that could be transferred to dozens of unrelated subjects unchanged.

31. **The em-dash hinge**
    “The system works—but not in the way you think.”
    Repeated em dashes used to deliver qualifications, reversals, and miniature dramatic reveals.

32. **The compulsive outline**
    An introduction, five headings, three bullets per heading, a summary, and a closing offer—for a question that needed two paragraphs.

33. **The boldface guidance system**
    Every paragraph contains a **key distinction**, **central insight**, or **important takeaway**, as though the reader needs continuous instructions about where to look.

34. **The excessive recap**
    The introduction previews the answer, the body gives the answer, and the conclusion paraphrases the answer—without adding anything.

35. **The suspiciously tidy symmetry**
    Every position gets three strengths and three weaknesses. Every tradeoff has two equally weighted sides. The layout supplies balance that the underlying evidence may not justify.

### A concentrated specimen

> You’re absolutely right to flag this. Let’s unpack it, because that distinction matters. This isn’t just about word choice—it’s about trust. The real question isn’t whether the writing is polished, but whether it serves the reader. Both things can be true: structure helps, and too much structure gets in the way. The goal? Clear, authentic, intentional communication. No fluff. No noise. Just meaning.

