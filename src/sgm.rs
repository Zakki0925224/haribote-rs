use crate::asm;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SegmentDescriptor
{
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access_right: u8,
    limit_high: u8,
    base_high: u8
}

impl SegmentDescriptor
{
    fn new(mut limit: u32, base: i32, mut ar: i32) -> SegmentDescriptor
    {
        if limit > 0xfffff
        {
            ar |= 0x8000;
            limit /= 0x1000;
        }

        return SegmentDescriptor
        {
            limit_low: limit as u16,
            base_low: base as u16,
            base_mid: (base >> 16) as u8,
            access_right: ar as u8,
            limit_high: ((limit >> 16) as u8 & 0x0f) | ((ar >> 8) as u8 & 0xf0),
            base_high: (base >> 24) as u8
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct GateDescriptor
{
    offset_low: u16,
    selector: u16,
    dw_count: u8,
    access_right: u8,
    offset_high: u16
}

impl GateDescriptor
{
    fn new(offset: u32, selector: i32, ar: i32) -> GateDescriptor
    {
        return GateDescriptor
        {
            offset_low: offset as u16,
            selector: selector as u16,
            dw_count: (ar >> 8) as u8,
            access_right: ar as u8,
            offset_high: (offset >> 16) as u16
        }
    }
}

const GDT_ADDR: i32 = 0x00270000;
const IDT_ADDR: i32 = 0x0026f800;

pub struct Segmentation
{
    pub gdt: SegmentDescriptor,
    pub idt: GateDescriptor
}

impl Segmentation
{
    pub fn new() -> Segmentation
    {
        return Segmentation
        {
            gdt: unsafe { *(GDT_ADDR as *const SegmentDescriptor) },
            idt: unsafe { *(IDT_ADDR as *const GateDescriptor) }
        }
    }

    pub fn init(&mut self)
    {
        // gdt初期化
        for i in 0..8192
        {
            let gdt = unsafe { &mut *((GDT_ADDR + i * 8) as *mut SegmentDescriptor) };
            *gdt = SegmentDescriptor::new(0, 0, 0);
        }

        let gdt = unsafe { &mut *((GDT_ADDR + 1 * 8) as *mut SegmentDescriptor) };
        *gdt = SegmentDescriptor::new(0xffffffff, 0x00000000, 0x4092);

        let gdt = unsafe { &mut *((GDT_ADDR + 2 * 8) as *mut SegmentDescriptor) };
        *gdt = SegmentDescriptor::new(0x0007ffff, 0x00280000, 0x409a);

        asm::load_gdtr(0xffff, 0x00270000);

        // idt初期化
        for i in 0..256
        {
            let idt = unsafe { &mut *((IDT_ADDR + i * 8) as *mut GateDescriptor) };
            *idt = GateDescriptor::new(0, 0, 0);
        }

        asm::load_idtr(0x7ff, 0x0026f800);
    }
}