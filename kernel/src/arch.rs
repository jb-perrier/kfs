#[no_mangle]
pub extern "C" fn x86_prep_page_table(buf: &mut [u32; 1024])
{
	for i in 0u32 .. 1024
	{
		buf[i as usize] = i * 0x1000 + 3;
	}
}