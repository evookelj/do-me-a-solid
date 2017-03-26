use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

pub const XRES: usize = 500;
pub const YRES: usize = 500;
pub const DEFAULT: [u32; 3] = [0,0,0];

pub fn plot(x: i32, y:i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	if x>(XRES as i32)-1 || (YRES as i32)-y-1 < 0 || y<0 || x<0 {
		return ;
	}
	let newy = YRES-(y as usize)-1;
	if (x as usize)<XRES && newy < YRES {
		for i in 0..3 {
			//screen[newy][x as usize][i] = color[i];
			screen[x as usize][newy][i] = color[i];
		}
	}
}

pub fn clear_screen(screen: &mut [[[u32; 3]; 500]; 500]) {
	for y in 0..YRES {
		for x in 0..XRES {
			screen[x as usize][y as usize] = DEFAULT;
		}
	}
}

pub fn save_ppm(screen: &mut [[[u32; 3]; 500]; 500], f: &str) {
	static HEADER: &'static str = "P3\n500 500 255\n";
	let path = Path::new(f);
	let display = path.display();

	//create file
	let mut file = match File::create(path) {
        Err(why) => panic!("Error creating {} because {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

	//write header to file
	match file.write_all(HEADER.as_bytes()) {
		Err(why) => panic!("Error writing header because {}", why.description()),
		Ok(_) => (),
	};
	match file.write_all(stringify(&screen).as_bytes()) {
		Err(why) => panic!("Error writing screen bc {}", why.description()),
		Ok(_) => (),
	};
}

fn stringify(screen: &[[[u32; 3]; 500]; 500]) -> String {
	let mut s = "".to_string();
	for y in 0..YRES {
		for x in 0..XRES {
			for i in 0..3 {
				s.push_str(&format!("{} ",screen[x][y][i]));
			}
			s.push_str(" ");
		}
	}
	return s;
}

pub fn disp(screen: &mut [[[u32; 3]; 500]; 500]) {
	save_ppm(screen, "img.ppm");
	Command::new("sh")
		.arg("-c")
 		.arg("display img.ppm")
 		.output()
 		.expect("failed to execute process");
}