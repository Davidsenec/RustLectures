// #[allow(dead_code)]
#[derive(Debug)]
enum Node {
    Cons(i32, Box<Node>),
    Nil,
}

// #[allow(dead_code)]
#[derive(Debug)]
struct BoxedStack {
    top: Box<Node>,
    
}

impl BoxedStack {
    fn new() -> Self {
        BoxedStack { top: Box::new(Node::Nil) }
    }

    fn push(&mut self, value:i32) {
        match &*self.top {
            Node::Nil => {
                self.top = Box::new(Node::Cons(value, Box::new(Node::Nil)));
                println!("Pushing {} onto the stack.", value);
            }
            Node::Cons(_,_) => {
                let old = std::mem::replace(&mut self.top, Box::new(Node::Nil));
                self.top = Box::new(Node::Cons(value, old));
                println!("Pushing {} onto the stack.", value);
            }
        }
    }
    
    fn pop(&mut self) -> Option<i32> {
       let old = std::mem::replace(&mut self.top, Box::new(Node::Nil));
        match *old {
            Node::Nil => None,
            Node::Cons(value, next) => {
                self.top = next;
                println!("Popping {} out of the stack.", value);
                Some(value)
            }
        }
    }

    fn peek(&self) -> Option<&i32> {
        match &*self.top {
            Node::Nil => None,
            Node::Cons(value,_ ) => Some(value),
        }
    }

    fn is_empty(&self) -> bool {
        match &*self.top {
            Node::Nil => { true }
            Node::Cons(_,_ ) => { false }
        }
    }

    fn print_stack(&self) {
        print!("Stack Content: ");
        fn reader(node: &Node) {
            match node {
                Node::Nil => {
                    print!("Nil");
                    println!();
                    return
                }
                Node::Cons(value, next) => {
                    print!("{} -> ", value);
                    reader(&*next);
                }
            }
        }
        reader(&self.top);
    }

}

fn main() {
    let mut box1 = BoxedStack::new();
    // println!("{:?}", box1);

    box1.push(10); // Push 
    box1.push(20);
    box1.push(30);
    
    box1.print_stack(); // Viewing
    box1.peek();
    
    box1.pop(); // Remove from the top make it drop
    box1.print_stack();
    
    box1.pop(); 
    box1.print_stack();
    
    box1.pop();
    box1.print_stack();

    println!("Is the stack empty? {}", box1.is_empty()); 

    // println!("{:?}", box1);

}
