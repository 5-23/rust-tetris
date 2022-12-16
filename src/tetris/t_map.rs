
// 이건 생각해보니 t_block이라는 모듈이 이미 정의되어있어서 mod super::t_block으로 쓸 필요가 없었네요
// 그냥 지워주시면 됩니다 use만써주고
// mod는 새 모듈을 만들어주는키워든데 mod.rs에서 t_block모듈을 생성해줬으니까요

// 아하

use rand::{Rng, thread_rng};
use std::fs::{File, self};
use std::io::{stdout};

use crossterm::execute;
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, ResetColor, Color};
use super::t_block::Tblock;
use super::t_move::Move;
use super::t_built_in::built_in::{make_shape, self, cls};
use std::thread;

#[derive(Debug, Clone)]
pub struct Tmap {
    pub map: Vec<Vec<usize>>,
    pub block: Tblock,
    pub point: usize,
    pub best_point: usize
}



/*
# 1
⬜⬜⬜⬜

# 2
⬛🟪
🟪🟪🟪

# 3
🟨🟨
🟨🟨

# 4
🟦
🟦🟦🟦

# 5
⬛⬛🟧
🟧🟧🟧

# 6
🟥🟥
⬛🟥🟥

# 7
⬛🟩🟩
🟩🟩
*/

impl Tmap {
    #[allow(unused_assignments)]
    pub fn new() -> Self {
        let mut map: Vec<Vec<usize>> = vec![];
        
        
        let mut rng = thread_rng();
        let block: Tblock = Tblock::new(rng.gen_range(1..8), None, 0);
        let root = std::env::current_dir().unwrap();
        let path = root.join("src/test.img").to_str().unwrap().replace("\\", "/");

        let checker = fs::read_to_string(path).unwrap().parse::<usize>();
        println!("{checker:?}");
        let best_point: usize = match checker{
            Ok(ok) => ok,
            Err(_) => {
                0
            }
        };


        map = vec![vec![0; 10]; 20];
        Self{
            map: map,
            block: block,
            point: 0,
            best_point: best_point
        }
    }

    pub fn set_block(&mut self){ 
        let block = self.block.clone();
        self.spawn_block();
        for shape in block.shape{
            self.map[shape[1]][shape[0]] = block.id;
        }
        let mut i: usize = 0;
        let mut add = 10;
        for map in &self.map.clone(){
            let mut ok: bool = true;
            for i in map{
                if *i == 0{
                    ok = false;
                    break;
                }
            }
            if ok{
                for j in 0..10{
                    self.map[i][j] = 0;
                    built_in::play_sound("pop");
                    cls();
                    self.encoding();
                    self.print_points();
                    self.point += add;
                }
                add += 10;
                
                self.map.remove(i);
                self.map.reverse();
                self.map.push(vec![0;10]);
                self.map.reverse();
                
                if self.point > self.best_point{
                    self.best_point = self.point;
                    let root = std::env::current_dir().unwrap();
                    let path = root.join("src/test.img");
                    let checker = fs::write(path, self.best_point.to_string());
                    match checker {
                        Ok(ok) => ok,
                        Err(_) => println!("save failed")
                    }
                }
            }
            i += 1;
        }
    }
    
    pub fn print_points(&self){
        println!("\n");
        let bold = Color::Rgb { r: 138, g: 70, b: 255 };
        let org = Color::Rgb { r: 129, g: 135, b: 251 };
        _ = execute!(
            stdout(),
            SetForegroundColor(Color::Black),
            SetBackgroundColor(org),
            Print("score     ".to_string()),
            ResetColor
        );
        _ = execute!(
            stdout(),
            SetForegroundColor(Color::White),
            SetBackgroundColor(bold),
            Print(self.point.to_string()),
            ResetColor
        );

        print!("\n");
        _ = execute!(
            stdout(),
            SetForegroundColor(Color::Black),
            SetBackgroundColor(org),
            Print("top score ".to_string()),
            ResetColor
        );
        _ = execute!(
            stdout(),
            SetForegroundColor(Color::White),
            SetBackgroundColor(bold),
            Print(self.best_point.to_string()),
            ResetColor
        );
    }

    pub fn down_block(&mut self){ 
        self.block.pos.y += 1;
        match make_shape(self.block.id, self.block.pos, self.block.deg) {
            Ok(ok) => { 
                let test_block = ok;
                let ok2 = self.check(&test_block);
                if ok2{
                    self.block.shape = test_block.clone();
                }else{
                    self.block.pos.y -= 1;
                    self.set_block();
                }
            }
            Err(_) => { 
                self.block.pos.y -= 1;
                self.set_block();
            }
        };
    }

    pub fn move_block(&mut self, direction: Move){
        let mut block_clone = self.block.clone();
        block_clone.t_move(direction);
        let ok = self.check(&block_clone.shape);
        if ok{
            self.block = block_clone.clone();
        }   
    }

    pub fn spin_block(&mut self){
        let mut block_clone = self.block.clone();
        block_clone.t_spin();
        if self.check(&block_clone.shape){
            self.block = block_clone.clone();
        }
    }

    fn spawn_block(&mut self){
        let mut rng = thread_rng();
        let block = Tblock::new(rng.gen_range(1..8), None, 0);
        for part in &block.shape{
            if self.map[part[1]][part[0]] != 0{
                self.block = Tblock::new(0, None, 0);
                return;
            }
        }
        self.block = block.clone();
    }

    fn check(&mut self, block_shape: &Vec<Vec<usize>>) -> bool{
        let mut ok: bool = true;
        for part in block_shape{
            if self.map[part[1]][part[0]] != 0{
                ok = false;
                break;
            }
        }
        ok
    }

    pub fn encoding(&self) {
        let mut map = self.map.clone();
        let block = self.block.clone();

        for shape in &block.shape{
            map[shape[1]][shape[0]] = block.id;
        }
        for _ in 0..12{
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::White),
                SetBackgroundColor(Color::White),
                Print("ㅤ".to_string()),
                ResetColor
            );
        }
        print!("\n");
        for i in &map{
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::White),
                SetBackgroundColor(Color::White),
                Print("ㅤ".to_string()),
                ResetColor
            );

            for j in i{
                let color = match j{
                    0 => Color::Rgb { r: 0, g: 0, b: 0 },
                    1 => Color::Rgb { r: 0, g: 240, b: 240 },
                    2 => Color::Rgb { r: 160, g: 0, b: 240 },
                    3 => Color::Rgb { r: 240, g: 240, b: 0 },
                    4 => Color::Rgb { r: 0, g: 0, b: 240 },
                    5 => Color::Rgb { r: 240, g: 160, b: 240 },
                    6 => Color::Rgb { r: 240, g: 0, b: 0 },
                    _ => Color::Rgb { r: 0, g: 240, b: 0 }
                };
                let b = Color::Rgb{ r: 0, g: 0, b: 0 };

                if color != b {
                    let _ = execute!(
                        stdout(),
                        SetForegroundColor(color),
                        SetBackgroundColor(color),
                        Print("ㅤ".to_string()),
                        ResetColor
                    );
                }else{
                    print!("ㅤ")
                }
            }
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::White),
                SetBackgroundColor(Color::White),
                Print("ㅤ".to_string()),
                ResetColor
            );
            print!("\n");
        }
        for _ in 0..12{
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::White),
                SetBackgroundColor(Color::White),
                Print("ㅤ".to_string()),
                ResetColor
            );
        }
    }
}