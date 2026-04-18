use std::collections::BTreeMap;
use zellij_tile::prelude::*;

// ─── State ───────────────────────────────────────────────────────────────────

#[derive(Default)]
struct State {
    /// ID of the focused opencode pane
    focused_pane_id: Option<PaneId>,
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
const MAX_CAPTURE_LINES: usize = 100;

/// Keywords that trigger auto-lock-to-top so the error is always visible
const ERROR_KEYWORDS: &[&str] = &[
    "ERROR",
    "error",
    "FAIL",
    "fail",
    "panic",
    "thread 'main'",
    "COMPILATION ERROR",
    "Build failed",
    "error:",
];

// ─── Plugin impl ─────────────────────────────────────────────────────────────

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        subscribe(&[EventType::PaneUpdate, EventType::Key]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::PaneUpdate(pane_manifest) => {
                let focused = pane_manifest
                    .panes
                    .values()
                    .flatten()
                    .find(|p| p.is_focused && p.title.to_lowercase().contains("opencode"));

                if let Some(pane) = focused {
                    self.focused_pane_id = Some(PaneId::Terminal(pane.id));
                } else {
                    self.focused_pane_id = None;
                }
                true
            }

            Event::Key(key) => self.handle_key(key),

            _ => false,
        }
    }

    fn render(&mut self, rows: usize, cols: usize) {
        self.rows = rows;
        self.cols = cols;

        if self.focused_pane_id.is_none() {
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
    fn handle_key(&mut self, key: KeyWithModifier) -> bool {
        if self.search_mode {
            return self.handle_search_key(key);
        }

        match key {
            // Fast scroll up (towards older content)
            key if key.bare_key == BareKey::PageUp || (key.bare_key == BareKey::Char('u') && key.has_no_modifiers()) => {
                for _ in 0..SCROLL_STEP {
                    scroll_up();
                }
                self.scroll_offset = self.scroll_offset.saturating_add(SCROLL_STEP);
                true
            }
            // Fast scroll down (towards newest content)
            key if key.bare_key == BareKey::PageDown || (key.bare_key == BareKey::Char('d') && key.has_no_modifiers()) => {
                for _ in 0..SCROLL_STEP {
                    scroll_down();
                }
                self.scroll_offset = self.scroll_offset.saturating_sub(SCROLL_STEP);
                true
            }
            // Jump to very top of buffer
            key if key.bare_key == BareKey::Home || key.bare_key == BareKey::Char('g') => {
                scroll_to_top();
                self.scroll_offset = usize::MAX;
                true
            }
            // Jump to bottom (follow mode)
            key if key.bare_key == BareKey::End || key.bare_key == BareKey::Char('G') => {
                scroll_to_bottom();
                self.scroll_offset = 0;
                self.error_locked = false;
                true
            }
            // Single line scroll
            key if key.bare_key == BareKey::Up => {
                scroll_up();
                self.scroll_offset = self.scroll_offset.saturating_add(1);
                true
            }
            key if key.bare_key == BareKey::Down => {
                scroll_down();
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
                true
            }
            // Open search bar
            key if key.bare_key == BareKey::Char('/') && key.has_no_modifiers() => {
                self.search_mode = true;
                self.search_buf.clear();
                true
            }
            // Unlock error lock and go to bottom
            key if key.bare_key == BareKey::Char('c') && key.has_no_modifiers() => {
                self.error_locked = false;
                self.scroll_offset = 0;
                scroll_to_bottom();
                true
            }
            _ => false,
        }
    }

    fn handle_search_key(&mut self, key: KeyWithModifier) -> bool {
        if key.bare_key == BareKey::Enter || key.bare_key == BareKey::Esc {
            self.search_mode = false;
            return true;
        }
        if key.bare_key == BareKey::Backspace {
            self.search_buf.pop();
            return true;
        }
        if let BareKey::Char(c) = key.bare_key {
            self.search_buf.push(c);
            return true;
        }
        false
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
