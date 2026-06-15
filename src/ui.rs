use crate::map::{Map, Tile};
use crate::resource::ResourceKind;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

fn cell(tile: Tile) -> (char, Color) {
    match tile {
        Tile::Empty => (' ', Color::White),
        Tile::Obstacle => ('O', Color::Rgb(164, 226, 229)), // Cyan clair
        Tile::Resource(ResourceKind::Energy) => ('E', Color::Rgb(144, 209, 46)), // Vert
        Tile::Resource(ResourceKind::Crystal) => ('C', Color::Rgb(252, 204, 240)), // Magenta clair
        Tile::Base => ('#', Color::Rgb(228, 240, 212)), // Vert clair
    }
}

pub fn draw(frame: &mut Frame, map: &Map) {
    let mut lines = Vec::with_capacity(map.height as usize);
    for y in 0..map.height {
        let mut spans = Vec::with_capacity(map.width as usize);
        for x in 0..map.width {
            let tile = map.get(x, y).unwrap_or(Tile::Empty);
            let (ch, color) = cell(tile);
            spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
        }
        lines.push(Line::from(spans));
    }
    let widget = Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::ALL).title(" Simulation "));
    frame.render_widget(widget, frame.area());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_base_a_le_bon_symbole() {
        assert_eq!(cell(Tile::Base), ('#', Color::Rgb(228, 240, 212)));
    }
}