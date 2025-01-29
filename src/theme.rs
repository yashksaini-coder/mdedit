//!
//! Implements a dark theme.
//!

use rat_theme::scheme::{
    BASE16, BASE16_RELAXED, IMPERIAL, MONEKAI, MONOCHROME, OCEAN, OXOCARBON, RADIUM, TUNDRA,
    VSCODE_DARK,
};
use rat_theme::Scheme;
use rat_widget::button::ButtonStyle;
use rat_widget::calendar::CalendarStyle;
use rat_widget::checkbox::CheckboxStyle;
use rat_widget::choice::ChoiceStyle;
use rat_widget::clipper::ClipperStyle;
use rat_widget::file_dialog::FileDialogStyle;
use rat_widget::line_number::LineNumberStyle;
use rat_widget::list::ListStyle;
use rat_widget::menu::MenuStyle;
use rat_widget::msgdialog::MsgDialogStyle;
use rat_widget::pager::PagerStyle;
use rat_widget::paragraph::ParagraphStyle;
use rat_widget::popup::PopupStyle;
use rat_widget::radio::{RadioLayout, RadioStyle};
use rat_widget::scrolled::{ScrollStyle, ScrollSymbols};
use rat_widget::shadow::{ShadowDirection, ShadowStyle};
use rat_widget::splitter::SplitStyle;
use rat_widget::tabbed::TabbedStyle;
use rat_widget::table::TableStyle;
use rat_widget::text::TextStyle;
use rat_widget::view::ViewStyle;
use ratatui::prelude::{Style, Stylize};
use ratatui::style::Color;
use ratatui::widgets::{Block, Borders};
use std::time::Duration;

/// One sample theme which prefers dark colors from the color-scheme
/// and generates styles for widgets.
///
/// The widget set fits for the widgets provided by
/// [rat-widget](https://www.docs.rs/rat-widget), for other needs
/// take it as an idea for your own implementation.
///
#[derive(Debug, Clone)]
pub struct DarkTheme {
    s: Scheme,
    name: String,
}

impl DarkTheme {
    pub fn new(name: String, s: Scheme) -> Self {
        Self { s, name }
    }
}

impl DarkTheme {
    /// Some display name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Hint at dark.
    pub fn dark_theme(&self) -> bool {
        true
    }

    /// The underlying scheme.
    pub fn s(&self) -> &Scheme {
        &self.s
    }

    /// Focus style
    pub fn focus(&self) -> Style {
        self.s.style(self.s.primary[2])
    }

    /// Selection style
    pub fn select(&self) -> Style {
        self.s.style(self.s.secondary[1])
    }

    /// Text field style.
    pub fn text_input(&self) -> Style {
        self.s.style(self.s.gray[3])
    }

    /// Focused text field style.
    pub fn text_focus(&self) -> Style {
        self.s.style(self.s.primary[0])
    }

    /// Text selection style.
    pub fn text_select(&self) -> Style {
        self.s.style(self.s.secondary[0])
    }

    /// Container base
    pub fn container_base(&self) -> Style {
        self.s.style(self.s.black[0])
    }

    /// Container border
    pub fn container_border(&self) -> Style {
        Style::default().fg(self.s.gray[0]).bg(self.s.black[0])
    }

    /// Container arrows
    pub fn container_arrow(&self) -> Style {
        Style::default().fg(self.s.secondary[0]).bg(self.s.black[0])
    }

    /// Background for popups.
    pub fn popup_base(&self) -> Style {
        self.s.style(self.s.white[0])
    }

    /// Label text inside container.
    pub fn popup_label(&self) -> Style {
        self.s.style(self.s.white[0])
    }

    /// Dialog arrows
    pub fn popup_border(&self) -> Style {
        Style::default().fg(self.s.gray[0]).bg(self.s.white[0])
    }

    /// Dialog arrows
    pub fn popup_arrow(&self) -> Style {
        Style::default().fg(self.s.secondary[0]).bg(self.s.white[0])
    }

    /// Background for dialogs.
    pub fn dialog_base(&self) -> Style {
        self.s.style(self.s.gray[1])
    }

    /// Label text inside container.
    pub fn dialog_label(&self) -> Style {
        self.s.style(self.s.gray[1])
    }

    /// Dialog arrows
    pub fn dialog_border(&self) -> Style {
        Style::default().fg(self.s.white[0]).bg(self.s.gray[1])
    }

    /// Dialog arrows
    pub fn dialog_arrow(&self) -> Style {
        Style::default().fg(self.s.secondary[2]).bg(self.s.gray[1])
    }

    /// Style for the status line.
    pub fn status_base(&self) -> Style {
        Style::default().fg(self.s.white[0]).bg(self.s.black[0])
    }

    /// Base style for buttons.
    pub fn button_base(&self) -> Style {
        self.s.style(self.s.gray[2])
    }

    /// Armed style for buttons.
    pub fn button_armed(&self) -> Style {
        self.s.style(self.s.secondary[0])
    }

    /// Complete MonthStyle.
    pub fn month_style(&self) -> CalendarStyle {
        CalendarStyle {
            style: self.s.style(self.s.black[2]),
            title: None,
            weeknum: Some(Style::new().fg(self.s.limegreen[2])),
            weekday: Some(Style::new().fg(self.s.limegreen[2])),
            day: None,
            select: Some(self.select()),
            focus: Some(self.focus()),
            ..CalendarStyle::default()
        }
    }

    /// Style for shadows.
    pub fn shadow_style(&self) -> ShadowStyle {
        ShadowStyle {
            style: Style::new().bg(self.s.black[0]),
            dir: ShadowDirection::BottomRight,
            ..ShadowStyle::default()
        }
    }

    /// Style for LineNumbers.
    pub fn line_nr_style(&self) -> LineNumberStyle {
        LineNumberStyle {
            style: self.container_base().fg(self.s.gray[1]),
            cursor: Some(self.text_select()),
            ..LineNumberStyle::default()
        }
    }

    /// Complete TextAreaStyle
    pub fn textarea_style(&self) -> TextStyle {
        TextStyle {
            style: self.container_base(),
            focus: Some(self.focus()),
            select: Some(self.text_select()),
            scroll: Some(self.scroll_style()),
            border_style: Some(self.container_border()),
            ..TextStyle::default()
        }
    }

    /// Complete TextInputStyle
    pub fn text_style(&self) -> TextStyle {
        TextStyle {
            style: self.text_input(),
            focus: Some(self.text_focus()),
            select: Some(self.text_select()),
            invalid: Some(Style::default().bg(self.s.red[3])),
            ..TextStyle::default()
        }
    }

    pub fn paragraph_style(&self) -> ParagraphStyle {
        ParagraphStyle {
            style: self.container_base(),
            focus: Some(self.focus()),
            scroll: Some(self.scroll_style()),
            ..Default::default()
        }
    }

    pub fn choice_style(&self) -> ChoiceStyle {
        ChoiceStyle {
            style: self.text_input(),
            select: Some(self.text_focus()),
            focus: Some(self.text_focus()),
            popup: PopupStyle {
                style: self.popup_base(),
                scroll: Some(self.popup_scroll_style()),
                block: Some(
                    Block::bordered()
                        .borders(Borders::LEFT)
                        .border_style(self.popup_arrow()),
                ),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn radio_style(&self) -> RadioStyle {
        RadioStyle {
            layout: Some(RadioLayout::Stacked),
            style: self.text_input(),
            focus: Some(self.text_focus()),
            ..Default::default()
        }
    }

    /// Complete CheckboxStyle
    pub fn checkbox_style(&self) -> CheckboxStyle {
        CheckboxStyle {
            style: self.text_input(),
            focus: Some(self.text_focus()),
            ..Default::default()
        }
    }

    /// Complete MenuStyle
    pub fn menu_style(&self) -> MenuStyle {
        MenuStyle {
            style: self.status_base(),
            title: Some(Style::default().fg(self.s.black[0]).bg(self.s.yellow[0])),
            select: Some(self.select()),
            focus: Some(self.focus()),
            right: Some(Style::default().fg(self.s.bluegreen[0])),
            disabled: Some(Style::default().fg(self.s.gray[0])),
            highlight: Some(Style::default().underlined()),
            popup: PopupStyle {
                style: self.status_base(),
                block: Some(Block::bordered()),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Complete MenuStyle
    pub fn menu_style_hidden(&self) -> MenuStyle {
        let mut style = self.status_base();
        style = style.fg(self.s.gray[0]);

        MenuStyle {
            style,
            title: Some(
                Style::default()
                    .fg(self.s.black[0])
                    .bg(self.s.true_dark_color(self.s.red[3])),
            ),
            select: Some(style),
            focus: Some(style),
            right: Some(Style::default().fg(self.s.bluegreen[0])),
            disabled: Some(Style::default().fg(self.s.gray[0])),
            highlight: Some(Style::default().underlined()),
            popup: PopupStyle {
                style: self.status_base(),
                block: Some(Block::bordered()),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Complete ButtonStyle
    pub fn button_style(&self) -> ButtonStyle {
        ButtonStyle {
            style: self.button_base(),
            focus: Some(self.focus()),
            armed: Some(self.select()),
            armed_delay: Some(Duration::from_millis(50)),
            ..Default::default()
        }
    }

    /// Complete TableStyle
    pub fn table_style(&self) -> TableStyle {
        TableStyle {
            style: self.container_base(),
            select_row: Some(self.select()),
            show_row_focus: true,
            focus_style: Some(self.focus()),
            border_style: Some(self.container_border()),
            scroll: Some(self.scroll_style()),
            ..Default::default()
        }
    }

    pub fn table_header(&self) -> Style {
        self.s.style(self.s.blue[2])
    }

    pub fn table_footer(&self) -> Style {
        self.s.style(self.s.blue[2])
    }

    /// Complete ListStyle
    pub fn list_style(&self) -> ListStyle {
        let mut base = self.container_base();
        base = base.fg(self.s.reduced_text_color(base.bg.expect("bg")));
        ListStyle {
            style: base,
            select: Some(self.select()),
            focus: Some(self.focus()),
            scroll: Some(self.scroll_style()),
            ..Default::default()
        }
    }

    /// Scroll style
    pub fn scroll_style(&self) -> ScrollStyle {
        ScrollStyle {
            thumb_style: Some(self.container_border()),
            track_style: Some(self.container_border()),
            min_style: Some(self.container_border()),
            begin_style: Some(self.container_arrow()),
            end_style: Some(self.container_arrow()),
            ..Default::default()
        }
    }

    /// Popup scroll style
    pub fn popup_scroll_style(&self) -> ScrollStyle {
        ScrollStyle {
            thumb_style: Some(self.popup_border()),
            track_style: Some(self.popup_border()),
            min_style: Some(self.popup_border()),
            begin_style: Some(self.popup_arrow()),
            end_style: Some(self.popup_arrow()),
            ..Default::default()
        }
    }

    /// Dialog scroll style
    pub fn dialog_scroll_style(&self) -> ScrollStyle {
        ScrollStyle {
            thumb_style: Some(self.dialog_border()),
            track_style: Some(self.dialog_border()),
            min_style: Some(self.dialog_border()),
            begin_style: Some(self.dialog_arrow()),
            end_style: Some(self.dialog_arrow()),
            ..Default::default()
        }
    }

    /// Split style
    pub fn split_style(&self) -> SplitStyle {
        SplitStyle {
            style: self.container_border(),
            arrow_style: Some(self.container_arrow()),
            drag_style: Some(self.focus()),
            ..Default::default()
        }
    }

    /// View style
    pub fn view_style(&self) -> ViewStyle {
        ViewStyle {
            scroll: Some(self.scroll_style()),
            ..Default::default()
        }
    }

    /// Tabbed style
    pub fn tabbed_style(&self) -> TabbedStyle {
        TabbedStyle {
            style: self.container_border(),
            tab: Some(self.s.gray(1)),
            select: Some(self.s.gray(3)),
            focus: Some(self.focus()),
            ..Default::default()
        }
    }

    /// Complete StatusLineStyle for a StatusLine with 3 indicator fields.
    /// This is what I need for the
    /// [minimal](https://github.com/thscharler/rat-salsa/blob/master/examples/minimal.rs)
    /// example, which shows timings for Render/Event/Action.
    pub fn statusline_style(&self) -> Vec<Style> {
        vec![
            self.status_base(),
            Style::default()
                .fg(self.s.text_color(self.s.white[0]))
                .bg(self.s.blue[3]),
            Style::default()
                .fg(self.s.text_color(self.s.white[0]))
                .bg(self.s.blue[2]),
            Style::default()
                .fg(self.s.text_color(self.s.white[0]))
                .bg(self.s.blue[1]),
        ]
    }

    /// FileDialog style.
    pub fn file_dialog_style(&self) -> FileDialogStyle {
        FileDialogStyle {
            style: self.dialog_base(),
            list: Some(self.list_style()),
            roots: Some(ListStyle {
                style: self.dialog_base(),
                ..self.list_style()
            }),
            text: Some(self.text_style()),
            button: Some(self.button_style()),
            block: Some(Block::bordered()),
            ..Default::default()
        }
    }

    /// Complete MsgDialogStyle.
    pub fn msg_dialog_style(&self) -> MsgDialogStyle {
        MsgDialogStyle {
            style: self.dialog_base(),
            button: Some(self.button_style()),
            ..Default::default()
        }
    }

    /// Pager style.
    pub fn pager_style(&self) -> PagerStyle {
        PagerStyle {
            style: self.container_base(),
            navigation: Some(self.container_arrow()),
            block: Some(
                Block::bordered()
                    .borders(Borders::TOP | Borders::BOTTOM)
                    .border_style(self.container_border()),
            ),
            ..Default::default()
        }
    }

    /// Clipper style.
    pub fn clipper_style(&self) -> ClipperStyle {
        ClipperStyle {
            style: self.container_base(),
            scroll: Some(self.scroll_style()),
            ..Default::default()
        }
    }

    /// ----------------------

    pub fn choice_style_tools(&self) -> ChoiceStyle {
        ChoiceStyle {
            style: self.container_base(),
            select: Some(self.focus()),
            focus: Some(self.focus()),
            popup: PopupStyle {
                style: self.container_base(),
                scroll: Some(self.scroll_style()),
                block: Some(
                    Block::bordered()
                        .borders(Borders::LEFT)
                        .border_style(self.container_border()),
                ),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// ----------------------

    pub fn doc_base_color(&self) -> Color {
        self.s.true_dark_color(self.s.green[0])
    }

    pub fn doc_base(&self) -> Style {
        self.s.reduced_style(self.doc_base_color())
    }

    /// Container border
    pub fn doc_border(&self) -> Style {
        Style::default()
            .fg(self.s.gray[0])
            .bg(self.doc_base_color())
    }

    /// Container arrows
    pub fn doc_arrow(&self) -> Style {
        Style::default()
            .fg(self.s.secondary[0])
            .bg(self.doc_base_color())
    }

    /// Scroll style
    pub fn doc_scroll_style(&self) -> ScrollStyle {
        ScrollStyle {
            thumb_style: Some(self.doc_border()),
            track_style: Some(self.doc_border()),
            min_style: Some(self.doc_border()),
            begin_style: Some(self.doc_arrow()),
            end_style: Some(self.doc_arrow()),
            vertical: Some(ScrollSymbols {
                track: ratatui::symbols::line::VERTICAL,
                thumb: ratatui::symbols::block::FULL,
                begin: "↑",
                end: "↓",
                min: " ",
            }),
            horizontal: Some(ScrollSymbols {
                track: ratatui::symbols::line::HORIZONTAL,
                thumb: ratatui::symbols::block::FULL,
                begin: "←",
                end: "→",
                min: " ",
            }),
            ..Default::default()
        }
    }

    /// Style for LineNumbers.
    pub fn line_nr_style_doc(&self) -> LineNumberStyle {
        let fg = match self.s.rate_text_color(self.doc_base().bg.expect("bg")) {
            None => self.s.gray[3],
            Some(true) => self.s.gray[3],
            Some(false) => self.s.gray[0],
        };
        LineNumberStyle {
            style: self.doc_base().fg(fg),
            cursor: Some(self.text_select()),
            ..LineNumberStyle::default()
        }
    }

    /// Complete TextAreaStyle
    pub fn textarea_style_doc(&self) -> TextStyle {
        TextStyle {
            style: self.doc_base(),
            focus: Some(self.focus()),
            select: Some(self.text_select()),
            scroll: Some(self.doc_scroll_style()),
            border_style: Some(self.doc_border()),
            ..TextStyle::default()
        }
    }

    /// Tabbed style
    pub fn tabbed_style_doc(&self) -> TabbedStyle {
        TabbedStyle {
            style: self.doc_border(),
            tab: Some(self.s.gray(1)),
            select: Some(self.s.gray(3)),
            focus: Some(self.focus()),
            ..Default::default()
        }
    }
}

/// A list of DarkTheme for all color schemes.
pub fn dark_themes() -> Vec<DarkTheme> {
    vec![
        DarkTheme::new("Imperial".to_string(), IMPERIAL),
        DarkTheme::new("Radium".to_string(), RADIUM),
        DarkTheme::new("Tundra".to_string(), TUNDRA),
        DarkTheme::new("Monochrome".to_string(), MONOCHROME),
        DarkTheme::new("Monekai".to_string(), MONEKAI),
        DarkTheme::new("Oxocarbon".to_string(), OXOCARBON),
        DarkTheme::new("VSCodeDark".to_string(), VSCODE_DARK),
        DarkTheme::new("Ocean".to_string(), OCEAN),
        DarkTheme::new("Base16".to_string(), BASE16),
        DarkTheme::new("Base16Relaxed".to_string(), BASE16_RELAXED),
    ]
}
