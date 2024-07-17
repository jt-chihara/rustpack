#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub fn bottom_left_placement(rectangles: &[Rectangle], container_width: u32, container_height: u32) -> Vec<(Rectangle, Position)> {
    let mut placed_rectangles: Vec<(Rectangle, Position)> = Vec::new();

    for &rect in rectangles {
        if rect.width > container_width || rect.height > container_height {
            println!("Could not place rectangle {:?}", rect);
            continue;
        }

        let mut best_pos = None;

        for y in 0..=(container_height - rect.height) {
            for x in 0..=(container_width - rect.width) {
                let pos = Position { x, y };
                if !intersects(&placed_rectangles, rect, pos) {
                    best_pos = Some(pos);
                    break;
                }
            }
            if best_pos.is_some() {
                break;
            }
        }

        if let Some(pos) = best_pos {
            placed_rectangles.push((rect, pos));
        } else {
            println!("Could not place rectangle {:?}", rect);
        }
    }

    placed_rectangles
}

fn intersects(placed_rectangles: &[(Rectangle, Position)], rect: Rectangle, pos: Position) -> bool {
    for &(placed_rect, placed_pos) in placed_rectangles {
        if pos.x < placed_pos.x + placed_rect.width &&
           pos.x + rect.width > placed_pos.x &&
           pos.y < placed_pos.y + placed_rect.height &&
           pos.y + rect.height > placed_pos.y {
            return true;
        }
    }
    false
}
