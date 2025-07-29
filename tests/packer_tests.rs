use rustpack::*;

#[test]
fn test_packer_basic_usage() {
    let mut packer = Packer::new();
    
    // 矩形を追加
    packer.add_rect(Rectangle { width: 10, height: 15 });
    packer.add_rect(Rectangle { width: 8, height: 12 });
    
    // ビンを追加
    packer.add_bin(100, 200);
    
    // パッキング実行
    packer.pack();
    
    // 結果を取得
    let rects = packer.rect_list();
    assert_eq!(rects.len(), 2);
    
    // 両方の矩形が配置されていることを確認
    assert!(rects.iter().all(|r| r.bin_id == 0));
    assert!(rects.iter().all(|r| r.x + r.width <= 100));
    assert!(rects.iter().all(|r| r.y + r.height <= 200));
}

#[test]
fn test_packer_with_rotation() {
    let mut packer = Packer::new();
    packer.set_pack_algo(PackAlgorithm::MaxRects);
    packer.enable_rotation();
    
    packer.add_rect(Rectangle { width: 20, height: 10 });
    packer.add_bin(15, 25);
    
    packer.pack();
    
    let rects = packer.rect_list();
    assert_eq!(rects.len(), 1);
    assert!(rects[0].rotated); // 回転されるはず
}

#[test]
fn test_packer_multiple_bins() {
    let mut packer = Packer::new();
    
    // 大きな矩形を複数追加
    packer.add_rect(Rectangle { width: 50, height: 50 });
    packer.add_rect(Rectangle { width: 50, height: 50 });
    
    // 小さなビンを追加
    packer.add_bin(60, 60);
    packer.add_bin(60, 60);
    
    packer.pack();
    
    let rects = packer.rect_list();
    assert_eq!(rects.len(), 2);
    
    // 異なるビンに配置されることを確認
    let bin_ids: std::collections::HashSet<_> = rects.iter().map(|r| r.bin_id).collect();
    assert_eq!(bin_ids.len(), 2);
}

#[test]
fn test_packer_algorithm_selection() {
    let mut packer = Packer::new();
    
    // アルゴリズムを設定
    packer.set_pack_algo(PackAlgorithm::Skyline);
    
    packer.add_rect(Rectangle { width: 10, height: 10 });
    packer.add_bin(20, 20);
    
    packer.pack();
    
    let rects = packer.rect_list();
    assert_eq!(rects.len(), 1);
}

#[test]
fn test_packer_insufficient_space() {
    let mut packer = Packer::new();
    
    // ビンに入らない大きな矩形
    packer.add_rect(Rectangle { width: 100, height: 100 });
    packer.add_bin(50, 50);
    
    packer.pack();
    
    let rects = packer.rect_list();
    assert_eq!(rects.len(), 0); // 配置できない
}

#[test]
fn test_packer_exact_fit() {
    let mut packer = Packer::new();
    
    packer.add_rect(Rectangle { width: 10, height: 10 });
    packer.add_bin(10, 10); // ぴったりサイズ
    
    packer.pack();
    
    let rects = packer.rect_list();
    assert_eq!(rects.len(), 1);
    assert_eq!(rects[0].x, 0);
    assert_eq!(rects[0].y, 0);
}