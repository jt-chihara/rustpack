use crate::{Rectangle, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkylineNode {
    pub x: u32,
    pub y: u32,
    pub width: u32,
}

/// Skyline法によるアイテム配置を行う関数
pub fn skyline_placement(rectangles: &[Rectangle], container_width: u32, container_height: u32, allow_rotate: bool) -> Vec<(Rectangle, Position, bool)> {
    let mut placed_rectangles: Vec<(Rectangle, Position, bool)> = Vec::new();
    let mut skyline: Vec<SkylineNode> = vec![SkylineNode { x: 0, y: 0, width: container_width }];

    for &rect in rectangles {
        let mut candidates = vec![(rect.width, rect.height, false)];
        if allow_rotate && rect.width != rect.height {
            candidates.push((rect.height, rect.width, true));
        }
        let mut best_y = u32::MAX;
        let mut best_x = 0;
        let mut best_idx = None;
        let mut best_rotated = false;

        for &(w, h, rotated) in &candidates {
            // 複数ノードにまたがって幅を満たす場合も正しく判定
            for (i, node) in skyline.iter().enumerate() {

                let mut y = node.y;
                let mut width_left = w;
                let mut j = i;
                while width_left > 0 && j < skyline.len() {
                    y = y.max(skyline[j].y);
                    if skyline[j].width >= width_left {
                        // 必要な幅がこのノードで満たせる
                        break;
                    }
                    width_left -= skyline[j].width;
                    j += 1;
                }
                // 幅が足りていない場合はスキップ
                if width_left > skyline.get(j).map_or(0, |n| n.width) {
                    continue;
                }
                // 高さがはみ出す場合はスキップ
                if y + h > container_height {
                    continue;
                }
                if y < best_y || (y == best_y && node.x < best_x) {
                    best_y = y;
                    best_x = node.x;
                    best_idx = Some(i);
                    best_rotated = rotated;

                }
            }
        }
        if let Some(idx) = best_idx {
            let pos = Position { x: best_x, y: best_y };
            placed_rectangles.push((rect, pos, best_rotated));
            let (w, h) = if best_rotated { (rect.height, rect.width) } else { (rect.width, rect.height) };
            // スカイラインを更新
            // 1. 配置した矩形の上に新ノードを追加
            let new_node = SkylineNode { x: best_x, y: best_y + h, width: w };
            skyline.insert(idx, new_node);
            // 2. 配置範囲にかかる既存ノードを分割・削除
            let i = idx + 1;
            while i < skyline.len() {
                if skyline[i].x < best_x + w {
                    let overlap = (best_x + w).saturating_sub(skyline[i].x);
                    if skyline[i].width > overlap {
                        skyline[i].x += overlap;
                        skyline[i].width -= overlap;
                        break;
                    } else {
                        skyline.remove(i);
                    }
                } else {
                    break;
                }
            }
            // 3. 隣接ノードの高さが同じなら結合
            let mut i = 0;
            while i + 1 < skyline.len() {
                if skyline[i].y == skyline[i + 1].y {
                    skyline[i].width += skyline[i + 1].width;
                    skyline.remove(i + 1);
                } else {
                    i += 1;
                }
            }
        }
    }
    placed_rectangles
}
