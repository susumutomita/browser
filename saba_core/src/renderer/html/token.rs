use crate::renderer::html::attribute::Attribute;
use alloc::string::String;
use alloc::vec::Vec;
use core::assert;
use core::iter::Iterator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Data,
    TagOpen,
    EndTagOpen,
    TagName,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnQuoted,
    AfterAttributeValueQuoted,
    SelfClosingStartTag,
    ScriptData,
    ScriptDataLessThanSign,
    ScriptDataEndTagOpen,
    ScriptDataEndTagName,
    TemporaryBuffer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlToken {
    StartTag {
        tag: String,
        self_closing: bool,
        attributes: Vec<Attribute>,
    },
    EndTag {
        tag: String,
    },
    Char(char),
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlTokenizer {
    state: State,
    pos: usize,
    reconsume: bool,
    latest_token: Option<HtmlToken>,
    input: Vec<char>,
    buf: String,
}

impl HtmlTokenizer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            state: State::Data,
            pos: 0,
            reconsume: false,
            latest_token: None,
            input,
            buf: String::new(),
        }
    }
    fn consume_next_input(&mut self) -> char {
        let c = self.input[self.pos];
        self.pos += 1;
        c
    }
    fn reconsume_input(&mut self) -> char {
        self.reconsume = false;
        self.input[self.pos - 1]
    }
    fn create_tag(&mut self, start_tag_token: bool) {
        if start_tag_token {
            self.latest_token = Some(HtmlToken::StartTag {
                tag: String::new(),
                self_closing: false,
                attributes: Vec::new(),
            });
        } else {
            self.latest_token = Some(HtmlToken::EndTag { tag: String::new() });
        }
    }
    fn append_tag_name(&mut self, c: char) {
        assert!(self.latest_token.is_some());
        if let Some(t) = self.latest_token.as_mut() {
            match t {
                HtmlToken::StartTag {
                    ref mut tag,
                    self_closing: _,
                    attributes: _,
                    ..
                }
                | HtmlToken::EndTag { ref mut tag, .. } => tag.push(c),
                _ => panic!("'latest_token' should be either StatTag or EndTag"),
            }
        }
    }
    fn start_new_attribute(&mut self) {
        assert!(self.latest_token.is_some());
        if let Some(t) = self.latest_token.as_mut() {
            match t {
                HtmlToken::StartTag {
                    tag: _,
                    self_closing: _,
                    ref mut attributes,
                } => {
                    attributes.push(Attribute::new(String::new(), String::new()));
                }
                _ => panic!("'latest_token' should be StartTag"),
            }
        }
    }
    fn append_attribute(&mut self, c: char, is_name: bool) {
        assert!(self.latest_token.is_some());
        if let Some(t) = self.latest_token.as_mut() {
            match t {
                HtmlToken::StartTag {
                    tag: _,
                    self_closing: _,
                    ref mut attributes,
                } => {
                    let len = attributes.len();
                    assert!(len > 0);

                    attributes[len - 1].add_char(c, is_name);
                }
                _ => panic!("'latest_token' should be StartTag"),
            }
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn append_attribute_name(&mut self, c: char, is_name: bool) {
        assert!(self.latest_token.is_some());
        if let Some(t) = self.latest_token.as_mut() {
            match t {
                HtmlToken::StartTag {
                    tag: _,
                    self_closing: _,
                    ref mut attributes,
                } => {
                    let len = attributes.len();
                    assert!(len > 0);

                    attributes[len - 1].add_char(c, is_name);
                }
                _ => panic!("'latest_token' should be eitherStartTag"),
            }
        }
    }
    fn take_latest_token(&mut self) -> Option<HtmlToken> {
        assert!(self.latest_token.is_some());
        let t = self.latest_token.as_ref().cloned();
        assert!(self.latest_token.is_none());
        t
    }

    fn set_self_closing_flag(&mut self) {
        assert!(self.latest_token.is_some());
        if let Some(t) = self.latest_token.as_mut() {
            match t {
                HtmlToken::StartTag {
                    tag: _,
                    ref mut self_closing,
                    attributes: _,
                } => *self_closing = true,
                _ => panic!("'latest_token' should be StartTag"),
            }
        }
    }
}

impl Iterator for HtmlTokenizer {
    type Item = HtmlToken;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        }
        loop {
            let c = match self.reconsume {
                true => self.reconsume_input(),
                false => self.consume_next_input(),
            };
            match self.state {
                State::Data => {
                    if c == '<' {
                        self.state = State::TagOpen;
                        continue;
                    }
                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }
                    return Some(HtmlToken::Char(c));
                }
                State::TagOpen => {
                    if c == '/' {
                        self.state = State::EndTagOpen;
                        continue;
                    }
                    if c.is_ascii_alphabetic() {
                        self.reconsume = true;

                        self.state = State::TagName;
                        self.create_tag(true);
                        continue;
                    }
                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }
                    self.reconsume = true;
                    self.state = State::Data;
                }
                State::EndTagOpen => {
                    self.reconsume = true;
                    self.state = State::TagName;
                    self.create_tag(false);
                    continue;
                }
                State::TagName => {
                    if c == ' ' {
                        self.state = State::BeforeAttributeName;
                        continue;
                    }
                    if c == '/' {
                        self.state = State::SelfClosingStartTag;
                        continue;
                    }
                    if c == '>' {
                        self.state = State::Data;
                        return self.take_latest_token();
                    }
                    if c.is_ascii_uppercase() {
                        self.append_tag_name(c.to_ascii_lowercase());
                        continue;
                    }
                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }
                    self.append_tag_name(c);
                }
                State::BeforeAttributeName => {
                    if c == '/' || c == '>' || self.is_eof() {
                        self.reconsume = true;
                        self.state = State::AfterAttributeName;
                        continue;
                    }
                    self.reconsume = true;
                    self.state = State::AttributeName;
                    self.start_new_attribute();
                }
                State::AttributeName => {
                    if c == ' ' || c == '/' || c == '>' || self.is_eof() {
                        self.reconsume = true;
                        self.state = State::AfterAttributeName;
                        continue;
                    }
                    if c == '=' {
                        self.state = State::BeforeAttributeValue;
                        continue;
                    }
                    if c.is_ascii_uppercase() {
                        self.append_attribute_name(c.to_ascii_lowercase(), /*is_name*/ true);
                        continue;
                    }
                    self.append_attribute_name(c, /*is_name*/ true);
                }
                State::AfterAttributeName => {
                    if c == ' ' {
                        continue;
                    }
                    if c == '/' {
                        self.state = State::SelfClosingStartTag;
                        continue;
                    }
                    if c == '=' {
                        self.state = State::BeforeAttributeValue;
                        continue;
                    }
                    if c == '>' {
                        self.state = State::Data;
                        return self.take_latest_token();
                    }
                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }
                    self.reconsume = true;
                    self.state = State::AttributeName;
                    self.start_new_attribute();
                }
                State::BeforeAttributeValue => {
                    if c == ' ' {
                        continue;
                    }
                    if c == '"' {
                        self.state = State::AttributeValueDoubleQuoted;
                        continue;
                    }
                    if c == '\'' {
                        self.state = State::AttributeValueSingleQuoted;
                        continue;
                    }
                    self.reconsume = true;
                    self.state = State::AttributeValueUnQuoted;
                }
                State::AttributeValueDoubleQuoted => {
                    if c == '"' {
                        self.state = State::AfterAttributeValueQuoted;
                        continue;
                    }
                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }
                    self.append_attribute(c, /*is_name*/ false);
                }
                State::AttributeValueSingleQuoted => {
                    if c == '\'' {
                        self.state = State::AfterAttributeValueQuoted;
                        continue;
                    }
                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }
                    self.append_attribute(c, /*is_name*/ false);
                }
                State::AttributeValueUnQuoted => {
                    if c == ' ' {
                        self.state = State::BeforeAttributeName;
                        continue;
                    }
                    if c == '/' {
                        self.state = State::Data;
                        return self.take_latest_token();
                    }
                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }
                    self.append_attribute(c, /*is_name*/ false);
                }
                State::AfterAttributeValueQuoted => {
                    if c == ' ' {
                        continue;
                    }
                    if c == '/' {
                        self.state = State::SelfClosingStartTag;
                        continue;
                    }
                    if c == '>' {
                        self.state = State::Data;
                        return self.take_latest_token();
                    }
                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }
                    self.reconsume = true;
                    self.state = State::AttributeValueUnQuoted;
                }
                State::SelfClosingStartTag => {
                    if c == '>' {
                        self.set_self_closing_flag();
                        self.state = State::Data;
                        return self.take_latest_token();
                    }
                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }
                }
                State::ScriptData => {
                    if c == '<' {
                        self.state = State::ScriptDataLessThanSign;
                        continue;
                    }
                    if self.is_eof() {
                        return Some(HtmlToken::Eof);
                    }
                    return Some(HtmlToken::Char(c));
                }
                State::ScriptDataLessThanSign => {
                    if c == '>' {
                        self.buf = String::new();
                        self.state = State::ScriptDataEndTagOpen;
                        continue;
                    }
                    self.reconsume = true;
                    self.state = State::ScriptData;
                    return Some(HtmlToken::Char('<'));
                }
                State::ScriptDataEndTagOpen => {
                    if c.is_ascii_alphabetic() {
                        self.reconsume = true;
                        self.state = State::ScriptDataEndTagName;
                        self.create_tag(false);
                        continue;
                    }
                    self.reconsume = true;
                    self.state = State::ScriptData;
                    return Some(HtmlToken::Char('<'));
                }
                State::ScriptDataEndTagName => {
                    if c == '>' {
                        self.state = State::ScriptData;
                        return self.take_latest_token();
                    }
                    if c.is_ascii_uppercase() {
                        self.buf.push(c.to_ascii_lowercase());
                        continue;
                    }
                    self.state = State::TemporaryBuffer;
                    self.buf = String::from("</") + &self.buf;
                    self.buf.push(c);
                    continue;
                }
                State::TemporaryBuffer => {
                    self.reconsume = true;
                    if self.buf.chars().count() == 0 {
                        self.state = State::ScriptData;
                        continue;
                    }
                    let c = self
                        .buf
                        .chars()
                        .nth(0)
                        .expect("self.buf should have at least 1 char");
                    self.buf.remove(0);
                    return Some(HtmlToken::Char(c));
                }
            }
        }
    }
}