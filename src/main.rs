use eventsys::CommandBuffer;

struct Renderer;

impl Renderer {
    fn do_things(&mut self) {
        println!("render things")
    }
}

struct World;

impl World {
    fn do_stuff(&mut self) {
        println!("world stuff")
    }
}

struct State {
    renderer: Renderer,
    world: World,
}

impl State {
    fn dispatch(&mut self, buf: &mut CommandBuffer<Command>) {
        while buf.len() > 0 {
            match buf.read_command().unwrap() {
                Command::Stuff => self.world.do_stuff(),
                Command::Things => self.renderer.do_things(),
                Command::Exit => panic!("goodnight!"),
            };
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Command {
    Stuff,
    Things,
    Exit,
}

fn main() {
    let stdin = std::io::stdin();
    let mut input_buf = String::new();
    let mut command_buf = CommandBuffer::new(5);

    let mut state = State {
        renderer: Renderer{},
        world: World{}
    };
    loop {
        stdin.read_line(&mut input_buf).unwrap();
        match input_buf.as_str().trim() {
            "stuff" => { command_buf.write_command(Command::Stuff); },
            "things" => { command_buf.write_command(Command::Things); },
            "exit" => { command_buf.write_command(Command::Exit); },
            "dispatch" => { state.dispatch(&mut command_buf); },
            _ => panic!("unrecognized command {}", input_buf.as_str()),
        };
        input_buf.clear();
    }
}
