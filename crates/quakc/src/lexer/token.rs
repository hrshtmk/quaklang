use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {

	// ---Keywords---

	#[token("Out")]
	Out,
	
	#[token("Inp")]
	Inp,
	
	#[token("int")]
	TypeInt,
	
	#[token("float")]
	TypeFlt,
	
	#[token("str")]
	TypeStr,
	
	#[token("bool")]
	TypeBool,

	#[token("char")]
	TypeChar,
	
	// ---Symbols---
	
	#[token("{")]
	LBrace,	

	#[token("@")]
	At,

	#[token("->")]
	RightArrow,

	#[token("<-")]
	LeftArrow,

	#[token("=")]
	Equals,

	#[token("==")]
	Equates,	

	#[token("}")]
	RBrace,		
}
