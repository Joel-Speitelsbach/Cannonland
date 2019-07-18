use battlefield::shot_type::ShotType;
use sdl2;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::{Point,Rect};
use sdl2::event::{Event,WindowEvent};
use sdl2::render::{Canvas,Texture,TextureCreator,BlendMode};
use sdl2::video::{WindowContext,Window};
use sdl2::image::LoadTexture;

use battlefield::{self,grid,Battlefield,shot_type};
use util;


const WIN_SCALE: i32 = 2;


pub fn new_window(sdl2_video: &sdl2::VideoSubsystem, size: (i32,i32)) -> Canvas<Window> {
    let (width, height) = size;
    let video_subsystem = sdl2_video;
    let window =
        video_subsystem
        .window("Cannonland",
            width as u32,
            height as u32)
        .build()
        .unwrap();
    let mut canvas =
        window
        .into_canvas()
        // .software()
        .build()
        .unwrap();
    canvas.window_mut().set_size(
        (width  * WIN_SCALE) as u32,
        (height * WIN_SCALE) as u32,
    ).unwrap();
    canvas.window_mut().set_position(
        sdl2::video::WindowPos::Centered,
        sdl2::video::WindowPos::Centered);
    canvas.set_scale(
        WIN_SCALE as f32, 
        WIN_SCALE as f32
    ).unwrap();
    canvas
}


pub struct PresenterState<'res> {
    canvas: &'res mut sdl2::render::Canvas<Window>,
    missile: Texture<'res>,
    grid_texture: Texture<'res>,
    _texture_creator: &'res TextureCreator<WindowContext>,

    prof_canvas_present: util::time::Prof,
    prof_canvas_copy: util::time::Prof,
    prof_pixel_data: util::time::Prof,
}
impl<'res> PresenterState<'res> {
    pub fn new (
        canvas: &'res mut Canvas<Window>,
        _texture_creator: &'res TextureCreator<WindowContext>,
        size: (i32,i32),
    ) -> PresenterState<'res> {
        let missile = _texture_creator.load_texture("./pics/missile.png").unwrap();

        let (width,height) = size;
            
        // create texture. the "Blend" mode makes sure
        // that the background ist not overwritten with black
        let mut grid_texture = _texture_creator.create_texture(
                 PixelFormatEnum::RGBA8888,
                 sdl2::render::TextureAccess::Static,
                 width as u32,
                 height as u32,
            ).unwrap();
        grid_texture.set_blend_mode(BlendMode::Blend);

        PresenterState {
            canvas,
            missile,
            _texture_creator,
            grid_texture,
            prof_canvas_present: util::time::Prof::just_label("_canvas_present"),
            prof_canvas_copy: util::time::Prof::just_label("_canvas_copy"),
            prof_pixel_data: util::time::Prof::just_label("_pixel_data"),
        }
    }
}


pub struct Presenter<'st,'b, 'res> {
    state: &'st mut PresenterState<'res>,
    battlefield: &'b Battlefield,
}
impl<'st,'b, 'res> Presenter<'st,'b, 'res> {
    pub fn new(
        state: &'st mut PresenterState<'res>,
        battlefield: &'b Battlefield,
    ) -> Presenter<'st,'b, 'res>
    {
        Presenter {
            state,
            battlefield,
        }
    }

    pub fn present(&mut self) -> () {
        self.draw_grid();
        self.draw_bunkers();
        self.draw_shots();

        self.state.prof_canvas_present.start();
        self.state.canvas.present();
        self.state.prof_canvas_present.pause();
    }

    pub fn respond_to(&mut self, event: &Event) {
        match *event {
            Event::Window{win_event: WindowEvent::Resized (_,_), ..} => 
                self.rescale_canvas(),
            _ => (),
        }
    }

    fn rescale_canvas(&mut self) {
        let (x,y) = self.state.canvas.window().size();
        self.state.canvas.set_scale
            (x as f32 / self.battlefield.grid.width  as f32,
             y as f32 / self.battlefield.grid.height as f32).unwrap();
    }
}


// draw grid
impl<'st,'b, 'res> Presenter<'st,'b, 'res> {
    fn draw_grid(&mut self) -> () {
        self.draw_background();
        self.draw_particles();
    }


    fn grid(&self) -> &grid::Grid {&self.battlefield.grid}


    fn draw_background(&mut self) -> () {
        self.state.canvas.set_draw_color(pixels::Color::RGBA(64,92,128,255));
        self.state.canvas.clear();
    }


    fn draw_particles(&mut self) -> () {
        let (width,height) = (
                self.battlefield.grid.width,
                self.battlefield.grid.height,
            );

        // create (raw) pixel data
        self.state.prof_pixel_data.start();
        let mut pixel_data = Vec::with_capacity(width*height*4);
        for (_y, row) in (&self.grid().grid).into_iter().enumerate() {
            for (_x, particle) in (&row).into_iter().enumerate() {
                let (r,g,b,a) = particle.get_rgba();
                pixel_data.push(a);
                pixel_data.push(b);
                pixel_data.push(g);
                pixel_data.push(r);
            }
        }
        self.state.prof_pixel_data.pause();

        // copy pixel_data into texture then into canvas
        self.state.prof_canvas_copy.start();
        self.state.grid_texture.update(None, &pixel_data, width*4).unwrap();
        self.state.canvas.copy(&self.state.grid_texture, None, None).unwrap();
        self.state.prof_canvas_copy.pause();
    }
}


// draw bunkers
impl<'st,'b, 'res> Presenter<'st,'b, 'res> {

    fn draw_bunkers(&mut self) -> () {
        for bunker in &self.battlefield.bunkers {
            if !bunker.is_alive() { continue; }

            let rgba: (u8,u8,u8,u8) = bunker.get_rgba();
            let color = pixels::Color::RGBA(rgba.0, rgba.1, rgba.2, rgba.3);

            Presenter::draw_cannon(&mut self.state.canvas, &bunker, color);
            Presenter::draw_building(&mut self.state.canvas, &bunker, color);
            self.draw_weapon(&bunker);
            Presenter::draw_charge(&mut self.state.canvas, &bunker);
            Presenter::draw_health(&mut self.state.canvas, &bunker);
        }
    }

    fn draw_cannon(canvas: &mut sdl2::render::Canvas<Window>, bunker: &battlefield::bunker::Bunker, color: pixels::Color) {
        let cannon_pos: (i16,i16,i16,i16) = bunker.get_cannon_pos_x1y1x2y2();
        canvas.aa_line(
            cannon_pos.0, cannon_pos.1,
            cannon_pos.2, cannon_pos.3,
            color).unwrap();
    }

    fn draw_building(canvas: &mut sdl2::render::Canvas<Window>, bunker: &battlefield::bunker::Bunker, color: pixels::Color) {
        canvas.filled_pie(bunker.x_pos, bunker.y_pos, bunker.get_radius() as i16, 180, 360, color).unwrap();
    }

    fn draw_weapon(&mut self, bunker: &battlefield::bunker::Bunker) {
        let y = bunker.y_pos - 4;
        let x = bunker.x_pos;

        match bunker.get_current_weapon() {
            shot_type::ShotType::CANNON => {
                self.draw_default_shot(&bunker.get_current_weapon(), x, y);
            },
            shot_type::ShotType::ROCKET  => {
                Self::draw_texture_shot(&mut self.state.canvas, &self.state.missile, x as i32,y as i32, 6, 12, 60.0);
            },
            shot_type::ShotType::SNOW => {
                self.draw_default_shot(&bunker.get_current_weapon(), x, y);
            }
        }
    }

    fn draw_charge(canvas: &mut sdl2::render::Canvas<Window>, bunker: &battlefield::bunker::Bunker) {
        let divisor = 4;

        let y1 = bunker.y_pos + 1;
        let y2 = y1 + 5;
        let x_zero = bunker.x_pos - (bunker.get_max_charge() as i16/2/divisor);
        let x_current = x_zero + bunker.get_charge() as i16/divisor;
        let x_max = x_zero + bunker.get_max_charge() as i16/divisor;

        canvas.box_(x_zero, y1, x_max, y2, pixels::Color::RGBA(128,128,128,128)).unwrap();
        canvas.box_(x_zero, y1, x_current, y2, pixels::Color::RGBA(0,0,255,255)).unwrap();
    }

    fn draw_health(canvas: &mut sdl2::render::Canvas<Window>, bunker: &battlefield::bunker::Bunker) {
        let divisor = 4;

        let y1 = bunker.y_pos + 7;
        let y2 = y1 + 5;
        let x_zero = bunker.x_pos - (bunker.get_max_health() as i16/2/divisor);
        let x_current = x_zero + bunker.get_health() as i16/divisor;
        let x_max = x_zero + bunker.get_max_health() as i16/divisor;

        canvas.box_(x_zero, y1, x_max, y2, pixels::Color::RGBA(255,0,0,128)).unwrap();
        canvas.box_(x_zero, y1, x_current, y2, pixels::Color::RGBA(0,255,0,255)).unwrap();
    }

}

// draw shots
impl<'st,'b, 'res> Presenter<'st,'b, 'res> {

    fn draw_shots(&mut self) -> () {
        for shot in &self.battlefield.shots {
            let shot_type = &shot.shot_type;

            match shot_type {
                shot_type::ShotType::CANNON => {
                    self.draw_default_shot(shot_type, shot.x_pos as i16, shot.y_pos as i16);
                },
                shot_type::ShotType::ROCKET  => {
                    Self::draw_texture_shot(
                        &mut self.state.canvas, 
                        &self.state.missile, 
                        shot.x_pos as i32, shot.y_pos as i32, 
                        8, 16, 
                        shot.get_angle() as f64
                    );
                },
                shot_type::ShotType::SNOW => {
                    self.draw_default_shot(shot_type, shot.x_pos as i16, shot.y_pos as i16);
                }
            }
        }
    }

    fn draw_default_shot(&self, shot_type: &ShotType, x_pos: i16, y_pos: i16) {
        self.state.canvas.filled_circle(x_pos, y_pos, shot_type.get_radius() as i16, shot_type.get_rgba()).unwrap();
    }

    fn draw_texture_shot(
        canvas: &mut Canvas<Window>, 
        texture: &Texture, 
        x_pos: i32, y_pos: i32,
        width: u32, height: u32, 
        angle: f64
    ) {
        let x_offset = width as i32/2;
        let y_offset = height as i32/2;
        let destination = Rect::new(x_pos-x_offset, y_pos-y_offset, width, height);
        let rotation_point = Point::new(x_offset, y_offset);

        canvas.copy_ex(texture, None, destination, angle, rotation_point, false, false).unwrap();
    }

}
