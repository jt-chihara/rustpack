use rustpack::*;

#[test]
fn test_bottom_left_no_rotate() {
    let rects = [
        Rectangle { width: 4, height: 2 },
        Rectangle { width: 3, height: 3 },
        Rectangle { width: 2, height: 5 },
    ];
    let res = bottom_left_placement(&rects, 8, 8, false);
    assert_eq!(res.len(), 3);
    // 配置位置や回転有無を個別に検証してもよい
}

#[test]
fn test_bottom_left_with_rotate() {
    let rects = [
        Rectangle { width: 4, height: 2 },
        Rectangle { width: 2, height: 4 },
    ];
    let res = bottom_left_placement(&rects, 4, 4, true);
    assert_eq!(res.len(), 2);
    assert!(res.iter().any(|&(_, _, rotated)| rotated));
}

#[test]
fn test_maxrects_with_rotate() {
    let rects = [
        Rectangle { width: 5, height: 2 },
        Rectangle { width: 2, height: 5 },
    ];
    let res = maxrects_placement(&rects, 5, 5, true);
    assert_eq!(res.len(), 2);
    assert!(res.iter().any(|&(_, _, rotated)| rotated));
}

#[test]
fn test_skyline_with_rotate() {
    let rects = [
        Rectangle { width: 2, height: 2 },
        Rectangle { width: 2, height: 2 },
    ];
    let res = skyline_placement(&rects, 4, 2, true);
    assert_eq!(res.len(), 2);
    // 回転は不要なケースだが、まず配置数確認

}

#[test]
fn test_guillotine_with_rotate() {
    let rects = [
        Rectangle { width: 2, height: 7 },
        Rectangle { width: 7, height: 2 },
    ];
    let res = guillotine_placement(&rects, 7, 7, true);
    assert_eq!(res.len(), 2);
    assert!(res.iter().any(|&(_, _, rotated)| rotated));
}
