use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use log::error;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;
use tracing::trace;
use tui_input::{backend::crossterm::EventHandler, Input};

use super::{Component, Frame};
use crate::{action::Action, config::key_event_to_string};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum Mode {
  #[default]
  Normal,
  Insert,
  Processing,
}

#[derive(Default)]
pub struct Home {
  pub show_help: bool,
  pub counter: usize,
  pub app_ticker: usize,

  // GENERAL
  pub app_difficulty_adjustment: String,
  pub app_prices: String,
  pub app_historical_price: String,
  pub app_usd: String,
  pub app_timestamp: String,

  // BLOCKS
  pub app_blocks_tip_height: usize,
  pub app_blocks_tip_hash: String,

  pub render_ticker: usize,
  pub mode: Mode,
  pub input: Input,
  pub action_tx: Option<UnboundedSender<Action>>,
  pub keymap: HashMap<KeyEvent, Action>,
  pub text: Vec<String>,
  pub last_events: Vec<KeyEvent>,
}

impl Home {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn keymap(mut self, keymap: HashMap<KeyEvent, Action>) -> Self {
    self.keymap = keymap;
    self
  }

  pub fn tick(&mut self) {
    log::info!("Tick");
    self.app_ticker = self.app_ticker.saturating_add(1);

    // GENERAL
    let binding = String::from("difficulty_adjustment");
    let api_string = mempool_space::api::api(&binding, "", false);
    let diff_adj_json = api_string.parse::<String>();
    self.app_difficulty_adjustment = diff_adj_json.expect("diff_adj_json");

    let binding = String::from("prices");
    let api_string = mempool_space::api::api(&binding, "", false);
    let prices_json = api_string.parse::<String>();
    self.app_prices = prices_json.expect("prices_json");

    let binding = String::from("historical_price");
    let api_string = mempool_space::api::api(&binding, "", false);
    let hist_price_json = api_string.parse::<String>();
    self.app_historical_price = hist_price_json.expect("hist_price_json");

    // ADDRESSES TODO

    // BLOCKS
    let binding = String::from("blocks_tip_height");
    let api_string = mempool_space::api::api(&binding, "", false);
    let int_blocks_tip_height = api_string.parse::<i32>().unwrap_or(0);
    self.app_blocks_tip_height = int_blocks_tip_height.try_into().expect("int_blocks_tip_height");

    let binding = String::from("blocks_tip_hash");
    let api_string = mempool_space::api::api(&binding, "", false);
    let blocks_tip_hash = api_string.parse::<String>();
    self.app_blocks_tip_hash = blocks_tip_hash.expect("blocks_tip_hash");

    self.last_events.drain(..);
  }

  pub fn render_tick(&mut self) {
    log::debug!("Render Tick");
    self.render_ticker = self.render_ticker.saturating_add(1);
  }

  pub fn add(&mut self, s: String) {
    self.text.push(s)
  }

  pub fn schedule_increment(&mut self, i: usize) {
    let tx = self.action_tx.clone().unwrap();
    tokio::spawn(async move {
      tx.send(Action::EnterProcessing).unwrap();
      tokio::time::sleep(Duration::from_secs(1)).await;
      tx.send(Action::Increment(i)).unwrap();
      tx.send(Action::ExitProcessing).unwrap();
    });
  }

  pub fn schedule_decrement(&mut self, i: usize) {
    let tx = self.action_tx.clone().unwrap();
    tokio::spawn(async move {
      tx.send(Action::EnterProcessing).unwrap();
      tokio::time::sleep(Duration::from_secs(1)).await;
      tx.send(Action::Decrement(i)).unwrap();
      tx.send(Action::ExitProcessing).unwrap();
    });
  }

  pub fn increment(&mut self, i: usize) {
    self.counter = self.counter.saturating_add(i);
  }

  pub fn decrement(&mut self, i: usize) {
    self.counter = self.counter.saturating_sub(i);
  }
}

impl Component for Home {
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.action_tx = Some(tx);
    Ok(())
  }

  fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    self.last_events.push(key);
    let action = match self.mode {
      Mode::Normal | Mode::Processing => return Ok(None),
      Mode::Insert => {
        match key.code {
          KeyCode::Esc => Action::EnterNormal,
          KeyCode::Enter => {
            if let Some(sender) = &self.action_tx {
              if let Err(e) = sender.send(Action::CompleteInput(self.input.value().to_string())) {
                error!("Failed to send action: {:?}", e);
              }
            }
            Action::EnterNormal
          },
          _ => {
            self.input.handle_event(&crossterm::event::Event::Key(key));
            Action::Update
          },
        }
      },
    };
    Ok(Some(action))
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    match action {
      Action::Tick => self.tick(),
      Action::Render => self.render_tick(),
      Action::ToggleShowHelp => self.show_help = !self.show_help,
      Action::ScheduleIncrement => self.schedule_increment(1),
      Action::ScheduleDecrement => self.schedule_decrement(1),
      Action::Increment(i) => self.increment(i),
      Action::Decrement(i) => self.decrement(i),
      Action::CompleteInput(s) => self.add(s),
      Action::EnterNormal => {
        self.mode = Mode::Normal;
      },
      Action::EnterInsert => {
        self.mode = Mode::Insert;
      },
      Action::EnterProcessing => {
        self.mode = Mode::Processing;
      },
      Action::ExitProcessing => {
        // TODO: Make this go to previous mode instead
        self.mode = Mode::Normal;
      },
      _ => (),
    }
    Ok(None)
  }

  fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
    let rects = Layout::vertical(
      [
        Constraint::Percentage(8),  // header rects[0]
        Constraint::Percentage(25), // block canvas rects[1]
        Constraint::Percentage(8),  // blockheight rects[2]
        Constraint::Percentage(8),  // blockhash rects[3]
        Constraint::Percentage(8),  // blockhash rects[3]
        Constraint::Percentage(8),  // blockhash rects[3]
        Constraint::Percentage(8),  // blockhash rects[3]
        Constraint::Percentage(8),  // blockhash rects[3]
        Constraint::Percentage(8),  // blockhash rects[3]
        Constraint::Min(0),         // rects[last]
      ]
      .as_ref(),
    )
    .split(rect);

    let mut blockheight: Vec<Line> = self.text.clone().iter().map(|l| Line::from(l.clone())).collect();
    blockheight.insert(0, format!("height: {} {}", self.app_blocks_tip_height, self.app_blocks_tip_hash).into());
    f.render_widget(
      Paragraph::new(blockheight)
        .wrap(Wrap { trim: true })
        .scroll((0, 0))
        .block(
          Block::default()
            .title("─── api/blocks/tip/height ─")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_style(match self.mode {
              Mode::Processing => Style::default().fg(Color::Yellow),
              _ => Style::default(),
            })
            .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Reset))
        .alignment(Alignment::Left),
      rects[2],
    );
    let mut blockhash: Vec<Line> = self.text.clone().iter().map(|l| Line::from(l.clone())).collect();
    blockhash.insert(0, format!("mempool.space/api/blocks/tip/hash: {}", self.app_blocks_tip_hash).into());
    f.render_widget(
      Paragraph::new(blockhash)
        .wrap(Wrap { trim: true })
        .scroll((0, 0))
        .block(
          Block::default()
            .title("─── api/blocks/tip/hash ─")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_style(match self.mode {
              Mode::Processing => Style::default().fg(Color::Yellow),
              _ => Style::default(),
            })
            .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Reset))
        .alignment(Alignment::Left),
      rects[3],
    );

    // text.insert(0, "".into());
    // text.insert(0, "Type into input and hit enter to display here".dim().into());
    // text.insert(0, format!("Render Ticker: {}", self.render_ticker).into());
    // text.insert(0, format!("App Ticker: {}", self.app_ticker).into());
    // text.insert(0, format!("mempool.space/api/v1/historical-price: {}", self.app_historical_price).into());
    // blockheight.insert(0, format!("mempool.space/api/v1/prices: {}", self.app_prices).into());
    // blockheight.insert(0, format!("mempool.space/api/difficulty-adjustment: {}", self.app_difficulty_adjustment).into());
    // blockheight.insert(0, format!("mempool.space/api/blocks/tip/hash: {}", self.app_blocks_tip_hash).into());

    // text.insert(0, format!("Counter: {}", self.counter).into());
    // text.insert(0, "".into());
    // text.insert(
    //  0,
    //  Line::from(vec![
    //    "Press ".into(),
    //    Span::styled("j", Style::default().fg(Color::Red)),
    //    " or ".into(),
    //    Span::styled("k", Style::default().fg(Color::Red)),
    //    " to ".into(),
    //    Span::styled("increment", Style::default().fg(Color::Yellow)),
    //    " or ".into(),
    //    Span::styled("decrement", Style::default().fg(Color::Yellow)),
    //    ".".into(),
    //  ]),
    //);

    f.render_widget(
      Tabs::new(vec!["🟠", "🟠", "🟠", "🟠"])
        .block(Block::bordered().title("─── mempool.space ─"))
        .style(Style::default().white())
        .highlight_style(Style::default().yellow())
        .select(0)
        .divider(" ")
        .padding(" ", " "),
      rects[0],
    );

    // f.render_widget(
    // Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
    //.block(Block::bordered().title("Tabs"))
    //.style(Style::default().white())
    //.highlight_style(Style::default().yellow())
    //.select(2)
    //.divider(symbols::DOT)
    //.padding("->", "<-"),
    // rects[1]);
    // f.render_widget(
    // Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
    //.block(Block::bordered().title("Tabs"))
    //.style(Style::default().white())
    //.highlight_style(Style::default().yellow())
    //.select(2)
    //.divider(symbols::DOT)
    //.padding("->", "<-"),
    // rects[2]);

    // f.render_widget(
    //  Paragraph::new(blockheight)
    //  .wrap(Wrap { trim: true })
    //  .scroll((0, 0))
    //    .block(
    //      Block::default()
    //        .title("─── mempool_space ─")
    //        .title_alignment(Alignment::Left)
    //        .borders(Borders::ALL)
    //        .border_style(match self.mode {
    //          Mode::Processing => Style::default().fg(Color::Yellow),
    //          _ => Style::default(),
    //        })
    //        .border_type(BorderType::Rounded),
    //    )
    //    .style(Style::default().fg(Color::Reset))
    //    .alignment(Alignment::Left),
    //  rects[1],
    //);

    let width = rects[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = self.input.visual_scroll(width as usize);
    let input = Paragraph::new(self.input.value())
      .style(match self.mode {
        Mode::Insert => Style::default().fg(Color::Yellow),
        _ => Style::default(),
      })
      .scroll((0, scroll as u16))
      .block(Block::default().borders(Borders::ALL).title(Line::from(vec![
        Span::raw(format!("{} \n", rects.len())),
        Span::raw("Enter Input Mode "),
        Span::styled("(Press ", Style::default().fg(Color::DarkGray)),
        Span::styled("/", Style::default().add_modifier(Modifier::BOLD).fg(Color::Gray)),
        Span::styled(" to start, ", Style::default().fg(Color::DarkGray)),
        Span::styled("ESC", Style::default().add_modifier(Modifier::BOLD).fg(Color::Gray)),
        Span::styled(" to finish)", Style::default().fg(Color::DarkGray)),
      ])));
    f.render_widget(input, rects[3]);

    if self.mode == Mode::Insert {
      f.set_cursor_position(Position::new(
        (rects[1].x + 1 + self.input.cursor() as u16).min(rects[1].x + rects[1].width - 2),
        rects[1].y + 1,
      ))
    }

    if self.show_help {
      let rect = rect.inner(Margin { horizontal: 4, vertical: 2 });
      f.render_widget(Clear, rect);
      let block = Block::default()
        .title(Line::from(vec![Span::styled("Key Bindings", Style::default().add_modifier(Modifier::BOLD))]))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));
      f.render_widget(block, rect);
      let rows = vec![
        Row::new(vec!["j", "Increment"]),
        Row::new(vec!["k", "Decrement"]),
        Row::new(vec!["/", "Enter Input"]),
        Row::new(vec!["ESC", "Exit Input"]),
        Row::new(vec!["Enter", "Submit Input"]),
        Row::new(vec!["Ctrl-q", "Quit"]), // interferes with text input
        Row::new(vec!["?", "Open Help"]),
      ];
      let widths = [Constraint::Percentage(10), Constraint::Percentage(90)];
      let table = Table::new(rows, widths)
        .header(Row::new(vec!["Key", "Action"]).bottom_margin(1).style(Style::default().add_modifier(Modifier::BOLD)))
        .column_spacing(1);
      f.render_widget(table, rect.inner(Margin { vertical: 4, horizontal: 2 }));
    };

    // key pressed logger
    f.render_widget(
      Block::default()
        .title(
          ratatui::widgets::block::Title::from(format!(
            "{:?}",
            &self.last_events.iter().map(key_event_to_string).collect::<Vec<_>>()
          ))
          .alignment(Alignment::Right),
        )
        .title_style(Style::default().add_modifier(Modifier::BOLD)),
      Rect { x: rect.x + 1, y: rect.height.saturating_sub(1), width: rect.width.saturating_sub(2), height: 1 },
    );

    Ok(())
  }
}
