# Day 29: Build a CLI Task Manager — solutions

The [project implementation](../projects/cli-task-manager/) is a complete sample for today's exercises.

## 1. Remove a task

Parse and validate a one-based task number, reject zero, remove the item with `Vec::remove(number - 1)`, save, and report the result. Check the index before removing so invalid input does not panic.

## 2. Reopen a completed task

Add a `reopen NUMBER` branch that looks up the task with `get_mut`, sets `complete` to `false`, and persists the updated list.

## 3. Add a due date

Add a serializable field such as `due_date: Option<String>`. Update task construction, JSON expectations, and tests. A dedicated date crate may be useful if validating calendar dates is required.

## 4. Test invalid input

Call `run` with an unknown command and with `done 0`; assert that each returns `io::ErrorKind::InvalidInput`.

## 5. Why accept a writer?

Passing a writer lets tests capture and inspect command output without launching a process or redirecting the real terminal. It also keeps command behavior separate from process setup.
