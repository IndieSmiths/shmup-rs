
use sdl2::EventPump;
use sdl2::video::WindowContext;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::image::InitFlag;


pub struct SdlStructs {
    pub canvas: WindowCanvas,
    pub texture_creator: TextureCreator<WindowContext>,
    pub event_pump: EventPump,
}


pub fn setup_and_get_structs() -> Result<SdlStructs, String> {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    /*
    XXX
    although aspect ratio is 16:9, the game world should be 4:3,
    with the remaining space at each side of the screen used for
    HUD
    */

    /* XXX
    // probably set window size with the width and height
    // grabbed as demonstrated below, but probably only if
    // the values are smaller than the resolution set further
    // below in canvas.set_logical_size
    //
    // also, when needed, research the different behaviour of
    // `current_display_mode` and `desktop_display_mode` methods

    let dm = video_subsystem.current_display_mode(0)?;
    let (width, height) = (dm.w, dm.h);
    println!("width: {}, height: {}", width, height);

    // */

    let window = video_subsystem.window("Shmup", 1920, 1080)
        .position_centered()
        .build()
        .unwrap();

    // XXX should we used a `software()` call (before build)
    // would there be a problem if I didn't use it?
    // and what about `accelerated()` instead?
    // research;
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_logical_size(960, 540).unwrap();

    let texture_creator = canvas.texture_creator();

    let event_pump = sdl_context.event_pump().unwrap();

    Ok (
        SdlStructs {
            canvas,
            texture_creator,
            event_pump,
        }
    )

}
