# Task
A simple CLI task manager application written in Rust. It lets you add, list, edit, and finish tasks with optional due dates.

---
## Installation
1. Clone the repository:
	`git clone https://github.com/Nqtural/task`
2. Change directory into the build directory:
	`cd task`
3. Build the executable:
	`cargo build --release`
4. The executable will end up at `./target/release/task`. It can be moved anywhere.

---
## Projects
Projects allow you to maintain separate task lists tied to directories. A project is automatically associated with the directory in which it is created. When you run task commands inside a directory (or any of its subdirectories) that belongs to a project, that project becomes the default context.

To reference a specific project by name, you may use the project’s directory name. A special project named global always exists and stores tasks not tied to any directory.

The application searches upward from the current directory to determine whether you are inside project.

---
## Usage
| Command                                                                | Explanation                                                                                                                                                                                                                                                                                                                                |
| ---------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `task project new`                                                     | Creates a new project in the current working directory.                                                                                                                                                                                                                                                                                    |
| `task project list`                                                    | Prints out all projects.                                                                                                                                                                                                                                                                                                                   |
| `task project delete [PROJECT`                                         | Deletes the given project. If no project is specified, deletes the project assiciated with the current working directory.                                                                                                                                                                                                                  |
| `task list [PROJECT]`                                                  | Prints out all tasks.                                                                                                                                                                                                                                                                                                                      |
| `task add <NAME> [--time <EXPIRATION TIME>] [PROJECT]`                 | The expiration time can be formatted in a couple different ways. Relative time can be specified as  `2w4d` (2 weeks and 4 days from now). Absolute date can be specified as `120925` or just `1209` for 12 September 2025. Time can be specified by just `16:15` for using the current day, or added onto absolute date with `1209-16:15`. |
| `task delete <ID> [--no-confirm] [PROJECT]`                            | Deletes task with a confirmation prompt, unless `--no-confirm`.                                                                                                                                                                                                                                                                            |
| `task edit <ID> {--name <NAME> \| --time <EXPIRATION TIME>} [PROJECT]` | Same time format as for `task add`.                                                                                                                                                                                                                                                                                                        |
| `task finish <ID> [PROJECT]`                                           | Toggles finish status of a task.                                                                                                                                                                                                                                                                                                           |
| `task help`                                                            | Prints out help message.                                                                                                                                                                                                                                                                                                                   |

---
## Contribution
### Disclaimer
There's 0 guarantee there will be any more development on this project. However, this is a perfect opportunity for a first time open source contributor because the stakes are nonexistent. If you do want to try to contribute, instructions are listed below.

### Issues
If you find a bug, have a feature request, or notice something that can be improved, you can open an [issue](https://github.com/Nqtural/task/issues).

When creating an issue:
- **Be descriptive:** Explain what the bug or feature is, and why it matters.
- **Steps to reproduce (for bugs):** If applicable, include clear steps so others can reproduce the issue.
- **Environment details:** Mention your operating system, Rust version, and anything else that might be relevant.
- **Labels:** Maintainers may assign labels (e.g., `bug`, `enhancement`, `good first issue`) to help categorize the issue.

This project is especially beginner-friendly, so even small suggestions or questions are welcome.

### Pull requests
Pull requests (PRs) are welcome! A PR is the way to propose changes to the codebase.

**Before opening a PR:**
1. **Check existing issues:** See if there’s already a discussion about your change.
2. **Link issues:** If your PR fixes or addresses an issue, mention it in the description (e.g., "My PR title (#12)").
3. **Keep it focused:** Try to make your PR about a single problem or feature. Smaller, focused PRs are easier to review and more likely to be merged quickly.
4. **Follow coding style:** Keep your code consistent with the existing style. Use `cargo clippy` to validate the code before committing.

**PR process:**
1. Fork the repository and create a new branch for your changes.
2. Make your changes, commit them with clear messages, and push to your fork.
3. Open a pull request against the `main` branch of this repository.
4. Be ready to answer questions or make adjustments if requested during review.

Even small contributions like fixing typos, adding comments, or improving documentation are valuable!
