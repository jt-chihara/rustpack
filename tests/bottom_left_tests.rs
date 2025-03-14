use rustpack::{bottom_left_placement, Rectangle, Position};

#[test]
fn test_placement_basic() {
    let rectangles = vec![
        Rectangle { width: 3, height: 2 },
        Rectangle { width: 1, height: 4 },
        Rectangle { width: 2, height: 2 },
    ];

    let container_width = 5;
    let container_height = 5;

    let placed_rectangles = bottom_left_placement(&rectangles, container_width, container_height);

    assert_eq!(placed_rectangles.len(), 3);
}

#[test]
fn test_placement_with_overflow() {
    let rectangles = vec![
        Rectangle { width: 3, height: 3 },
        Rectangle { width: 3, height: 3 },
    ];

    let container_width = 5;
    let container_height = 5;

    let placed_rectangles = bottom_left_placement(&rectangles, container_width, container_height);

    assert_eq!(placed_rectangles.len(), 1);
}

#[test]
fn test_placement_no_space() {
    let rectangles = vec![
        Rectangle { width: 6, height: 6 },
    ];

    let container_width = 5;
    let container_height = 5;

    let placed_rectangles = bottom_left_placement(&rectangles, container_width, container_height);

    // 矩形が配置できない場合
    assert_eq!(placed_rectangles.len(), 0);
}

#[test]
fn test_placement_varied_sizes() {
    let rectangles = vec![
        Rectangle { width: 2, height: 2 },
        Rectangle { width: 3, height: 1 },
        Rectangle { width: 1, height: 3 },
        Rectangle { width: 4, height: 4 },
    ];

    let container_width = 5;
    let container_height = 5;

    let placed_rectangles = bottom_left_placement(&rectangles, container_width, container_height);

    assert_eq!(placed_rectangles.len(), 3);
}

#[test]
fn test_placement_exact_fit() {
    let rectangles = vec![
        Rectangle { width: 2, height: 2 },
        Rectangle { width: 3, height: 3 },
    ];

    let container_width = 5;
    let container_height = 5;

    let placed_rectangles = bottom_left_placement(&rectangles, container_width, container_height);

    assert_eq!(placed_rectangles.len(), 2);
}

#[test]
fn test_placement_positions() {
    let rectangles = vec![
        Rectangle { width: 2, height: 2 },
        Rectangle { width: 3, height: 2 },
    ];

    let container_width = 5;
    let container_height = 5;

    let placed_rectangles = bottom_left_placement(&rectangles, container_width, container_height);

    assert_eq!(placed_rectangles.len(), 2);
    
    assert_eq!(placed_rectangles[0].1, Position { x: 0, y: 0 });
    
    assert_eq!(placed_rectangles[1].1, Position { x: 2, y: 0 });
}
