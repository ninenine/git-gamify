# Git Gamify

Git Gamify is a fun and engaging way to gamify your Git workflow. It tracks your contributions, awards points for your commits, and grants badges for various achievements. Boost your motivation and track your progress as you contribute to your projects!

## Features

- Calculates points based on your Git activity
- Awards badges for specific achievements
- Displays a leaderboard of top contributors
- Tracks your entire Git history for comprehensive scoring

## Installation

1. Clone this repository:
   ```
   git clone https://github.com/ninenine/git-gamify.git
   ```

2. Build the project:
   ```
   cd git-gamify
   cargo build --release
   ```

3. Copy the compiled binary to your project's Git hooks directory:
   ```
   cp target/release/git-gamify /path/to/your/project/.git/hooks/post-commit
   ```

4. Make the hook executable:
   ```
   chmod +x /path/to/your/project/.git/hooks/post-commit
   ```

## How It Works

Git Gamify runs automatically after each commit. It analyzes your entire Git history, calculates your score, and checks for any new badges you've earned.

### Earning Points

You earn points for each commit based on the following criteria:

- 1 point for each line added
- 0.5 points for each line deleted
- 5 points for each file changed

### Available Badges

Earn badges by achieving specific milestones:

1. **First Commit**: Make your first commit to the repository.
2. **First Merge**: Complete your first merge.
3. **First Revert**: Perform your first revert operation.
4. **Commit Streak**: Commit code for 7 consecutive days.
5. **Code Reviewer**: Merge a pull request.
6. **Bug Squasher**: Fix a bug (commit message contains "fix" or "bug").
7. **Feature Implementer**: Implement a new feature (commit message contains "feature" or "add").
8. **Documentation Writer**: Update documentation (commit message contains "doc").
9. **100 Commits**: Reach 100 total commits.
10. **500 Commits**: Reach 500 total commits.
11. **Refactor Master**: Refactor code (commit message contains "refactor" or "clean up").
12. **1000 Points**: Accumulate 1000 total points.
13. **5000 Points**: Accumulate 5000 total points.

## Leaderboard

After each commit, Git Gamify will display a leaderboard showing the top 5 contributors based on their total points.

## Contributing

We welcome contributions to Git Gamify! Please feel free to submit issues, fork the repository and send pull requests!

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
