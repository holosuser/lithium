use sheller::run;
fn init(){
    run!("mkdir -p /lithium/pkgs");
    run!("mkdir -p /lithium/pm");
}
fn install(){
    use std::thread;
    use std::time::Duration;
    let args:Vec<String>=std::env::args().collect();
    let load:String=std::env::args().skip(2).collect::<Vec<_>>().join("");
    let repos="https://raw.githubusercontent.com/wholos/LithiumPackages/main/binary/";
    println!("lithium: installing {} package",load);
    run!("wget -q --directory=/lithium/pkgs/ {}{}",repos,load);
    thread::sleep(Duration::from_secs(1));
    run!("cp /lithium/pkgs/{} /bin/{}",load,load);
    thread::sleep(Duration::from_secs(1));
    run!("chmod +x /bin/{}",load);
}
fn remove(){
    let args:Vec<String>=std::env::args().collect();
    let kill:String=std::env::args().skip(2).collect::<Vec<_>>().join("");
    println!("lithium: removing {} package",kill);
    run!("rm /bin/{} /lithium/pkgs/{}",kill,kill);
}
fn update(){
    use std::thread;
    use std::time::Duration;
    println!("lithium: updating package manager");
    run!("rm /bin/lithium /lithium/pm/lithium");
    thread::sleep(Duration::from_secs(1));
    run!("wget -q --directory=/lithium/pm/ https://raw.githubusercontent.com/wholos/LithiumPackages/main/lithium/lithium");
    thread::sleep(Duration::from_secs(1));
    run!("ln -s /lithium/pm/lithium /bin/lithium");
    thread::sleep(Duration::from_secs(1));
    run!("chmod +x /lithium/pm/lithium /bin/lithium");
}
fn main(){
    let args:Vec<String>=std::env::args().collect();
    if args.iter().any(|arg: &String|arg=="--help"){
        println!("lithium: ");
        println!("init :: initalization lithium to /mnt");
        println!("load :: install package to /bin/ directory");
        println!("kill :: remove package from /bin/ directory");
        println!("upd  :: update packages from /bin/ directory");
    }
    if args.iter().any(|arg: &String|arg=="--version"){
        println!("lithium: version is :: 0.5");
    }
    if args.iter().any(|arg: &String|arg=="init"){
        init();
    }
    if args.iter().any(|arg: &String|arg=="load"){
        install();
    }
    if args.iter().any(|arg: &String|arg=="kill"){
        remove();
    }
    if args.iter().any(|arg: &String|arg=="upd"){
        update();
    }
    if args.len() == 1 {
        eprintln!("lithium: arguments not found.");
    }
}
