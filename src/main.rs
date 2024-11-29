use tetra::graphics::{self, Color, DrawParams, Texture}; //we will use textures
use tetra::input::{self, Key}; //we will take input keys
use tetra::math::Vec2; //will need 2d vector
use tetra::{Context, ContextBuilder, State}; //Will need context and state of game


const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;

const PADDLE_SPEED: f32 = 8.0;
fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct GameState
{
    paddle_texture: Texture,
    paddle_position: Vec2<f32>,
}

impl GameState
{
    fn new(ctx: &mut Context) -> tetra::Result<GameState>
    {
        let paddle_texture = Texture::new(ctx, "./resources/player1.png")?;
        let paddle_position =
            Vec2::new(32.0, (WINDOW_HEIGHT - paddle_texture.height() as f32) / 2.0);
        Ok(GameState{paddle_texture, paddle_position})
    }
}
impl State for GameState
{
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result
    {
        tetra::graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        let rotation :f32= 90.0;
        self.paddle_texture.draw(ctx, DrawParams::new().position(self.paddle_position).rotation(rotation.to_radians()));

        Ok(())
    }
    fn update(&mut self, ctx: &mut Context) -> tetra::Result //update method is implemented in tetra so that it is called 60 times per second, Always.
    {
        if input::is_key_down(ctx, Key::W)
        {
            self.paddle_position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::S)
        {
            self.paddle_position.y += PADDLE_SPEED;
        }
        Ok(())
    }
}
