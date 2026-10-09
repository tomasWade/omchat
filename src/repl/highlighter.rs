use super::REPL_COMMANDS;

use crate::{config::GlobalConfig, utils::NO_COLOR};

use nu_ansi_term::{Color, Style};
use reedline::{Highlighter, StyledText};

const DEFAULT_COLOR: Color = Color::Default;
const MATCH_COLOR: Color = Color::Green;

pub struct ReplHighlighter;

impl ReplHighlighter {
    pub fn new(_config: &GlobalConfig) -> Self {
        Self
    }
}

impl Highlighter for ReplHighlighter {
    fn highlight(&self, line: &str, _cursor: usize) -> StyledText {
        let mut styled_text = StyledText::new();

        if *NO_COLOR {
            styled_text.push((Style::default(), line.to_string()));
        } else {
            let mut candidates: Vec<String> = Vec::new();
            for cmd in REPL_COMMANDS.iter() {
                candidates.push(cmd.name.to_string());
                if let Some(rest) = cmd.name.strip_prefix('/') {
                    candidates.push(format!(".{rest}"));
                }
            }
            let matches: Vec<String> = candidates
                .into_iter()
                .filter(|v| line.contains(v.as_str()))
                .collect();
            if let Some(longest_match) =
                matches
                    .iter()
                    .fold(None, |acc: Option<&String>, item| match acc {
                        Some(acc) if acc.len() >= item.len() => Some(acc),
                        _ => Some(item),
                    })
            {
                let buffer_split: Vec<&str> = line.splitn(2, longest_match.as_str()).collect();

                styled_text.push((Style::new().fg(DEFAULT_COLOR), buffer_split[0].to_string()));
                styled_text.push((Style::new().fg(MATCH_COLOR), longest_match.to_string()));
                styled_text.push((Style::new().fg(DEFAULT_COLOR), buffer_split[1].to_string()));
            } else {
                styled_text.push((Style::new().fg(DEFAULT_COLOR), line.to_string()));
            }
        }

        styled_text
    }
}
