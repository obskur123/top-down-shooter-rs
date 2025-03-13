use macroquad::prelude::*;
use macroquad::rand;


#[macroquad::main("top down shooter")]
async fn main() {
    let mut p = Rect { h: 20.0, w: 20.0, x: 0.0,  y: 0.0 };
    let mut ammo = Vec::<Ammo>::new(); 
    let mut shoot_flag = false;
    let mut enemies = Vec::<Enemy>::new();
    let mut enemy_flag = 0; //30 cap
    let mut game_loop = true;

    let mut player_image = Image::from_file_with_format(include_bytes!("assets/sprites/knight.png"), Some(ImageFormat::Png));

    let mut knight = player_image.unwrap().sub_image(Rect {
        x: 0.0,
        y: 0.0,
        w: 30.0,
        h:30.0
    });

    let mut player_texture = Texture2D::from_image(&knight);


    loop {

        while game_loop {

            enemy_flag += 1;
        
            clear_background(WHITE);
            
            //draw_rectangle(p.x, p.y, p.w, p.h, BLACK);
            draw_texture(&player_texture, p.x, p.y, WHITE);
            
            let keys = get_keys_down();

            if keys.contains(&KeyCode::S) {
                p.y += 3.0;
            }

            if keys.contains(&KeyCode::W) {
                p.y += -3.0;
            }
        
            if keys.contains(&KeyCode::D) {
                p.x += 3.0;
            }
            
            if keys.contains(&KeyCode::A) {
                p.x += -3.0;
            }

            Ammo::manage_ammo(&mut ammo, &mut enemies);
        
            Ammo::manage_shoot( (p.x, p.y), &mut shoot_flag, &mut ammo);    

            if enemy_flag == 120 {
                enemy_flag = 0;
                let enemy = Enemy::new((p.x, p.y));
                enemies.push(enemy);
            }

            if !enemies.is_empty() {

                enemies.iter_mut().for_each(|e| {
                    e.update((p.x, p.y));
                    e.collided_with_player(p, &mut game_loop);
                    e.draw();
                });

            }

            next_frame().await
        }

        
        while !game_loop {

            
            clear_background(BLACK);

            draw_text("GAME OVER", screen_width() / 2.0 - 200.0, screen_height() / 2.0, 100.0, RED);

            next_frame().await
        }

    }
}


struct Enemy {

    initial_position: Rect,
    player_position: (f32, f32),
    trajectory: (f32, f32),
    alive: bool

}


impl Enemy {

    fn new (player_position: (f32, f32)) -> Enemy {


        let random_x = rand::gen_range(0, screen_width() as i32);
        let random_y = rand::gen_range(0, screen_height() as i32);


        let r = (player_position.0 - random_x as f32 , player_position.1 - random_y as f32);
        let magnitude = f32::sqrt(r.0 * r.0 + r.1 * r.1);
        let trajectory = (r.0 / magnitude, r.1 / magnitude);

        let initial_position_rect = Rect {
            x: random_x as f32,
            y: random_y as f32,
            w: 20.0,
            h: 20.0
        };


        Enemy {
            initial_position: initial_position_rect,
            player_position,
            trajectory,
            alive: true

        }
    }


    fn update(&mut self, new_player_position: (f32, f32)) {
        let r = (new_player_position.0 - self.initial_position.x , new_player_position.1 - self.initial_position.y);
        let magnitude = f32::sqrt(r.0 * r.0 + r.1 * r.1);
        let trajectory = (r.0 / magnitude, r.1 / magnitude);

        self.trajectory = trajectory;
        self.initial_position.x += self.trajectory.0;
        self.initial_position.y += self.trajectory.1;
    }


    fn draw(&self) {
        draw_circle(self.initial_position.x, self.initial_position.y, 20.0, YELLOW);
    }

    fn collided_with_player(&mut self, player_rect: Rect, game_loop: &mut bool) {

        match self.initial_position.intersect(player_rect) {
            Some(_) => *game_loop = false,
            None => {}
        }

    }


}


#[derive(Copy, Clone)]
struct Ammo {
    //normalized vector trajectory
    trajectory: (f32, f32),
    initial_position: Rect,
    click_position: (f32, f32),
    out_of_bounds: bool

}


impl Ammo {
    
    fn update(&mut self, speed: f32) {
        self.initial_position.x += self.trajectory.0 * speed;
        self.initial_position.y += self.trajectory.1 * speed;
        self.is_out_of_bounds();
    }

    fn new(initial_position: (f32, f32), click_position: (f32, f32)) -> Ammo {
        let r = (click_position.0 - initial_position.0 , click_position.1 - initial_position.1);
        let magnitude = f32::sqrt(r.0 * r.0 + r.1 * r.1);
        let trajectory = (r.0 / magnitude, r.1 / magnitude);

        let initial_position_rect = Rect {
            x: initial_position.0,
            y: initial_position.1,
            w: 20.0,
            h: 20.0
        };

        return Ammo {
            initial_position: initial_position_rect,
            click_position,
            trajectory,
            out_of_bounds: false
        };
    }

    fn draw(&mut self) {
        draw_rectangle(self.initial_position.x, self.initial_position.y, self.initial_position.w, self.initial_position.h, RED);
    }


    fn is_out_of_bounds(&mut self) {
        let h = screen_height();
        let w = screen_width();
        if self.initial_position.y > h || self.initial_position.y < 0.0{
            self.out_of_bounds = true;
        }
        else if self.initial_position.x > w || self.initial_position.x < 0.0 {
            self.out_of_bounds = true;
        }
    }

    fn manage_ammo(ammo: &mut Vec<Ammo>, enemies: &mut Vec<Enemy>) {

        if !ammo.is_empty() {

            ammo.iter_mut().for_each(|x| {
                //x.manage_hit_enemy(enemy);
                enemies.iter_mut().for_each(|k| {
                    x.manage_hit_enemy(k);
                });


                x.update(5.0);
                x.draw();
            });

            enemies.retain(|e| e.alive);

            ammo.retain(|x| !x.out_of_bounds);
        
        }
    }

    fn manage_shoot(initial_position: (f32, f32), shoot_flag: &mut bool, ammo: &mut Vec<Ammo>) {

        if is_mouse_button_down(MouseButton::Left) && !*shoot_flag {
            let new_ammo = Ammo::new((initial_position.0, initial_position.1), mouse_position());
            ammo.push(new_ammo);
            *shoot_flag = true;
        }

        else if !is_mouse_button_down(MouseButton::Left) {
            *shoot_flag = false; 
        }
    }

    fn manage_hit_enemy(&mut self, enemy: &mut Enemy) {

        match self.initial_position.intersect(enemy.initial_position) {
            Some(_) => {
                println!("Hitted");
                self.out_of_bounds = true;
                enemy.alive = false
            },
            None => {}
        }
        

    }
}

        