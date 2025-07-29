use rustpack::*;
use std::time::Instant;

#[test]
fn test_performance_comparison() {
    let rects: Vec<Rectangle> = (0..50).map(|i| Rectangle {
        width: (i % 10) + 5,
        height: (i % 8) + 3,
    }).collect();
    
    let algorithms = [
        PackAlgorithm::BottomLeft,
        PackAlgorithm::MaxRects,
        PackAlgorithm::Skyline,
        PackAlgorithm::Guillotine,
    ];
    
    for algo in algorithms.iter() {
        let start = Instant::now();
        
        let mut packer = Packer::new();
        packer.set_pack_algo(*algo);
        
        for rect in &rects {
            packer.add_rect(*rect);
        }
        packer.add_bin(100, 100);
        packer.add_bin(100, 100);
        
        packer.pack();
        let duration = start.elapsed();
        
        let placed_count = packer.rect_list().len();
        println!("{:?}: {}ms, {} rectangles placed", algo, duration.as_millis(), placed_count);
        
        // 少なくとも30個は配置できるはず
        assert!(placed_count >= 30);
    }
}

#[test]
fn test_packer_stress_test() {
    let rects: Vec<Rectangle> = (0..100).map(|i| Rectangle {
        width: (i % 15) + 2,
        height: (i % 12) + 2,
    }).collect();
    
    let mut packer = Packer::new();
    packer.set_pack_algo(PackAlgorithm::MaxRects);
    packer.enable_rotation();
    
    for rect in &rects {
        packer.add_rect(*rect);
    }
    
    // 複数のビンを用意
    for _ in 0..10 {
        packer.add_bin(50, 50);
    }
    
    let start = Instant::now();
    packer.pack();
    let duration = start.elapsed();
    
    let placed_count = packer.rect_list().len();
    println!("Stress test: {}ms, {}/{} rectangles placed", 
             duration.as_millis(), placed_count, rects.len());
    
    // ストレステストでも高い配置率を期待
    assert!(placed_count >= 80);
}

#[test]  
fn test_rotation_effectiveness() {
    let rects = vec![
        Rectangle { width: 20, height: 5 },
        Rectangle { width: 5, height: 20 },
        Rectangle { width: 15, height: 8 },
    ];
    
    // 回転なし
    let mut packer_no_rot = Packer::new();
    for rect in &rects {
        packer_no_rot.add_rect(*rect);
    }
    packer_no_rot.add_bin(25, 25);
    packer_no_rot.pack();
    let without_rotation = packer_no_rot.rect_list().len();
    
    // 回転あり
    let mut packer_with_rot = Packer::new();
    packer_with_rot.enable_rotation();
    for rect in &rects {
        packer_with_rot.add_rect(*rect);
    }
    packer_with_rot.add_bin(25, 25);
    packer_with_rot.pack();
    let with_rotation = packer_with_rot.rect_list().len();
    
    println!("Without rotation: {} placed, With rotation: {} placed", 
             without_rotation, with_rotation);
    
    // 回転ありの方が同等以上の配置数を期待
    assert!(with_rotation >= without_rotation);
}