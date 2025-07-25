use crate::{Rectangle, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuillotineRect {
    pub position: Position,
    pub width: u32,
    pub height: u32,
}

/// Guillotine法によるアイテム配置を行う関数
pub fn guillotine_placement(rectangles: &[Rectangle], container_width: u32, container_height: u32, allow_rotate: bool) -> Vec<(Rectangle, Position, bool)> {
    let mut placed_rectangles: Vec<(Rectangle, Position, bool)> = Vec::new();
    let mut free_rects: Vec<GuillotineRect> = vec![GuillotineRect {
        position: Position { x: 0, y: 0 },
        width: container_width,
        height: container_height,
    }];

    for &rect in rectangles {
        let mut candidates = vec![(rect.width, rect.height, false)];
        if allow_rotate && rect.width != rect.height {
            candidates.push((rect.height, rect.width, true));
        }
        let mut best_idx = None;
        let mut best_area = u32::MAX;
        let mut best_pos = Position { x: 0, y: 0 };
        let mut best_rotated = false;
        for &(w, h, rotated) in &candidates {
            for (i, free) in free_rects.iter().enumerate() {
                if w <= free.width && h <= free.height {
                    let area = free.width * free.height;
                    if area < best_area {
                        best_area = area;
                        best_idx = Some(i);
                        best_pos = free.position;
                        best_rotated = rotated;
                    }
                }
            }
        }
        if let Some(idx) = best_idx {
            placed_rectangles.push((rect, best_pos, best_rotated));
            let (w, h) = if best_rotated { (rect.height, rect.width) } else { (rect.width, rect.height) };
            let used = free_rects[idx];
            free_rects.remove(idx);
            // Guillotine分割（右＋下）
            let right = GuillotineRect {
                position: Position { x: used.position.x + w, y: used.position.y },
                width: used.width - w,
                height: h,
            };
            let below = GuillotineRect {
                position: Position { x: used.position.x, y: used.position.y + h },
                width: used.width,
                height: used.height - h,
            };
            if right.width > 0 && right.height > 0 {
                free_rects.push(right);
            }
            if below.width > 0 && below.height > 0 {
                free_rects.push(below);
            }
        }
    }
    placed_rectangles
}
