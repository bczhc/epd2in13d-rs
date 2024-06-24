pub const EPD_2IN13D_WIDTH: usize = 104;
pub const EPD_2IN13D_HEIGHT: usize = 212;

#[link(name = "epd2in13d")]
extern "system" {
    pub fn DEV_Delay_ms(ms: f64);
    pub fn DEV_Module_Init() -> u8;
    pub fn EPD_2IN13D_Init();
    pub fn EPD_2IN13D_Clear();
    pub fn EPD_2IN13D_Display(image: *mut u8);
    pub fn EPD_2IN13D_DisplayPart(image: *mut u8);
    pub fn EPD_2IN13D_Sleep();
}
