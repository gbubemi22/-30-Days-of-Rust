# Day 30: Final Project and Next Steps — sample review

The exact design choices may differ, but the included [task manager](../projects/cli-task-manager/) provides one complete implementation to study.

- A `Vec<Task>` preserves the order tasks were added.
- `Task` owns its description and completion state.
- `Result` reports I/O, JSON, and command-input errors to the caller.
- Serde derives convert task values to and from JSON.
- Tests exercise the command handler and persistence behavior without invoking a terminal process.
- The binary handles environment configuration, output streams, and process exit status.

For an extension, start by describing its expected behavior, implement it in a small step, add tests for success and failure cases, then update the README.
