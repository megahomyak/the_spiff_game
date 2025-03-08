pub type Index = usize;

#[derive(Debug)]
pub struct Char {
    pub index: Index,
    pub value: char,
    pub is_escaped: bool,
}

mod und {
    // Finished
    use super::*;

    #[derive(Debug)]
    pub struct Text {
        pub first: Char,
        pub rest: Vec<Char>,
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

mod pon {
    // Finished
    use super::*;

    #[derive(Debug)]
    pub struct Word {
        pub first: Char,
        pub rest: Vec<Char>,
    }

    struct MaybeWord {
        first: Option<Char>,
        rest: Vec<Char>,
    }
    impl MaybeWord {
        fn new() -> Self {
            Self {
                first: None,
                rest: Vec::new(),
            }
        }
    }

    #[derive(Debug)]
    pub struct Name {
        pub first: Word,
        pub rest: Vec<Word>,
    }

    struct MaybeName {
        first: Option<Word>,
        rest: Vec<Word>,
    }

    #[derive(Debug)]
    pub enum Node {
        Group(und::Group),
        Name(Name),
    }

    pub fn convert(und: und::Group) -> Vec<Node> {
        let mut nodes = Vec::new();
        for node in und.contents.into_iter() {
            match node {
                und::Node::Group(group) => nodes.push(Node::Group(group)),
                und::Node::Text(text) => {
                    let mut current_char = Some(text.first);
                    let mut text_rest = text.rest.into_iter();
                    let mut name = MaybeName {
                        first: None,
                        rest: Vec::new(),
                    };
                    let mut current_word = MaybeWord::new();
                    loop {
                        let (is_word_boundary, is_text_ended) = match current_char {
                            Some(c) => {
                                let is_word_boundary = !c.is_escaped && c.value.is_whitespace();
                                if !is_word_boundary {
                                    if let None = current_word.first {
                                        current_word.first = Some(c);
                                    } else {
                                        current_word.rest.push(c);
                                    }
                                }
                                (is_word_boundary, false)
                            }
                            None => (true, true),
                        };
                        if is_word_boundary {
                            let word = std::mem::replace(&mut current_word, MaybeWord::new());
                            if let Some(first) = word.first {
                                let word = Word {
                                    first,
                                    rest: word.rest,
                                };
                                if let None = name.first {
                                    name.first = Some(word);
                                } else {
                                    name.rest.push(word);
                                }
                            }
                            if is_text_ended {
                                break;
                            }
                        }
                        current_char = text_rest.next();
                    }
                    if let Some(first) = name.first {
                        nodes.push(Node::Name(Name {
                            first,
                            rest: name.rest,
                        }));
                    }
                }
            }
        }
        nodes
    }
}

mod spiff {

}

fn main() {
    let und = und::parse(r"test (ab\c) \(\ ) def hi(".char_indices());
    dbg!(&und.root);
    let pon = pon::convert(und.root);
    dbg!(&und.escape_at_end);
    dbg!(&und.unclosed_openers);
    dbg!(&und.unexpected_closers);
    dbg!(&pon);
}
