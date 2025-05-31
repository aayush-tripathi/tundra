// src/lexer/helper.rs 
use std::collections::HashMap;
use super::token::TokenType;

#[derive(Default)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    token_type: Option<TokenType>,
}

impl TrieNode {
    pub fn insert(&mut self, word: &str, token_type: TokenType) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_insert_with(TrieNode::default);
        }
        node.token_type = Some(token_type);
    }

    pub fn get_token_type(&self, word: &str) -> Option<TokenType> {
        let mut node = self;
        for c in word.chars() {
            match node.children.get(&c) {
                Some(next) => node = next,
                None => return None,
            }
        }
        node.token_type
    }
}
