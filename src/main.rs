use log::{error, info};
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::Instant;
use tokio::{io::AsyncBufReadExt, signal::ctrl_c, sync::mpsc};

use env_logger::{Builder, Env};
use futures::{prelude::*, select};
use libp2p::gossipsub::MessageId;
use libp2p::gossipsub::{
    GossipsubEvent, GossipsubMessage, IdentTopic as Topic, MessageAuthenticity, ValidationMode,
};
use libp2p::{gossipsub, identity, swarm::SwarmEvent, Multiaddr, PeerId};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::{Duration, UNIX_EPOCH};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;
use tokio::task;

use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use crossterm::{event::{read, poll}, Result};

use tokio::sync::oneshot;
use tokio::sync::oneshot::Sender;

use std::collections::VecDeque;

use std::time::{ SystemTime};

use chrono::prelude::*;

use bincode::{config, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
struct MessageOnLine {
    author: String,
    message: String,
    style: u8 
}


use std::process;

use rand::Rng;


use std::{ io};
static ANIMALS: [&str; 83] = [
"alligator",
"ant-eater",
"armadillo",
"auroch",
"axolotl",
"badger",
"bat",
"bear",
"beaver",
"blobfish",
"buffalo",
"camel",
"chameleon",
"cheetah",
"chipmunk",
"chinchilla",
"chupacabra",
"cormorant",
"coyote",
"crow",
"dingo",
"dinosaur",
"dog",
"dolphin",
"dragon",
"duck",
"dumbo octopus",
"elephant",
"ferret",
"fox",
"frog",
"giraffe",
"goose",
"gopher",
"grizzly",
"hamster",
"hedgehog",
"hippo",
"hyena",
"jackal",
"jackalope",
"ibex",
"ifrit",
"iguana",
"kangaroo",
"kiwi",
"koala",
"kraken",
"lemur",
"leopard",
"liger",
"lion",
"llama",
"manatee",
"mink",
"monkey",
"moose",
"narwhal",
"nyan cat",
"orangutan",
"otter",
"panda",
"penguin",
"platypus",
"python",
"pumpkin",
"quagga",
"quokka",
"rabbit",
"raccoon",
"rhino",
"sheep",
"shrew",
"skunk",
"slow loris",
"squirrel",
"tiger",
"turtle",
"unicorn",
"walrus",
"wolf",
"wolverine",
"wombat" ];

static ADJECTIVES: [&str; 228] = [
"adorable",
"adventurous",
"aggressive",
"agreeable",
"alert",
"alive",
"amused",
"angry",
"annoyed",
"annoying",
"anxious",
"arrogant",
"ashamed",
"attractive",
"average",
"awful",
"bad",
"beautiful",
"better",
"bewildered",
"black",
"bloody",
"blue",
"blue-eyed",
"blushing",
"bored",
"brainy",
"brave",
"breakable",
"bright",
"busy",
"calm",
"careful",
"cautious",
"charming",
"cheerful",
"clean",
"clear",
"clever",
"cloudy",
"clumsy",
"colorful",
"combative",
"comfortable",
"concerned",
"condemned",
"confused",
"cooperative",
"courageous",
"crazy",
"creepy",
"crowded",
"cruel",
"curious",
"cute",
"dangerous",
"dark",
"dead",
"defeated",
"defiant",
"delightful",
"depressed",
"determined",
"different",
"difficult",
"disgusted",
"distinct",
"disturbed",
"dizzy",
"doubtful",
"drab",
"dull",
"eager",
"easy",
"elated",
"elegant",
"embarrassed",
"enchanting",
"encouraging",
"energetic",
"enthusiastic",
"envious",
"evil",
"excited",
"expensive",
"exuberant",
"fair",
"faithful",
"famous",
"fancy",
"fantastic",
"fierce",
"filthy",
"fine",
"foolish",
"fragile",
"frail",
"frantic",
"friendly",
"frightened",
"funny",
"gentle",
"gifted",
"glamorous",
"gleaming",
"glorious",
"good",
"gorgeous",
"graceful",
"grieving",
"grotesque",
"grumpy",
"handsome",
"happy",
"healthy",
"helpful",
"helpless",
"hilarious",
"homeless",
"homely",
"horrible",
"hungry",
"hurt",
"ill",
"important",
"impossible",
"inexpensive",
"innocent",
"inquisitive",
"itchy",
"jealous",
"jittery",
"jolly",
"joyous",
"kind",
"lazy",
"light",
"lively",
"lonely",
"long",
"lovely",
"lucky",
"magnificent",
"misty",
"modern",
"motionless",
"muddy",
"mushy",
"mysterious",
"nasty",
"naughty",
"nervous",
"nice",
"nutty",
"obedient",
"obnoxious",
"odd",
"old-fashioned",
"open",
"outrageous",
"outstanding",
"panicky",
"perfect",
"plain",
"pleasant",
"poised",
"poor",
"powerful",
"precious",
"prickly",
"proud",
"putrid",
"puzzled",
"quaint",
"real",
"relieved",
"repulsive",
"rich",
"scary",
"selfish",
"shiny",
"shy",
"silly",
"sleepy",
"smiling",
"smoggy",
"sore",
"sparkling",
"splendid",
"spotless",
"stormy",
"strange",
"stupid",
"successful",
"super",
"talented",
"tame",
"tasty",
"tender",
"tense",
"terrible",
"thankful",
"thoughtful",
"thoughtless",
"tired",
"tough",
"troubled",
"ugliest",
"ugly",
"uninterested",
"unsightly",
"unusual",
"upset",
"uptight",
"vast",
"victorious",
"vivacious",
"wandering",
"weary",
"wicked",
"wide-eyed",
"wild",
"witty",
"worried",
"worrisome",
"wrong",
"zany",
"zealous",
];

enum Origin {
    Local,
    Remote,
    Joined
}

enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: VecDeque<(String, Origin)>,
    max_messages: usize,
    name: String
}

impl Default for App {
    fn default() -> App {
        let mut rng = rand::thread_rng();
        let an = rng.gen_range(0..82);
        let ad = rng.gen_range(0..227);

        let name = format!("{} {}", ADJECTIVES[ad], ANIMALS[an]);
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: VecDeque::with_capacity(10),
            max_messages: 10,
            name
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(3),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[2]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }

    let style = Style::default().fg(Color::Green);
    let style2 = Style::default().fg(Color::Red).add_modifier(Modifier::ITALIC);

    let text_height = ListItem::new("demo").height();
    let chat_height = chunks[0].height;

    let max_messages = ( chat_height as usize/ text_height ) as usize - 2;

    app.max_messages = max_messages;

    //println!("{} / {} = {}", text_height, chat_height , max_messages);
    //println!("{:?}", f.size());

    if max_messages < app.messages.len() {
        //app.messages.push_back((format!("RESIZING MIN {} > {}", max_messages, app.messages.capacity()), Origin::Local));

        let diff = (max_messages as i32 - app.messages.len() as i32) ;

        for i in 0..diff.abs() {
            app.messages.pop_front();
        }
    }

    let dt = Utc::now();
    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {

            let span = match m {
                (s, Origin::Local) => {Span::styled(s, style)},
                (s, Origin::Remote) => {Span::raw(s)}
                (s, Origin::Joined) => {Span::styled(s, style2)},
            };

            let spans = Spans::from(span);

            let content = vec![spans];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    f.render_widget(messages, chunks[0]);
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App, tui_sender: UnboundedSender<(String, u8)>, tui_receiver: Receiver<(String, u8)>) -> io::Result<()> {

    let dt = Utc::now();
    let joined = format!("[{:02}:{:02}:{:02}] {} joined the chat", dt.hour(), dt.minute(), dt.second(), app.name);
    let mut first = true;

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if first {
            match app.input_mode {
                InputMode::Normal => {
                    app.messages.push_back((joined.clone(), Origin::Joined));
                    tui_sender.send((joined.clone(), 1)).expect("Failed tui_sender");
                    first = false;
                },
                _ => {}
            }
        }

        if poll(Duration::from_millis(100))? {
            // It's guaranteed that `read` wont block, because `poll` returned
            // `Ok(true)`.
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            app.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Editing => {
                        match key.code {
                            KeyCode::Enter => {
                                let dt = Utc::now();

                                let message: String = app.input.drain(..).collect();
                                if app.messages.len() == app.max_messages {
                                    //println!("FUCCCU");
                                    //app.messages.pop_front();
                                }
                                //println!("LETS FOOO");
                                app.messages.push_back((format!("[{:02}:{:02}:{:02}] {}", dt.hour(), dt.minute(), dt.second(), message.clone()), Origin::Local));
                                tui_sender.send((format!("[{:02}:{:02}:{:02}] {}: {}", dt.hour(), dt.minute(), dt.second(), app.name, message), 0)).expect("Failed tui_sender");
                            }
                            KeyCode::Char(c) => {
                                app.input.push(c);
                            }
                            KeyCode::Backspace => {
                                app.input.pop();
                            }
                            KeyCode::Esc => {
                                app.input_mode = InputMode::Normal;
                            }
                            _ => {}
                        }
                    },
                }
            }
        } else {

            let new_message = tui_receiver.recv_timeout(Duration::from_millis(100));
            if let Ok(msg) = new_message {

                let message = msg.0;
                let style  = msg.1;

                if app.messages.len() == app.max_messages {
                    app.messages.pop_front();
                    //println!("FUCCCU 22");
                }

                if style == 1 {
                    app.messages.push_back((message, Origin::Joined));
                } else {
                    app.messages.push_back((message, Origin::Remote));

                }
                //println!("LETS FOOO");
            }
            // Timeout expired, no `Event` is available
        }
        
    }
}


#[tokio::main]
async fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    // sync => async
    let (tui_sender, mut tokio_receiver) = mpsc::unbounded_channel::<(String, u8)>();

    // async => sync
    let (tokio_sender, tui_receiver) = channel::<(String, u8)>();




    // Create a random PeerId
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    //println!("Local peer id: {:?}", local_peer_id);

    // Set up an encrypted TCP Transport over the Mplex and Yamux protocols
    let transport = libp2p::development_transport(local_key.clone()).await.expect("Pila");

    let topic = Topic::new("test-net");

    let mut swarm = {
        // To content-address message, we can take the hash of message and use it as an ID.
        let message_id_fn = |message: &GossipsubMessage| {
            let mut s = DefaultHasher::new();
            let now = Instant::now();
            message.data.hash(&mut s);
            now.hash(&mut s);
            let s = s.finish();
            MessageId::from(s.to_string())
        };

        // Set a custom gossipsub
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
            .validation_mode(ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
            .message_id_fn(message_id_fn) // content-address messages. No two messages of the
            // same content will be propagated.
            .build()
            .expect("Valid config");
        // build a gossipsub network behaviour
        let mut gossipsub: gossipsub::Gossipsub =
            gossipsub::Gossipsub::new(MessageAuthenticity::Signed(local_key), gossipsub_config)
                .expect("Correct configuration");

        // subscribes to our topic
        gossipsub.subscribe(&topic).unwrap();

        // add an explicit peer if one was provided
        if let Some(explicit) = std::env::args().nth(2) {
            let explicit = explicit.clone();
            match explicit.parse() {
                Ok(id) => gossipsub.add_explicit_peer(&id),
                Err(err) => println!("Failed to parse explicit peer id: {:?}", err),
            }
        }

        // build the swarm
        libp2p::Swarm::new(transport, gossipsub, local_peer_id)
    };

    // Listen on all interfaces and whatever port the OS assigns
    swarm
        .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
        .unwrap();

    // Reach out to another node if specified
    if let Some(to_dial) = std::env::args().nth(1) {
        let address: Multiaddr = to_dial.parse().expect("User to provide valid address.");
        match swarm.dial(address.clone()) {
            Ok(_) => {}//println!("Dialed {:?}", address),
            Err(e) => println!("Dial {:?} failed: {:?}", address, e),
        };
    }

    // Read full lines from stdin
    //let mut stdin = io::BufReader::new(io::stdin()).lines();

    let app = App::default();
    let name = app.name.clone();
    let send = tui_sender.clone();
    let _res = task::spawn_blocking(move || {

        enable_raw_mode().expect("unable to raw mode");
        let mut stdout = io::stdout();

        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("CMON");
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("CMON");

        // create app and run it
        let res = run_app(&mut terminal, app, tui_sender.clone(), tui_receiver);

        // restore terminal
        disable_raw_mode().expect("CMON");
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        ).expect("CMON");
        terminal.show_cursor().expect("CMON");

        if let Err(err) = res {
            println!("{:?}", err)
        }

        process::exit(0);
    });

    let mut rng = rand::thread_rng();
    let an = rng.gen_range(0..82);
    let ad = rng.gen_range(0..227);

    let name = format!("{} {}", ADJECTIVES[ad], ANIMALS[an]);
    let config = config::standard();

    loop {
        tokio::select! {

            line = tokio_receiver.recv() => {
                if let Some((input_line, style)) = line {
                    let message = MessageOnLine {
                        author: name.clone(),
                        message: input_line,
                        style: style
                    };

                    let encoded: Vec<u8> = bincode::encode_to_vec(&message, config).unwrap();

                    if let Err(e) = swarm
                        .behaviour_mut()
                        .publish(topic.clone(), encoded)
                    {
                        println!("Publish error: {:?}", e);
                    }
                }
            },

            //line = stdin.next_line() => {
            //    if let Some(input_line) = line.expect("a valid line") {
            //        if let Err(e) = swarm
            //            .behaviour_mut()
            //            .publish(topic.clone(), input_line.as_bytes())
            //        {
            //            println!("Publish error: {:?}", e);
            //        }
            //    }

            //},
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(GossipsubEvent::Message {
                    propagation_source: peer_id,
                    message_id: _id,
                    message,
                }) => {
                    let (decoded, len): (MessageOnLine, usize) = bincode::decode_from_slice(&message.data[..], config).unwrap();

                    tokio_sender.send((decoded.message, decoded.style)).expect("Failed tokio_sender");
                },
                //println!(
                //    "{:.6}: {}",
                //    peer_id,
                //    String::from_utf8_lossy(&message.data)
                //),
                SwarmEvent::NewListenAddr { address, .. } => {
                    //println!("Listening on {:?}", address);
                    //swarm.behaviour_mut().publish(topic.clone(), joined.as_bytes()).expect("pila");
                }
                _ => {}
            }

        }
    }
}
