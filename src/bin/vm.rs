fn main() {
    use vsm::vm::*;
    use Inst::*;

    let prog = vec![
        Push(Val(5)),
        Push(Val(3)),
        Add,
        Peek,
        Print,
        Peek,
    ];

    let mut vm = VM::new(prog);

    println!("INST\t\tARGS\t\tMEM");
    println!("----------------------------------------");
    
    while vm.live() {
        vm.step();
    }

    println!("\nOUTPUT");
    println!("----------------------------------------");

    for line in vm.output() {
        println!("{}", line);
    }
}
    
