use crate::{Rectangle, bottom_left_placement, maxrects_placement, skyline_placement, guillotine_placement};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackAlgorithm {
    BottomLeft,
    MaxRects,
    MaxRectsBssf,  // Best Short Side Fit
    MaxRectsBaf,   // Best Area Fit
    MaxRectsBlsf,  // Best Long Side Fit
    Skyline,
    SkylineBl,     // Bottom Left
    Guillotine,
    GuillotineBssfSas, // Best Short Side Fit - Shorter Axis Split
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bin {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackedRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub rotated: bool,
    pub bin_id: usize,
}

pub struct Packer {
    rects: Vec<Rectangle>,
    bins: Vec<Bin>,
    algorithm: PackAlgorithm,
    allow_rotation: bool,
    packed_rects: Vec<PackedRect>,
}

impl Packer {
    pub fn new() -> Self {
        Self {
            rects: Vec::new(),
            bins: Vec::new(),
            algorithm: PackAlgorithm::MaxRects,
            allow_rotation: false,
            packed_rects: Vec::new(),
        }
    }

    pub fn add_rect(&mut self, rect: Rectangle) {
        self.rects.push(rect);
    }

    pub fn add_bin(&mut self, width: u32, height: u32) {
        self.bins.push(Bin { width, height });
    }

    pub fn set_pack_algo(&mut self, algo: PackAlgorithm) {
        self.algorithm = algo;
    }

    pub fn enable_rotation(&mut self) {
        self.allow_rotation = true;
    }

    pub fn disable_rotation(&mut self) {
        self.allow_rotation = false;
    }

    pub fn pack(&mut self) {
        self.packed_rects.clear();
        
        let mut remaining_rects = self.rects.clone();
        
        for (bin_id, bin) in self.bins.iter().enumerate() {
            if remaining_rects.is_empty() {
                break;
            }
            
            let placed = match self.algorithm {
                PackAlgorithm::BottomLeft => {
                    bottom_left_placement(&remaining_rects, bin.width, bin.height, self.allow_rotation)
                }
                PackAlgorithm::MaxRects | 
                PackAlgorithm::MaxRectsBssf | 
                PackAlgorithm::MaxRectsBaf | 
                PackAlgorithm::MaxRectsBlsf => {
                    maxrects_placement(&remaining_rects, bin.width, bin.height, self.allow_rotation)
                }
                PackAlgorithm::Skyline | 
                PackAlgorithm::SkylineBl => {
                    skyline_placement(&remaining_rects, bin.width, bin.height, self.allow_rotation)
                }
                PackAlgorithm::Guillotine | 
                PackAlgorithm::GuillotineBssfSas => {
                    guillotine_placement(&remaining_rects, bin.width, bin.height, self.allow_rotation)
                }
            };
            
            // 配置された矩形をPackedRectに変換
            for &(orig_rect, pos, rotated) in placed.iter() {
                let (width, height) = if rotated {
                    (orig_rect.height, orig_rect.width)
                } else {
                    (orig_rect.width, orig_rect.height)
                };
                
                self.packed_rects.push(PackedRect {
                    x: pos.x,
                    y: pos.y,
                    width,
                    height,
                    rotated,
                    bin_id,
                });
                
                // 配置された矩形を残りのリストから削除
                if let Some(rect_idx) = remaining_rects.iter().position(|&r| r == orig_rect) {
                    remaining_rects.remove(rect_idx);
                }
            }
        }
    }

    pub fn rect_list(&self) -> &[PackedRect] {
        &self.packed_rects
    }
}

impl Default for Packer {
    fn default() -> Self {
        Self::new()
    }
}