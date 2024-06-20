use pronto_graphics::Window;
use pronto_graphics::Color;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const WIDTH_F: f32 = 800.;
const HEIGHT_F: f32 = 600.;

const RADIUS: f32 = 50.;

struct Ball {
    pos_x: f32,
    pos_y: f32,

    vel_x: f32,
    vel_y: f32,

    color: Color
}

struct BallReturn<'a> {
   window: Window<'a>,
    ball: Ball
}
fn update_ball(mut pg: Window, mut ball: Ball) -> BallReturn {
    pg.fill_color(ball.color);

    ball.pos_x += ball.vel_x;
    ball.pos_y += ball.vel_y;

    if ball.pos_x > WIDTH_F - RADIUS || ball.pos_x < 0. + RADIUS {
        ball.vel_x *= -1.
    }

    if ball.pos_y > HEIGHT_F - RADIUS || ball.pos_y < 0. + RADIUS {
        ball.vel_y *= -1.
    }

    pg.circle((ball.pos_x, ball.pos_y), 50.);

    return BallReturn { window: pg, ball: ball }
}

fn main() {
    let mut pg = Window::new(WIDTH, HEIGHT, "I lost.");

    let mut ball1 = Ball {
        pos_x: RADIUS,
        pos_y: RADIUS,
        vel_x: 2.,
        vel_y: 2.,
        color: Color::RED
    };
    let mut ball2 = Ball {
        pos_x: WIDTH_F - RADIUS,
        pos_y: RADIUS,
        vel_x: -2.,
        vel_y: 2.,
        color: Color::BLACK
    };

    loop {
        let ball1ret = update_ball(pg, ball1);
        pg = ball1ret.window;

        let ball2ret = update_ball(pg, ball2);
        pg = ball2ret.window;

        ball1 = ball1ret.ball;
        ball2 = ball2ret.ball;
        
        pg.update();
    }
}

