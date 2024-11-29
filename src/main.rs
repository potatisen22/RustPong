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

struct Entity {
    texture: Texture,
    position: Vec2<f32>,
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity { texture, position }
    }
}
struct GameState
{
    player1: Entity,
    player2: Entity,
}

impl GameState
{
    fn new(ctx: &mut Context) -> tetra::Result<GameState>
    {
        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player1_position = Vec2::new(
            32.0,
            (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0);

        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let player2_position = Vec2::new(
            WINDOW_WIDTH - player2_texture.width() as f32 - 32.0,
            (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
        );



        Ok(GameState{
            player1: Entity::new(player1_texture, player1_position),
            player2: Entity::new(player2_texture, player2_position),
        })
    }
}
impl State for GameState
{
    fn update(&mut self, ctx: &mut Context) -> tetra::Result //update method is implemented in tetra so that it is called 60 times per second, Always.
    {
        if input::is_key_down(ctx, Key::W)
        {
            self.player1.position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::S)
        {
            self.player1.position.y += PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Up)
        {
            self.player2.position.y -= PADDLE_SPEED;
        }
        if input::is_key_down(ctx, Key::Down)
        {
            self.player2.position.y += PADDLE_SPEED;
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result
    {
        tetra::graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        let rotation :f32= 90.0;
        self.player1.texture.draw(ctx, DrawParams::new().position(self.player1.position).rotation(rotation.to_radians()));
        self.player2.texture.draw(ctx, DrawParams::new().position(self.player2.position).rotation(rotation.to_radians()));

        Ok(())
    }

}
