use rustpack::*;

#[test]
fn test_maxrects_bssf() {
    let mut packer = Packer::new();
    packer.set_pack_algo(PackAlgorithm::MaxRectsBssf);
    
    packer.add_rect(Rectangle { width: 10, height: 5 });
    packer.add_rect(Rectangle { width: 8, height: 6 });
    packer.add_bin(20, 15);
    
    packer.pack();
    
    let rects = packer.rect_list();
    assert_eq!(rects.len(), 2);
}

#[test]
fn test_maxrects_baf() {
    let mut packer = Packer::new();
    packer.set_pack_algo(PackAlgorithm::MaxRectsBaf);
    
    packer.add_rect(Rectangle { width: 12, height: 8 });
    packer.add_rect(Rectangle { width: 6, height: 4 });
    packer.add_bin(20, 15);
    
    packer.pack();
    
    let rects = packer.rect_list();
    assert_eq!(rects.len(), 2);
}

#[test]
fn test_maxrects_blsf() {
    let mut packer = Packer::new();
    packer.set_pack_algo(PackAlgorithm::MaxRectsBlsf);
    
    packer.add_rect(Rectangle { width: 5, height: 5 });
    packer.add_rect(Rectangle { width: 3, height: 3 });
    packer.add_bin(10, 10);
    
    packer.pack();
    
    let rects = packer.rect_list();
    assert_eq!(rects.len(), 2);
}

#[test]
fn test_skyline_bl() {
    let mut packer = Packer::new();
    packer.set_pack_algo(PackAlgorithm::SkylineBl);
    
    packer.add_rect(Rectangle { width: 6, height: 4 });
    packer.add_rect(Rectangle { width: 4, height: 6 });
    packer.add_bin(12, 8);
    
    packer.pack();
    
    let rects = packer.rect_list();
    assert_eq!(rects.len(), 2);
}

#[test]
fn test_guillotine_bssf_sas() {
    let mut packer = Packer::new();
    packer.set_pack_algo(PackAlgorithm::GuillotineBssfSas);
    
    packer.add_rect(Rectangle { width: 8, height: 6 });
    packer.add_rect(Rectangle { width: 4, height: 3 });
    packer.add_bin(15, 10);
    
    packer.pack();
    
    let rects = packer.rect_list();
    assert_eq!(rects.len(), 2);
}