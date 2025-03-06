pub type Index = usize;

mod und { // Finished
    use super::*;

    #[derive(Debug)]
    pub struct Text {
        pub first: Char,
        pub rest: Vec<Char>,
    }

    #[derive(Debug)]
    pub struct Char {
        pub index: Index,
        pub value: char,
        pub is_escaped: bool,
    }

    #[derive(Debug)]
    pub enum Node {
        Group(Group),
        Text(Text),
    }

    #[derive(Debug)]
    pub struct Result {
        pub escape_at_end: bool,
        pub unexpected_closers: Vec<Index>,
        pub unclosed_openers: Vec<Index>,
        pub root: Group,
    }

    #[derive(Debug)]
    pub struct Group {
        pub index: Index,
        pub contents: Vec<Node>,
    }
    impl Group {
        fn new(index: usize) -> Self {
            Group {
                contents: Vec::new(),
                index,
            }
        }
    }

    struct MaybeText {
        first: Option<Char>,
        rest: Vec<Char>,
    }
    impl MaybeText {
        fn new() -> Self {
            Self {
                first: None,
                rest: Vec::new(),
            }
        }
        fn push(&mut self, c: Char) {
            if self.first.is_none() {
                self.first = Some(c);
            } else {
                self.rest.push(c);
            }
        }
    }

    struct Layers {
        root: Group,
        overlays: Vec<Group>,
    }
    impl Layers {
        fn get_top(&mut self) -> &mut Vec<Node> {
            match self.overlays.last_mut() {
                Some(overlay) => &mut overlay.contents,
                None => &mut self.root.contents,
            }
        }
    }

    pub fn parse(mut input: impl Iterator<Item = (Index, char)>) -> Result {
        let mut layers = Layers {
            overlays: Vec::new(),
            root: Group::new(0),
        };

        let mut text = MaybeText::new();

        let mut unexpected_closers = Vec::new();
        let mut unclosed_openers = Vec::new();

        let mut is_escaped = false;

        loop {
            let c = input.next();
            if let Some((index, c)) = c {
                if is_escaped {
                    is_escaped = false;
                    text.push(Char {
                        index,
                        is_escaped: true,
                        value: c,
                    });
                    continue;
                }
            }
            match c {
                Some((_, '\\')) => is_escaped = true,
                None | Some((_, '(')) | Some((_, ')')) => {
                    if let Some(first) = text.first {
                        layers.get_top().push(Node::Text(Text {
                            first,
                            rest: text.rest,
                        }));
                        text = MaybeText::new();
                    }
                    if let Some((index, '(')) = c {
                        layers.overlays.push(Group::new(index));
                    } else {
                        match layers.overlays.pop() {
                            None => {
                                if let Some((index, ')')) = c {
                                    unexpected_closers.push(index);
                                } else {
                                    break;
                                }
                            }
                            Some(overlay) => {
                                if let None = c {
                                    unclosed_openers.push(overlay.index);
                                }
                                layers.get_top().push(Node::Group(overlay));
                            }
                        }
                    }
                }
                Some((index, c)) => text.push(Char {
                    index,
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

mod pon { // Bugged, see main() `print` outputs
    use super::*;

    #[derive(Debug)]
    pub struct Word {
        pub first: char,
        pub rest: String,
    }

    struct FirstChar {
        value: char,
        index: Index,
    }

    struct MaybeWord {
        first: Option<FirstChar>,
        rest: String,
    }
    impl MaybeWord {
        fn new() -> Self {
            Self {
                first: None,
                rest: String::new(),
            }
        }
    }

    #[derive(Debug)]
    pub struct Name {
        pub index: Index,
        pub first: Word,
        pub rest: Vec<Word>,
    }

    struct FirstWord {
        value: Word,
        index: Index,
    }

    struct MaybeName {
        first: Option<FirstWord>,
        rest: Vec<Word>,
    }

    #[derive(Debug)]
    pub enum Node {
        Group(und::Group),
        Name(Name),
    }

    fn is_word_boundary(c: &und::Char) -> bool {
        c.value.is_whitespace() && !c.is_escaped
    }

    pub fn convert(und: und::Group) -> Vec<Node> {
        let mut nodes = Vec::new();
        for node in und.contents.into_iter() {
            match node {
                und::Node::Group(group) => nodes.push(Node::Group(group)),
                und::Node::Text(text) => {
                    let mut rest = text.rest.into_iter();
                    let mut name = MaybeName {
                        first: None,
                        rest: Vec::new(),
                    };
                    let mut word = MaybeWord::new();
                    let mut current_char = text.first;
                    loop {
                        if !is_word_boundary(&current_char) {
                            if word.first.is_none() {
                                word.first = Some(FirstChar {
                                    value: current_char.value,
                                    index: current_char.index,
                                });
                            } else {
                                word.rest.push(current_char.value);
                            }
                        }
                        let mut next_char = rest.next();
                        let ended = next_char.is_none();
                        if is_word_boundary(&current_char) {
                            next_char = None;
                        }
                        match next_char {
                            None => {
                                if let Some(first) = word.first {
                                    let full_word = Word {
                                        first: first.value,
                                        rest: word.rest,
                                    };
                                    word = MaybeWord::new();
                                    if name.first.is_none() {
                                        name.first = Some(FirstWord {
                                            index: first.index,
                                            value: full_word,
                                        });
                                    } else {
                                        name.rest.push(full_word);
                                    }
                                }
                                if ended {
                                    break;
                                }
                            }
                            Some(next_char) => current_char = next_char,
                        }
                    }
                    if let Some(first) = name.first {
                        nodes.push(Node::Name(Name {
                            first: first.value,
                            rest: name.rest,
                            index: first.index,
                        }));
                    }
                }
            }
        }
        nodes
    }
}

mod spiff {}

fn main() {
    let und = und::parse("test (ab\\c) \\(\\ ) def hi".char_indices());
    dbg!(&und.root);
    let pon = pon::convert(und.root);
    dbg!(&und.escape_at_end);
    dbg!(&und.unclosed_openers);
    dbg!(&und.unexpected_closers);
    dbg!(&pon);
}
