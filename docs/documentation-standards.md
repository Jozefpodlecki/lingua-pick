# Documentation standards

## Where information belongs

- `README.md`: project overview, prerequisites, setup, and common run/build commands.
- `AGENTS.md`: concise instructions for coding agents, established project constraints, and links to required documentation.
- `docs/README.md`: an index of project documents.
- `docs/`: detailed product requirements, architecture, contributor standards, and significant decisions.
- Rust documentation comments: public API contracts and examples. Inline comments: local reasoning that is not obvious from the code.

Keep one authoritative description of each detailed rule or decision. Link to it from other documents instead of maintaining duplicate explanations.

## Writing and accuracy

- Write Markdown with descriptive headings and relative links to repository files.
- Use plain language, concrete examples where useful, and fenced code blocks with a language label.
- Distinguish implemented behavior, agreed requirements, proposals, and open questions. Never describe planned behavior as already working.
- Check paths, commands, and dependency details against the repository. State the working directory for commands and relevant platform assumptions.
- Keep secrets, credentials, and personal data out of documentation and examples.
- Record an image or content asset's source and license when adding externally sourced material.

## Keeping documents current

- Update affected documentation in the same change as behavior, setup, storage formats, or architecture changes.
- Add new documents to `docs/README.md`. Add required contributor reading to `AGENTS.md` when appropriate.
- Keep task progress and temporary investigation notes out of permanent standards.
- For a significant architecture decision, add a short record under `docs/decisions/` using a name such as `001-language-content-layout.md`. Include its status, context, decision, and consequences. Clearly mark proposals until a decision is established.
- Check that new links resolve and review documentation changes for accuracy. Documentation-only changes do not require an application build unless they also change executable configuration or code.
