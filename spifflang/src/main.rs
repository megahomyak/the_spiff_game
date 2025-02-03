mod und {
    pub type Index = usize;

    pub type Text = Vec<Char>;

    pub struct Char {
        value: char,
        is_escaped: bool,
    }

    pub struct Node {
        kind: NodeKind,
        index: Index,
    }

    pub enum NodeKind {
        Group(Group),
        Text(Text),
    }

    pub struct Result {
        escape_at_end: bool,
        unexpected_closers: Vec<Index>,
        unclosed_openers: Vec<Index>,
        root: Group,
    }

    pub type Group = Vec<Node>;

    struct Overlay {
        index: Index,
        group: Group,
    }

    struct Layers {
        root: Group,
        overlays: Vec<Overlay>,
    }

    fn get_top(layers: &mut Layers) -> &mut Group {
        match layers.overlays.last_mut() {
            Some(overlay) => &mut overlay.group,
            None => &mut layers.root,
        }
    }

    pub fn parse(input: &str) -> Result {
        let mut layers = Layers {
            overlays: Vec::new(),
            root: Vec::new(),
        };

        let mut input_next_index = 0;
        let mut input_current_index = 0;

        let mut text_index = 0;
        let mut text_buffer = Vec::new();

        let mut unexpected_closers = Vec::new();
        let mut unclosed_openers = Vec::new();

        let mut is_escaped = false;

        loop {
            let c_option = unsafe { input.get_unchecked(input_next_index..) }
                .chars()
                .next();
            if let Some(c) = c_option {
                input_current_index = input_next_index;
                input_next_index += c.len_utf8();
                if is_escaped {
                    is_escaped = false;
                    text_buffer.push(Char {
                        is_escaped: true,
                        value: c,
                    });
                    continue;
                }
            }
            match c_option {
                Some('\\') => is_escaped = true,
                None | Some('(') | Some(')') => {
                    if !text_buffer.is_empty() {
                        get_top(&mut layers).push(Node {
                            kind: NodeKind::Text(text_buffer),
                            index: text_index,
                        });
                        text_buffer = Vec::new();
                    }
                    text_index = input_next_index;
                    if c_option == Some('(') {
                        layers.overlays.push(Overlay {
                            index: input_current_index,
                            group: Vec::new(),
                        })
                    } else {
                        match layers.overlays.pop() {
                            None => {
                                if c_option == Some(')') {
                                    unexpected_closers.push(input_current_index);
                                } else {
                                    break;
                                }
                            }
                            Some(overlay) => {
                                if c_option == None {
                                    unclosed_openers.push(overlay.index);
                                }
                                get_top(&mut layers).push(Node {
                                    index: overlay.index,
                                    kind: NodeKind::Group(overlay.group),
                                })
                            }
                        }
                    }
                }
                Some(c) => text_buffer.push(Char {
                    value: c,
                    is_escaped: false,
                }),
            }
        }

        Result {
            unclosed_openers,
            unexpected_closers,
            root: layers.root,
            escape_at_end: is_escaped,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
