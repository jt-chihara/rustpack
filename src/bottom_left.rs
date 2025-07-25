use crate::{Rectangle, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BLPoint {
    pub position: Position,
    pub is_valid: bool,
}

/// BL法によるアイテム配置を行う関数
pub fn bottom_left_placement(rectangles: &[Rectangle], container_width: u32, container_height: u32, allow_rotate: bool) -> Vec<(Rectangle, Position, bool)> {
    let mut placed_rectangles: Vec<(Rectangle, Position, bool)> = Vec::new();
    for &rect in rectangles {
        let mut candidates = vec![(rect.width, rect.height, false)];
        if allow_rotate && rect.width != rect.height {
            candidates.push((rect.height, rect.width, true));
        }
        let mut best: Option<(Position, bool)> = None;
        for &(w, h, rotated) in &candidates {
            // 回転時のみ入る場合も考慮する

            if w > container_width || h > container_height {
                continue;
            }
            let mut bl_points = vec![BLPoint {
                position: Position { x: 0, y: 0 },
                is_valid: true,
            }];
            if !placed_rectangles.is_empty() {
                for &(ref placed_rect, placed_pos, placed_rot) in &placed_rectangles {
                    let (pw, ph) = if placed_rot {
                        (placed_rect.height, placed_rect.width)
                    } else {
                        (placed_rect.width, placed_rect.height)
                    };
                    let right_point = Position {
                        x: placed_pos.x + pw,
                        y: placed_pos.y,
                    };
                    let top_point = Position {
                        x: placed_pos.x,
                        y: placed_pos.y + ph,
                    };
                    if !bl_points.iter().any(|p| p.position == right_point) {
                        bl_points.push(BLPoint {
                            position: right_point,
                            is_valid: true,
                        });
                    }
                    if !bl_points.iter().any(|p| p.position == top_point) {
                        bl_points.push(BLPoint {
                            position: top_point,
                            is_valid: true,
                        });
                    }
                }
            }
            for bl_point in bl_points.iter_mut() {
                if bl_point.is_valid {
                    let pos = bl_point.position;
                    if pos.x + w > container_width || pos.y + h > container_height {
                        bl_point.is_valid = false;
                        continue;
                    }
                    // 衝突判定
                    let mut overlap = false;
                    for &(ref placed_rect, placed_pos, placed_rot) in &placed_rectangles {
                        let (pw, ph) = if placed_rot {
                            (placed_rect.height, placed_rect.width)
                        } else {
                            (placed_rect.width, placed_rect.height)
                        };
                        if pos.x < placed_pos.x + pw &&
                           pos.x + w > placed_pos.x &&
                           pos.y < placed_pos.y + ph &&
                           pos.y + h > placed_pos.y {
                            overlap = true;
                            break;
                        }
                    }
                    if overlap {
                        bl_point.is_valid = false;
                        continue;
                    }
                    if !super::is_true_bl_point(pos, &placed_rectangles.iter().map(|(r, p, rot)| {
                        let (w, h) = if *rot { (r.height, r.width) } else { (r.width, r.height) };
                        (Rectangle { width: w, height: h }, *p)
                    }).collect::<Vec<_>>()) {
                        bl_point.is_valid = false;
                        continue;
                    }
                }
            }
            if let Some(best_bl_point) = bl_points.iter().filter(|p| p.is_valid).min_by_key(|p| (p.position.y, p.position.x)) {
                if best.is_none() || (best_bl_point.position.y, best_bl_point.position.x) < (best.as_ref().unwrap().0.y, best.as_ref().unwrap().0.x) {
                    best = Some((best_bl_point.position, rotated));
                }
            }
        }
        if let Some((pos, rotated)) = best {
            placed_rectangles.push((rect, pos, rotated));
        }
    }
    placed_rectangles
}
