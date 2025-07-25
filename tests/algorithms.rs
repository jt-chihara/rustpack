use rustpack::*;

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
