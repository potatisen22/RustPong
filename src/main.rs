use tetra::graphics::{Color, Rectangle, DrawParams, Texture}; //we will use textures
use tetra::input::{self, Key}; //we will take input keys
use tetra::math::Vec2; //will need 2d vector
use tetra::window;
use tetra::{Context, ContextBuilder, State}; //Will need context and state of game

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;

const PADDLE_SPEED: f32 = 8.0;

const BALL_SPEED: f32 = 5.0;
const PADDLE_SPIN: f32 = 4.0;

const BALL_ACC: f32 = 0.05;
fn main() -> tetra::Result
{
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct Entity
{
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl Entity
{
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity::with_velocity(texture, position, Vec2::zero())
    }
    fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity
    {
        Entity
        {
            texture,
            position,
            velocity,
        }
    }
    fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    fn height(&self) -> f32 {
        self.texture.height() as f32
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }

    fn centre(&self) -> Vec2<f32>
    {
        Vec2::new(
            self.position.x + (self.width()/2.0),
            self.position.y + (self.height()/2.0),
        )
    }

}
struct GameState
{
    player1: Entity,
    player2: Entity,
    ball: Entity,
}

impl GameState
{
    fn new(ctx: &mut Context) -> tetra::Result<GameState>
    {
        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);
        let ball_texture = Texture::new(ctx, "./resources/ballGrey.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH/2.0,
            WINDOW_HEIGHT/2.0,
        );

        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player1_position = Vec2::new(
            8.0,
            (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0);

        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let player2_position = Vec2::new(
            WINDOW_WIDTH - 32.0,
            (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
        );

        Ok(GameState{
            player1: Entity::new(player1_texture, player1_position),
            player2: Entity::new(player2_texture, player2_position),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity),
        })
    }
}
impl State for GameState
{
    fn update(&mut self, ctx: &mut Context) -> tetra::Result //update method is implemented in tetra so that it is called 60 times per second, Always.
    {
        self.ball.position += self.ball.velocity;

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

        let player1_bounds = self.player1.bounds();
        let player2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();

        let paddle_hit = if ball_bounds.intersects(&player1_bounds)
        {
            Some(&self.player1)
        }
        else if ball_bounds.intersects(&player2_bounds)
        {
            Some(&self.player2)
        }
        else { None };


        if let Some(paddle) = paddle_hit
        {
            //Increase balls velocity and flip x direction
            self.ball.velocity.x =
                -(self.ball.velocity.x +(BALL_ACC * self.ball.velocity.x.signum()));
            //calculate offset of the hit [-1,1]
            let offset = (paddle.centre().y - self.ball.centre().y)/ paddle.height();

            //apply spin
            self.ball.velocity.y += PADDLE_SPIN  * -offset;

        }

        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT
        {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        if self.ball.position.x < 0.0
        {
            window::quit(ctx);
            println!("Player 2 wins!")
        }

        if self.ball.position.x > WINDOW_WIDTH
        {
            window::quit(ctx);
            println!("Player 1 wins!")
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result
    {
        tetra::graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        self.player1.texture.draw(ctx, DrawParams::new().position(self.player1.position));
        self.player2.texture.draw(ctx, DrawParams::new().position(self.player2.position));
        self.ball.texture.draw(ctx, self.ball.position);
        Ok(())
    }
}
