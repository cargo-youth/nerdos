use crate::sync::UPIntrFreeCell;
use alloc::vec::Vec;
use lazy_static::*;
use virtio_drivers::Hal;
use crate::mm::PhysFrame;
use crate::mm::address::phys_to_virt;
use crate::mm::address::virt_to_phys;

lazy_static! {
    static ref QUEUE_FRAMES: UPIntrFreeCell<Vec<PhysFrame>> =
        unsafe { UPIntrFreeCell::new(Vec::new()) };
}

pub struct VirtioHal;

impl Hal for VirtioHal {
    fn dma_alloc(pages: usize) -> usize {
        let trakcers = PhysFrame::frame_alloc_more(pages);
        let pa = trakcers.as_ref().unwrap().last().unwrap().start_paddr();
        QUEUE_FRAMES
            .exclusive_access()
            .append(&mut trakcers.unwrap());
        pa.as_usize()
    }

    fn dma_dealloc(_pa: usize, _pages: usize) -> i32 {
        0
    }

    fn phys_to_virt(addr: usize) -> usize {
        phys_to_virt(addr)
    }

    fn virt_to_phys(vaddr: usize) -> usize {
        virt_to_phys(vaddr)
    }
}
