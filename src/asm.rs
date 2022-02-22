use core::arch::asm;

pub fn load_eflags() -> i32
{
    let mut result;

    unsafe
    {
        asm!("pushfd");
        asm!("pop {}", out(reg) result);
    }
    return result;
}

pub fn store_eflags(flags: i32)
{
    unsafe
    {
        asm!("mov eax, {}", in(reg) flags);
        asm!("push eax");
        asm!("popfd");
    }
}

pub fn hlt()
{
    unsafe
    {
        asm!("hlt");
    }
}

pub fn cli()
{
    unsafe
    {
        asm!("cli");
    }
}

pub fn out8(port: u32, data: u8)
{
    unsafe
    {
        asm!("out dx, al", in("edx") port, in("al") data);
    }
}