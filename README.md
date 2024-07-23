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

3. Create a script in your project's Git hooks directory to call `git-gamify` along with other pre-commit hooks:

   1. Ensure you have an existing pre-commit hook or create a placeholder if needed:

      ```
      touch /path/to/your/project/.git/hooks/pre-commit
      chmod +x /path/to/your/project/.git/hooks/pre-commit
      ```

   2. Copy the compiled `git-gamify` binary to a location accessible by the script. For example, copying it to the hooks directory:

      ```
      cp target/release/git-gamify /path/to/your/project/.git/hooks/git-gamify
      ```

   3. Update the `pre-commit` script to call `git-gamify`:

      ```sh
      #!/bin/sh
      # Your existing pre-commit hook commands here

      # Run git-gamify
      /path/to/your/project/.git/hooks/git-gamify

      # Add more hooks if needed
      ```

   4. Run `git commit` to trigger the pre-commit hook and start using Git Gamify!

## How It Works

Git Gamify runs automatically after each commit. It analyzes your entire Git history, calculates your score, and checks for any new badges you've earned.

1. **Collecting Commits**:

   - `git log --pretty=format:%H%n%an%n%ad%n%s --date=iso --numstat` is executed to retrieve commit details.
   - The output is parsed to gather information about each commit, such as the author, date, message, lines added, lines deleted, and files changed.

2. **Retrieving Git Username**:
   - `git config user.name` is executed to get the name of the user making the commits. This is used to attribute scores and badges to the correct user.

By using these Git commands, Git Gamify can gather all necessary data to track contributions, calculate scores, and award badges based on the entire history of the repository.

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

After each commit, Git Gamify will display a leaderboard showing the top 10 contributors based on their total points.

## Contributing

All contibutions are welcome, please feel free to submit issues, fork the repository and send pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
