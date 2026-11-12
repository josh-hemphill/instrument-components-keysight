---
name: council
description: Use when the user requests the Council workflow, a Council review, or iterative grading by multiple reviewers. Convene at least three independent sub-agents of the latest available Sol model at high reasoning effort, approve a rubric by majority vote, review and score the work, implement improvements, and repeat until every category meets its requested grade.
---

# Council

Act as the facilitator and implementer. The Council consists of independent reviewing sub-agents; the facilitator does not cast a vote. Invoking this skill authorizes the sub-agent reviews and improvement loop within the user's task scope.

## 1. Establish the review contract

Identify the work, its intended audience and purpose, the requested categories, and the desired grade in each category. Reuse the user's supplied rubric. If categories are missing, propose categories suited to the work. If targets are missing, propose **at least 9.2/10 in every category**. Proceed with these reasonable defaults and let the Council assess the rubric; do not introduce a routine user approval step.

Preserve the user's exact thresholds and operators. `> 9.1` excludes 9.1; `>= 9.1` includes it. Do not round a failing score into a pass. The default scoring precision is one decimal place. A high overall average cannot compensate for a failing category.

Create a rubric with:

- A version, scope, artifact identity, and evidence reviewers must inspect.
- A stable ID, definition, target, and comparison operator for each category.
- Concrete scoring anchors, including what meeting the target looks like.
- Evaluation methods and required coverage, such as viewports, interaction states, tests, or source sections.
- Any mandatory acceptance conditions explicitly required by the user or task.
- The Council size, model, reasoning effort, voting rule, and aggregation rule.

Use [review templates and UI example](references/review-templates.md) as a starting point. Adapt the example to the actual work; it is not a compulsory universal rubric.

## 2. Convene the Council

Use **three members by default**, or five or more when additional breadth is useful. Prefer an odd count; honor an explicitly requested size of at least three. All members review every category, even if given different areas of emphasis.

Resolve the **latest available Sol model** from the runtime's supported model list at invocation time. In a runtime listing `gpt-6.1-sol` as the latest Sol, select `gpt-6.1-sol`; do not use an older Sol merely because it appears in an example. Set reasoning effort explicitly to **high** for every Council member. Do not silently substitute another model family or effort level. If the required model, effort, or sub-agent capability is unavailable, report the limitation and do not claim a Council result.

When using `collaboration.spawn_agent`, select `fork_turns: "none"` so model and reasoning overrides are supported and independent reviews do not inherit other reviewers' opinions. Supply the same complete task, artifact access, rubric, and necessary constraints to every member, plus **one short, distinct, neutral framing sentence per member** to encourage different interpretations. Use the [neutral prompt variations](references/review-templates.md); keep the extra text minimal. Every member still evaluates every category using the same standards. Do not suggest the work is good, imply a desired verdict, assign lenient or harsh personas, or praise the author. Record each framing and keep it stable across rounds. Set `model` to the resolved Sol identifier and `reasoning_effort: "high"`. Choose distinct member task names. Start independent reviews concurrently when supported; avoid conflicting writes.

Keep the membership and model fixed during a review cycle. Replace a member only for an actual availability or execution failure, recording why. Never replace a dissenting member or add reviewers to manufacture a passing majority. If the model changes, rerun the whole review with the new configuration.

Members inspect and report; the facilitator implements changes. Members must not edit shared work during review. The same frozen artifact and rubric version must be available to every reviewer. If concurrent edits change the artifact, invalidate affected reviews and review a new frozen revision.

## 3. Let the Council approve or amend the rubric

Before grading, send the proposed rubric independently to every member. Explicitly invite each member to **request changes to categories, definitions, anchors, coverage, or evaluation methods**, or approve the rubric as written. Require a clear `APPROVE` or `REQUEST_CHANGES` ballot with reasons and concrete amendments.

The quorum is the full configured Council: obtain all N valid ballots. A majority is **floor(N/2) + 1** affirmative votes. Missing or invalid responses are not approvals; retry or replace unavailable members transparently. Do not shrink N to pass a vote.

Summarize all requested amendments, including minority concerns. Incorporate useful clarifications consistent with the user's instructions, then submit the complete revised rubric to all members for another independent ballot. Repeat until a single rubric version has a majority of approvals. A majority may approve despite minority objections; retain the dissent and explain the disposition.

The Council cannot lower user targets, remove user-required categories, or reduce scope to make passing easier. Resolve ambiguity from context where possible; ask the user only when a consequential conflict cannot be resolved within their instructions. A request to change the rubric is not itself a request for user permission.

Freeze the approved rubric. Later amendments require another Council rubric ballot and regrading of every category under the new version. Do not combine grades from different rubric versions.

## 4. Request independent, detailed reviews

Give each member the approved rubric and access to the same current artifact revision. Each member must inspect actual evidence. A UI review should examine the rendered UI and relevant interactions, viewports, and states where tools allow; code inspection alone does not establish visual quality or usability. If required evidence is unavailable, report that limitation instead of fabricating a grade.

Require each member to return:

- Member identity, actual model/effort, artifact revision, and rubric version.
- A numeric score from 0 to 10 for **every category**, using the agreed precision.
- Evidence supporting each score, concrete strengths and deficiencies, and confidence or limitations.
- A pass/fail vote for each category derived from its exact threshold.
- Prioritized, actionable changes with locations or examples and an explanation of which category each would improve.
- Any unmet mandatory acceptance conditions, unresolved disagreements, and proposed rubric amendments if a flaw became apparent.

Require reviewers to distinguish observations from preferences and uncertainty. They should judge honestly, without negotiating grades or being instructed to converge. An unverifiable category is `UNVERIFIED`, never an assumed passing score.

For every grading round, use fresh independent reviewer contexts where possible. Returning members may be used if they assess the work anew. Do not show members one another's ballots, the aggregate score, or a request to award a passing grade before their independent submissions. Provide factual changes and prior issues only when needed to verify fixes; do not anchor reviewers with previous scores. Collect all reviews before synthesis. Permit one evidence-based clarification round if reviewers made factual mistakes, preserving original and revised ballots.

## 5. Aggregate scores and votes

Validate every response against the same revision and rubric. Request missing categories or evidence before accepting a ballot. Require all configured members to submit valid reviews; otherwise the round remains incomplete.

For each category:

1. List all member scores and count threshold-derived pass votes.
2. Use the **median** as the Council grade for an odd-sized Council.
3. For an even-sized Council, use the **lower middle score** so that passing still requires a strict majority. Do not average the middle pair.
4. Mark the category `PASS` only if its Council grade satisfies the exact threshold and at least floor(N/2) + 1 members vote pass. A required `UNVERIFIED` assessment makes that category incomplete pending evidence, rather than numerically aggregating it.

Report the score range and minority findings alongside the majority result. Majority agreement among models is a judgment, not proof of objective correctness. Validate factual defects and mandatory conditions using direct evidence; a popularity vote cannot override a demonstrated failure of a required condition.

The work passes only when **every category passes in the same complete round on the current artifact** and all mandatory acceptance conditions are satisfied. Never use an overall average or carry forward an earlier category pass as the final result.

## 6. Implement improvements and repeat

If any category fails, synthesize the findings into a prioritized change list. Address specific deficiencies and material minority findings, checking opinions against the user's goals. Resolve conflicting suggestions with evidence and the approved rubric. The facilitator makes the authorized changes, performs appropriate verification, and records the new artifact revision.

Submit the updated work to the full Council for another detailed review under the same approved rubric. Review **all categories** each round to catch regressions. Continue the review → improve → verify → review loop until all requested grades are met. Do not end after a single failing review or introduce an arbitrary round cap that the user did not request.

If reviews stall or oscillate, diagnose the cause and change the implementation strategy or obtain missing evidence. Do not repeatedly resample unchanged work, soften anchors, lower targets, discard low scores, cherry-pick reviewers, or pressure members to raise grades. Substantive rubric defects go through the amendment process, preserving the user's goals.

Stop before meeting the targets only for a real blocker, such as unavailable required evidence, exhausted runtime resources, an irreconcilable requirement, or a user instruction to stop. Clearly label the result **incomplete**, report outstanding grades and the concrete blocker, and retain enough state to resume. Never claim the requested outcome was reached when it was not.

## 7. Keep a review record and report the result

Use the project's established artifact location and format, or a task-specific scratch location. Keep the approved rubric and ballots, model configuration, revision identifiers, individual reviews, aggregate tables, dissent, and changes per round. Reuse this record when resuming; do not restart grading from memory or treat stale grades as current.

Keep progress updates concise: what passed, what remains below target, and which changes come next. The final report should link the reviewed artifact and review record and show a compact table of category, target, individual scores, Council grade, pass votes, and status. State the round, rubric version, artifact revision, and actual model/effort. Mention material dissent and any remaining limitations. Claim completion only for a verified passing final round.
