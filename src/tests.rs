//
// src
// test.rs: Implements tests.
//
// Created by jenra.
// Created on October 18 2020.
//

use crate::lexer::*;

#[test]
fn lexer_misc_chars() {
	let string = String::from(".< >()\n#~");
	let mut lexer = Lexer::new(&string);
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::Dot);
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::LT);
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::GT);
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::LParen);
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::RParen);
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::Newline);
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::Hash);
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::Err(String::from("~")));
	assert!(if let None = lexer.next() { true } else { false });
}

#[test]
fn lexer_symbols() {
	let string = String::from("hewwo HEWWO _underscore");
	let mut lexer = Lexer::new(&string);
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::Symbol(String::from("hewwo")));
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::Symbol(String::from("HEWWO")));
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::Symbol(String::from("_underscore")));
	assert!(if let None = lexer.next() { true } else { false });
}

#[test]
fn lexer_numbers() {
	let string = String::from("%00101010 052 42 $2a $2A");
	let mut lexer = Lexer::new(&string);
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::Bin(42));
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::Oct(42));
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::Dec(42));
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::Hex(42));
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::Hex(42));
	assert!(if let None = lexer.next() { true } else { false });
}

#[test]
fn lexer_strings() {
	let string = String::from("\"hewwo\" \"this is a string\" \"this\nis\na\nmultiline\nstring\n\" \"this is an invalid string");
	let mut lexer = Lexer::new(&string);
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::String(String::from("hewwo")));
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::String(String::from("this is a string")));
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::String(String::from("this\nis\na\nmultiline\nstring\n")));
	assert_eq!(lexer.next().unwrap().token_value, TokenValue::Err(String::from("\"this is an invalid string")));
	assert!(if let None = lexer.next() { true } else { false });
}
