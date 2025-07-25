use crate::{Rectangle, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreeRect {
    pub position: Position,
    pub width: u32,
    pub height: u32,
}

/// MaxRects法によるアイテム配置を行う関数
pub fn maxrects_placement(rectangles: &[Rectangle], container_width: u32, container_height: u32, allow_rotate: bool) -> Vec<(Rectangle, Position, bool)> {
    let mut placed_rectangles: Vec<(Rectangle, Position, bool)> = Vec::new();
    let mut free_rects: Vec<FreeRect> = vec![FreeRect {
        position: Position { x: 0, y: 0 },
        width: container_width,
        height: container_height,
    }];

    for &rect in rectangles {
        let mut candidates = vec![(rect.width, rect.height, false)];
        if allow_rotate && rect.width != rect.height {
            candidates.push((rect.height, rect.width, true));
        }
        let mut best_index = None;
        let mut best_area = u32::MAX;
        let mut best_pos = Position { x: 0, y: 0 };
        let mut best_rotated = false;
        for &(w, h, rotated) in &candidates {
            for (i, free) in free_rects.iter().enumerate() {
                if w <= free.width && h <= free.height {
                    let area = free.width * free.height;
                    if area < best_area {
                        best_area = area;
                        best_index = Some(i);
                        best_pos = free.position;
                        best_rotated = rotated;
                    }
                }
            }
        }
        if let Some(idx) = best_index {
            placed_rectangles.push((rect, best_pos, best_rotated));
            let (w, h) = if best_rotated { (rect.height, rect.width) } else { (rect.width, rect.height) };
            // 空き領域を分割
            let used = free_rects[idx];
            free_rects.remove(idx);
            // 右側
            if used.width > w {
                free_rects.push(FreeRect {
                    position: Position { x: used.position.x + w, y: used.position.y },
                    width: used.width - w,
                    height: h,
                });
            }
            // 下側
            if used.height > h {
                free_rects.push(FreeRect {
                    position: Position { x: used.position.x, y: used.position.y + h },
                    width: used.width,
                    height: used.height - h,
                });
            }
        }
    }
    placed_rectangles
}
