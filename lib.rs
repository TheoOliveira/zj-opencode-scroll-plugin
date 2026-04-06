use std::collections::BTreeMap;
use zellij_tile::prelude::*;

// ─── State ───────────────────────────────────────────────────────────────────

#[derive(Default)]
struct State {
    /// Whether any focused pane has "opencode" in its title
    opencode_active: bool,
    /// Lines captured from the pane's stdout (last N lines)
    log_lines: Vec<String>,
    /// Current scroll offset (0 = bottom / most recent)
    scroll_offset: usize,
    /// Whether scroll is locked at the top because an error was found
    error_locked: bool,
    /// Search keyword entered by the user
    search_buf: String,
    /// Whether the search bar is open
    search_mode: bool,
    /// Terminal dimensions
    rows: usize,
    cols: usize,
}

register_plugin!(State);

// ─── Constants ───────────────────────────────────────────────────────────────

const SCROLL_STEP: usize = 10;
const MAX_LOG_LINES: usize = 5_000;

/// Keywords that trigger auto-lock-to-top so the error is always visible
const ERROR_KEYWORDS: &[&str] = &["ERROR", "error", "FAIL", "fail", "panic", "thread 'main'"];

// ─── Plugin impl ─────────────────────────────────────────────────────────────

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        subscribe(&[
            EventType::PaneUpdate,
            EventType::Key,
        ]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            // ── Pane update: detect opencode & capture new output ────────────
            Event::PaneUpdate(pane_manifest) => {
                let focused = pane_manifest
                    .panes
                    .values()
                    .flatten()
                    .find(|p| p.is_focused);

                if let Some(pane) = focused {
                    self.opencode_active = pane.title.to_lowercase().contains("opencode");
                }
                true
            }

            // ── Key handling ─────────────────────────────────────────────────
            Event::Key(key) => self.handle_key(key),

            _ => false,
        }
    }

    fn render(&mut self, rows: usize, cols: usize) {
        self.rows = rows;
        self.cols = cols;

        if !self.opencode_active {
            self.render_idle(cols);
            return;
        }

        self.render_status_bar(cols);

        if self.search_mode {
            self.render_search_bar(cols);
        }
    }
}

// ─── Key handling ─────────────────────────────────────────────────────────────

impl State {
    fn handle_key(&mut self, key: Key) -> bool {
        if self.search_mode {
            return self.handle_search_key(key);
        }

        match key {
            // Fast scroll up (towards older content)
            Key::Char('u') | Key::PageUp => {
                scroll_up(SCROLL_STEP);
                self.scroll_offset = self.scroll_offset.saturating_add(SCROLL_STEP);
                true
            }
            // Fast scroll down (towards newest content)
            Key::Char('d') | Key::PageDown => {
                scroll_down(SCROLL_STEP);
                self.scroll_offset = self.scroll_offset.saturating_sub(SCROLL_STEP);
                true
            }
            // Jump to very top of buffer
            Key::Char('g') | Key::Home => {
                scroll_to_top();
                self.scroll_offset = usize::MAX;
                true
            }
            // Jump to bottom (follow mode)
            Key::Char('G') | Key::End => {
                scroll_to_bottom();
                self.scroll_offset = 0;
                self.error_locked = false;
                true
            }
            // Single line scroll
            Key::Up => {
                scroll_up(1);
                self.scroll_offset = self.scroll_offset.saturating_add(1);
                true
            }
            Key::Down => {
                scroll_down(1);
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
                true
            }
            // Open search bar
            Key::Char('/') => {
                self.search_mode = true;
                self.search_buf.clear();
                true
            }
            // Unlock error lock and go to bottom
            Key::Char('c') => {
                self.error_locked = false;
                self.scroll_offset = 0;
                scroll_to_bottom();
                true
            }
            _ => false,
        }
    }

    fn handle_search_key(&mut self, key: Key) -> bool {
        match key {
            Key::Char('\n') | Key::Esc => {
                self.search_mode = false;
                true
            }
            Key::Backspace => {
                self.search_buf.pop();
                true
            }
            Key::Char(c) => {
                self.search_buf.push(c);
                true
            }
            _ => false,
        }
    }
}

// ─── Render helpers ───────────────────────────────────────────────────────────

impl State {
    fn render_idle(&self, cols: usize) {
        let msg = " opencode-scroll │ Aguardando pane opencode... ";
        let padding = " ".repeat(cols.saturating_sub(msg.len()));
        print_text_with_coordinates(
            Text::new(format!("{}{}", msg, padding))
                .color_range(3, 0..msg.len()),
            0,
            0,
            None,
            None,
        );
    }

    fn render_status_bar(&self, cols: usize) {
        // Build the status string
        let lock_indicator = if self.error_locked {
            " ⚠ ERRO DETECTADO – 'c' para continuar "
        } else {
            ""
        };

        let scroll_indicator = if self.scroll_offset == 0 {
            "▼ seguindo".to_string()
        } else {
            format!("↑ +{} linhas", self.scroll_offset)
        };

        let hints = " u/d:scroll±10  g/G:topo/fim  /:busca  c:limpar ";
        let status = format!(
            " opencode-scroll │ {} │ {}{}{}",
            scroll_indicator,
            lock_indicator,
            hints,
            " ".repeat(cols)
        );
        let status = &status[..status.len().min(cols)];

        // Color differently when error is locked
        if self.error_locked {
            print_text_with_coordinates(
                Text::new(status).color_range(2, 0..status.len()),
                0,
                0,
                None,
                None,
            );
        } else {
            print_text_with_coordinates(
                Text::new(status).color_range(3, 0..status.len()),
                0,
                0,
                None,
                None,
            );
        }
    }

    fn render_search_bar(&self, cols: usize) {
        let prompt = format!(" Buscar: {}█", self.search_buf);
        let padded = format!("{:<width$}", prompt, width = cols);
        print_text_with_coordinates(
            Text::new(&padded).color_range(1, 0..padded.len()),
            0,
            1,
            None,
            None,
        );
    }
}
