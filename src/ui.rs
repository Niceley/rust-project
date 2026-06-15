use crate::map::{Map, Tile};
use crate::resource::ResourceKind;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};

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
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(frame.area());

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

    let map_widget = Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::ALL).title(" Simulation "));
    frame.render_widget(map_widget, chunks[0]);

    let footer = Paragraph::new("Énergie : 0   |   Cristaux : 0   |   (touche pour quitter)")
        .block(Block::default().borders(Borders::ALL).title(" Ressources collectées "));
    frame.render_widget(footer, chunks[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_base_a_le_bon_symbole() {
        assert_eq!(cell(Tile::Base), ('#', Color::Rgb(228, 240, 212)));
    }

    #[test]
    fn les_ressources_ont_les_bons_symboles() {
        assert_eq!(cell(Tile::Resource(ResourceKind::Energy)).0, 'E');
        assert_eq!(cell(Tile::Resource(ResourceKind::Crystal)).0, 'C');
    }

    #[test]
    fn un_obstacle_est_affiche_en_o_majuscule() {
        assert_eq!(cell(Tile::Obstacle).0, 'O');
    }
}
