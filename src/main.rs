use chrono::NaiveDateTime;
use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

const SCORE_FILE: &str = ".git/gamification_scores.txt";
const BADGE_FILE: &str = ".git/gamification_badges.txt";

struct Commit {
    author: String,
    date: NaiveDateTime,
    message: String,
    lines_added: u32,
    lines_deleted: u32,
    files_changed: u32,
}

struct BadgeCondition {
    name: &'static str,
    condition: fn(&str, u32, &[Commit]) -> bool,
}

fn main() {
    match run() {
        Ok(()) => println!("🎉 Hooray! All set and ready to go! 🎉"),
        Err(e) => eprintln!("🚨 Oops! Something went wrong: {}", e),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let all_commits = get_all_commits()?;
    let user = get_git_user()?;

    let scores = calculate_scores(&all_commits);
    save_scores(&scores)?;

    let user_score = *scores.get(&user).unwrap_or(&0);
    println!(
        "⭐ Hey {}! Your current score is {}. Keep rocking! ⭐",
        user, user_score
    );

    check_badges(&user, user_score, &all_commits)?;
    display_leaderboard(&scores);

    Ok(())
}

fn get_all_commits() -> Result<Vec<Commit>, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&[
            "log",
            "--pretty=format:%H%n%an%n%ad%n%s",
            "--date=iso",
            "--numstat",
        ])
        .output()
        .map_err(|e| format!("Failed to get git log: {}", e))?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut commits = Vec::new();
    let mut lines = output_str.lines();

    while let (Some(_hash), Some(author), Some(date), Some(message)) =
        (lines.next(), lines.next(), lines.next(), lines.next())
    {
        let mut lines_added = 0;
        let mut lines_deleted = 0;
        let mut files_changed = 0;

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                lines_added += parts[0].parse::<u32>().unwrap_or(0);
                lines_deleted += parts[1].parse::<u32>().unwrap_or(0);
                files_changed += 1;
            }
        }

        let date_parsed = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S %z")
            .map_err(|e| format!("Failed to parse date '{}': {}", date, e))?;

        commits.push(Commit {
            author: author.to_string(),
            date: date_parsed,
            message: message.to_string(),
            lines_added,
            lines_deleted,
            files_changed,
        });
    }

    Ok(commits)
}

fn calculate_scores(commits: &[Commit]) -> HashMap<String, u32> {
    let mut scores = HashMap::new();

    for commit in commits {
        let points = calculate_points(
            commit.lines_added,
            commit.lines_deleted,
            commit.files_changed,
        );
        *scores.entry(commit.author.clone()).or_insert(0) += points;
    }

    scores
}

fn calculate_points(lines_added: u32, lines_deleted: u32, files_changed: u32) -> u32 {
    lines_added + lines_deleted / 2 + files_changed * 5
}

fn save_scores(scores: &HashMap<String, u32>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(SCORE_FILE)
        .map_err(|e| format!("Failed to open score file {}: {}", SCORE_FILE, e))?;

    for (user, score) in scores {
        writeln!(file, "{}:{}", user, score)
            .map_err(|e| format!("Failed to write score: {}", e))?;
    }

    Ok(())
}

fn check_badges(
    user: &str,
    score: u32,
    commits: &[Commit],
) -> Result<(), Box<dyn std::error::Error>> {
    let badge_conditions = vec![
        BadgeCondition {
            name: "First Commit",
            condition: |user, _, commits| commits.iter().filter(|c| c.author == user).count() == 1,
        },
        BadgeCondition {
            name: "First Merge",
            condition: |user, _, commits| {
                commits
                    .iter()
                    .any(|c| c.author == user && c.message.to_lowercase().contains("merge"))
            },
        },
        BadgeCondition {
            name: "First Revert",
            condition: |user, _, commits| {
                commits
                    .iter()
                    .any(|c| c.author == user && c.message.to_lowercase().contains("revert"))
            },
        },
        BadgeCondition {
            name: "Commit Streak",
            condition: |user, _, commits| {
                let user_commits: Vec<_> = commits.iter().filter(|c| c.author == user).collect();
                let mut streak = 1;
                let mut max_streak = 1;
                for window in user_commits.windows(2) {
                    if (window[0].date.date() - window[1].date.date()).num_days() == 1 {
                        streak += 1;
                        max_streak = max_streak.max(streak);
                    } else {
                        streak = 1;
                    }
                }
                max_streak >= 7
            },
        },
        BadgeCondition {
            name: "Code Reviewer",
            condition: |user, _, commits| {
                commits.iter().any(|c| {
                    c.author == user && c.message.to_lowercase().contains("merge pull request")
                })
            },
        },
        BadgeCondition {
            name: "Bug Squasher",
            condition: |user, _, commits| {
                commits.iter().any(|c| {
                    c.author == user
                        && (c.message.to_lowercase().contains("fix")
                            || c.message.to_lowercase().contains("bug"))
                })
            },
        },
        BadgeCondition {
            name: "Feature Implementer",
            condition: |user, _, commits| {
                commits.iter().any(|c| {
                    c.author == user
                        && (c.message.to_lowercase().contains("feature")
                            || c.message.to_lowercase().contains("add"))
                })
            },
        },
        BadgeCondition {
            name: "Documentation Writer",
            condition: |user, _, commits| {
                commits
                    .iter()
                    .any(|c| c.author == user && c.message.to_lowercase().contains("doc"))
            },
        },
        BadgeCondition {
            name: "100 Commits",
            condition: |user, _, commits| {
                commits.iter().filter(|c| c.author == user).count() >= 100
            },
        },
        BadgeCondition {
            name: "500 Commits",
            condition: |user, _, commits| {
                commits.iter().filter(|c| c.author == user).count() >= 500
            },
        },
        BadgeCondition {
            name: "Refactor Master",
            condition: |user, _, commits| {
                commits.iter().any(|c| {
                    c.author == user
                        && (c.message.to_lowercase().contains("refactor")
                            || c.message.to_lowercase().contains("clean up"))
                })
            },
        },
        BadgeCondition {
            name: "1000 Points",
            condition: |_, score, _| score >= 1000,
        },
        BadgeCondition {
            name: "5000 Points",
            condition: |_, score, _| score >= 5000,
        },
    ];

    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(BADGE_FILE)
        .map_err(|e| format!("Failed to open badge file {}: {}", BADGE_FILE, e))?;

    let reader = BufReader::new(
        file.try_clone()
            .map_err(|e| format!("Failed to clone file handle: {}", e))?,
    );
    let existing_badges: HashSet<String> = reader.lines().filter_map(Result::ok).collect();

    for badge in badge_conditions {
        let badge_key = format!("{}:{}", user, badge.name);
        if !existing_badges.contains(&badge_key) && (badge.condition)(user, score, commits) {
            writeln!(file, "{}", badge_key).map_err(|e| format!("Failed to write badge: {}", e))?;
            println!(
                "🏆 Congratulations! You've earned the '{}' badge! 🏆",
                badge.name
            );
        }
    }

    Ok(())
}

fn display_leaderboard(scores: &HashMap<String, u32>) {
    println!("📊 Leaderboard (Top 10): 📊");
    let mut sorted_scores: Vec<_> = scores.iter().collect();
    sorted_scores.sort_by(|a, b| b.1.cmp(a.1));

    for (i, (user, score)) in sorted_scores.iter().take(10).enumerate() {
        println!("{}. {}: {} points", i + 1, user, score);
    }
}

fn get_git_user() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["config", "user.name"])
        .output()
        .map_err(|e| format!("Failed to get git user: {}", e))?;

    let user_name = String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to parse git user name: {}", e))?
        .trim()
        .to_string();

    if user_name.is_empty() {
        return Err("Git user name is empty".into());
    }

    Ok(user_name)
}
