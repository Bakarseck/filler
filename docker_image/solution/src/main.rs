use std::fs::File;
use std::io::Write;
use std::io;

fn main() -> std::io::Result<()> {

    let mut f = File::create("debug.log")?;
    let mut x = 0;
    let mut y = 0;

    
    loop {
        
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);
        
        
        if line.starts_with("Anfield") {
            f.write_all(line.as_bytes())?;
        }

        println!("0 0\n");
    }
}
