#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

/// BL点を表す構造体
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BLPoint {
    pub position: Position,
    pub is_valid: bool,
}

/// BL法によるアイテム配置を行う関数
pub fn bottom_left_placement(rectangles: &[Rectangle], container_width: u32, container_height: u32) -> Vec<(Rectangle, Position)> {
    let mut placed_rectangles: Vec<(Rectangle, Position)> = Vec::new();
    
    // 各矩形を処理
    for &rect in rectangles {
        // コンテナに収まらない矩形はスキップ
        if rect.width > container_width || rect.height > container_height {
            continue;
        }
        
        // BL点のリストを初期化
        let mut bl_points = vec![BLPoint {
            position: Position { x: 0, y: 0 },
            is_valid: true,
        }];
        
        // 各配置済み矩形から新しいBL点を追加
        if !placed_rectangles.is_empty() {
            for &(placed_rect, placed_pos) in &placed_rectangles {
                let right_point = Position {
                    x: placed_pos.x + placed_rect.width,
                    y: placed_pos.y,
                };
                
                let top_point = Position {
                    x: placed_pos.x,
                    y: placed_pos.y + placed_rect.height,
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
                
                if pos.x + rect.width > container_width || pos.y + rect.height > container_height {
                    bl_point.is_valid = false;
                    continue;
                }
                
                if intersects(&placed_rectangles, rect, pos) {
                    bl_point.is_valid = false;
                    continue;
                }
                
                if !is_true_bl_point(pos, &placed_rectangles) {
                    bl_point.is_valid = false;
                    continue;
                }
            }
        }
        
        // 最も左下のBL点を選択（y座標が最小、同じ場合はx座標が最小）
        if let Some(best_bl_point) = bl_points.iter()
            .filter(|p| p.is_valid)
            .min_by_key(|p| (p.position.y, p.position.x)) {
            placed_rectangles.push((rect, best_bl_point.position));
        } else {
            // 有効なBL点がない場合はこの矩形は配置できないので次の矩形に
        }
    }
    
    placed_rectangles
}

/// 矩形同士の衝突を判定する関数
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

/// BL安定点が真にBL点かどうかを確認する関数
fn is_true_bl_point(pos: Position, placed_rectangles: &[(Rectangle, Position)]) -> bool {
    // 位置が(0,0)の場合は特殊ケースとして常に真のBL点
    if pos.x == 0 && pos.y == 0 {
        return true;
    }
    
    // 左側に矩形があるかどうか
    let has_rect_left = if pos.x == 0 {
        true
    } else {
        placed_rectangles.iter().any(|&(placed_rect, placed_pos)| {
            placed_pos.x + placed_rect.width == pos.x &&
            placed_pos.y < pos.y + 1 &&
            placed_pos.y + placed_rect.height > pos.y
        })
    };
    
    let has_rect_below = if pos.y == 0 {
        true 
    } else {
        placed_rectangles.iter().any(|&(placed_rect, placed_pos)| {
            placed_pos.y + placed_rect.height == pos.y &&
            placed_pos.x < pos.x + 1 &&
            placed_pos.x + placed_rect.width > pos.x
        })
    };
    
    has_rect_left && has_rect_below
}
