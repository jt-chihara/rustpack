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

mod bottom_left;
mod maxrects;
mod skyline;
mod guillotine;

pub use bottom_left::bottom_left_placement;
pub use maxrects::{maxrects_placement, FreeRect};
pub use skyline::{skyline_placement, SkylineNode};
pub use guillotine::{guillotine_placement, GuillotineRect};


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
