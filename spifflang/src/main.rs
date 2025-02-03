pub type Index = usize;

mod und {
    use super::*;

    pub struct Text {
        pub first: Char,
        pub rest: Vec<Char>,
    }

    pub struct Char {
        pub index: Index,
        pub value: char,
        pub is_escaped: bool,
    }

    pub enum Node {
        Group(Group),
        Text(Text),
    }

    pub struct Result {
        pub escape_at_end: bool,
        pub unexpected_closers: Vec<Index>,
        pub unclosed_openers: Vec<Index>,
        pub root: Group,
    }

    pub struct Group {
        pub index: Index,
        pub contents: Vec<Node>,
    }

    struct Overlay {
        index: Index,
        group: Group,
    }

    struct Layers {
        root: Group,
        overlays: Vec<Overlay>,
    }

    fn get_top(layers: &mut Layers) -> &mut Vec<Node> {
        match layers.overlays.last_mut() {
            Some(overlay) => &mut overlay.group.contents,
            None => &mut layers.root.contents,
        }
    }

    fn push_char(text: &mut MaybeEmptyText, c: Char) {
        if text.first.is_none() {
            text.first = Some(c);
        } else {
            text.rest.push(c);
        }
    }

    fn make_text() -> MaybeEmptyText {
        MaybeEmptyText { first: None, rest: Vec::new() }
    }

    struct MaybeEmptyText {
        first: Option<Char>,
        rest: Vec<Char>,
    }

    pub fn parse(input: &str) -> Result {
        let mut layers = Layers {
            overlays: Vec::new(),
            root: Group { index: 0, contents: Vec::new() },
        };

        let mut input_next_index = 0;
        let mut input_current_index = 0;

        let mut text_index = 0;
        let mut text_buffer = MaybeEmptyText { first: None, rest: Vec::new() };

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
                    push_char(&mut text_buffer, Char {
                        index: input_current_index,
                        is_escaped: true,
                        value: c,
                    });
                    continue;
                }
            }
            match c_option {
                Some('\\') => is_escaped = true,
                None | Some('(') | Some(')') => {
                    if let Some(first) = text_buffer.first {
                        get_top(&mut layers).push(Node::Text(Text {
                            first,
                            rest: text_buffer.rest,
                        }));
                        text_buffer = MaybeEmptyText { first: None, rest: Vec::new() };
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
                                    kind: Node::Group(overlay.group),
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

mod pon {
    use super::*;

    pub struct Name {
        words: Vec<String>,
    }

    pub enum NodeKind {
        Group(und::Group),
        Name(Name),
    }

    pub struct Node {
        kind: NodeKind,
        index: Index,
    }

    pub fn convert(und: und::Group) -> Vec<Node> {
        let mut nodes = Vec::new();
        for node in und {
            match node.kind {
                und::Node::Group(group) => nodes.push(group),
                und::Node::Text(text) => {
                    let mut text = text.into_iter();
                    let mut current_word = String::new();
                    let mut name_index = None;
                    loop {
                        match text.next() {
                            None | Some(c) if c.value.is_whitespace() {

                            }
                            Some(c) {
                                if name_index.is_none() {
                                    name_index = Some(node.)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

mod spiff {

}

fn main() {

}
