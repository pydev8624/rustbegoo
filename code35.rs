fn main() {
    let mut x:i32;
    x=0;
    'halgeh:loop {
        
        if x == 3 {
            println!("halgeh end");
            break 'halgeh;
        }
        else{
            println!("{}",x);
            x+=1;
        }
        
    };
    
}
