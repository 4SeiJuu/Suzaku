// Generated from ./language-extensions/src/java/generated/JavaParser.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::javaparserlistener::*;
use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const ABSTRACT:isize=1; 
		pub const ASSERT:isize=2; 
		pub const BOOLEAN:isize=3; 
		pub const BREAK:isize=4; 
		pub const BYTE:isize=5; 
		pub const CASE:isize=6; 
		pub const CATCH:isize=7; 
		pub const CHAR:isize=8; 
		pub const CLASS:isize=9; 
		pub const CONST:isize=10; 
		pub const CONTINUE:isize=11; 
		pub const DEFAULT:isize=12; 
		pub const DO:isize=13; 
		pub const DOUBLE:isize=14; 
		pub const ELSE:isize=15; 
		pub const ENUM:isize=16; 
		pub const EXTENDS:isize=17; 
		pub const FINAL:isize=18; 
		pub const FINALLY:isize=19; 
		pub const FLOAT:isize=20; 
		pub const FOR:isize=21; 
		pub const IF:isize=22; 
		pub const GOTO:isize=23; 
		pub const IMPLEMENTS:isize=24; 
		pub const IMPORT:isize=25; 
		pub const INSTANCEOF:isize=26; 
		pub const INT:isize=27; 
		pub const INTERFACE:isize=28; 
		pub const LONG:isize=29; 
		pub const NATIVE:isize=30; 
		pub const NEW:isize=31; 
		pub const PACKAGE:isize=32; 
		pub const PRIVATE:isize=33; 
		pub const PROTECTED:isize=34; 
		pub const PUBLIC:isize=35; 
		pub const RETURN:isize=36; 
		pub const SHORT:isize=37; 
		pub const STATIC:isize=38; 
		pub const STRICTFP:isize=39; 
		pub const SUPER:isize=40; 
		pub const SWITCH:isize=41; 
		pub const SYNCHRONIZED:isize=42; 
		pub const THIS:isize=43; 
		pub const THROW:isize=44; 
		pub const THROWS:isize=45; 
		pub const TRANSIENT:isize=46; 
		pub const TRY:isize=47; 
		pub const VOID:isize=48; 
		pub const VOLATILE:isize=49; 
		pub const WHILE:isize=50; 
		pub const MODULE:isize=51; 
		pub const OPEN:isize=52; 
		pub const REQUIRES:isize=53; 
		pub const EXPORTS:isize=54; 
		pub const OPENS:isize=55; 
		pub const TO:isize=56; 
		pub const USES:isize=57; 
		pub const PROVIDES:isize=58; 
		pub const WITH:isize=59; 
		pub const TRANSITIVE:isize=60; 
		pub const VAR:isize=61; 
		pub const YIELD:isize=62; 
		pub const RECORD:isize=63; 
		pub const SEALED:isize=64; 
		pub const PERMITS:isize=65; 
		pub const NON_SEALED:isize=66; 
		pub const DECIMAL_LITERAL:isize=67; 
		pub const HEX_LITERAL:isize=68; 
		pub const OCT_LITERAL:isize=69; 
		pub const BINARY_LITERAL:isize=70; 
		pub const FLOAT_LITERAL:isize=71; 
		pub const HEX_FLOAT_LITERAL:isize=72; 
		pub const BOOL_LITERAL:isize=73; 
		pub const CHAR_LITERAL:isize=74; 
		pub const STRING_LITERAL:isize=75; 
		pub const TEXT_BLOCK:isize=76; 
		pub const NULL_LITERAL:isize=77; 
		pub const LPAREN:isize=78; 
		pub const RPAREN:isize=79; 
		pub const LBRACE:isize=80; 
		pub const RBRACE:isize=81; 
		pub const LBRACK:isize=82; 
		pub const RBRACK:isize=83; 
		pub const SEMI:isize=84; 
		pub const COMMA:isize=85; 
		pub const DOT:isize=86; 
		pub const ASSIGN:isize=87; 
		pub const GT:isize=88; 
		pub const LT:isize=89; 
		pub const BANG:isize=90; 
		pub const TILDE:isize=91; 
		pub const QUESTION:isize=92; 
		pub const COLON:isize=93; 
		pub const EQUAL:isize=94; 
		pub const LE:isize=95; 
		pub const GE:isize=96; 
		pub const NOTEQUAL:isize=97; 
		pub const AND:isize=98; 
		pub const OR:isize=99; 
		pub const INC:isize=100; 
		pub const DEC:isize=101; 
		pub const ADD:isize=102; 
		pub const SUB:isize=103; 
		pub const MUL:isize=104; 
		pub const DIV:isize=105; 
		pub const BITAND:isize=106; 
		pub const BITOR:isize=107; 
		pub const CARET:isize=108; 
		pub const MOD:isize=109; 
		pub const ADD_ASSIGN:isize=110; 
		pub const SUB_ASSIGN:isize=111; 
		pub const MUL_ASSIGN:isize=112; 
		pub const DIV_ASSIGN:isize=113; 
		pub const AND_ASSIGN:isize=114; 
		pub const OR_ASSIGN:isize=115; 
		pub const XOR_ASSIGN:isize=116; 
		pub const MOD_ASSIGN:isize=117; 
		pub const LSHIFT_ASSIGN:isize=118; 
		pub const RSHIFT_ASSIGN:isize=119; 
		pub const URSHIFT_ASSIGN:isize=120; 
		pub const ARROW:isize=121; 
		pub const COLONCOLON:isize=122; 
		pub const AT:isize=123; 
		pub const ELLIPSIS:isize=124; 
		pub const WS:isize=125; 
		pub const COMMENT:isize=126; 
		pub const LINE_COMMENT:isize=127; 
		pub const IDENTIFIER:isize=128;
	pub const RULE_compilationUnit:usize = 0; 
	pub const RULE_packageDeclaration:usize = 1; 
	pub const RULE_importDeclaration:usize = 2; 
	pub const RULE_typeDeclaration:usize = 3; 
	pub const RULE_modifier:usize = 4; 
	pub const RULE_variableModifier:usize = 5; 
	pub const RULE_classDeclaration:usize = 6; 
	pub const RULE_typeParameters:usize = 7; 
	pub const RULE_typeParameter:usize = 8; 
	pub const RULE_typeBound:usize = 9; 
	pub const RULE_enumDeclaration:usize = 10; 
	pub const RULE_enumConstants:usize = 11; 
	pub const RULE_enumConstant:usize = 12; 
	pub const RULE_enumBodyDeclarations:usize = 13; 
	pub const RULE_interfaceDeclaration:usize = 14; 
	pub const RULE_classBody:usize = 15; 
	pub const RULE_interfaceBody:usize = 16; 
	pub const RULE_classBodyDeclaration:usize = 17; 
	pub const RULE_memberDeclaration:usize = 18; 
	pub const RULE_methodDeclaration:usize = 19; 
	pub const RULE_methodBody:usize = 20; 
	pub const RULE_typeTypeOrVoid:usize = 21; 
	pub const RULE_genericMethodDeclaration:usize = 22; 
	pub const RULE_genericConstructorDeclaration:usize = 23; 
	pub const RULE_constructorDeclaration:usize = 24; 
	pub const RULE_compactConstructorDeclaration:usize = 25; 
	pub const RULE_fieldDeclaration:usize = 26; 
	pub const RULE_interfaceBodyDeclaration:usize = 27; 
	pub const RULE_interfaceMemberDeclaration:usize = 28; 
	pub const RULE_constDeclaration:usize = 29; 
	pub const RULE_constantDeclarator:usize = 30; 
	pub const RULE_interfaceMethodDeclaration:usize = 31; 
	pub const RULE_interfaceMethodModifier:usize = 32; 
	pub const RULE_genericInterfaceMethodDeclaration:usize = 33; 
	pub const RULE_interfaceCommonBodyDeclaration:usize = 34; 
	pub const RULE_variableDeclarators:usize = 35; 
	pub const RULE_variableDeclarator:usize = 36; 
	pub const RULE_variableDeclaratorId:usize = 37; 
	pub const RULE_variableInitializer:usize = 38; 
	pub const RULE_arrayInitializer:usize = 39; 
	pub const RULE_classOrInterfaceType:usize = 40; 
	pub const RULE_typeArgument:usize = 41; 
	pub const RULE_qualifiedNameList:usize = 42; 
	pub const RULE_formalParameters:usize = 43; 
	pub const RULE_receiverParameter:usize = 44; 
	pub const RULE_formalParameterList:usize = 45; 
	pub const RULE_formalParameter:usize = 46; 
	pub const RULE_lastFormalParameter:usize = 47; 
	pub const RULE_lambdaLVTIList:usize = 48; 
	pub const RULE_lambdaLVTIParameter:usize = 49; 
	pub const RULE_qualifiedName:usize = 50; 
	pub const RULE_literal:usize = 51; 
	pub const RULE_integerLiteral:usize = 52; 
	pub const RULE_floatLiteral:usize = 53; 
	pub const RULE_altAnnotationQualifiedName:usize = 54; 
	pub const RULE_annotation:usize = 55; 
	pub const RULE_elementValuePairs:usize = 56; 
	pub const RULE_elementValuePair:usize = 57; 
	pub const RULE_elementValue:usize = 58; 
	pub const RULE_elementValueArrayInitializer:usize = 59; 
	pub const RULE_annotationTypeDeclaration:usize = 60; 
	pub const RULE_annotationTypeBody:usize = 61; 
	pub const RULE_annotationTypeElementDeclaration:usize = 62; 
	pub const RULE_annotationTypeElementRest:usize = 63; 
	pub const RULE_annotationMethodOrConstantRest:usize = 64; 
	pub const RULE_annotationMethodRest:usize = 65; 
	pub const RULE_annotationConstantRest:usize = 66; 
	pub const RULE_defaultValue:usize = 67; 
	pub const RULE_moduleDeclaration:usize = 68; 
	pub const RULE_moduleBody:usize = 69; 
	pub const RULE_moduleDirective:usize = 70; 
	pub const RULE_requiresModifier:usize = 71; 
	pub const RULE_recordDeclaration:usize = 72; 
	pub const RULE_recordHeader:usize = 73; 
	pub const RULE_recordComponentList:usize = 74; 
	pub const RULE_recordComponent:usize = 75; 
	pub const RULE_recordBody:usize = 76; 
	pub const RULE_block:usize = 77; 
	pub const RULE_blockStatement:usize = 78; 
	pub const RULE_localVariableDeclaration:usize = 79; 
	pub const RULE_identifier:usize = 80; 
	pub const RULE_typeIdentifier:usize = 81; 
	pub const RULE_localTypeDeclaration:usize = 82; 
	pub const RULE_statement:usize = 83; 
	pub const RULE_catchClause:usize = 84; 
	pub const RULE_catchType:usize = 85; 
	pub const RULE_finallyBlock:usize = 86; 
	pub const RULE_resourceSpecification:usize = 87; 
	pub const RULE_resources:usize = 88; 
	pub const RULE_resource:usize = 89; 
	pub const RULE_switchBlockStatementGroup:usize = 90; 
	pub const RULE_switchLabel:usize = 91; 
	pub const RULE_forControl:usize = 92; 
	pub const RULE_forInit:usize = 93; 
	pub const RULE_enhancedForControl:usize = 94; 
	pub const RULE_parExpression:usize = 95; 
	pub const RULE_expressionList:usize = 96; 
	pub const RULE_methodCall:usize = 97; 
	pub const RULE_expression:usize = 98; 
	pub const RULE_pattern:usize = 99; 
	pub const RULE_lambdaExpression:usize = 100; 
	pub const RULE_lambdaParameters:usize = 101; 
	pub const RULE_lambdaBody:usize = 102; 
	pub const RULE_primary:usize = 103; 
	pub const RULE_switchExpression:usize = 104; 
	pub const RULE_switchLabeledRule:usize = 105; 
	pub const RULE_guardedPattern:usize = 106; 
	pub const RULE_switchRuleOutcome:usize = 107; 
	pub const RULE_classType:usize = 108; 
	pub const RULE_creator:usize = 109; 
	pub const RULE_createdName:usize = 110; 
	pub const RULE_innerCreator:usize = 111; 
	pub const RULE_arrayCreatorRest:usize = 112; 
	pub const RULE_classCreatorRest:usize = 113; 
	pub const RULE_explicitGenericInvocation:usize = 114; 
	pub const RULE_typeArgumentsOrDiamond:usize = 115; 
	pub const RULE_nonWildcardTypeArgumentsOrDiamond:usize = 116; 
	pub const RULE_nonWildcardTypeArguments:usize = 117; 
	pub const RULE_typeList:usize = 118; 
	pub const RULE_typeType:usize = 119; 
	pub const RULE_primitiveType:usize = 120; 
	pub const RULE_typeArguments:usize = 121; 
	pub const RULE_superSuffix:usize = 122; 
	pub const RULE_explicitGenericInvocationSuffix:usize = 123; 
	pub const RULE_arguments:usize = 124;
	pub const ruleNames: [&'static str; 125] =  [
		"compilationUnit", "packageDeclaration", "importDeclaration", "typeDeclaration", 
		"modifier", "variableModifier", "classDeclaration", "typeParameters", 
		"typeParameter", "typeBound", "enumDeclaration", "enumConstants", "enumConstant", 
		"enumBodyDeclarations", "interfaceDeclaration", "classBody", "interfaceBody", 
		"classBodyDeclaration", "memberDeclaration", "methodDeclaration", "methodBody", 
		"typeTypeOrVoid", "genericMethodDeclaration", "genericConstructorDeclaration", 
		"constructorDeclaration", "compactConstructorDeclaration", "fieldDeclaration", 
		"interfaceBodyDeclaration", "interfaceMemberDeclaration", "constDeclaration", 
		"constantDeclarator", "interfaceMethodDeclaration", "interfaceMethodModifier", 
		"genericInterfaceMethodDeclaration", "interfaceCommonBodyDeclaration", 
		"variableDeclarators", "variableDeclarator", "variableDeclaratorId", "variableInitializer", 
		"arrayInitializer", "classOrInterfaceType", "typeArgument", "qualifiedNameList", 
		"formalParameters", "receiverParameter", "formalParameterList", "formalParameter", 
		"lastFormalParameter", "lambdaLVTIList", "lambdaLVTIParameter", "qualifiedName", 
		"literal", "integerLiteral", "floatLiteral", "altAnnotationQualifiedName", 
		"annotation", "elementValuePairs", "elementValuePair", "elementValue", 
		"elementValueArrayInitializer", "annotationTypeDeclaration", "annotationTypeBody", 
		"annotationTypeElementDeclaration", "annotationTypeElementRest", "annotationMethodOrConstantRest", 
		"annotationMethodRest", "annotationConstantRest", "defaultValue", "moduleDeclaration", 
		"moduleBody", "moduleDirective", "requiresModifier", "recordDeclaration", 
		"recordHeader", "recordComponentList", "recordComponent", "recordBody", 
		"block", "blockStatement", "localVariableDeclaration", "identifier", "typeIdentifier", 
		"localTypeDeclaration", "statement", "catchClause", "catchType", "finallyBlock", 
		"resourceSpecification", "resources", "resource", "switchBlockStatementGroup", 
		"switchLabel", "forControl", "forInit", "enhancedForControl", "parExpression", 
		"expressionList", "methodCall", "expression", "pattern", "lambdaExpression", 
		"lambdaParameters", "lambdaBody", "primary", "switchExpression", "switchLabeledRule", 
		"guardedPattern", "switchRuleOutcome", "classType", "creator", "createdName", 
		"innerCreator", "arrayCreatorRest", "classCreatorRest", "explicitGenericInvocation", 
		"typeArgumentsOrDiamond", "nonWildcardTypeArgumentsOrDiamond", "nonWildcardTypeArguments", 
		"typeList", "typeType", "primitiveType", "typeArguments", "superSuffix", 
		"explicitGenericInvocationSuffix", "arguments"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;125] = [
		None, Some("'abstract'"), Some("'assert'"), Some("'boolean'"), Some("'break'"), 
		Some("'byte'"), Some("'case'"), Some("'catch'"), Some("'char'"), Some("'class'"), 
		Some("'const'"), Some("'continue'"), Some("'default'"), Some("'do'"), 
		Some("'double'"), Some("'else'"), Some("'enum'"), Some("'extends'"), Some("'final'"), 
		Some("'finally'"), Some("'float'"), Some("'for'"), Some("'if'"), Some("'goto'"), 
		Some("'implements'"), Some("'import'"), Some("'instanceof'"), Some("'int'"), 
		Some("'interface'"), Some("'long'"), Some("'native'"), Some("'new'"), 
		Some("'package'"), Some("'private'"), Some("'protected'"), Some("'public'"), 
		Some("'return'"), Some("'short'"), Some("'static'"), Some("'strictfp'"), 
		Some("'super'"), Some("'switch'"), Some("'synchronized'"), Some("'this'"), 
		Some("'throw'"), Some("'throws'"), Some("'transient'"), Some("'try'"), 
		Some("'void'"), Some("'volatile'"), Some("'while'"), Some("'module'"), 
		Some("'open'"), Some("'requires'"), Some("'exports'"), Some("'opens'"), 
		Some("'to'"), Some("'uses'"), Some("'provides'"), Some("'with'"), Some("'transitive'"), 
		Some("'var'"), Some("'yield'"), Some("'record'"), Some("'sealed'"), Some("'permits'"), 
		Some("'non-sealed'"), None, None, None, None, None, None, None, None, 
		None, None, Some("'null'"), Some("'('"), Some("')'"), Some("'{'"), Some("'}'"), 
		Some("'['"), Some("']'"), Some("';'"), Some("','"), Some("'.'"), Some("'='"), 
		Some("'>'"), Some("'<'"), Some("'!'"), Some("'~'"), Some("'?'"), Some("':'"), 
		Some("'=='"), Some("'<='"), Some("'>='"), Some("'!='"), Some("'&&'"), 
		Some("'||'"), Some("'++'"), Some("'--'"), Some("'+'"), Some("'-'"), Some("'*'"), 
		Some("'/'"), Some("'&'"), Some("'|'"), Some("'^'"), Some("'%'"), Some("'+='"), 
		Some("'-='"), Some("'*='"), Some("'/='"), Some("'&='"), Some("'|='"), 
		Some("'^='"), Some("'%='"), Some("'<<='"), Some("'>>='"), Some("'>>>='"), 
		Some("'->'"), Some("'::'"), Some("'@'"), Some("'...'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;129]  = [
		None, Some("ABSTRACT"), Some("ASSERT"), Some("BOOLEAN"), Some("BREAK"), 
		Some("BYTE"), Some("CASE"), Some("CATCH"), Some("CHAR"), Some("CLASS"), 
		Some("CONST"), Some("CONTINUE"), Some("DEFAULT"), Some("DO"), Some("DOUBLE"), 
		Some("ELSE"), Some("ENUM"), Some("EXTENDS"), Some("FINAL"), Some("FINALLY"), 
		Some("FLOAT"), Some("FOR"), Some("IF"), Some("GOTO"), Some("IMPLEMENTS"), 
		Some("IMPORT"), Some("INSTANCEOF"), Some("INT"), Some("INTERFACE"), Some("LONG"), 
		Some("NATIVE"), Some("NEW"), Some("PACKAGE"), Some("PRIVATE"), Some("PROTECTED"), 
		Some("PUBLIC"), Some("RETURN"), Some("SHORT"), Some("STATIC"), Some("STRICTFP"), 
		Some("SUPER"), Some("SWITCH"), Some("SYNCHRONIZED"), Some("THIS"), Some("THROW"), 
		Some("THROWS"), Some("TRANSIENT"), Some("TRY"), Some("VOID"), Some("VOLATILE"), 
		Some("WHILE"), Some("MODULE"), Some("OPEN"), Some("REQUIRES"), Some("EXPORTS"), 
		Some("OPENS"), Some("TO"), Some("USES"), Some("PROVIDES"), Some("WITH"), 
		Some("TRANSITIVE"), Some("VAR"), Some("YIELD"), Some("RECORD"), Some("SEALED"), 
		Some("PERMITS"), Some("NON_SEALED"), Some("DECIMAL_LITERAL"), Some("HEX_LITERAL"), 
		Some("OCT_LITERAL"), Some("BINARY_LITERAL"), Some("FLOAT_LITERAL"), Some("HEX_FLOAT_LITERAL"), 
		Some("BOOL_LITERAL"), Some("CHAR_LITERAL"), Some("STRING_LITERAL"), Some("TEXT_BLOCK"), 
		Some("NULL_LITERAL"), Some("LPAREN"), Some("RPAREN"), Some("LBRACE"), 
		Some("RBRACE"), Some("LBRACK"), Some("RBRACK"), Some("SEMI"), Some("COMMA"), 
		Some("DOT"), Some("ASSIGN"), Some("GT"), Some("LT"), Some("BANG"), Some("TILDE"), 
		Some("QUESTION"), Some("COLON"), Some("EQUAL"), Some("LE"), Some("GE"), 
		Some("NOTEQUAL"), Some("AND"), Some("OR"), Some("INC"), Some("DEC"), Some("ADD"), 
		Some("SUB"), Some("MUL"), Some("DIV"), Some("BITAND"), Some("BITOR"), 
		Some("CARET"), Some("MOD"), Some("ADD_ASSIGN"), Some("SUB_ASSIGN"), Some("MUL_ASSIGN"), 
		Some("DIV_ASSIGN"), Some("AND_ASSIGN"), Some("OR_ASSIGN"), Some("XOR_ASSIGN"), 
		Some("MOD_ASSIGN"), Some("LSHIFT_ASSIGN"), Some("RSHIFT_ASSIGN"), Some("URSHIFT_ASSIGN"), 
		Some("ARROW"), Some("COLONCOLON"), Some("AT"), Some("ELLIPSIS"), Some("WS"), 
		Some("COMMENT"), Some("LINE_COMMENT"), Some("IDENTIFIER")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,JavaParserExt<'input>, I, JavaParserContextType , dyn JavaParserListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type JavaParserTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, JavaParserContextType , dyn JavaParserListener<'input> + 'a>;

/// Parser for JavaParser grammar
pub struct JavaParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				JavaParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> JavaParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> JavaParser<'input, I, DefaultErrorStrategy<'input,JavaParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for JavaParser
pub trait JavaParserContext<'input>:
	for<'x> Listenable<dyn JavaParserListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=JavaParserContextType>
{}

antlr_rust::coerce_from!{ 'input : JavaParserContext<'input> }

impl<'input> JavaParserContext<'input> for TerminalNode<'input,JavaParserContextType> {}
impl<'input> JavaParserContext<'input> for ErrorNode<'input,JavaParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn JavaParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn JavaParserListener<'input> + 'input }

pub struct JavaParserContextType;
antlr_rust::tid!{JavaParserContextType}

impl<'input> ParserNodeType<'input> for JavaParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn JavaParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct JavaParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> JavaParserExt<'input>{
}
antlr_rust::tid! { JavaParserExt<'a> }

impl<'input> TokenAware<'input> for JavaParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for JavaParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for JavaParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "JavaParser.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn JavaParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					98 => JavaParser::<'input,I,_>::expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					106 => JavaParser::<'input,I,_>::guardedPattern_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> JavaParser<'input, I, DefaultErrorStrategy<'input,JavaParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expression_sempred(_localctx: Option<&ExpressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 18)
				}
				1=>{
					recog.precpred(None, 17)
				}
				2=>{
					recog.precpred(None, 16)
				}
				3=>{
					recog.precpred(None, 15)
				}
				4=>{
					recog.precpred(None, 13)
				}
				5=>{
					recog.precpred(None, 12)
				}
				6=>{
					recog.precpred(None, 11)
				}
				7=>{
					recog.precpred(None, 10)
				}
				8=>{
					recog.precpred(None, 9)
				}
				9=>{
					recog.precpred(None, 8)
				}
				10=>{
					recog.precpred(None, 7)
				}
				11=>{
					recog.precpred(None, 6)
				}
				12=>{
					recog.precpred(None, 26)
				}
				13=>{
					recog.precpred(None, 25)
				}
				14=>{
					recog.precpred(None, 21)
				}
				15=>{
					recog.precpred(None, 14)
				}
				16=>{
					recog.precpred(None, 3)
				}
			_ => true
		}
	}
	fn guardedPattern_sempred(_localctx: Option<&GuardedPatternContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				17=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
}
//------------------- compilationUnit ----------------
pub type CompilationUnitContextAll<'input> = CompilationUnitContext<'input>;


pub type CompilationUnitContext<'input> = BaseParserRuleContext<'input,CompilationUnitContextExt<'input>>;

#[derive(Clone)]
pub struct CompilationUnitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for CompilationUnitContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for CompilationUnitContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_compilationUnit(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_compilationUnit(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for CompilationUnitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compilationUnit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compilationUnit }
}
antlr_rust::tid!{CompilationUnitContextExt<'a>}

impl<'input> CompilationUnitContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompilationUnitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompilationUnitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompilationUnitContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<CompilationUnitContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token COMMENT in current rule
fn COMMENT_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMENT, starting from 0.
/// Returns `None` if number of children corresponding to token COMMENT is less or equal than `i`.
fn COMMENT(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMENT, i)
}
fn packageDeclaration(&self) -> Option<Rc<PackageDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn importDeclaration_all(&self) ->  Vec<Rc<ImportDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn importDeclaration(&self, i: usize) -> Option<Rc<ImportDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn typeDeclaration_all(&self) ->  Vec<Rc<TypeDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeDeclaration(&self, i: usize) -> Option<Rc<TypeDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn moduleDeclaration(&self) -> Option<Rc<ModuleDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}

}

impl<'input> CompilationUnitContextAttrs<'input> for CompilationUnitContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compilationUnit(&mut self,)
	-> Result<Rc<CompilationUnitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompilationUnitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_compilationUnit);
        let mut _localctx: Rc<CompilationUnitContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(274);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(4,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(253);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==COMMENT {
						{
						{
						recog.base.set_state(250);
						recog.base.match_token(COMMENT,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(255);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(257);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(1,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule packageDeclaration*/
							recog.base.set_state(256);
							recog.packageDeclaration()?;

							}
						}

						_ => {}
					}
					recog.base.set_state(262);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==IMPORT {
						{
						{
						/*InvokeRule importDeclaration*/
						recog.base.set_state(259);
						recog.importDeclaration()?;

						}
						}
						recog.base.set_state(264);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(268);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ABSTRACT) | (1usize << CLASS) | (1usize << ENUM) | (1usize << FINAL) | (1usize << INTERFACE) | (1usize << NATIVE))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (PRIVATE - 33)) | (1usize << (PROTECTED - 33)) | (1usize << (PUBLIC - 33)) | (1usize << (STATIC - 33)) | (1usize << (STRICTFP - 33)) | (1usize << (SYNCHRONIZED - 33)) | (1usize << (TRANSIENT - 33)) | (1usize << (VOLATILE - 33)) | (1usize << (MODULE - 33)) | (1usize << (OPEN - 33)) | (1usize << (REQUIRES - 33)) | (1usize << (EXPORTS - 33)) | (1usize << (OPENS - 33)) | (1usize << (TO - 33)) | (1usize << (USES - 33)) | (1usize << (PROVIDES - 33)) | (1usize << (WITH - 33)) | (1usize << (TRANSITIVE - 33)) | (1usize << (VAR - 33)) | (1usize << (YIELD - 33)) | (1usize << (RECORD - 33)) | (1usize << (SEALED - 33)))) != 0) || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (PERMITS - 65)) | (1usize << (NON_SEALED - 65)) | (1usize << (SEMI - 65)))) != 0) || _la==AT || _la==IDENTIFIER {
						{
						{
						/*InvokeRule typeDeclaration*/
						recog.base.set_state(265);
						recog.typeDeclaration()?;

						}
						}
						recog.base.set_state(270);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule moduleDeclaration*/
					recog.base.set_state(271);
					recog.moduleDeclaration()?;

					recog.base.set_state(272);
					recog.base.match_token(EOF,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- packageDeclaration ----------------
pub type PackageDeclarationContextAll<'input> = PackageDeclarationContext<'input>;


pub type PackageDeclarationContext<'input> = BaseParserRuleContext<'input,PackageDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct PackageDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for PackageDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for PackageDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_packageDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_packageDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PackageDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_packageDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_packageDeclaration }
}
antlr_rust::tid!{PackageDeclarationContextExt<'a>}

impl<'input> PackageDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PackageDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PackageDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PackageDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<PackageDeclarationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PACKAGE
/// Returns `None` if there is no child corresponding to token PACKAGE
fn PACKAGE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(PACKAGE, 0)
}
fn qualifiedName(&self) -> Option<Rc<QualifiedNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PackageDeclarationContextAttrs<'input> for PackageDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn packageDeclaration(&mut self,)
	-> Result<Rc<PackageDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PackageDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_packageDeclaration);
        let mut _localctx: Rc<PackageDeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(279);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 51)) & !0x3f) == 0 && ((1usize << (_la - 51)) & ((1usize << (MODULE - 51)) | (1usize << (OPEN - 51)) | (1usize << (REQUIRES - 51)) | (1usize << (EXPORTS - 51)) | (1usize << (OPENS - 51)) | (1usize << (TO - 51)) | (1usize << (USES - 51)) | (1usize << (PROVIDES - 51)) | (1usize << (WITH - 51)) | (1usize << (TRANSITIVE - 51)) | (1usize << (VAR - 51)) | (1usize << (YIELD - 51)) | (1usize << (RECORD - 51)) | (1usize << (SEALED - 51)) | (1usize << (PERMITS - 51)))) != 0) || _la==AT || _la==IDENTIFIER {
				{
				{
				/*InvokeRule annotation*/
				recog.base.set_state(276);
				recog.annotation()?;

				}
				}
				recog.base.set_state(281);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(282);
			recog.base.match_token(PACKAGE,&mut recog.err_handler)?;

			/*InvokeRule qualifiedName*/
			recog.base.set_state(283);
			recog.qualifiedName()?;

			recog.base.set_state(284);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- importDeclaration ----------------
pub type ImportDeclarationContextAll<'input> = ImportDeclarationContext<'input>;


pub type ImportDeclarationContext<'input> = BaseParserRuleContext<'input,ImportDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ImportDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ImportDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ImportDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_importDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_importDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ImportDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importDeclaration }
}
antlr_rust::tid!{ImportDeclarationContextExt<'a>}

impl<'input> ImportDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ImportDeclarationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IMPORT
/// Returns `None` if there is no child corresponding to token IMPORT
fn IMPORT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(IMPORT, 0)
}
fn qualifiedName(&self) -> Option<Rc<QualifiedNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
/// Retrieves first TerminalNode corresponding to token STATIC
/// Returns `None` if there is no child corresponding to token STATIC
fn STATIC(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(STATIC, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
/// Retrieves first TerminalNode corresponding to token MUL
/// Returns `None` if there is no child corresponding to token MUL
fn MUL(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(MUL, 0)
}

}

impl<'input> ImportDeclarationContextAttrs<'input> for ImportDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn importDeclaration(&mut self,)
	-> Result<Rc<ImportDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_importDeclaration);
        let mut _localctx: Rc<ImportDeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(286);
			recog.base.match_token(IMPORT,&mut recog.err_handler)?;

			recog.base.set_state(288);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==STATIC {
				{
				recog.base.set_state(287);
				recog.base.match_token(STATIC,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule qualifiedName*/
			recog.base.set_state(290);
			recog.qualifiedName()?;

			recog.base.set_state(293);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==DOT {
				{
				recog.base.set_state(291);
				recog.base.match_token(DOT,&mut recog.err_handler)?;

				recog.base.set_state(292);
				recog.base.match_token(MUL,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(295);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeDeclaration ----------------
pub type TypeDeclarationContextAll<'input> = TypeDeclarationContext<'input>;


pub type TypeDeclarationContext<'input> = BaseParserRuleContext<'input,TypeDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct TypeDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for TypeDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for TypeDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_typeDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_typeDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TypeDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeDeclaration }
}
antlr_rust::tid!{TypeDeclarationContextExt<'a>}

impl<'input> TypeDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<TypeDeclarationContextExt<'input>>{

fn classDeclaration(&self) -> Option<Rc<ClassDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumDeclaration(&self) -> Option<Rc<EnumDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn interfaceDeclaration(&self) -> Option<Rc<InterfaceDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotationTypeDeclaration(&self) -> Option<Rc<AnnotationTypeDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn recordDeclaration(&self) -> Option<Rc<RecordDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn modifier_all(&self) ->  Vec<Rc<ModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn modifier(&self, i: usize) -> Option<Rc<ModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}

}

impl<'input> TypeDeclarationContextAttrs<'input> for TypeDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeDeclaration(&mut self,)
	-> Result<Rc<TypeDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_typeDeclaration);
        let mut _localctx: Rc<TypeDeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(317);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ABSTRACT | CLASS | ENUM | FINAL | INTERFACE | NATIVE | PRIVATE | PROTECTED |
			 PUBLIC | STATIC | STRICTFP | SYNCHRONIZED | TRANSIENT | VOLATILE | MODULE |
			 OPEN | REQUIRES | EXPORTS | OPENS | TO | USES | PROVIDES | WITH | TRANSITIVE |
			 VAR | YIELD | RECORD | SEALED | PERMITS | NON_SEALED | AT | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(300);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(8,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule annotation*/
							recog.base.set_state(297);
							recog.annotation()?;

							}
							} 
						}
						recog.base.set_state(302);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(8,&mut recog.base)?;
					}
					recog.base.set_state(306);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ABSTRACT) | (1usize << FINAL) | (1usize << NATIVE))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (PRIVATE - 33)) | (1usize << (PROTECTED - 33)) | (1usize << (PUBLIC - 33)) | (1usize << (STATIC - 33)) | (1usize << (STRICTFP - 33)) | (1usize << (SYNCHRONIZED - 33)) | (1usize << (TRANSIENT - 33)) | (1usize << (VOLATILE - 33)) | (1usize << (SEALED - 33)))) != 0) || _la==NON_SEALED {
						{
						{
						/*InvokeRule modifier*/
						recog.base.set_state(303);
						recog.modifier()?;

						}
						}
						recog.base.set_state(308);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(314);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 CLASS 
						=> {
							{
							/*InvokeRule classDeclaration*/
							recog.base.set_state(309);
							recog.classDeclaration()?;

							}
						}

					 ENUM 
						=> {
							{
							/*InvokeRule enumDeclaration*/
							recog.base.set_state(310);
							recog.enumDeclaration()?;

							}
						}

					 INTERFACE 
						=> {
							{
							/*InvokeRule interfaceDeclaration*/
							recog.base.set_state(311);
							recog.interfaceDeclaration()?;

							}
						}

					 AT 
						=> {
							{
							/*InvokeRule annotationTypeDeclaration*/
							recog.base.set_state(312);
							recog.annotationTypeDeclaration()?;

							}
						}

					 RECORD 
						=> {
							{
							/*InvokeRule recordDeclaration*/
							recog.base.set_state(313);
							recog.recordDeclaration()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}

			 SEMI 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(316);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- modifier ----------------
pub type ModifierContextAll<'input> = ModifierContext<'input>;


pub type ModifierContext<'input> = BaseParserRuleContext<'input,ModifierContextExt<'input>>;

#[derive(Clone)]
pub struct ModifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ModifierContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ModifierContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_modifier(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_modifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ModifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_modifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_modifier }
}
antlr_rust::tid!{ModifierContextExt<'a>}

impl<'input> ModifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModifierContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ModifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NATIVE
/// Returns `None` if there is no child corresponding to token NATIVE
fn NATIVE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(NATIVE, 0)
}
/// Retrieves first TerminalNode corresponding to token SYNCHRONIZED
/// Returns `None` if there is no child corresponding to token SYNCHRONIZED
fn SYNCHRONIZED(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SYNCHRONIZED, 0)
}
/// Retrieves first TerminalNode corresponding to token TRANSIENT
/// Returns `None` if there is no child corresponding to token TRANSIENT
fn TRANSIENT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(TRANSIENT, 0)
}
/// Retrieves first TerminalNode corresponding to token VOLATILE
/// Returns `None` if there is no child corresponding to token VOLATILE
fn VOLATILE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(VOLATILE, 0)
}
/// Retrieves first TerminalNode corresponding to token PUBLIC
/// Returns `None` if there is no child corresponding to token PUBLIC
fn PUBLIC(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(PUBLIC, 0)
}
/// Retrieves first TerminalNode corresponding to token PROTECTED
/// Returns `None` if there is no child corresponding to token PROTECTED
fn PROTECTED(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(PROTECTED, 0)
}
/// Retrieves first TerminalNode corresponding to token PRIVATE
/// Returns `None` if there is no child corresponding to token PRIVATE
fn PRIVATE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(PRIVATE, 0)
}
/// Retrieves first TerminalNode corresponding to token STATIC
/// Returns `None` if there is no child corresponding to token STATIC
fn STATIC(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(STATIC, 0)
}
/// Retrieves first TerminalNode corresponding to token ABSTRACT
/// Returns `None` if there is no child corresponding to token ABSTRACT
fn ABSTRACT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(ABSTRACT, 0)
}
/// Retrieves first TerminalNode corresponding to token FINAL
/// Returns `None` if there is no child corresponding to token FINAL
fn FINAL(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(FINAL, 0)
}
/// Retrieves first TerminalNode corresponding to token STRICTFP
/// Returns `None` if there is no child corresponding to token STRICTFP
fn STRICTFP(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(STRICTFP, 0)
}
/// Retrieves first TerminalNode corresponding to token SEALED
/// Returns `None` if there is no child corresponding to token SEALED
fn SEALED(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEALED, 0)
}
/// Retrieves first TerminalNode corresponding to token NON_SEALED
/// Returns `None` if there is no child corresponding to token NON_SEALED
fn NON_SEALED(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(NON_SEALED, 0)
}

}

impl<'input> ModifierContextAttrs<'input> for ModifierContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn modifier(&mut self,)
	-> Result<Rc<ModifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_modifier);
        let mut _localctx: Rc<ModifierContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(319);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ABSTRACT) | (1usize << FINAL) | (1usize << NATIVE))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (PRIVATE - 33)) | (1usize << (PROTECTED - 33)) | (1usize << (PUBLIC - 33)) | (1usize << (STATIC - 33)) | (1usize << (STRICTFP - 33)) | (1usize << (SYNCHRONIZED - 33)) | (1usize << (TRANSIENT - 33)) | (1usize << (VOLATILE - 33)) | (1usize << (SEALED - 33)))) != 0) || _la==NON_SEALED) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- variableModifier ----------------
pub type VariableModifierContextAll<'input> = VariableModifierContext<'input>;


pub type VariableModifierContext<'input> = BaseParserRuleContext<'input,VariableModifierContextExt<'input>>;

#[derive(Clone)]
pub struct VariableModifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for VariableModifierContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for VariableModifierContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_variableModifier(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_variableModifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for VariableModifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variableModifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variableModifier }
}
antlr_rust::tid!{VariableModifierContextExt<'a>}

impl<'input> VariableModifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariableModifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableModifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VariableModifierContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<VariableModifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FINAL
/// Returns `None` if there is no child corresponding to token FINAL
fn FINAL(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(FINAL, 0)
}
fn annotation(&self) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> VariableModifierContextAttrs<'input> for VariableModifierContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variableModifier(&mut self,)
	-> Result<Rc<VariableModifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableModifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_variableModifier);
        let mut _localctx: Rc<VariableModifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(323);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 FINAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(321);
					recog.base.match_token(FINAL,&mut recog.err_handler)?;

					}
				}

			 MODULE | OPEN | REQUIRES | EXPORTS | OPENS | TO | USES | PROVIDES | WITH |
			 TRANSITIVE | VAR | YIELD | RECORD | SEALED | PERMITS | AT | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule annotation*/
					recog.base.set_state(322);
					recog.annotation()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- classDeclaration ----------------
pub type ClassDeclarationContextAll<'input> = ClassDeclarationContext<'input>;


pub type ClassDeclarationContext<'input> = BaseParserRuleContext<'input,ClassDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ClassDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ClassDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ClassDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_classDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_classDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ClassDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_classDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_classDeclaration }
}
antlr_rust::tid!{ClassDeclarationContextExt<'a>}

impl<'input> ClassDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ClassDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ClassDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ClassDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ClassDeclarationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CLASS
/// Returns `None` if there is no child corresponding to token CLASS
fn CLASS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(CLASS, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn classBody(&self) -> Option<Rc<ClassBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeParameters(&self) -> Option<Rc<TypeParametersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EXTENDS
/// Returns `None` if there is no child corresponding to token EXTENDS
fn EXTENDS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(EXTENDS, 0)
}
fn typeType(&self) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IMPLEMENTS
/// Returns `None` if there is no child corresponding to token IMPLEMENTS
fn IMPLEMENTS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(IMPLEMENTS, 0)
}
fn typeList_all(&self) ->  Vec<Rc<TypeListContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeList(&self, i: usize) -> Option<Rc<TypeListContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token PERMITS
/// Returns `None` if there is no child corresponding to token PERMITS
fn PERMITS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(PERMITS, 0)
}

}

impl<'input> ClassDeclarationContextAttrs<'input> for ClassDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn classDeclaration(&mut self,)
	-> Result<Rc<ClassDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ClassDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_classDeclaration);
        let mut _localctx: Rc<ClassDeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(325);
			recog.base.match_token(CLASS,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(326);
			recog.identifier()?;

			recog.base.set_state(328);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LT {
				{
				/*InvokeRule typeParameters*/
				recog.base.set_state(327);
				recog.typeParameters()?;

				}
			}

			recog.base.set_state(332);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==EXTENDS {
				{
				recog.base.set_state(330);
				recog.base.match_token(EXTENDS,&mut recog.err_handler)?;

				/*InvokeRule typeType*/
				recog.base.set_state(331);
				recog.typeType()?;

				}
			}

			recog.base.set_state(336);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IMPLEMENTS {
				{
				recog.base.set_state(334);
				recog.base.match_token(IMPLEMENTS,&mut recog.err_handler)?;

				/*InvokeRule typeList*/
				recog.base.set_state(335);
				recog.typeList()?;

				}
			}

			recog.base.set_state(340);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==PERMITS {
				{
				recog.base.set_state(338);
				recog.base.match_token(PERMITS,&mut recog.err_handler)?;

				/*InvokeRule typeList*/
				recog.base.set_state(339);
				recog.typeList()?;

				}
			}

			/*InvokeRule classBody*/
			recog.base.set_state(342);
			recog.classBody()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeParameters ----------------
pub type TypeParametersContextAll<'input> = TypeParametersContext<'input>;


pub type TypeParametersContext<'input> = BaseParserRuleContext<'input,TypeParametersContextExt<'input>>;

#[derive(Clone)]
pub struct TypeParametersContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for TypeParametersContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for TypeParametersContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_typeParameters(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_typeParameters(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TypeParametersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeParameters }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeParameters }
}
antlr_rust::tid!{TypeParametersContextExt<'a>}

impl<'input> TypeParametersContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeParametersContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeParametersContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeParametersContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<TypeParametersContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LT
/// Returns `None` if there is no child corresponding to token LT
fn LT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LT, 0)
}
fn typeParameter_all(&self) ->  Vec<Rc<TypeParameterContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeParameter(&self, i: usize) -> Option<Rc<TypeParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token GT
/// Returns `None` if there is no child corresponding to token GT
fn GT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(GT, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> TypeParametersContextAttrs<'input> for TypeParametersContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeParameters(&mut self,)
	-> Result<Rc<TypeParametersContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeParametersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_typeParameters);
        let mut _localctx: Rc<TypeParametersContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(344);
			recog.base.match_token(LT,&mut recog.err_handler)?;

			/*InvokeRule typeParameter*/
			recog.base.set_state(345);
			recog.typeParameter()?;

			recog.base.set_state(350);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(346);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule typeParameter*/
				recog.base.set_state(347);
				recog.typeParameter()?;

				}
				}
				recog.base.set_state(352);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(353);
			recog.base.match_token(GT,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeParameter ----------------
pub type TypeParameterContextAll<'input> = TypeParameterContext<'input>;


pub type TypeParameterContext<'input> = BaseParserRuleContext<'input,TypeParameterContextExt<'input>>;

#[derive(Clone)]
pub struct TypeParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for TypeParameterContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for TypeParameterContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_typeParameter(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_typeParameter(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TypeParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeParameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeParameter }
}
antlr_rust::tid!{TypeParameterContextExt<'a>}

impl<'input> TypeParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeParameterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeParameterContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<TypeParameterContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token EXTENDS
/// Returns `None` if there is no child corresponding to token EXTENDS
fn EXTENDS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(EXTENDS, 0)
}
fn typeBound(&self) -> Option<Rc<TypeBoundContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TypeParameterContextAttrs<'input> for TypeParameterContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeParameter(&mut self,)
	-> Result<Rc<TypeParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_typeParameter);
        let mut _localctx: Rc<TypeParameterContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(358);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(18,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule annotation*/
					recog.base.set_state(355);
					recog.annotation()?;

					}
					} 
				}
				recog.base.set_state(360);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(18,&mut recog.base)?;
			}
			/*InvokeRule identifier*/
			recog.base.set_state(361);
			recog.identifier()?;

			recog.base.set_state(370);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==EXTENDS {
				{
				recog.base.set_state(362);
				recog.base.match_token(EXTENDS,&mut recog.err_handler)?;

				recog.base.set_state(366);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(19,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						/*InvokeRule annotation*/
						recog.base.set_state(363);
						recog.annotation()?;

						}
						} 
					}
					recog.base.set_state(368);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(19,&mut recog.base)?;
				}
				/*InvokeRule typeBound*/
				recog.base.set_state(369);
				recog.typeBound()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeBound ----------------
pub type TypeBoundContextAll<'input> = TypeBoundContext<'input>;


pub type TypeBoundContext<'input> = BaseParserRuleContext<'input,TypeBoundContextExt<'input>>;

#[derive(Clone)]
pub struct TypeBoundContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for TypeBoundContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for TypeBoundContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_typeBound(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_typeBound(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TypeBoundContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeBound }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeBound }
}
antlr_rust::tid!{TypeBoundContextExt<'a>}

impl<'input> TypeBoundContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeBoundContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeBoundContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeBoundContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<TypeBoundContextExt<'input>>{

fn typeType_all(&self) ->  Vec<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeType(&self, i: usize) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token BITAND in current rule
fn BITAND_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token BITAND, starting from 0.
/// Returns `None` if number of children corresponding to token BITAND is less or equal than `i`.
fn BITAND(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(BITAND, i)
}

}

impl<'input> TypeBoundContextAttrs<'input> for TypeBoundContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeBound(&mut self,)
	-> Result<Rc<TypeBoundContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeBoundContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_typeBound);
        let mut _localctx: Rc<TypeBoundContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typeType*/
			recog.base.set_state(372);
			recog.typeType()?;

			recog.base.set_state(377);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==BITAND {
				{
				{
				recog.base.set_state(373);
				recog.base.match_token(BITAND,&mut recog.err_handler)?;

				/*InvokeRule typeType*/
				recog.base.set_state(374);
				recog.typeType()?;

				}
				}
				recog.base.set_state(379);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- enumDeclaration ----------------
pub type EnumDeclarationContextAll<'input> = EnumDeclarationContext<'input>;


pub type EnumDeclarationContext<'input> = BaseParserRuleContext<'input,EnumDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct EnumDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for EnumDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for EnumDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_enumDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_enumDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for EnumDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumDeclaration }
}
antlr_rust::tid!{EnumDeclarationContextExt<'a>}

impl<'input> EnumDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnumDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnumDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<EnumDeclarationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ENUM
/// Returns `None` if there is no child corresponding to token ENUM
fn ENUM(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(ENUM, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token IMPLEMENTS
/// Returns `None` if there is no child corresponding to token IMPLEMENTS
fn IMPLEMENTS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(IMPLEMENTS, 0)
}
fn typeList(&self) -> Option<Rc<TypeListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumConstants(&self) -> Option<Rc<EnumConstantsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}
fn enumBodyDeclarations(&self) -> Option<Rc<EnumBodyDeclarationsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EnumDeclarationContextAttrs<'input> for EnumDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enumDeclaration(&mut self,)
	-> Result<Rc<EnumDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_enumDeclaration);
        let mut _localctx: Rc<EnumDeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(380);
			recog.base.match_token(ENUM,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(381);
			recog.identifier()?;

			recog.base.set_state(384);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IMPLEMENTS {
				{
				recog.base.set_state(382);
				recog.base.match_token(IMPLEMENTS,&mut recog.err_handler)?;

				/*InvokeRule typeList*/
				recog.base.set_state(383);
				recog.typeList()?;

				}
			}

			recog.base.set_state(386);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(388);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 51)) & !0x3f) == 0 && ((1usize << (_la - 51)) & ((1usize << (MODULE - 51)) | (1usize << (OPEN - 51)) | (1usize << (REQUIRES - 51)) | (1usize << (EXPORTS - 51)) | (1usize << (OPENS - 51)) | (1usize << (TO - 51)) | (1usize << (USES - 51)) | (1usize << (PROVIDES - 51)) | (1usize << (WITH - 51)) | (1usize << (TRANSITIVE - 51)) | (1usize << (VAR - 51)) | (1usize << (YIELD - 51)) | (1usize << (RECORD - 51)) | (1usize << (SEALED - 51)) | (1usize << (PERMITS - 51)))) != 0) || _la==AT || _la==IDENTIFIER {
				{
				/*InvokeRule enumConstants*/
				recog.base.set_state(387);
				recog.enumConstants()?;

				}
			}

			recog.base.set_state(391);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				recog.base.set_state(390);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(394);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==SEMI {
				{
				/*InvokeRule enumBodyDeclarations*/
				recog.base.set_state(393);
				recog.enumBodyDeclarations()?;

				}
			}

			recog.base.set_state(396);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- enumConstants ----------------
pub type EnumConstantsContextAll<'input> = EnumConstantsContext<'input>;


pub type EnumConstantsContext<'input> = BaseParserRuleContext<'input,EnumConstantsContextExt<'input>>;

#[derive(Clone)]
pub struct EnumConstantsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for EnumConstantsContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for EnumConstantsContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_enumConstants(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_enumConstants(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for EnumConstantsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumConstants }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumConstants }
}
antlr_rust::tid!{EnumConstantsContextExt<'a>}

impl<'input> EnumConstantsContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnumConstantsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumConstantsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnumConstantsContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<EnumConstantsContextExt<'input>>{

fn enumConstant_all(&self) ->  Vec<Rc<EnumConstantContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumConstant(&self, i: usize) -> Option<Rc<EnumConstantContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> EnumConstantsContextAttrs<'input> for EnumConstantsContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enumConstants(&mut self,)
	-> Result<Rc<EnumConstantsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumConstantsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_enumConstants);
        let mut _localctx: Rc<EnumConstantsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule enumConstant*/
			recog.base.set_state(398);
			recog.enumConstant()?;

			recog.base.set_state(403);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(26,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(399);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule enumConstant*/
					recog.base.set_state(400);
					recog.enumConstant()?;

					}
					} 
				}
				recog.base.set_state(405);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(26,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- enumConstant ----------------
pub type EnumConstantContextAll<'input> = EnumConstantContext<'input>;


pub type EnumConstantContext<'input> = BaseParserRuleContext<'input,EnumConstantContextExt<'input>>;

#[derive(Clone)]
pub struct EnumConstantContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for EnumConstantContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for EnumConstantContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_enumConstant(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_enumConstant(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for EnumConstantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumConstant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumConstant }
}
antlr_rust::tid!{EnumConstantContextExt<'a>}

impl<'input> EnumConstantContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnumConstantContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumConstantContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnumConstantContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<EnumConstantContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn classBody(&self) -> Option<Rc<ClassBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EnumConstantContextAttrs<'input> for EnumConstantContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enumConstant(&mut self,)
	-> Result<Rc<EnumConstantContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumConstantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_enumConstant);
        let mut _localctx: Rc<EnumConstantContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(409);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(27,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule annotation*/
					recog.base.set_state(406);
					recog.annotation()?;

					}
					} 
				}
				recog.base.set_state(411);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(27,&mut recog.base)?;
			}
			/*InvokeRule identifier*/
			recog.base.set_state(412);
			recog.identifier()?;

			recog.base.set_state(414);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LPAREN {
				{
				/*InvokeRule arguments*/
				recog.base.set_state(413);
				recog.arguments()?;

				}
			}

			recog.base.set_state(417);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LBRACE {
				{
				/*InvokeRule classBody*/
				recog.base.set_state(416);
				recog.classBody()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- enumBodyDeclarations ----------------
pub type EnumBodyDeclarationsContextAll<'input> = EnumBodyDeclarationsContext<'input>;


pub type EnumBodyDeclarationsContext<'input> = BaseParserRuleContext<'input,EnumBodyDeclarationsContextExt<'input>>;

#[derive(Clone)]
pub struct EnumBodyDeclarationsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for EnumBodyDeclarationsContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for EnumBodyDeclarationsContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_enumBodyDeclarations(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_enumBodyDeclarations(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for EnumBodyDeclarationsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumBodyDeclarations }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumBodyDeclarations }
}
antlr_rust::tid!{EnumBodyDeclarationsContextExt<'a>}

impl<'input> EnumBodyDeclarationsContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnumBodyDeclarationsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumBodyDeclarationsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnumBodyDeclarationsContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<EnumBodyDeclarationsContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
fn classBodyDeclaration_all(&self) ->  Vec<Rc<ClassBodyDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn classBodyDeclaration(&self, i: usize) -> Option<Rc<ClassBodyDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> EnumBodyDeclarationsContextAttrs<'input> for EnumBodyDeclarationsContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enumBodyDeclarations(&mut self,)
	-> Result<Rc<EnumBodyDeclarationsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumBodyDeclarationsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_enumBodyDeclarations);
        let mut _localctx: Rc<EnumBodyDeclarationsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(419);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			recog.base.set_state(423);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ABSTRACT) | (1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << CLASS) | (1usize << DOUBLE) | (1usize << ENUM) | (1usize << FINAL) | (1usize << FLOAT) | (1usize << INT) | (1usize << INTERFACE) | (1usize << LONG) | (1usize << NATIVE))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (PRIVATE - 33)) | (1usize << (PROTECTED - 33)) | (1usize << (PUBLIC - 33)) | (1usize << (SHORT - 33)) | (1usize << (STATIC - 33)) | (1usize << (STRICTFP - 33)) | (1usize << (SYNCHRONIZED - 33)) | (1usize << (TRANSIENT - 33)) | (1usize << (VOID - 33)) | (1usize << (VOLATILE - 33)) | (1usize << (MODULE - 33)) | (1usize << (OPEN - 33)) | (1usize << (REQUIRES - 33)) | (1usize << (EXPORTS - 33)) | (1usize << (OPENS - 33)) | (1usize << (TO - 33)) | (1usize << (USES - 33)) | (1usize << (PROVIDES - 33)) | (1usize << (WITH - 33)) | (1usize << (TRANSITIVE - 33)) | (1usize << (VAR - 33)) | (1usize << (YIELD - 33)) | (1usize << (RECORD - 33)) | (1usize << (SEALED - 33)))) != 0) || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (PERMITS - 65)) | (1usize << (NON_SEALED - 65)) | (1usize << (LBRACE - 65)) | (1usize << (SEMI - 65)) | (1usize << (LT - 65)))) != 0) || _la==AT || _la==IDENTIFIER {
				{
				{
				/*InvokeRule classBodyDeclaration*/
				recog.base.set_state(420);
				recog.classBodyDeclaration()?;

				}
				}
				recog.base.set_state(425);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- interfaceDeclaration ----------------
pub type InterfaceDeclarationContextAll<'input> = InterfaceDeclarationContext<'input>;


pub type InterfaceDeclarationContext<'input> = BaseParserRuleContext<'input,InterfaceDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct InterfaceDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for InterfaceDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for InterfaceDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_interfaceDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_interfaceDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for InterfaceDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_interfaceDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_interfaceDeclaration }
}
antlr_rust::tid!{InterfaceDeclarationContextExt<'a>}

impl<'input> InterfaceDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InterfaceDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InterfaceDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InterfaceDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<InterfaceDeclarationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INTERFACE
/// Returns `None` if there is no child corresponding to token INTERFACE
fn INTERFACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(INTERFACE, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn interfaceBody(&self) -> Option<Rc<InterfaceBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeParameters(&self) -> Option<Rc<TypeParametersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EXTENDS
/// Returns `None` if there is no child corresponding to token EXTENDS
fn EXTENDS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(EXTENDS, 0)
}
fn typeList_all(&self) ->  Vec<Rc<TypeListContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeList(&self, i: usize) -> Option<Rc<TypeListContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token PERMITS
/// Returns `None` if there is no child corresponding to token PERMITS
fn PERMITS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(PERMITS, 0)
}

}

impl<'input> InterfaceDeclarationContextAttrs<'input> for InterfaceDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn interfaceDeclaration(&mut self,)
	-> Result<Rc<InterfaceDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InterfaceDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_interfaceDeclaration);
        let mut _localctx: Rc<InterfaceDeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(426);
			recog.base.match_token(INTERFACE,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(427);
			recog.identifier()?;

			recog.base.set_state(429);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LT {
				{
				/*InvokeRule typeParameters*/
				recog.base.set_state(428);
				recog.typeParameters()?;

				}
			}

			recog.base.set_state(433);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==EXTENDS {
				{
				recog.base.set_state(431);
				recog.base.match_token(EXTENDS,&mut recog.err_handler)?;

				/*InvokeRule typeList*/
				recog.base.set_state(432);
				recog.typeList()?;

				}
			}

			recog.base.set_state(437);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==PERMITS {
				{
				recog.base.set_state(435);
				recog.base.match_token(PERMITS,&mut recog.err_handler)?;

				/*InvokeRule typeList*/
				recog.base.set_state(436);
				recog.typeList()?;

				}
			}

			/*InvokeRule interfaceBody*/
			recog.base.set_state(439);
			recog.interfaceBody()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- classBody ----------------
pub type ClassBodyContextAll<'input> = ClassBodyContext<'input>;


pub type ClassBodyContext<'input> = BaseParserRuleContext<'input,ClassBodyContextExt<'input>>;

#[derive(Clone)]
pub struct ClassBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ClassBodyContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ClassBodyContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_classBody(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_classBody(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ClassBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_classBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_classBody }
}
antlr_rust::tid!{ClassBodyContextExt<'a>}

impl<'input> ClassBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ClassBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ClassBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ClassBodyContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ClassBodyContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn classBodyDeclaration_all(&self) ->  Vec<Rc<ClassBodyDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn classBodyDeclaration(&self, i: usize) -> Option<Rc<ClassBodyDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ClassBodyContextAttrs<'input> for ClassBodyContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn classBody(&mut self,)
	-> Result<Rc<ClassBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ClassBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_classBody);
        let mut _localctx: Rc<ClassBodyContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(441);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(445);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ABSTRACT) | (1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << CLASS) | (1usize << DOUBLE) | (1usize << ENUM) | (1usize << FINAL) | (1usize << FLOAT) | (1usize << INT) | (1usize << INTERFACE) | (1usize << LONG) | (1usize << NATIVE))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (PRIVATE - 33)) | (1usize << (PROTECTED - 33)) | (1usize << (PUBLIC - 33)) | (1usize << (SHORT - 33)) | (1usize << (STATIC - 33)) | (1usize << (STRICTFP - 33)) | (1usize << (SYNCHRONIZED - 33)) | (1usize << (TRANSIENT - 33)) | (1usize << (VOID - 33)) | (1usize << (VOLATILE - 33)) | (1usize << (MODULE - 33)) | (1usize << (OPEN - 33)) | (1usize << (REQUIRES - 33)) | (1usize << (EXPORTS - 33)) | (1usize << (OPENS - 33)) | (1usize << (TO - 33)) | (1usize << (USES - 33)) | (1usize << (PROVIDES - 33)) | (1usize << (WITH - 33)) | (1usize << (TRANSITIVE - 33)) | (1usize << (VAR - 33)) | (1usize << (YIELD - 33)) | (1usize << (RECORD - 33)) | (1usize << (SEALED - 33)))) != 0) || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (PERMITS - 65)) | (1usize << (NON_SEALED - 65)) | (1usize << (LBRACE - 65)) | (1usize << (SEMI - 65)) | (1usize << (LT - 65)))) != 0) || _la==AT || _la==IDENTIFIER {
				{
				{
				/*InvokeRule classBodyDeclaration*/
				recog.base.set_state(442);
				recog.classBodyDeclaration()?;

				}
				}
				recog.base.set_state(447);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(448);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- interfaceBody ----------------
pub type InterfaceBodyContextAll<'input> = InterfaceBodyContext<'input>;


pub type InterfaceBodyContext<'input> = BaseParserRuleContext<'input,InterfaceBodyContextExt<'input>>;

#[derive(Clone)]
pub struct InterfaceBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for InterfaceBodyContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for InterfaceBodyContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_interfaceBody(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_interfaceBody(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for InterfaceBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_interfaceBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_interfaceBody }
}
antlr_rust::tid!{InterfaceBodyContextExt<'a>}

impl<'input> InterfaceBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InterfaceBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InterfaceBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InterfaceBodyContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<InterfaceBodyContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn interfaceBodyDeclaration_all(&self) ->  Vec<Rc<InterfaceBodyDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn interfaceBodyDeclaration(&self, i: usize) -> Option<Rc<InterfaceBodyDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> InterfaceBodyContextAttrs<'input> for InterfaceBodyContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn interfaceBody(&mut self,)
	-> Result<Rc<InterfaceBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InterfaceBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_interfaceBody);
        let mut _localctx: Rc<InterfaceBodyContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(450);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(454);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ABSTRACT) | (1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << CLASS) | (1usize << DEFAULT) | (1usize << DOUBLE) | (1usize << ENUM) | (1usize << FINAL) | (1usize << FLOAT) | (1usize << INT) | (1usize << INTERFACE) | (1usize << LONG) | (1usize << NATIVE))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (PRIVATE - 33)) | (1usize << (PROTECTED - 33)) | (1usize << (PUBLIC - 33)) | (1usize << (SHORT - 33)) | (1usize << (STATIC - 33)) | (1usize << (STRICTFP - 33)) | (1usize << (SYNCHRONIZED - 33)) | (1usize << (TRANSIENT - 33)) | (1usize << (VOID - 33)) | (1usize << (VOLATILE - 33)) | (1usize << (MODULE - 33)) | (1usize << (OPEN - 33)) | (1usize << (REQUIRES - 33)) | (1usize << (EXPORTS - 33)) | (1usize << (OPENS - 33)) | (1usize << (TO - 33)) | (1usize << (USES - 33)) | (1usize << (PROVIDES - 33)) | (1usize << (WITH - 33)) | (1usize << (TRANSITIVE - 33)) | (1usize << (VAR - 33)) | (1usize << (YIELD - 33)) | (1usize << (RECORD - 33)) | (1usize << (SEALED - 33)))) != 0) || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (PERMITS - 65)) | (1usize << (NON_SEALED - 65)) | (1usize << (SEMI - 65)) | (1usize << (LT - 65)))) != 0) || _la==AT || _la==IDENTIFIER {
				{
				{
				/*InvokeRule interfaceBodyDeclaration*/
				recog.base.set_state(451);
				recog.interfaceBodyDeclaration()?;

				}
				}
				recog.base.set_state(456);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(457);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- classBodyDeclaration ----------------
pub type ClassBodyDeclarationContextAll<'input> = ClassBodyDeclarationContext<'input>;


pub type ClassBodyDeclarationContext<'input> = BaseParserRuleContext<'input,ClassBodyDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ClassBodyDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ClassBodyDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ClassBodyDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_classBodyDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_classBodyDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ClassBodyDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_classBodyDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_classBodyDeclaration }
}
antlr_rust::tid!{ClassBodyDeclarationContextExt<'a>}

impl<'input> ClassBodyDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ClassBodyDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ClassBodyDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ClassBodyDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ClassBodyDeclarationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token STATIC
/// Returns `None` if there is no child corresponding to token STATIC
fn STATIC(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(STATIC, 0)
}
fn memberDeclaration(&self) -> Option<Rc<MemberDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn modifier_all(&self) ->  Vec<Rc<ModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn modifier(&self, i: usize) -> Option<Rc<ModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ClassBodyDeclarationContextAttrs<'input> for ClassBodyDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn classBodyDeclaration(&mut self,)
	-> Result<Rc<ClassBodyDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ClassBodyDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_classBodyDeclaration);
        let mut _localctx: Rc<ClassBodyDeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(477);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(39,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(459);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(461);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==STATIC {
						{
						recog.base.set_state(460);
						recog.base.match_token(STATIC,&mut recog.err_handler)?;

						}
					}

					/*InvokeRule block*/
					recog.base.set_state(463);
					recog.block()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(467);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(37,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule annotation*/
							recog.base.set_state(464);
							recog.annotation()?;

							}
							} 
						}
						recog.base.set_state(469);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(37,&mut recog.base)?;
					}
					recog.base.set_state(473);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(38,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule modifier*/
							recog.base.set_state(470);
							recog.modifier()?;

							}
							} 
						}
						recog.base.set_state(475);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(38,&mut recog.base)?;
					}
					/*InvokeRule memberDeclaration*/
					recog.base.set_state(476);
					recog.memberDeclaration()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- memberDeclaration ----------------
pub type MemberDeclarationContextAll<'input> = MemberDeclarationContext<'input>;


pub type MemberDeclarationContext<'input> = BaseParserRuleContext<'input,MemberDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct MemberDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for MemberDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for MemberDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_memberDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_memberDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for MemberDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_memberDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_memberDeclaration }
}
antlr_rust::tid!{MemberDeclarationContextExt<'a>}

impl<'input> MemberDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MemberDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MemberDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MemberDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<MemberDeclarationContextExt<'input>>{

fn recordDeclaration(&self) -> Option<Rc<RecordDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn methodDeclaration(&self) -> Option<Rc<MethodDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn genericMethodDeclaration(&self) -> Option<Rc<GenericMethodDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fieldDeclaration(&self) -> Option<Rc<FieldDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn constructorDeclaration(&self) -> Option<Rc<ConstructorDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn genericConstructorDeclaration(&self) -> Option<Rc<GenericConstructorDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn interfaceDeclaration(&self) -> Option<Rc<InterfaceDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotationTypeDeclaration(&self) -> Option<Rc<AnnotationTypeDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn classDeclaration(&self) -> Option<Rc<ClassDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumDeclaration(&self) -> Option<Rc<EnumDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MemberDeclarationContextAttrs<'input> for MemberDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn memberDeclaration(&mut self,)
	-> Result<Rc<MemberDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MemberDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_memberDeclaration);
        let mut _localctx: Rc<MemberDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(489);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(40,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule recordDeclaration*/
					recog.base.set_state(479);
					recog.recordDeclaration()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule methodDeclaration*/
					recog.base.set_state(480);
					recog.methodDeclaration()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule genericMethodDeclaration*/
					recog.base.set_state(481);
					recog.genericMethodDeclaration()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule fieldDeclaration*/
					recog.base.set_state(482);
					recog.fieldDeclaration()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule constructorDeclaration*/
					recog.base.set_state(483);
					recog.constructorDeclaration()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule genericConstructorDeclaration*/
					recog.base.set_state(484);
					recog.genericConstructorDeclaration()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule interfaceDeclaration*/
					recog.base.set_state(485);
					recog.interfaceDeclaration()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule annotationTypeDeclaration*/
					recog.base.set_state(486);
					recog.annotationTypeDeclaration()?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule classDeclaration*/
					recog.base.set_state(487);
					recog.classDeclaration()?;

					}
				}
			,
				10 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					/*InvokeRule enumDeclaration*/
					recog.base.set_state(488);
					recog.enumDeclaration()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- methodDeclaration ----------------
pub type MethodDeclarationContextAll<'input> = MethodDeclarationContext<'input>;


pub type MethodDeclarationContext<'input> = BaseParserRuleContext<'input,MethodDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct MethodDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for MethodDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for MethodDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_methodDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_methodDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for MethodDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_methodDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_methodDeclaration }
}
antlr_rust::tid!{MethodDeclarationContextExt<'a>}

impl<'input> MethodDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MethodDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MethodDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MethodDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<MethodDeclarationContextExt<'input>>{

fn typeTypeOrVoid(&self) -> Option<Rc<TypeTypeOrVoidContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn formalParameters(&self) -> Option<Rc<FormalParametersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn methodBody(&self) -> Option<Rc<MethodBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token LBRACK in current rule
fn LBRACK_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LBRACK, starting from 0.
/// Returns `None` if number of children corresponding to token LBRACK is less or equal than `i`.
fn LBRACK(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACK, i)
}
/// Retrieves all `TerminalNode`s corresponding to token RBRACK in current rule
fn RBRACK_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RBRACK, starting from 0.
/// Returns `None` if number of children corresponding to token RBRACK is less or equal than `i`.
fn RBRACK(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACK, i)
}
/// Retrieves first TerminalNode corresponding to token THROWS
/// Returns `None` if there is no child corresponding to token THROWS
fn THROWS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(THROWS, 0)
}
fn qualifiedNameList(&self) -> Option<Rc<QualifiedNameListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MethodDeclarationContextAttrs<'input> for MethodDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn methodDeclaration(&mut self,)
	-> Result<Rc<MethodDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MethodDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_methodDeclaration);
        let mut _localctx: Rc<MethodDeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typeTypeOrVoid*/
			recog.base.set_state(491);
			recog.typeTypeOrVoid()?;

			/*InvokeRule identifier*/
			recog.base.set_state(492);
			recog.identifier()?;

			/*InvokeRule formalParameters*/
			recog.base.set_state(493);
			recog.formalParameters()?;

			recog.base.set_state(498);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==LBRACK {
				{
				{
				recog.base.set_state(494);
				recog.base.match_token(LBRACK,&mut recog.err_handler)?;

				recog.base.set_state(495);
				recog.base.match_token(RBRACK,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(500);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(503);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==THROWS {
				{
				recog.base.set_state(501);
				recog.base.match_token(THROWS,&mut recog.err_handler)?;

				/*InvokeRule qualifiedNameList*/
				recog.base.set_state(502);
				recog.qualifiedNameList()?;

				}
			}

			/*InvokeRule methodBody*/
			recog.base.set_state(505);
			recog.methodBody()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- methodBody ----------------
pub type MethodBodyContextAll<'input> = MethodBodyContext<'input>;


pub type MethodBodyContext<'input> = BaseParserRuleContext<'input,MethodBodyContextExt<'input>>;

#[derive(Clone)]
pub struct MethodBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for MethodBodyContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for MethodBodyContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_methodBody(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_methodBody(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for MethodBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_methodBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_methodBody }
}
antlr_rust::tid!{MethodBodyContextExt<'a>}

impl<'input> MethodBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MethodBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MethodBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MethodBodyContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<MethodBodyContextExt<'input>>{

fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}

}

impl<'input> MethodBodyContextAttrs<'input> for MethodBodyContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn methodBody(&mut self,)
	-> Result<Rc<MethodBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MethodBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_methodBody);
        let mut _localctx: Rc<MethodBodyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(509);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LBRACE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule block*/
					recog.base.set_state(507);
					recog.block()?;

					}
				}

			 SEMI 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(508);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeTypeOrVoid ----------------
pub type TypeTypeOrVoidContextAll<'input> = TypeTypeOrVoidContext<'input>;


pub type TypeTypeOrVoidContext<'input> = BaseParserRuleContext<'input,TypeTypeOrVoidContextExt<'input>>;

#[derive(Clone)]
pub struct TypeTypeOrVoidContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for TypeTypeOrVoidContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for TypeTypeOrVoidContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_typeTypeOrVoid(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_typeTypeOrVoid(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TypeTypeOrVoidContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeTypeOrVoid }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeTypeOrVoid }
}
antlr_rust::tid!{TypeTypeOrVoidContextExt<'a>}

impl<'input> TypeTypeOrVoidContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeTypeOrVoidContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeTypeOrVoidContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeTypeOrVoidContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<TypeTypeOrVoidContextExt<'input>>{

fn typeType(&self) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token VOID
/// Returns `None` if there is no child corresponding to token VOID
fn VOID(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(VOID, 0)
}

}

impl<'input> TypeTypeOrVoidContextAttrs<'input> for TypeTypeOrVoidContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeTypeOrVoid(&mut self,)
	-> Result<Rc<TypeTypeOrVoidContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeTypeOrVoidContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_typeTypeOrVoid);
        let mut _localctx: Rc<TypeTypeOrVoidContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(513);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BOOLEAN | BYTE | CHAR | DOUBLE | FLOAT | INT | LONG | SHORT | MODULE |
			 OPEN | REQUIRES | EXPORTS | OPENS | TO | USES | PROVIDES | WITH | TRANSITIVE |
			 VAR | YIELD | RECORD | SEALED | PERMITS | AT | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule typeType*/
					recog.base.set_state(511);
					recog.typeType()?;

					}
				}

			 VOID 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(512);
					recog.base.match_token(VOID,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- genericMethodDeclaration ----------------
pub type GenericMethodDeclarationContextAll<'input> = GenericMethodDeclarationContext<'input>;


pub type GenericMethodDeclarationContext<'input> = BaseParserRuleContext<'input,GenericMethodDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct GenericMethodDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for GenericMethodDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for GenericMethodDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_genericMethodDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_genericMethodDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for GenericMethodDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_genericMethodDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_genericMethodDeclaration }
}
antlr_rust::tid!{GenericMethodDeclarationContextExt<'a>}

impl<'input> GenericMethodDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GenericMethodDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GenericMethodDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GenericMethodDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<GenericMethodDeclarationContextExt<'input>>{

fn typeParameters(&self) -> Option<Rc<TypeParametersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn methodDeclaration(&self) -> Option<Rc<MethodDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> GenericMethodDeclarationContextAttrs<'input> for GenericMethodDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn genericMethodDeclaration(&mut self,)
	-> Result<Rc<GenericMethodDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GenericMethodDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_genericMethodDeclaration);
        let mut _localctx: Rc<GenericMethodDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typeParameters*/
			recog.base.set_state(515);
			recog.typeParameters()?;

			/*InvokeRule methodDeclaration*/
			recog.base.set_state(516);
			recog.methodDeclaration()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- genericConstructorDeclaration ----------------
pub type GenericConstructorDeclarationContextAll<'input> = GenericConstructorDeclarationContext<'input>;


pub type GenericConstructorDeclarationContext<'input> = BaseParserRuleContext<'input,GenericConstructorDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct GenericConstructorDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for GenericConstructorDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for GenericConstructorDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_genericConstructorDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_genericConstructorDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for GenericConstructorDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_genericConstructorDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_genericConstructorDeclaration }
}
antlr_rust::tid!{GenericConstructorDeclarationContextExt<'a>}

impl<'input> GenericConstructorDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GenericConstructorDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GenericConstructorDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GenericConstructorDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<GenericConstructorDeclarationContextExt<'input>>{

fn typeParameters(&self) -> Option<Rc<TypeParametersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn constructorDeclaration(&self) -> Option<Rc<ConstructorDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> GenericConstructorDeclarationContextAttrs<'input> for GenericConstructorDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn genericConstructorDeclaration(&mut self,)
	-> Result<Rc<GenericConstructorDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GenericConstructorDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_genericConstructorDeclaration);
        let mut _localctx: Rc<GenericConstructorDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typeParameters*/
			recog.base.set_state(518);
			recog.typeParameters()?;

			/*InvokeRule constructorDeclaration*/
			recog.base.set_state(519);
			recog.constructorDeclaration()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- constructorDeclaration ----------------
pub type ConstructorDeclarationContextAll<'input> = ConstructorDeclarationContext<'input>;


pub type ConstructorDeclarationContext<'input> = BaseParserRuleContext<'input,ConstructorDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ConstructorDeclarationContextExt<'input>{
	pub constructorBody: Option<Rc<BlockContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ConstructorDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ConstructorDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_constructorDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_constructorDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ConstructorDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constructorDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constructorDeclaration }
}
antlr_rust::tid!{ConstructorDeclarationContextExt<'a>}

impl<'input> ConstructorDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConstructorDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConstructorDeclarationContextExt{
				constructorBody: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ConstructorDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ConstructorDeclarationContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn formalParameters(&self) -> Option<Rc<FormalParametersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token THROWS
/// Returns `None` if there is no child corresponding to token THROWS
fn THROWS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(THROWS, 0)
}
fn qualifiedNameList(&self) -> Option<Rc<QualifiedNameListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ConstructorDeclarationContextAttrs<'input> for ConstructorDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn constructorDeclaration(&mut self,)
	-> Result<Rc<ConstructorDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConstructorDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_constructorDeclaration);
        let mut _localctx: Rc<ConstructorDeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(521);
			recog.identifier()?;

			/*InvokeRule formalParameters*/
			recog.base.set_state(522);
			recog.formalParameters()?;

			recog.base.set_state(525);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==THROWS {
				{
				recog.base.set_state(523);
				recog.base.match_token(THROWS,&mut recog.err_handler)?;

				/*InvokeRule qualifiedNameList*/
				recog.base.set_state(524);
				recog.qualifiedNameList()?;

				}
			}

			/*InvokeRule block*/
			recog.base.set_state(527);
			let tmp = recog.block()?;
			 cast_mut::<_,ConstructorDeclarationContext >(&mut _localctx).constructorBody = Some(tmp.clone());
			  

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- compactConstructorDeclaration ----------------
pub type CompactConstructorDeclarationContextAll<'input> = CompactConstructorDeclarationContext<'input>;


pub type CompactConstructorDeclarationContext<'input> = BaseParserRuleContext<'input,CompactConstructorDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct CompactConstructorDeclarationContextExt<'input>{
	pub constructorBody: Option<Rc<BlockContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for CompactConstructorDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for CompactConstructorDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_compactConstructorDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_compactConstructorDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for CompactConstructorDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compactConstructorDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compactConstructorDeclaration }
}
antlr_rust::tid!{CompactConstructorDeclarationContextExt<'a>}

impl<'input> CompactConstructorDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompactConstructorDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompactConstructorDeclarationContextExt{
				constructorBody: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait CompactConstructorDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<CompactConstructorDeclarationContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn modifier_all(&self) ->  Vec<Rc<ModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn modifier(&self, i: usize) -> Option<Rc<ModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> CompactConstructorDeclarationContextAttrs<'input> for CompactConstructorDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn compactConstructorDeclaration(&mut self,)
	-> Result<Rc<CompactConstructorDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompactConstructorDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_compactConstructorDeclaration);
        let mut _localctx: Rc<CompactConstructorDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(532);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(46,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule annotation*/
					recog.base.set_state(529);
					recog.annotation()?;

					}
					} 
				}
				recog.base.set_state(534);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(46,&mut recog.base)?;
			}
			recog.base.set_state(538);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(47,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule modifier*/
					recog.base.set_state(535);
					recog.modifier()?;

					}
					} 
				}
				recog.base.set_state(540);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(47,&mut recog.base)?;
			}
			/*InvokeRule identifier*/
			recog.base.set_state(541);
			recog.identifier()?;

			/*InvokeRule block*/
			recog.base.set_state(542);
			let tmp = recog.block()?;
			 cast_mut::<_,CompactConstructorDeclarationContext >(&mut _localctx).constructorBody = Some(tmp.clone());
			  

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fieldDeclaration ----------------
pub type FieldDeclarationContextAll<'input> = FieldDeclarationContext<'input>;


pub type FieldDeclarationContext<'input> = BaseParserRuleContext<'input,FieldDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct FieldDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for FieldDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for FieldDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_fieldDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_fieldDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for FieldDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fieldDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fieldDeclaration }
}
antlr_rust::tid!{FieldDeclarationContextExt<'a>}

impl<'input> FieldDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FieldDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FieldDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FieldDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<FieldDeclarationContextExt<'input>>{

fn typeType(&self) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableDeclarators(&self) -> Option<Rc<VariableDeclaratorsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}

}

impl<'input> FieldDeclarationContextAttrs<'input> for FieldDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fieldDeclaration(&mut self,)
	-> Result<Rc<FieldDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FieldDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_fieldDeclaration);
        let mut _localctx: Rc<FieldDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typeType*/
			recog.base.set_state(544);
			recog.typeType()?;

			/*InvokeRule variableDeclarators*/
			recog.base.set_state(545);
			recog.variableDeclarators()?;

			recog.base.set_state(546);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- interfaceBodyDeclaration ----------------
pub type InterfaceBodyDeclarationContextAll<'input> = InterfaceBodyDeclarationContext<'input>;


pub type InterfaceBodyDeclarationContext<'input> = BaseParserRuleContext<'input,InterfaceBodyDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct InterfaceBodyDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for InterfaceBodyDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for InterfaceBodyDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_interfaceBodyDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_interfaceBodyDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for InterfaceBodyDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_interfaceBodyDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_interfaceBodyDeclaration }
}
antlr_rust::tid!{InterfaceBodyDeclarationContextExt<'a>}

impl<'input> InterfaceBodyDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InterfaceBodyDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InterfaceBodyDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InterfaceBodyDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<InterfaceBodyDeclarationContextExt<'input>>{

fn interfaceMemberDeclaration(&self) -> Option<Rc<InterfaceMemberDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn modifier_all(&self) ->  Vec<Rc<ModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn modifier(&self, i: usize) -> Option<Rc<ModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}

}

impl<'input> InterfaceBodyDeclarationContextAttrs<'input> for InterfaceBodyDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn interfaceBodyDeclaration(&mut self,)
	-> Result<Rc<InterfaceBodyDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InterfaceBodyDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_interfaceBodyDeclaration);
        let mut _localctx: Rc<InterfaceBodyDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(562);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ABSTRACT | BOOLEAN | BYTE | CHAR | CLASS | DEFAULT | DOUBLE | ENUM |
			 FINAL | FLOAT | INT | INTERFACE | LONG | NATIVE | PRIVATE | PROTECTED |
			 PUBLIC | SHORT | STATIC | STRICTFP | SYNCHRONIZED | TRANSIENT | VOID |
			 VOLATILE | MODULE | OPEN | REQUIRES | EXPORTS | OPENS | TO | USES | PROVIDES |
			 WITH | TRANSITIVE | VAR | YIELD | RECORD | SEALED | PERMITS | NON_SEALED |
			 LT | AT | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(551);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(48,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule annotation*/
							recog.base.set_state(548);
							recog.annotation()?;

							}
							} 
						}
						recog.base.set_state(553);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(48,&mut recog.base)?;
					}
					recog.base.set_state(557);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(49,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule modifier*/
							recog.base.set_state(554);
							recog.modifier()?;

							}
							} 
						}
						recog.base.set_state(559);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(49,&mut recog.base)?;
					}
					/*InvokeRule interfaceMemberDeclaration*/
					recog.base.set_state(560);
					recog.interfaceMemberDeclaration()?;

					}
				}

			 SEMI 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(561);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- interfaceMemberDeclaration ----------------
pub type InterfaceMemberDeclarationContextAll<'input> = InterfaceMemberDeclarationContext<'input>;


pub type InterfaceMemberDeclarationContext<'input> = BaseParserRuleContext<'input,InterfaceMemberDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct InterfaceMemberDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for InterfaceMemberDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for InterfaceMemberDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_interfaceMemberDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_interfaceMemberDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for InterfaceMemberDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_interfaceMemberDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_interfaceMemberDeclaration }
}
antlr_rust::tid!{InterfaceMemberDeclarationContextExt<'a>}

impl<'input> InterfaceMemberDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InterfaceMemberDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InterfaceMemberDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InterfaceMemberDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<InterfaceMemberDeclarationContextExt<'input>>{

fn constDeclaration(&self) -> Option<Rc<ConstDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn interfaceMethodDeclaration(&self) -> Option<Rc<InterfaceMethodDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn genericInterfaceMethodDeclaration(&self) -> Option<Rc<GenericInterfaceMethodDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn interfaceDeclaration(&self) -> Option<Rc<InterfaceDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotationTypeDeclaration(&self) -> Option<Rc<AnnotationTypeDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn classDeclaration(&self) -> Option<Rc<ClassDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumDeclaration(&self) -> Option<Rc<EnumDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn recordDeclaration(&self) -> Option<Rc<RecordDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InterfaceMemberDeclarationContextAttrs<'input> for InterfaceMemberDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn interfaceMemberDeclaration(&mut self,)
	-> Result<Rc<InterfaceMemberDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InterfaceMemberDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_interfaceMemberDeclaration);
        let mut _localctx: Rc<InterfaceMemberDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(572);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(51,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule constDeclaration*/
					recog.base.set_state(564);
					recog.constDeclaration()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule interfaceMethodDeclaration*/
					recog.base.set_state(565);
					recog.interfaceMethodDeclaration()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule genericInterfaceMethodDeclaration*/
					recog.base.set_state(566);
					recog.genericInterfaceMethodDeclaration()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule interfaceDeclaration*/
					recog.base.set_state(567);
					recog.interfaceDeclaration()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule annotationTypeDeclaration*/
					recog.base.set_state(568);
					recog.annotationTypeDeclaration()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule classDeclaration*/
					recog.base.set_state(569);
					recog.classDeclaration()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule enumDeclaration*/
					recog.base.set_state(570);
					recog.enumDeclaration()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule recordDeclaration*/
					recog.base.set_state(571);
					recog.recordDeclaration()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- constDeclaration ----------------
pub type ConstDeclarationContextAll<'input> = ConstDeclarationContext<'input>;


pub type ConstDeclarationContext<'input> = BaseParserRuleContext<'input,ConstDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ConstDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ConstDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ConstDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_constDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_constDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ConstDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constDeclaration }
}
antlr_rust::tid!{ConstDeclarationContextExt<'a>}

impl<'input> ConstDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConstDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConstDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConstDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ConstDeclarationContextExt<'input>>{

fn typeType(&self) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn constantDeclarator_all(&self) ->  Vec<Rc<ConstantDeclaratorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn constantDeclarator(&self, i: usize) -> Option<Rc<ConstantDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ConstDeclarationContextAttrs<'input> for ConstDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn constDeclaration(&mut self,)
	-> Result<Rc<ConstDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConstDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_constDeclaration);
        let mut _localctx: Rc<ConstDeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typeType*/
			recog.base.set_state(574);
			recog.typeType()?;

			/*InvokeRule constantDeclarator*/
			recog.base.set_state(575);
			recog.constantDeclarator()?;

			recog.base.set_state(580);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(576);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule constantDeclarator*/
				recog.base.set_state(577);
				recog.constantDeclarator()?;

				}
				}
				recog.base.set_state(582);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(583);
			recog.base.match_token(SEMI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- constantDeclarator ----------------
pub type ConstantDeclaratorContextAll<'input> = ConstantDeclaratorContext<'input>;


pub type ConstantDeclaratorContext<'input> = BaseParserRuleContext<'input,ConstantDeclaratorContextExt<'input>>;

#[derive(Clone)]
pub struct ConstantDeclaratorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ConstantDeclaratorContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ConstantDeclaratorContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_constantDeclarator(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_constantDeclarator(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ConstantDeclaratorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constantDeclarator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constantDeclarator }
}
antlr_rust::tid!{ConstantDeclaratorContextExt<'a>}

impl<'input> ConstantDeclaratorContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConstantDeclaratorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConstantDeclaratorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConstantDeclaratorContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ConstantDeclaratorContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN
/// Returns `None` if there is no child corresponding to token ASSIGN
fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN, 0)
}
fn variableInitializer(&self) -> Option<Rc<VariableInitializerContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token LBRACK in current rule
fn LBRACK_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LBRACK, starting from 0.
/// Returns `None` if number of children corresponding to token LBRACK is less or equal than `i`.
fn LBRACK(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACK, i)
}
/// Retrieves all `TerminalNode`s corresponding to token RBRACK in current rule
fn RBRACK_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RBRACK, starting from 0.
/// Returns `None` if number of children corresponding to token RBRACK is less or equal than `i`.
fn RBRACK(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACK, i)
}

}

impl<'input> ConstantDeclaratorContextAttrs<'input> for ConstantDeclaratorContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn constantDeclarator(&mut self,)
	-> Result<Rc<ConstantDeclaratorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConstantDeclaratorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_constantDeclarator);
        let mut _localctx: Rc<ConstantDeclaratorContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(585);
			recog.identifier()?;

			recog.base.set_state(590);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==LBRACK {
				{
				{
				recog.base.set_state(586);
				recog.base.match_token(LBRACK,&mut recog.err_handler)?;

				recog.base.set_state(587);
				recog.base.match_token(RBRACK,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(592);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(593);
			recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

			/*InvokeRule variableInitializer*/
			recog.base.set_state(594);
			recog.variableInitializer()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- interfaceMethodDeclaration ----------------
pub type InterfaceMethodDeclarationContextAll<'input> = InterfaceMethodDeclarationContext<'input>;


pub type InterfaceMethodDeclarationContext<'input> = BaseParserRuleContext<'input,InterfaceMethodDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct InterfaceMethodDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for InterfaceMethodDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for InterfaceMethodDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_interfaceMethodDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_interfaceMethodDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for InterfaceMethodDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_interfaceMethodDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_interfaceMethodDeclaration }
}
antlr_rust::tid!{InterfaceMethodDeclarationContextExt<'a>}

impl<'input> InterfaceMethodDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InterfaceMethodDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InterfaceMethodDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InterfaceMethodDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<InterfaceMethodDeclarationContextExt<'input>>{

fn interfaceCommonBodyDeclaration(&self) -> Option<Rc<InterfaceCommonBodyDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn interfaceMethodModifier_all(&self) ->  Vec<Rc<InterfaceMethodModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn interfaceMethodModifier(&self, i: usize) -> Option<Rc<InterfaceMethodModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> InterfaceMethodDeclarationContextAttrs<'input> for InterfaceMethodDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn interfaceMethodDeclaration(&mut self,)
	-> Result<Rc<InterfaceMethodDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InterfaceMethodDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_interfaceMethodDeclaration);
        let mut _localctx: Rc<InterfaceMethodDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(599);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(54,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule interfaceMethodModifier*/
					recog.base.set_state(596);
					recog.interfaceMethodModifier()?;

					}
					} 
				}
				recog.base.set_state(601);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(54,&mut recog.base)?;
			}
			/*InvokeRule interfaceCommonBodyDeclaration*/
			recog.base.set_state(602);
			recog.interfaceCommonBodyDeclaration()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- interfaceMethodModifier ----------------
pub type InterfaceMethodModifierContextAll<'input> = InterfaceMethodModifierContext<'input>;


pub type InterfaceMethodModifierContext<'input> = BaseParserRuleContext<'input,InterfaceMethodModifierContextExt<'input>>;

#[derive(Clone)]
pub struct InterfaceMethodModifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for InterfaceMethodModifierContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for InterfaceMethodModifierContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_interfaceMethodModifier(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_interfaceMethodModifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for InterfaceMethodModifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_interfaceMethodModifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_interfaceMethodModifier }
}
antlr_rust::tid!{InterfaceMethodModifierContextExt<'a>}

impl<'input> InterfaceMethodModifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InterfaceMethodModifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InterfaceMethodModifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InterfaceMethodModifierContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<InterfaceMethodModifierContextExt<'input>>{

fn annotation(&self) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PUBLIC
/// Returns `None` if there is no child corresponding to token PUBLIC
fn PUBLIC(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(PUBLIC, 0)
}
/// Retrieves first TerminalNode corresponding to token ABSTRACT
/// Returns `None` if there is no child corresponding to token ABSTRACT
fn ABSTRACT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(ABSTRACT, 0)
}
/// Retrieves first TerminalNode corresponding to token DEFAULT
/// Returns `None` if there is no child corresponding to token DEFAULT
fn DEFAULT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DEFAULT, 0)
}
/// Retrieves first TerminalNode corresponding to token STATIC
/// Returns `None` if there is no child corresponding to token STATIC
fn STATIC(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(STATIC, 0)
}
/// Retrieves first TerminalNode corresponding to token STRICTFP
/// Returns `None` if there is no child corresponding to token STRICTFP
fn STRICTFP(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(STRICTFP, 0)
}

}

impl<'input> InterfaceMethodModifierContextAttrs<'input> for InterfaceMethodModifierContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn interfaceMethodModifier(&mut self,)
	-> Result<Rc<InterfaceMethodModifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InterfaceMethodModifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_interfaceMethodModifier);
        let mut _localctx: Rc<InterfaceMethodModifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(610);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 MODULE | OPEN | REQUIRES | EXPORTS | OPENS | TO | USES | PROVIDES | WITH |
			 TRANSITIVE | VAR | YIELD | RECORD | SEALED | PERMITS | AT | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule annotation*/
					recog.base.set_state(604);
					recog.annotation()?;

					}
				}

			 PUBLIC 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(605);
					recog.base.match_token(PUBLIC,&mut recog.err_handler)?;

					}
				}

			 ABSTRACT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(606);
					recog.base.match_token(ABSTRACT,&mut recog.err_handler)?;

					}
				}

			 DEFAULT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(607);
					recog.base.match_token(DEFAULT,&mut recog.err_handler)?;

					}
				}

			 STATIC 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(608);
					recog.base.match_token(STATIC,&mut recog.err_handler)?;

					}
				}

			 STRICTFP 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(609);
					recog.base.match_token(STRICTFP,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- genericInterfaceMethodDeclaration ----------------
pub type GenericInterfaceMethodDeclarationContextAll<'input> = GenericInterfaceMethodDeclarationContext<'input>;


pub type GenericInterfaceMethodDeclarationContext<'input> = BaseParserRuleContext<'input,GenericInterfaceMethodDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct GenericInterfaceMethodDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for GenericInterfaceMethodDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for GenericInterfaceMethodDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_genericInterfaceMethodDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_genericInterfaceMethodDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for GenericInterfaceMethodDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_genericInterfaceMethodDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_genericInterfaceMethodDeclaration }
}
antlr_rust::tid!{GenericInterfaceMethodDeclarationContextExt<'a>}

impl<'input> GenericInterfaceMethodDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GenericInterfaceMethodDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GenericInterfaceMethodDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GenericInterfaceMethodDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<GenericInterfaceMethodDeclarationContextExt<'input>>{

fn typeParameters(&self) -> Option<Rc<TypeParametersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn interfaceCommonBodyDeclaration(&self) -> Option<Rc<InterfaceCommonBodyDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn interfaceMethodModifier_all(&self) ->  Vec<Rc<InterfaceMethodModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn interfaceMethodModifier(&self, i: usize) -> Option<Rc<InterfaceMethodModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> GenericInterfaceMethodDeclarationContextAttrs<'input> for GenericInterfaceMethodDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn genericInterfaceMethodDeclaration(&mut self,)
	-> Result<Rc<GenericInterfaceMethodDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GenericInterfaceMethodDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_genericInterfaceMethodDeclaration);
        let mut _localctx: Rc<GenericInterfaceMethodDeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(615);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ABSTRACT || _la==DEFAULT || ((((_la - 35)) & !0x3f) == 0 && ((1usize << (_la - 35)) & ((1usize << (PUBLIC - 35)) | (1usize << (STATIC - 35)) | (1usize << (STRICTFP - 35)) | (1usize << (MODULE - 35)) | (1usize << (OPEN - 35)) | (1usize << (REQUIRES - 35)) | (1usize << (EXPORTS - 35)) | (1usize << (OPENS - 35)) | (1usize << (TO - 35)) | (1usize << (USES - 35)) | (1usize << (PROVIDES - 35)) | (1usize << (WITH - 35)) | (1usize << (TRANSITIVE - 35)) | (1usize << (VAR - 35)) | (1usize << (YIELD - 35)) | (1usize << (RECORD - 35)) | (1usize << (SEALED - 35)) | (1usize << (PERMITS - 35)))) != 0) || _la==AT || _la==IDENTIFIER {
				{
				{
				/*InvokeRule interfaceMethodModifier*/
				recog.base.set_state(612);
				recog.interfaceMethodModifier()?;

				}
				}
				recog.base.set_state(617);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule typeParameters*/
			recog.base.set_state(618);
			recog.typeParameters()?;

			/*InvokeRule interfaceCommonBodyDeclaration*/
			recog.base.set_state(619);
			recog.interfaceCommonBodyDeclaration()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- interfaceCommonBodyDeclaration ----------------
pub type InterfaceCommonBodyDeclarationContextAll<'input> = InterfaceCommonBodyDeclarationContext<'input>;


pub type InterfaceCommonBodyDeclarationContext<'input> = BaseParserRuleContext<'input,InterfaceCommonBodyDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct InterfaceCommonBodyDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for InterfaceCommonBodyDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for InterfaceCommonBodyDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_interfaceCommonBodyDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_interfaceCommonBodyDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for InterfaceCommonBodyDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_interfaceCommonBodyDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_interfaceCommonBodyDeclaration }
}
antlr_rust::tid!{InterfaceCommonBodyDeclarationContextExt<'a>}

impl<'input> InterfaceCommonBodyDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InterfaceCommonBodyDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InterfaceCommonBodyDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InterfaceCommonBodyDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<InterfaceCommonBodyDeclarationContextExt<'input>>{

fn typeTypeOrVoid(&self) -> Option<Rc<TypeTypeOrVoidContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn formalParameters(&self) -> Option<Rc<FormalParametersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn methodBody(&self) -> Option<Rc<MethodBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token LBRACK in current rule
fn LBRACK_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LBRACK, starting from 0.
/// Returns `None` if number of children corresponding to token LBRACK is less or equal than `i`.
fn LBRACK(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACK, i)
}
/// Retrieves all `TerminalNode`s corresponding to token RBRACK in current rule
fn RBRACK_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RBRACK, starting from 0.
/// Returns `None` if number of children corresponding to token RBRACK is less or equal than `i`.
fn RBRACK(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACK, i)
}
/// Retrieves first TerminalNode corresponding to token THROWS
/// Returns `None` if there is no child corresponding to token THROWS
fn THROWS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(THROWS, 0)
}
fn qualifiedNameList(&self) -> Option<Rc<QualifiedNameListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InterfaceCommonBodyDeclarationContextAttrs<'input> for InterfaceCommonBodyDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn interfaceCommonBodyDeclaration(&mut self,)
	-> Result<Rc<InterfaceCommonBodyDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InterfaceCommonBodyDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_interfaceCommonBodyDeclaration);
        let mut _localctx: Rc<InterfaceCommonBodyDeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(624);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(57,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule annotation*/
					recog.base.set_state(621);
					recog.annotation()?;

					}
					} 
				}
				recog.base.set_state(626);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(57,&mut recog.base)?;
			}
			/*InvokeRule typeTypeOrVoid*/
			recog.base.set_state(627);
			recog.typeTypeOrVoid()?;

			/*InvokeRule identifier*/
			recog.base.set_state(628);
			recog.identifier()?;

			/*InvokeRule formalParameters*/
			recog.base.set_state(629);
			recog.formalParameters()?;

			recog.base.set_state(634);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==LBRACK {
				{
				{
				recog.base.set_state(630);
				recog.base.match_token(LBRACK,&mut recog.err_handler)?;

				recog.base.set_state(631);
				recog.base.match_token(RBRACK,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(636);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(639);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==THROWS {
				{
				recog.base.set_state(637);
				recog.base.match_token(THROWS,&mut recog.err_handler)?;

				/*InvokeRule qualifiedNameList*/
				recog.base.set_state(638);
				recog.qualifiedNameList()?;

				}
			}

			/*InvokeRule methodBody*/
			recog.base.set_state(641);
			recog.methodBody()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- variableDeclarators ----------------
pub type VariableDeclaratorsContextAll<'input> = VariableDeclaratorsContext<'input>;


pub type VariableDeclaratorsContext<'input> = BaseParserRuleContext<'input,VariableDeclaratorsContextExt<'input>>;

#[derive(Clone)]
pub struct VariableDeclaratorsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for VariableDeclaratorsContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for VariableDeclaratorsContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_variableDeclarators(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_variableDeclarators(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for VariableDeclaratorsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variableDeclarators }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variableDeclarators }
}
antlr_rust::tid!{VariableDeclaratorsContextExt<'a>}

impl<'input> VariableDeclaratorsContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariableDeclaratorsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableDeclaratorsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VariableDeclaratorsContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<VariableDeclaratorsContextExt<'input>>{

fn variableDeclarator_all(&self) ->  Vec<Rc<VariableDeclaratorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variableDeclarator(&self, i: usize) -> Option<Rc<VariableDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> VariableDeclaratorsContextAttrs<'input> for VariableDeclaratorsContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variableDeclarators(&mut self,)
	-> Result<Rc<VariableDeclaratorsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableDeclaratorsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_variableDeclarators);
        let mut _localctx: Rc<VariableDeclaratorsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule variableDeclarator*/
			recog.base.set_state(643);
			recog.variableDeclarator()?;

			recog.base.set_state(648);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(644);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule variableDeclarator*/
				recog.base.set_state(645);
				recog.variableDeclarator()?;

				}
				}
				recog.base.set_state(650);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- variableDeclarator ----------------
pub type VariableDeclaratorContextAll<'input> = VariableDeclaratorContext<'input>;


pub type VariableDeclaratorContext<'input> = BaseParserRuleContext<'input,VariableDeclaratorContextExt<'input>>;

#[derive(Clone)]
pub struct VariableDeclaratorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for VariableDeclaratorContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for VariableDeclaratorContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_variableDeclarator(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_variableDeclarator(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for VariableDeclaratorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variableDeclarator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variableDeclarator }
}
antlr_rust::tid!{VariableDeclaratorContextExt<'a>}

impl<'input> VariableDeclaratorContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariableDeclaratorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableDeclaratorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VariableDeclaratorContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<VariableDeclaratorContextExt<'input>>{

fn variableDeclaratorId(&self) -> Option<Rc<VariableDeclaratorIdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN
/// Returns `None` if there is no child corresponding to token ASSIGN
fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN, 0)
}
fn variableInitializer(&self) -> Option<Rc<VariableInitializerContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> VariableDeclaratorContextAttrs<'input> for VariableDeclaratorContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variableDeclarator(&mut self,)
	-> Result<Rc<VariableDeclaratorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableDeclaratorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_variableDeclarator);
        let mut _localctx: Rc<VariableDeclaratorContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule variableDeclaratorId*/
			recog.base.set_state(651);
			recog.variableDeclaratorId()?;

			recog.base.set_state(654);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ASSIGN {
				{
				recog.base.set_state(652);
				recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

				/*InvokeRule variableInitializer*/
				recog.base.set_state(653);
				recog.variableInitializer()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- variableDeclaratorId ----------------
pub type VariableDeclaratorIdContextAll<'input> = VariableDeclaratorIdContext<'input>;


pub type VariableDeclaratorIdContext<'input> = BaseParserRuleContext<'input,VariableDeclaratorIdContextExt<'input>>;

#[derive(Clone)]
pub struct VariableDeclaratorIdContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for VariableDeclaratorIdContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for VariableDeclaratorIdContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_variableDeclaratorId(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_variableDeclaratorId(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for VariableDeclaratorIdContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variableDeclaratorId }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variableDeclaratorId }
}
antlr_rust::tid!{VariableDeclaratorIdContextExt<'a>}

impl<'input> VariableDeclaratorIdContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariableDeclaratorIdContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableDeclaratorIdContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VariableDeclaratorIdContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<VariableDeclaratorIdContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token LBRACK in current rule
fn LBRACK_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LBRACK, starting from 0.
/// Returns `None` if number of children corresponding to token LBRACK is less or equal than `i`.
fn LBRACK(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACK, i)
}
/// Retrieves all `TerminalNode`s corresponding to token RBRACK in current rule
fn RBRACK_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RBRACK, starting from 0.
/// Returns `None` if number of children corresponding to token RBRACK is less or equal than `i`.
fn RBRACK(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACK, i)
}

}

impl<'input> VariableDeclaratorIdContextAttrs<'input> for VariableDeclaratorIdContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variableDeclaratorId(&mut self,)
	-> Result<Rc<VariableDeclaratorIdContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableDeclaratorIdContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_variableDeclaratorId);
        let mut _localctx: Rc<VariableDeclaratorIdContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(656);
			recog.identifier()?;

			recog.base.set_state(661);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==LBRACK {
				{
				{
				recog.base.set_state(657);
				recog.base.match_token(LBRACK,&mut recog.err_handler)?;

				recog.base.set_state(658);
				recog.base.match_token(RBRACK,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(663);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- variableInitializer ----------------
pub type VariableInitializerContextAll<'input> = VariableInitializerContext<'input>;


pub type VariableInitializerContext<'input> = BaseParserRuleContext<'input,VariableInitializerContextExt<'input>>;

#[derive(Clone)]
pub struct VariableInitializerContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for VariableInitializerContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for VariableInitializerContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_variableInitializer(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_variableInitializer(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for VariableInitializerContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variableInitializer }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variableInitializer }
}
antlr_rust::tid!{VariableInitializerContextExt<'a>}

impl<'input> VariableInitializerContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariableInitializerContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableInitializerContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VariableInitializerContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<VariableInitializerContextExt<'input>>{

fn arrayInitializer(&self) -> Option<Rc<ArrayInitializerContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> VariableInitializerContextAttrs<'input> for VariableInitializerContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variableInitializer(&mut self,)
	-> Result<Rc<VariableInitializerContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableInitializerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_variableInitializer);
        let mut _localctx: Rc<VariableInitializerContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(666);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LBRACE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule arrayInitializer*/
					recog.base.set_state(664);
					recog.arrayInitializer()?;

					}
				}

			 BOOLEAN | BYTE | CHAR | DOUBLE | FLOAT | INT | LONG | NEW | SHORT | SUPER |
			 SWITCH | THIS | VOID | MODULE | OPEN | REQUIRES | EXPORTS | OPENS | TO |
			 USES | PROVIDES | WITH | TRANSITIVE | VAR | YIELD | RECORD | SEALED |
			 PERMITS | DECIMAL_LITERAL | HEX_LITERAL | OCT_LITERAL | BINARY_LITERAL |
			 FLOAT_LITERAL | HEX_FLOAT_LITERAL | BOOL_LITERAL | CHAR_LITERAL | STRING_LITERAL |
			 TEXT_BLOCK | NULL_LITERAL | LPAREN | LT | BANG | TILDE | INC | DEC |
			 ADD | SUB | AT | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expression*/
					recog.base.set_state(665);
					recog.expression_rec(0)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- arrayInitializer ----------------
pub type ArrayInitializerContextAll<'input> = ArrayInitializerContext<'input>;


pub type ArrayInitializerContext<'input> = BaseParserRuleContext<'input,ArrayInitializerContextExt<'input>>;

#[derive(Clone)]
pub struct ArrayInitializerContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ArrayInitializerContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ArrayInitializerContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_arrayInitializer(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_arrayInitializer(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ArrayInitializerContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arrayInitializer }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arrayInitializer }
}
antlr_rust::tid!{ArrayInitializerContextExt<'a>}

impl<'input> ArrayInitializerContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArrayInitializerContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArrayInitializerContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArrayInitializerContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ArrayInitializerContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn variableInitializer_all(&self) ->  Vec<Rc<VariableInitializerContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variableInitializer(&self, i: usize) -> Option<Rc<VariableInitializerContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ArrayInitializerContextAttrs<'input> for ArrayInitializerContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn arrayInitializer(&mut self,)
	-> Result<Rc<ArrayInitializerContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArrayInitializerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_arrayInitializer);
        let mut _localctx: Rc<ArrayInitializerContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(668);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(680);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << DOUBLE) | (1usize << FLOAT) | (1usize << INT) | (1usize << LONG) | (1usize << NEW))) != 0) || ((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (SHORT - 37)) | (1usize << (SUPER - 37)) | (1usize << (SWITCH - 37)) | (1usize << (THIS - 37)) | (1usize << (VOID - 37)) | (1usize << (MODULE - 37)) | (1usize << (OPEN - 37)) | (1usize << (REQUIRES - 37)) | (1usize << (EXPORTS - 37)) | (1usize << (OPENS - 37)) | (1usize << (TO - 37)) | (1usize << (USES - 37)) | (1usize << (PROVIDES - 37)) | (1usize << (WITH - 37)) | (1usize << (TRANSITIVE - 37)) | (1usize << (VAR - 37)) | (1usize << (YIELD - 37)) | (1usize << (RECORD - 37)) | (1usize << (SEALED - 37)) | (1usize << (PERMITS - 37)) | (1usize << (DECIMAL_LITERAL - 37)) | (1usize << (HEX_LITERAL - 37)))) != 0) || ((((_la - 69)) & !0x3f) == 0 && ((1usize << (_la - 69)) & ((1usize << (OCT_LITERAL - 69)) | (1usize << (BINARY_LITERAL - 69)) | (1usize << (FLOAT_LITERAL - 69)) | (1usize << (HEX_FLOAT_LITERAL - 69)) | (1usize << (BOOL_LITERAL - 69)) | (1usize << (CHAR_LITERAL - 69)) | (1usize << (STRING_LITERAL - 69)) | (1usize << (TEXT_BLOCK - 69)) | (1usize << (NULL_LITERAL - 69)) | (1usize << (LPAREN - 69)) | (1usize << (LBRACE - 69)) | (1usize << (LT - 69)) | (1usize << (BANG - 69)) | (1usize << (TILDE - 69)) | (1usize << (INC - 69)))) != 0) || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (DEC - 101)) | (1usize << (ADD - 101)) | (1usize << (SUB - 101)) | (1usize << (AT - 101)) | (1usize << (IDENTIFIER - 101)))) != 0) {
				{
				/*InvokeRule variableInitializer*/
				recog.base.set_state(669);
				recog.variableInitializer()?;

				recog.base.set_state(674);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(64,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(670);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule variableInitializer*/
						recog.base.set_state(671);
						recog.variableInitializer()?;

						}
						} 
					}
					recog.base.set_state(676);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(64,&mut recog.base)?;
				}
				recog.base.set_state(678);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==COMMA {
					{
					recog.base.set_state(677);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					}
				}

				}
			}

			recog.base.set_state(682);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- classOrInterfaceType ----------------
pub type ClassOrInterfaceTypeContextAll<'input> = ClassOrInterfaceTypeContext<'input>;


pub type ClassOrInterfaceTypeContext<'input> = BaseParserRuleContext<'input,ClassOrInterfaceTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ClassOrInterfaceTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ClassOrInterfaceTypeContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ClassOrInterfaceTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_classOrInterfaceType(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_classOrInterfaceType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ClassOrInterfaceTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_classOrInterfaceType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_classOrInterfaceType }
}
antlr_rust::tid!{ClassOrInterfaceTypeContextExt<'a>}

impl<'input> ClassOrInterfaceTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ClassOrInterfaceTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ClassOrInterfaceTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ClassOrInterfaceTypeContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ClassOrInterfaceTypeContextExt<'input>>{

fn typeIdentifier(&self) -> Option<Rc<TypeIdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DOT, i)
}
fn typeArguments_all(&self) ->  Vec<Rc<TypeArgumentsContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeArguments(&self, i: usize) -> Option<Rc<TypeArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ClassOrInterfaceTypeContextAttrs<'input> for ClassOrInterfaceTypeContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn classOrInterfaceType(&mut self,)
	-> Result<Rc<ClassOrInterfaceTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ClassOrInterfaceTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_classOrInterfaceType);
        let mut _localctx: Rc<ClassOrInterfaceTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(692);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(68,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule identifier*/
					recog.base.set_state(684);
					recog.identifier()?;

					recog.base.set_state(686);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LT {
						{
						/*InvokeRule typeArguments*/
						recog.base.set_state(685);
						recog.typeArguments()?;

						}
					}

					recog.base.set_state(688);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(694);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(68,&mut recog.base)?;
			}
			/*InvokeRule typeIdentifier*/
			recog.base.set_state(695);
			recog.typeIdentifier()?;

			recog.base.set_state(697);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(69,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule typeArguments*/
					recog.base.set_state(696);
					recog.typeArguments()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeArgument ----------------
pub type TypeArgumentContextAll<'input> = TypeArgumentContext<'input>;


pub type TypeArgumentContext<'input> = BaseParserRuleContext<'input,TypeArgumentContextExt<'input>>;

#[derive(Clone)]
pub struct TypeArgumentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for TypeArgumentContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for TypeArgumentContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_typeArgument(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_typeArgument(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TypeArgumentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeArgument }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeArgument }
}
antlr_rust::tid!{TypeArgumentContextExt<'a>}

impl<'input> TypeArgumentContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeArgumentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeArgumentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeArgumentContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<TypeArgumentContextExt<'input>>{

fn typeType(&self) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token QUESTION
/// Returns `None` if there is no child corresponding to token QUESTION
fn QUESTION(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(QUESTION, 0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token EXTENDS
/// Returns `None` if there is no child corresponding to token EXTENDS
fn EXTENDS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(EXTENDS, 0)
}
/// Retrieves first TerminalNode corresponding to token SUPER
/// Returns `None` if there is no child corresponding to token SUPER
fn SUPER(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SUPER, 0)
}

}

impl<'input> TypeArgumentContextAttrs<'input> for TypeArgumentContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeArgument(&mut self,)
	-> Result<Rc<TypeArgumentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeArgumentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_typeArgument);
        let mut _localctx: Rc<TypeArgumentContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(711);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(72,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule typeType*/
					recog.base.set_state(699);
					recog.typeType()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(703);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 51)) & !0x3f) == 0 && ((1usize << (_la - 51)) & ((1usize << (MODULE - 51)) | (1usize << (OPEN - 51)) | (1usize << (REQUIRES - 51)) | (1usize << (EXPORTS - 51)) | (1usize << (OPENS - 51)) | (1usize << (TO - 51)) | (1usize << (USES - 51)) | (1usize << (PROVIDES - 51)) | (1usize << (WITH - 51)) | (1usize << (TRANSITIVE - 51)) | (1usize << (VAR - 51)) | (1usize << (YIELD - 51)) | (1usize << (RECORD - 51)) | (1usize << (SEALED - 51)) | (1usize << (PERMITS - 51)))) != 0) || _la==AT || _la==IDENTIFIER {
						{
						{
						/*InvokeRule annotation*/
						recog.base.set_state(700);
						recog.annotation()?;

						}
						}
						recog.base.set_state(705);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(706);
					recog.base.match_token(QUESTION,&mut recog.err_handler)?;

					recog.base.set_state(709);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==EXTENDS || _la==SUPER {
						{
						recog.base.set_state(707);
						_la = recog.base.input.la(1);
						if { !(_la==EXTENDS || _la==SUPER) } {
							recog.err_handler.recover_inline(&mut recog.base)?;

						}
						else {
							if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler);
						}
						/*InvokeRule typeType*/
						recog.base.set_state(708);
						recog.typeType()?;

						}
					}

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- qualifiedNameList ----------------
pub type QualifiedNameListContextAll<'input> = QualifiedNameListContext<'input>;


pub type QualifiedNameListContext<'input> = BaseParserRuleContext<'input,QualifiedNameListContextExt<'input>>;

#[derive(Clone)]
pub struct QualifiedNameListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for QualifiedNameListContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for QualifiedNameListContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_qualifiedNameList(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_qualifiedNameList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for QualifiedNameListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_qualifiedNameList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_qualifiedNameList }
}
antlr_rust::tid!{QualifiedNameListContextExt<'a>}

impl<'input> QualifiedNameListContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<QualifiedNameListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QualifiedNameListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait QualifiedNameListContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<QualifiedNameListContextExt<'input>>{

fn qualifiedName_all(&self) ->  Vec<Rc<QualifiedNameContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn qualifiedName(&self, i: usize) -> Option<Rc<QualifiedNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> QualifiedNameListContextAttrs<'input> for QualifiedNameListContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn qualifiedNameList(&mut self,)
	-> Result<Rc<QualifiedNameListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QualifiedNameListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_qualifiedNameList);
        let mut _localctx: Rc<QualifiedNameListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule qualifiedName*/
			recog.base.set_state(713);
			recog.qualifiedName()?;

			recog.base.set_state(718);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(714);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule qualifiedName*/
				recog.base.set_state(715);
				recog.qualifiedName()?;

				}
				}
				recog.base.set_state(720);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- formalParameters ----------------
pub type FormalParametersContextAll<'input> = FormalParametersContext<'input>;


pub type FormalParametersContext<'input> = BaseParserRuleContext<'input,FormalParametersContextExt<'input>>;

#[derive(Clone)]
pub struct FormalParametersContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for FormalParametersContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for FormalParametersContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_formalParameters(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_formalParameters(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for FormalParametersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_formalParameters }
	//fn type_rule_index() -> usize where Self: Sized { RULE_formalParameters }
}
antlr_rust::tid!{FormalParametersContextExt<'a>}

impl<'input> FormalParametersContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FormalParametersContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FormalParametersContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FormalParametersContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<FormalParametersContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn receiverParameter(&self) -> Option<Rc<ReceiverParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}
fn formalParameterList(&self) -> Option<Rc<FormalParameterListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FormalParametersContextAttrs<'input> for FormalParametersContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn formalParameters(&mut self,)
	-> Result<Rc<FormalParametersContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FormalParametersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_formalParameters);
        let mut _localctx: Rc<FormalParametersContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(721);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(733);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(77,&mut recog.base)? {
				1 =>{
					{
					recog.base.set_state(723);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << DOUBLE) | (1usize << FLOAT) | (1usize << INT) | (1usize << LONG))) != 0) || ((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (SHORT - 37)) | (1usize << (MODULE - 37)) | (1usize << (OPEN - 37)) | (1usize << (REQUIRES - 37)) | (1usize << (EXPORTS - 37)) | (1usize << (OPENS - 37)) | (1usize << (TO - 37)) | (1usize << (USES - 37)) | (1usize << (PROVIDES - 37)) | (1usize << (WITH - 37)) | (1usize << (TRANSITIVE - 37)) | (1usize << (VAR - 37)) | (1usize << (YIELD - 37)) | (1usize << (RECORD - 37)) | (1usize << (SEALED - 37)) | (1usize << (PERMITS - 37)))) != 0) || _la==AT || _la==IDENTIFIER {
						{
						/*InvokeRule receiverParameter*/
						recog.base.set_state(722);
						recog.receiverParameter()?;

						}
					}

					}
				}
			,
				2 =>{
					{
					/*InvokeRule receiverParameter*/
					recog.base.set_state(725);
					recog.receiverParameter()?;

					recog.base.set_state(728);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMA {
						{
						recog.base.set_state(726);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule formalParameterList*/
						recog.base.set_state(727);
						recog.formalParameterList()?;

						}
					}

					}
				}
			,
				3 =>{
					{
					recog.base.set_state(731);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << DOUBLE) | (1usize << FINAL) | (1usize << FLOAT) | (1usize << INT) | (1usize << LONG))) != 0) || ((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (SHORT - 37)) | (1usize << (MODULE - 37)) | (1usize << (OPEN - 37)) | (1usize << (REQUIRES - 37)) | (1usize << (EXPORTS - 37)) | (1usize << (OPENS - 37)) | (1usize << (TO - 37)) | (1usize << (USES - 37)) | (1usize << (PROVIDES - 37)) | (1usize << (WITH - 37)) | (1usize << (TRANSITIVE - 37)) | (1usize << (VAR - 37)) | (1usize << (YIELD - 37)) | (1usize << (RECORD - 37)) | (1usize << (SEALED - 37)) | (1usize << (PERMITS - 37)))) != 0) || _la==AT || _la==IDENTIFIER {
						{
						/*InvokeRule formalParameterList*/
						recog.base.set_state(730);
						recog.formalParameterList()?;

						}
					}

					}
				}

				_ => {}
			}
			recog.base.set_state(735);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- receiverParameter ----------------
pub type ReceiverParameterContextAll<'input> = ReceiverParameterContext<'input>;


pub type ReceiverParameterContext<'input> = BaseParserRuleContext<'input,ReceiverParameterContextExt<'input>>;

#[derive(Clone)]
pub struct ReceiverParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ReceiverParameterContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ReceiverParameterContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_receiverParameter(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_receiverParameter(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ReceiverParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_receiverParameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_receiverParameter }
}
antlr_rust::tid!{ReceiverParameterContextExt<'a>}

impl<'input> ReceiverParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReceiverParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReceiverParameterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReceiverParameterContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ReceiverParameterContextExt<'input>>{

fn typeType(&self) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token THIS
/// Returns `None` if there is no child corresponding to token THIS
fn THIS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(THIS, 0)
}
fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DOT, i)
}

}

impl<'input> ReceiverParameterContextAttrs<'input> for ReceiverParameterContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn receiverParameter(&mut self,)
	-> Result<Rc<ReceiverParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReceiverParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_receiverParameter);
        let mut _localctx: Rc<ReceiverParameterContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typeType*/
			recog.base.set_state(737);
			recog.typeType()?;

			recog.base.set_state(743);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 51)) & !0x3f) == 0 && ((1usize << (_la - 51)) & ((1usize << (MODULE - 51)) | (1usize << (OPEN - 51)) | (1usize << (REQUIRES - 51)) | (1usize << (EXPORTS - 51)) | (1usize << (OPENS - 51)) | (1usize << (TO - 51)) | (1usize << (USES - 51)) | (1usize << (PROVIDES - 51)) | (1usize << (WITH - 51)) | (1usize << (TRANSITIVE - 51)) | (1usize << (VAR - 51)) | (1usize << (YIELD - 51)) | (1usize << (RECORD - 51)) | (1usize << (SEALED - 51)) | (1usize << (PERMITS - 51)))) != 0) || _la==IDENTIFIER {
				{
				{
				/*InvokeRule identifier*/
				recog.base.set_state(738);
				recog.identifier()?;

				recog.base.set_state(739);
				recog.base.match_token(DOT,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(745);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(746);
			recog.base.match_token(THIS,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- formalParameterList ----------------
pub type FormalParameterListContextAll<'input> = FormalParameterListContext<'input>;


pub type FormalParameterListContext<'input> = BaseParserRuleContext<'input,FormalParameterListContextExt<'input>>;

#[derive(Clone)]
pub struct FormalParameterListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for FormalParameterListContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for FormalParameterListContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_formalParameterList(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_formalParameterList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for FormalParameterListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_formalParameterList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_formalParameterList }
}
antlr_rust::tid!{FormalParameterListContextExt<'a>}

impl<'input> FormalParameterListContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FormalParameterListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FormalParameterListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FormalParameterListContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<FormalParameterListContextExt<'input>>{

fn formalParameter_all(&self) ->  Vec<Rc<FormalParameterContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn formalParameter(&self, i: usize) -> Option<Rc<FormalParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}
fn lastFormalParameter(&self) -> Option<Rc<LastFormalParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FormalParameterListContextAttrs<'input> for FormalParameterListContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn formalParameterList(&mut self,)
	-> Result<Rc<FormalParameterListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FormalParameterListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_formalParameterList);
        let mut _localctx: Rc<FormalParameterListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(761);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(81,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule formalParameter*/
					recog.base.set_state(748);
					recog.formalParameter()?;

					recog.base.set_state(753);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(79,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(749);
							recog.base.match_token(COMMA,&mut recog.err_handler)?;

							/*InvokeRule formalParameter*/
							recog.base.set_state(750);
							recog.formalParameter()?;

							}
							} 
						}
						recog.base.set_state(755);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(79,&mut recog.base)?;
					}
					recog.base.set_state(758);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMA {
						{
						recog.base.set_state(756);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule lastFormalParameter*/
						recog.base.set_state(757);
						recog.lastFormalParameter()?;

						}
					}

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule lastFormalParameter*/
					recog.base.set_state(760);
					recog.lastFormalParameter()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- formalParameter ----------------
pub type FormalParameterContextAll<'input> = FormalParameterContext<'input>;


pub type FormalParameterContext<'input> = BaseParserRuleContext<'input,FormalParameterContextExt<'input>>;

#[derive(Clone)]
pub struct FormalParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for FormalParameterContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for FormalParameterContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_formalParameter(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_formalParameter(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for FormalParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_formalParameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_formalParameter }
}
antlr_rust::tid!{FormalParameterContextExt<'a>}

impl<'input> FormalParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FormalParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FormalParameterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FormalParameterContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<FormalParameterContextExt<'input>>{

fn typeType(&self) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableDeclaratorId(&self) -> Option<Rc<VariableDeclaratorIdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableModifier_all(&self) ->  Vec<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variableModifier(&self, i: usize) -> Option<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> FormalParameterContextAttrs<'input> for FormalParameterContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn formalParameter(&mut self,)
	-> Result<Rc<FormalParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FormalParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_formalParameter);
        let mut _localctx: Rc<FormalParameterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(766);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(82,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule variableModifier*/
					recog.base.set_state(763);
					recog.variableModifier()?;

					}
					} 
				}
				recog.base.set_state(768);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(82,&mut recog.base)?;
			}
			/*InvokeRule typeType*/
			recog.base.set_state(769);
			recog.typeType()?;

			/*InvokeRule variableDeclaratorId*/
			recog.base.set_state(770);
			recog.variableDeclaratorId()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- lastFormalParameter ----------------
pub type LastFormalParameterContextAll<'input> = LastFormalParameterContext<'input>;


pub type LastFormalParameterContext<'input> = BaseParserRuleContext<'input,LastFormalParameterContextExt<'input>>;

#[derive(Clone)]
pub struct LastFormalParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for LastFormalParameterContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for LastFormalParameterContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lastFormalParameter(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_lastFormalParameter(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for LastFormalParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lastFormalParameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lastFormalParameter }
}
antlr_rust::tid!{LastFormalParameterContextExt<'a>}

impl<'input> LastFormalParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LastFormalParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LastFormalParameterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LastFormalParameterContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<LastFormalParameterContextExt<'input>>{

fn typeType(&self) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ELLIPSIS
/// Returns `None` if there is no child corresponding to token ELLIPSIS
fn ELLIPSIS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(ELLIPSIS, 0)
}
fn variableDeclaratorId(&self) -> Option<Rc<VariableDeclaratorIdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableModifier_all(&self) ->  Vec<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variableModifier(&self, i: usize) -> Option<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> LastFormalParameterContextAttrs<'input> for LastFormalParameterContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lastFormalParameter(&mut self,)
	-> Result<Rc<LastFormalParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LastFormalParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_lastFormalParameter);
        let mut _localctx: Rc<LastFormalParameterContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(775);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(83,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule variableModifier*/
					recog.base.set_state(772);
					recog.variableModifier()?;

					}
					} 
				}
				recog.base.set_state(777);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(83,&mut recog.base)?;
			}
			/*InvokeRule typeType*/
			recog.base.set_state(778);
			recog.typeType()?;

			recog.base.set_state(782);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 51)) & !0x3f) == 0 && ((1usize << (_la - 51)) & ((1usize << (MODULE - 51)) | (1usize << (OPEN - 51)) | (1usize << (REQUIRES - 51)) | (1usize << (EXPORTS - 51)) | (1usize << (OPENS - 51)) | (1usize << (TO - 51)) | (1usize << (USES - 51)) | (1usize << (PROVIDES - 51)) | (1usize << (WITH - 51)) | (1usize << (TRANSITIVE - 51)) | (1usize << (VAR - 51)) | (1usize << (YIELD - 51)) | (1usize << (RECORD - 51)) | (1usize << (SEALED - 51)) | (1usize << (PERMITS - 51)))) != 0) || _la==AT || _la==IDENTIFIER {
				{
				{
				/*InvokeRule annotation*/
				recog.base.set_state(779);
				recog.annotation()?;

				}
				}
				recog.base.set_state(784);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(785);
			recog.base.match_token(ELLIPSIS,&mut recog.err_handler)?;

			/*InvokeRule variableDeclaratorId*/
			recog.base.set_state(786);
			recog.variableDeclaratorId()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- lambdaLVTIList ----------------
pub type LambdaLVTIListContextAll<'input> = LambdaLVTIListContext<'input>;


pub type LambdaLVTIListContext<'input> = BaseParserRuleContext<'input,LambdaLVTIListContextExt<'input>>;

#[derive(Clone)]
pub struct LambdaLVTIListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for LambdaLVTIListContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for LambdaLVTIListContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lambdaLVTIList(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_lambdaLVTIList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for LambdaLVTIListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lambdaLVTIList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lambdaLVTIList }
}
antlr_rust::tid!{LambdaLVTIListContextExt<'a>}

impl<'input> LambdaLVTIListContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LambdaLVTIListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LambdaLVTIListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LambdaLVTIListContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<LambdaLVTIListContextExt<'input>>{

fn lambdaLVTIParameter_all(&self) ->  Vec<Rc<LambdaLVTIParameterContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn lambdaLVTIParameter(&self, i: usize) -> Option<Rc<LambdaLVTIParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> LambdaLVTIListContextAttrs<'input> for LambdaLVTIListContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lambdaLVTIList(&mut self,)
	-> Result<Rc<LambdaLVTIListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LambdaLVTIListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_lambdaLVTIList);
        let mut _localctx: Rc<LambdaLVTIListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule lambdaLVTIParameter*/
			recog.base.set_state(788);
			recog.lambdaLVTIParameter()?;

			recog.base.set_state(793);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(789);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule lambdaLVTIParameter*/
				recog.base.set_state(790);
				recog.lambdaLVTIParameter()?;

				}
				}
				recog.base.set_state(795);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- lambdaLVTIParameter ----------------
pub type LambdaLVTIParameterContextAll<'input> = LambdaLVTIParameterContext<'input>;


pub type LambdaLVTIParameterContext<'input> = BaseParserRuleContext<'input,LambdaLVTIParameterContextExt<'input>>;

#[derive(Clone)]
pub struct LambdaLVTIParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for LambdaLVTIParameterContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for LambdaLVTIParameterContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lambdaLVTIParameter(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_lambdaLVTIParameter(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for LambdaLVTIParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lambdaLVTIParameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lambdaLVTIParameter }
}
antlr_rust::tid!{LambdaLVTIParameterContextExt<'a>}

impl<'input> LambdaLVTIParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LambdaLVTIParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LambdaLVTIParameterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LambdaLVTIParameterContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<LambdaLVTIParameterContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token VAR
/// Returns `None` if there is no child corresponding to token VAR
fn VAR(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(VAR, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableModifier_all(&self) ->  Vec<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variableModifier(&self, i: usize) -> Option<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> LambdaLVTIParameterContextAttrs<'input> for LambdaLVTIParameterContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lambdaLVTIParameter(&mut self,)
	-> Result<Rc<LambdaLVTIParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LambdaLVTIParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_lambdaLVTIParameter);
        let mut _localctx: Rc<LambdaLVTIParameterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(799);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(86,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule variableModifier*/
					recog.base.set_state(796);
					recog.variableModifier()?;

					}
					} 
				}
				recog.base.set_state(801);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(86,&mut recog.base)?;
			}
			recog.base.set_state(802);
			recog.base.match_token(VAR,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(803);
			recog.identifier()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- qualifiedName ----------------
pub type QualifiedNameContextAll<'input> = QualifiedNameContext<'input>;


pub type QualifiedNameContext<'input> = BaseParserRuleContext<'input,QualifiedNameContextExt<'input>>;

#[derive(Clone)]
pub struct QualifiedNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for QualifiedNameContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for QualifiedNameContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_qualifiedName(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_qualifiedName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for QualifiedNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_qualifiedName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_qualifiedName }
}
antlr_rust::tid!{QualifiedNameContextExt<'a>}

impl<'input> QualifiedNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<QualifiedNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QualifiedNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait QualifiedNameContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<QualifiedNameContextExt<'input>>{

fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DOT, i)
}

}

impl<'input> QualifiedNameContextAttrs<'input> for QualifiedNameContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn qualifiedName(&mut self,)
	-> Result<Rc<QualifiedNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QualifiedNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_qualifiedName);
        let mut _localctx: Rc<QualifiedNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(805);
			recog.identifier()?;

			recog.base.set_state(810);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(87,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(806);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					/*InvokeRule identifier*/
					recog.base.set_state(807);
					recog.identifier()?;

					}
					} 
				}
				recog.base.set_state(812);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(87,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- literal ----------------
pub type LiteralContextAll<'input> = LiteralContext<'input>;


pub type LiteralContext<'input> = BaseParserRuleContext<'input,LiteralContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for LiteralContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for LiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_literal(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_literal(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for LiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}
antlr_rust::tid!{LiteralContextExt<'a>}

impl<'input> LiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LiteralContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<LiteralContextExt<'input>>{

fn integerLiteral(&self) -> Option<Rc<IntegerLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn floatLiteral(&self) -> Option<Rc<FloatLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CHAR_LITERAL
/// Returns `None` if there is no child corresponding to token CHAR_LITERAL
fn CHAR_LITERAL(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(CHAR_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOL_LITERAL
/// Returns `None` if there is no child corresponding to token BOOL_LITERAL
fn BOOL_LITERAL(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(BOOL_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token NULL_LITERAL
/// Returns `None` if there is no child corresponding to token NULL_LITERAL
fn NULL_LITERAL(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(NULL_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token TEXT_BLOCK
/// Returns `None` if there is no child corresponding to token TEXT_BLOCK
fn TEXT_BLOCK(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(TEXT_BLOCK, 0)
}

}

impl<'input> LiteralContextAttrs<'input> for LiteralContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn literal(&mut self,)
	-> Result<Rc<LiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_literal);
        let mut _localctx: Rc<LiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(820);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 DECIMAL_LITERAL | HEX_LITERAL | OCT_LITERAL | BINARY_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule integerLiteral*/
					recog.base.set_state(813);
					recog.integerLiteral()?;

					}
				}

			 FLOAT_LITERAL | HEX_FLOAT_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule floatLiteral*/
					recog.base.set_state(814);
					recog.floatLiteral()?;

					}
				}

			 CHAR_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(815);
					recog.base.match_token(CHAR_LITERAL,&mut recog.err_handler)?;

					}
				}

			 STRING_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(816);
					recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

					}
				}

			 BOOL_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(817);
					recog.base.match_token(BOOL_LITERAL,&mut recog.err_handler)?;

					}
				}

			 NULL_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(818);
					recog.base.match_token(NULL_LITERAL,&mut recog.err_handler)?;

					}
				}

			 TEXT_BLOCK 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					recog.base.set_state(819);
					recog.base.match_token(TEXT_BLOCK,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- integerLiteral ----------------
pub type IntegerLiteralContextAll<'input> = IntegerLiteralContext<'input>;


pub type IntegerLiteralContext<'input> = BaseParserRuleContext<'input,IntegerLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct IntegerLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for IntegerLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for IntegerLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_integerLiteral(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_integerLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for IntegerLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_integerLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_integerLiteral }
}
antlr_rust::tid!{IntegerLiteralContextExt<'a>}

impl<'input> IntegerLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IntegerLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IntegerLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IntegerLiteralContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<IntegerLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DECIMAL_LITERAL
/// Returns `None` if there is no child corresponding to token DECIMAL_LITERAL
fn DECIMAL_LITERAL(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DECIMAL_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token HEX_LITERAL
/// Returns `None` if there is no child corresponding to token HEX_LITERAL
fn HEX_LITERAL(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(HEX_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token OCT_LITERAL
/// Returns `None` if there is no child corresponding to token OCT_LITERAL
fn OCT_LITERAL(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(OCT_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token BINARY_LITERAL
/// Returns `None` if there is no child corresponding to token BINARY_LITERAL
fn BINARY_LITERAL(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(BINARY_LITERAL, 0)
}

}

impl<'input> IntegerLiteralContextAttrs<'input> for IntegerLiteralContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn integerLiteral(&mut self,)
	-> Result<Rc<IntegerLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IntegerLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_integerLiteral);
        let mut _localctx: Rc<IntegerLiteralContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(822);
			_la = recog.base.input.la(1);
			if { !(((((_la - 67)) & !0x3f) == 0 && ((1usize << (_la - 67)) & ((1usize << (DECIMAL_LITERAL - 67)) | (1usize << (HEX_LITERAL - 67)) | (1usize << (OCT_LITERAL - 67)) | (1usize << (BINARY_LITERAL - 67)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- floatLiteral ----------------
pub type FloatLiteralContextAll<'input> = FloatLiteralContext<'input>;


pub type FloatLiteralContext<'input> = BaseParserRuleContext<'input,FloatLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct FloatLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for FloatLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for FloatLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_floatLiteral(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_floatLiteral(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for FloatLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_floatLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_floatLiteral }
}
antlr_rust::tid!{FloatLiteralContextExt<'a>}

impl<'input> FloatLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FloatLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FloatLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FloatLiteralContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<FloatLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FLOAT_LITERAL
/// Returns `None` if there is no child corresponding to token FLOAT_LITERAL
fn FLOAT_LITERAL(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(FLOAT_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token HEX_FLOAT_LITERAL
/// Returns `None` if there is no child corresponding to token HEX_FLOAT_LITERAL
fn HEX_FLOAT_LITERAL(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(HEX_FLOAT_LITERAL, 0)
}

}

impl<'input> FloatLiteralContextAttrs<'input> for FloatLiteralContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn floatLiteral(&mut self,)
	-> Result<Rc<FloatLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FloatLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_floatLiteral);
        let mut _localctx: Rc<FloatLiteralContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(824);
			_la = recog.base.input.la(1);
			if { !(_la==FLOAT_LITERAL || _la==HEX_FLOAT_LITERAL) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- altAnnotationQualifiedName ----------------
pub type AltAnnotationQualifiedNameContextAll<'input> = AltAnnotationQualifiedNameContext<'input>;


pub type AltAnnotationQualifiedNameContext<'input> = BaseParserRuleContext<'input,AltAnnotationQualifiedNameContextExt<'input>>;

#[derive(Clone)]
pub struct AltAnnotationQualifiedNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for AltAnnotationQualifiedNameContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for AltAnnotationQualifiedNameContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_altAnnotationQualifiedName(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_altAnnotationQualifiedName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AltAnnotationQualifiedNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_altAnnotationQualifiedName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_altAnnotationQualifiedName }
}
antlr_rust::tid!{AltAnnotationQualifiedNameContextExt<'a>}

impl<'input> AltAnnotationQualifiedNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AltAnnotationQualifiedNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AltAnnotationQualifiedNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AltAnnotationQualifiedNameContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<AltAnnotationQualifiedNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token AT
/// Returns `None` if there is no child corresponding to token AT
fn AT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(AT, 0)
}
fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DOT, i)
}

}

impl<'input> AltAnnotationQualifiedNameContextAttrs<'input> for AltAnnotationQualifiedNameContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn altAnnotationQualifiedName(&mut self,)
	-> Result<Rc<AltAnnotationQualifiedNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AltAnnotationQualifiedNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_altAnnotationQualifiedName);
        let mut _localctx: Rc<AltAnnotationQualifiedNameContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(831);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 51)) & !0x3f) == 0 && ((1usize << (_la - 51)) & ((1usize << (MODULE - 51)) | (1usize << (OPEN - 51)) | (1usize << (REQUIRES - 51)) | (1usize << (EXPORTS - 51)) | (1usize << (OPENS - 51)) | (1usize << (TO - 51)) | (1usize << (USES - 51)) | (1usize << (PROVIDES - 51)) | (1usize << (WITH - 51)) | (1usize << (TRANSITIVE - 51)) | (1usize << (VAR - 51)) | (1usize << (YIELD - 51)) | (1usize << (RECORD - 51)) | (1usize << (SEALED - 51)) | (1usize << (PERMITS - 51)))) != 0) || _la==IDENTIFIER {
				{
				{
				/*InvokeRule identifier*/
				recog.base.set_state(826);
				recog.identifier()?;

				recog.base.set_state(827);
				recog.base.match_token(DOT,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(833);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(834);
			recog.base.match_token(AT,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(835);
			recog.identifier()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- annotation ----------------
pub type AnnotationContextAll<'input> = AnnotationContext<'input>;


pub type AnnotationContext<'input> = BaseParserRuleContext<'input,AnnotationContextExt<'input>>;

#[derive(Clone)]
pub struct AnnotationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for AnnotationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for AnnotationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_annotation(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_annotation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AnnotationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotation }
}
antlr_rust::tid!{AnnotationContextExt<'a>}

impl<'input> AnnotationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnnotationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnnotationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AnnotationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<AnnotationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token AT
/// Returns `None` if there is no child corresponding to token AT
fn AT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(AT, 0)
}
fn qualifiedName(&self) -> Option<Rc<QualifiedNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn altAnnotationQualifiedName(&self) -> Option<Rc<AltAnnotationQualifiedNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn elementValuePairs(&self) -> Option<Rc<ElementValuePairsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn elementValue(&self) -> Option<Rc<ElementValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AnnotationContextAttrs<'input> for AnnotationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn annotation(&mut self,)
	-> Result<Rc<AnnotationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnnotationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_annotation);
        let mut _localctx: Rc<AnnotationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(840);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(90,&mut recog.base)? {
				1 =>{
					{
					recog.base.set_state(837);
					recog.base.match_token(AT,&mut recog.err_handler)?;

					/*InvokeRule qualifiedName*/
					recog.base.set_state(838);
					recog.qualifiedName()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule altAnnotationQualifiedName*/
					recog.base.set_state(839);
					recog.altAnnotationQualifiedName()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(848);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LPAREN {
				{
				recog.base.set_state(842);
				recog.base.match_token(LPAREN,&mut recog.err_handler)?;

				recog.base.set_state(845);
				recog.err_handler.sync(&mut recog.base)?;
				match  recog.interpreter.adaptive_predict(91,&mut recog.base)? {
					x if x == 1=>{
						{
						/*InvokeRule elementValuePairs*/
						recog.base.set_state(843);
						recog.elementValuePairs()?;

						}
					}

					x if x == 2=>{
						{
						/*InvokeRule elementValue*/
						recog.base.set_state(844);
						recog.elementValue()?;

						}
					}

					_ => {}
				}
				recog.base.set_state(847);
				recog.base.match_token(RPAREN,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- elementValuePairs ----------------
pub type ElementValuePairsContextAll<'input> = ElementValuePairsContext<'input>;


pub type ElementValuePairsContext<'input> = BaseParserRuleContext<'input,ElementValuePairsContextExt<'input>>;

#[derive(Clone)]
pub struct ElementValuePairsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ElementValuePairsContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ElementValuePairsContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_elementValuePairs(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_elementValuePairs(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ElementValuePairsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_elementValuePairs }
	//fn type_rule_index() -> usize where Self: Sized { RULE_elementValuePairs }
}
antlr_rust::tid!{ElementValuePairsContextExt<'a>}

impl<'input> ElementValuePairsContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ElementValuePairsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ElementValuePairsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ElementValuePairsContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ElementValuePairsContextExt<'input>>{

fn elementValuePair_all(&self) ->  Vec<Rc<ElementValuePairContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn elementValuePair(&self, i: usize) -> Option<Rc<ElementValuePairContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ElementValuePairsContextAttrs<'input> for ElementValuePairsContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn elementValuePairs(&mut self,)
	-> Result<Rc<ElementValuePairsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ElementValuePairsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_elementValuePairs);
        let mut _localctx: Rc<ElementValuePairsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule elementValuePair*/
			recog.base.set_state(850);
			recog.elementValuePair()?;

			recog.base.set_state(855);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(851);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule elementValuePair*/
				recog.base.set_state(852);
				recog.elementValuePair()?;

				}
				}
				recog.base.set_state(857);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- elementValuePair ----------------
pub type ElementValuePairContextAll<'input> = ElementValuePairContext<'input>;


pub type ElementValuePairContext<'input> = BaseParserRuleContext<'input,ElementValuePairContextExt<'input>>;

#[derive(Clone)]
pub struct ElementValuePairContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ElementValuePairContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ElementValuePairContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_elementValuePair(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_elementValuePair(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ElementValuePairContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_elementValuePair }
	//fn type_rule_index() -> usize where Self: Sized { RULE_elementValuePair }
}
antlr_rust::tid!{ElementValuePairContextExt<'a>}

impl<'input> ElementValuePairContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ElementValuePairContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ElementValuePairContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ElementValuePairContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ElementValuePairContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN
/// Returns `None` if there is no child corresponding to token ASSIGN
fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN, 0)
}
fn elementValue(&self) -> Option<Rc<ElementValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ElementValuePairContextAttrs<'input> for ElementValuePairContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn elementValuePair(&mut self,)
	-> Result<Rc<ElementValuePairContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ElementValuePairContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_elementValuePair);
        let mut _localctx: Rc<ElementValuePairContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(858);
			recog.identifier()?;

			recog.base.set_state(859);
			recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

			/*InvokeRule elementValue*/
			recog.base.set_state(860);
			recog.elementValue()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- elementValue ----------------
pub type ElementValueContextAll<'input> = ElementValueContext<'input>;


pub type ElementValueContext<'input> = BaseParserRuleContext<'input,ElementValueContextExt<'input>>;

#[derive(Clone)]
pub struct ElementValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ElementValueContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ElementValueContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_elementValue(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_elementValue(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ElementValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_elementValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_elementValue }
}
antlr_rust::tid!{ElementValueContextExt<'a>}

impl<'input> ElementValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ElementValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ElementValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ElementValueContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ElementValueContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation(&self) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn elementValueArrayInitializer(&self) -> Option<Rc<ElementValueArrayInitializerContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ElementValueContextAttrs<'input> for ElementValueContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn elementValue(&mut self,)
	-> Result<Rc<ElementValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ElementValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_elementValue);
        let mut _localctx: Rc<ElementValueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(865);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(94,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule expression*/
					recog.base.set_state(862);
					recog.expression_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule annotation*/
					recog.base.set_state(863);
					recog.annotation()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule elementValueArrayInitializer*/
					recog.base.set_state(864);
					recog.elementValueArrayInitializer()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- elementValueArrayInitializer ----------------
pub type ElementValueArrayInitializerContextAll<'input> = ElementValueArrayInitializerContext<'input>;


pub type ElementValueArrayInitializerContext<'input> = BaseParserRuleContext<'input,ElementValueArrayInitializerContextExt<'input>>;

#[derive(Clone)]
pub struct ElementValueArrayInitializerContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ElementValueArrayInitializerContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ElementValueArrayInitializerContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_elementValueArrayInitializer(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_elementValueArrayInitializer(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ElementValueArrayInitializerContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_elementValueArrayInitializer }
	//fn type_rule_index() -> usize where Self: Sized { RULE_elementValueArrayInitializer }
}
antlr_rust::tid!{ElementValueArrayInitializerContextExt<'a>}

impl<'input> ElementValueArrayInitializerContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ElementValueArrayInitializerContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ElementValueArrayInitializerContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ElementValueArrayInitializerContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ElementValueArrayInitializerContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn elementValue_all(&self) ->  Vec<Rc<ElementValueContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn elementValue(&self, i: usize) -> Option<Rc<ElementValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ElementValueArrayInitializerContextAttrs<'input> for ElementValueArrayInitializerContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn elementValueArrayInitializer(&mut self,)
	-> Result<Rc<ElementValueArrayInitializerContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ElementValueArrayInitializerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_elementValueArrayInitializer);
        let mut _localctx: Rc<ElementValueArrayInitializerContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(867);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(876);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << DOUBLE) | (1usize << FLOAT) | (1usize << INT) | (1usize << LONG) | (1usize << NEW))) != 0) || ((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (SHORT - 37)) | (1usize << (SUPER - 37)) | (1usize << (SWITCH - 37)) | (1usize << (THIS - 37)) | (1usize << (VOID - 37)) | (1usize << (MODULE - 37)) | (1usize << (OPEN - 37)) | (1usize << (REQUIRES - 37)) | (1usize << (EXPORTS - 37)) | (1usize << (OPENS - 37)) | (1usize << (TO - 37)) | (1usize << (USES - 37)) | (1usize << (PROVIDES - 37)) | (1usize << (WITH - 37)) | (1usize << (TRANSITIVE - 37)) | (1usize << (VAR - 37)) | (1usize << (YIELD - 37)) | (1usize << (RECORD - 37)) | (1usize << (SEALED - 37)) | (1usize << (PERMITS - 37)) | (1usize << (DECIMAL_LITERAL - 37)) | (1usize << (HEX_LITERAL - 37)))) != 0) || ((((_la - 69)) & !0x3f) == 0 && ((1usize << (_la - 69)) & ((1usize << (OCT_LITERAL - 69)) | (1usize << (BINARY_LITERAL - 69)) | (1usize << (FLOAT_LITERAL - 69)) | (1usize << (HEX_FLOAT_LITERAL - 69)) | (1usize << (BOOL_LITERAL - 69)) | (1usize << (CHAR_LITERAL - 69)) | (1usize << (STRING_LITERAL - 69)) | (1usize << (TEXT_BLOCK - 69)) | (1usize << (NULL_LITERAL - 69)) | (1usize << (LPAREN - 69)) | (1usize << (LBRACE - 69)) | (1usize << (LT - 69)) | (1usize << (BANG - 69)) | (1usize << (TILDE - 69)) | (1usize << (INC - 69)))) != 0) || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (DEC - 101)) | (1usize << (ADD - 101)) | (1usize << (SUB - 101)) | (1usize << (AT - 101)) | (1usize << (IDENTIFIER - 101)))) != 0) {
				{
				/*InvokeRule elementValue*/
				recog.base.set_state(868);
				recog.elementValue()?;

				recog.base.set_state(873);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(95,&mut recog.base)?;
				while { _alt!=2 && _alt!=INVALID_ALT } {
					if _alt==1 {
						{
						{
						recog.base.set_state(869);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule elementValue*/
						recog.base.set_state(870);
						recog.elementValue()?;

						}
						} 
					}
					recog.base.set_state(875);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(95,&mut recog.base)?;
				}
				}
			}

			recog.base.set_state(879);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				recog.base.set_state(878);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(881);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- annotationTypeDeclaration ----------------
pub type AnnotationTypeDeclarationContextAll<'input> = AnnotationTypeDeclarationContext<'input>;


pub type AnnotationTypeDeclarationContext<'input> = BaseParserRuleContext<'input,AnnotationTypeDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct AnnotationTypeDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for AnnotationTypeDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for AnnotationTypeDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_annotationTypeDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_annotationTypeDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AnnotationTypeDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotationTypeDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotationTypeDeclaration }
}
antlr_rust::tid!{AnnotationTypeDeclarationContextExt<'a>}

impl<'input> AnnotationTypeDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnnotationTypeDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnnotationTypeDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AnnotationTypeDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<AnnotationTypeDeclarationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token AT
/// Returns `None` if there is no child corresponding to token AT
fn AT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(AT, 0)
}
/// Retrieves first TerminalNode corresponding to token INTERFACE
/// Returns `None` if there is no child corresponding to token INTERFACE
fn INTERFACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(INTERFACE, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotationTypeBody(&self) -> Option<Rc<AnnotationTypeBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AnnotationTypeDeclarationContextAttrs<'input> for AnnotationTypeDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn annotationTypeDeclaration(&mut self,)
	-> Result<Rc<AnnotationTypeDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnnotationTypeDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_annotationTypeDeclaration);
        let mut _localctx: Rc<AnnotationTypeDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(883);
			recog.base.match_token(AT,&mut recog.err_handler)?;

			recog.base.set_state(884);
			recog.base.match_token(INTERFACE,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(885);
			recog.identifier()?;

			/*InvokeRule annotationTypeBody*/
			recog.base.set_state(886);
			recog.annotationTypeBody()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- annotationTypeBody ----------------
pub type AnnotationTypeBodyContextAll<'input> = AnnotationTypeBodyContext<'input>;


pub type AnnotationTypeBodyContext<'input> = BaseParserRuleContext<'input,AnnotationTypeBodyContextExt<'input>>;

#[derive(Clone)]
pub struct AnnotationTypeBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for AnnotationTypeBodyContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for AnnotationTypeBodyContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_annotationTypeBody(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_annotationTypeBody(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AnnotationTypeBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotationTypeBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotationTypeBody }
}
antlr_rust::tid!{AnnotationTypeBodyContextExt<'a>}

impl<'input> AnnotationTypeBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnnotationTypeBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnnotationTypeBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AnnotationTypeBodyContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<AnnotationTypeBodyContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn annotationTypeElementDeclaration_all(&self) ->  Vec<Rc<AnnotationTypeElementDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotationTypeElementDeclaration(&self, i: usize) -> Option<Rc<AnnotationTypeElementDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> AnnotationTypeBodyContextAttrs<'input> for AnnotationTypeBodyContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn annotationTypeBody(&mut self,)
	-> Result<Rc<AnnotationTypeBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnnotationTypeBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_annotationTypeBody);
        let mut _localctx: Rc<AnnotationTypeBodyContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(888);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(892);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ABSTRACT) | (1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << CLASS) | (1usize << DOUBLE) | (1usize << ENUM) | (1usize << FINAL) | (1usize << FLOAT) | (1usize << INT) | (1usize << INTERFACE) | (1usize << LONG) | (1usize << NATIVE))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (PRIVATE - 33)) | (1usize << (PROTECTED - 33)) | (1usize << (PUBLIC - 33)) | (1usize << (SHORT - 33)) | (1usize << (STATIC - 33)) | (1usize << (STRICTFP - 33)) | (1usize << (SYNCHRONIZED - 33)) | (1usize << (TRANSIENT - 33)) | (1usize << (VOLATILE - 33)) | (1usize << (MODULE - 33)) | (1usize << (OPEN - 33)) | (1usize << (REQUIRES - 33)) | (1usize << (EXPORTS - 33)) | (1usize << (OPENS - 33)) | (1usize << (TO - 33)) | (1usize << (USES - 33)) | (1usize << (PROVIDES - 33)) | (1usize << (WITH - 33)) | (1usize << (TRANSITIVE - 33)) | (1usize << (VAR - 33)) | (1usize << (YIELD - 33)) | (1usize << (RECORD - 33)) | (1usize << (SEALED - 33)))) != 0) || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (PERMITS - 65)) | (1usize << (NON_SEALED - 65)) | (1usize << (SEMI - 65)))) != 0) || _la==AT || _la==IDENTIFIER {
				{
				{
				/*InvokeRule annotationTypeElementDeclaration*/
				recog.base.set_state(889);
				recog.annotationTypeElementDeclaration()?;

				}
				}
				recog.base.set_state(894);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(895);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- annotationTypeElementDeclaration ----------------
pub type AnnotationTypeElementDeclarationContextAll<'input> = AnnotationTypeElementDeclarationContext<'input>;


pub type AnnotationTypeElementDeclarationContext<'input> = BaseParserRuleContext<'input,AnnotationTypeElementDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct AnnotationTypeElementDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for AnnotationTypeElementDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for AnnotationTypeElementDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_annotationTypeElementDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_annotationTypeElementDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AnnotationTypeElementDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotationTypeElementDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotationTypeElementDeclaration }
}
antlr_rust::tid!{AnnotationTypeElementDeclarationContextExt<'a>}

impl<'input> AnnotationTypeElementDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnnotationTypeElementDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnnotationTypeElementDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AnnotationTypeElementDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<AnnotationTypeElementDeclarationContextExt<'input>>{

fn annotationTypeElementRest(&self) -> Option<Rc<AnnotationTypeElementRestContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn modifier_all(&self) ->  Vec<Rc<ModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn modifier(&self, i: usize) -> Option<Rc<ModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}

}

impl<'input> AnnotationTypeElementDeclarationContextAttrs<'input> for AnnotationTypeElementDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn annotationTypeElementDeclaration(&mut self,)
	-> Result<Rc<AnnotationTypeElementDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnnotationTypeElementDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_annotationTypeElementDeclaration);
        let mut _localctx: Rc<AnnotationTypeElementDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(911);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ABSTRACT | BOOLEAN | BYTE | CHAR | CLASS | DOUBLE | ENUM | FINAL | FLOAT |
			 INT | INTERFACE | LONG | NATIVE | PRIVATE | PROTECTED | PUBLIC | SHORT |
			 STATIC | STRICTFP | SYNCHRONIZED | TRANSIENT | VOLATILE | MODULE | OPEN |
			 REQUIRES | EXPORTS | OPENS | TO | USES | PROVIDES | WITH | TRANSITIVE |
			 VAR | YIELD | RECORD | SEALED | PERMITS | NON_SEALED | AT | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(900);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(99,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule annotation*/
							recog.base.set_state(897);
							recog.annotation()?;

							}
							} 
						}
						recog.base.set_state(902);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(99,&mut recog.base)?;
					}
					recog.base.set_state(906);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(100,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule modifier*/
							recog.base.set_state(903);
							recog.modifier()?;

							}
							} 
						}
						recog.base.set_state(908);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(100,&mut recog.base)?;
					}
					/*InvokeRule annotationTypeElementRest*/
					recog.base.set_state(909);
					recog.annotationTypeElementRest()?;

					}
				}

			 SEMI 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(910);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- annotationTypeElementRest ----------------
pub type AnnotationTypeElementRestContextAll<'input> = AnnotationTypeElementRestContext<'input>;


pub type AnnotationTypeElementRestContext<'input> = BaseParserRuleContext<'input,AnnotationTypeElementRestContextExt<'input>>;

#[derive(Clone)]
pub struct AnnotationTypeElementRestContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for AnnotationTypeElementRestContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for AnnotationTypeElementRestContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_annotationTypeElementRest(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_annotationTypeElementRest(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AnnotationTypeElementRestContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotationTypeElementRest }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotationTypeElementRest }
}
antlr_rust::tid!{AnnotationTypeElementRestContextExt<'a>}

impl<'input> AnnotationTypeElementRestContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnnotationTypeElementRestContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnnotationTypeElementRestContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AnnotationTypeElementRestContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<AnnotationTypeElementRestContextExt<'input>>{

fn typeType(&self) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotationMethodOrConstantRest(&self) -> Option<Rc<AnnotationMethodOrConstantRestContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
fn classDeclaration(&self) -> Option<Rc<ClassDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn interfaceDeclaration(&self) -> Option<Rc<InterfaceDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumDeclaration(&self) -> Option<Rc<EnumDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotationTypeDeclaration(&self) -> Option<Rc<AnnotationTypeDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn recordDeclaration(&self) -> Option<Rc<RecordDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AnnotationTypeElementRestContextAttrs<'input> for AnnotationTypeElementRestContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn annotationTypeElementRest(&mut self,)
	-> Result<Rc<AnnotationTypeElementRestContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnnotationTypeElementRestContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_annotationTypeElementRest);
        let mut _localctx: Rc<AnnotationTypeElementRestContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(937);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(107,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule typeType*/
					recog.base.set_state(913);
					recog.typeType()?;

					/*InvokeRule annotationMethodOrConstantRest*/
					recog.base.set_state(914);
					recog.annotationMethodOrConstantRest()?;

					recog.base.set_state(915);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule classDeclaration*/
					recog.base.set_state(917);
					recog.classDeclaration()?;

					recog.base.set_state(919);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(102,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(918);
							recog.base.match_token(SEMI,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule interfaceDeclaration*/
					recog.base.set_state(921);
					recog.interfaceDeclaration()?;

					recog.base.set_state(923);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(103,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(922);
							recog.base.match_token(SEMI,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule enumDeclaration*/
					recog.base.set_state(925);
					recog.enumDeclaration()?;

					recog.base.set_state(927);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(104,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(926);
							recog.base.match_token(SEMI,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule annotationTypeDeclaration*/
					recog.base.set_state(929);
					recog.annotationTypeDeclaration()?;

					recog.base.set_state(931);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(105,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(930);
							recog.base.match_token(SEMI,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule recordDeclaration*/
					recog.base.set_state(933);
					recog.recordDeclaration()?;

					recog.base.set_state(935);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(106,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(934);
							recog.base.match_token(SEMI,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- annotationMethodOrConstantRest ----------------
pub type AnnotationMethodOrConstantRestContextAll<'input> = AnnotationMethodOrConstantRestContext<'input>;


pub type AnnotationMethodOrConstantRestContext<'input> = BaseParserRuleContext<'input,AnnotationMethodOrConstantRestContextExt<'input>>;

#[derive(Clone)]
pub struct AnnotationMethodOrConstantRestContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for AnnotationMethodOrConstantRestContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for AnnotationMethodOrConstantRestContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_annotationMethodOrConstantRest(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_annotationMethodOrConstantRest(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AnnotationMethodOrConstantRestContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotationMethodOrConstantRest }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotationMethodOrConstantRest }
}
antlr_rust::tid!{AnnotationMethodOrConstantRestContextExt<'a>}

impl<'input> AnnotationMethodOrConstantRestContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnnotationMethodOrConstantRestContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnnotationMethodOrConstantRestContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AnnotationMethodOrConstantRestContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<AnnotationMethodOrConstantRestContextExt<'input>>{

fn annotationMethodRest(&self) -> Option<Rc<AnnotationMethodRestContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotationConstantRest(&self) -> Option<Rc<AnnotationConstantRestContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AnnotationMethodOrConstantRestContextAttrs<'input> for AnnotationMethodOrConstantRestContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn annotationMethodOrConstantRest(&mut self,)
	-> Result<Rc<AnnotationMethodOrConstantRestContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnnotationMethodOrConstantRestContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_annotationMethodOrConstantRest);
        let mut _localctx: Rc<AnnotationMethodOrConstantRestContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(941);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(108,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule annotationMethodRest*/
					recog.base.set_state(939);
					recog.annotationMethodRest()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule annotationConstantRest*/
					recog.base.set_state(940);
					recog.annotationConstantRest()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- annotationMethodRest ----------------
pub type AnnotationMethodRestContextAll<'input> = AnnotationMethodRestContext<'input>;


pub type AnnotationMethodRestContext<'input> = BaseParserRuleContext<'input,AnnotationMethodRestContextExt<'input>>;

#[derive(Clone)]
pub struct AnnotationMethodRestContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for AnnotationMethodRestContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for AnnotationMethodRestContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_annotationMethodRest(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_annotationMethodRest(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AnnotationMethodRestContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotationMethodRest }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotationMethodRest }
}
antlr_rust::tid!{AnnotationMethodRestContextExt<'a>}

impl<'input> AnnotationMethodRestContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnnotationMethodRestContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnnotationMethodRestContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AnnotationMethodRestContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<AnnotationMethodRestContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn defaultValue(&self) -> Option<Rc<DefaultValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AnnotationMethodRestContextAttrs<'input> for AnnotationMethodRestContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn annotationMethodRest(&mut self,)
	-> Result<Rc<AnnotationMethodRestContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnnotationMethodRestContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_annotationMethodRest);
        let mut _localctx: Rc<AnnotationMethodRestContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(943);
			recog.identifier()?;

			recog.base.set_state(944);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(945);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			recog.base.set_state(947);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==DEFAULT {
				{
				/*InvokeRule defaultValue*/
				recog.base.set_state(946);
				recog.defaultValue()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- annotationConstantRest ----------------
pub type AnnotationConstantRestContextAll<'input> = AnnotationConstantRestContext<'input>;


pub type AnnotationConstantRestContext<'input> = BaseParserRuleContext<'input,AnnotationConstantRestContextExt<'input>>;

#[derive(Clone)]
pub struct AnnotationConstantRestContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for AnnotationConstantRestContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for AnnotationConstantRestContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_annotationConstantRest(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_annotationConstantRest(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AnnotationConstantRestContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotationConstantRest }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotationConstantRest }
}
antlr_rust::tid!{AnnotationConstantRestContextExt<'a>}

impl<'input> AnnotationConstantRestContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnnotationConstantRestContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnnotationConstantRestContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AnnotationConstantRestContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<AnnotationConstantRestContextExt<'input>>{

fn variableDeclarators(&self) -> Option<Rc<VariableDeclaratorsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AnnotationConstantRestContextAttrs<'input> for AnnotationConstantRestContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn annotationConstantRest(&mut self,)
	-> Result<Rc<AnnotationConstantRestContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnnotationConstantRestContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_annotationConstantRest);
        let mut _localctx: Rc<AnnotationConstantRestContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule variableDeclarators*/
			recog.base.set_state(949);
			recog.variableDeclarators()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- defaultValue ----------------
pub type DefaultValueContextAll<'input> = DefaultValueContext<'input>;


pub type DefaultValueContext<'input> = BaseParserRuleContext<'input,DefaultValueContextExt<'input>>;

#[derive(Clone)]
pub struct DefaultValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for DefaultValueContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for DefaultValueContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_defaultValue(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_defaultValue(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for DefaultValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_defaultValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_defaultValue }
}
antlr_rust::tid!{DefaultValueContextExt<'a>}

impl<'input> DefaultValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DefaultValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DefaultValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DefaultValueContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<DefaultValueContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DEFAULT
/// Returns `None` if there is no child corresponding to token DEFAULT
fn DEFAULT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DEFAULT, 0)
}
fn elementValue(&self) -> Option<Rc<ElementValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DefaultValueContextAttrs<'input> for DefaultValueContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn defaultValue(&mut self,)
	-> Result<Rc<DefaultValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DefaultValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_defaultValue);
        let mut _localctx: Rc<DefaultValueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(951);
			recog.base.match_token(DEFAULT,&mut recog.err_handler)?;

			/*InvokeRule elementValue*/
			recog.base.set_state(952);
			recog.elementValue()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- moduleDeclaration ----------------
pub type ModuleDeclarationContextAll<'input> = ModuleDeclarationContext<'input>;


pub type ModuleDeclarationContext<'input> = BaseParserRuleContext<'input,ModuleDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ModuleDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ModuleDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ModuleDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_moduleDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_moduleDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ModuleDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_moduleDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_moduleDeclaration }
}
antlr_rust::tid!{ModuleDeclarationContextExt<'a>}

impl<'input> ModuleDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModuleDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModuleDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModuleDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ModuleDeclarationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MODULE
/// Returns `None` if there is no child corresponding to token MODULE
fn MODULE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(MODULE, 0)
}
fn qualifiedName(&self) -> Option<Rc<QualifiedNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn moduleBody(&self) -> Option<Rc<ModuleBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OPEN
/// Returns `None` if there is no child corresponding to token OPEN
fn OPEN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(OPEN, 0)
}

}

impl<'input> ModuleDeclarationContextAttrs<'input> for ModuleDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn moduleDeclaration(&mut self,)
	-> Result<Rc<ModuleDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModuleDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_moduleDeclaration);
        let mut _localctx: Rc<ModuleDeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(955);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==OPEN {
				{
				recog.base.set_state(954);
				recog.base.match_token(OPEN,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(957);
			recog.base.match_token(MODULE,&mut recog.err_handler)?;

			/*InvokeRule qualifiedName*/
			recog.base.set_state(958);
			recog.qualifiedName()?;

			/*InvokeRule moduleBody*/
			recog.base.set_state(959);
			recog.moduleBody()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- moduleBody ----------------
pub type ModuleBodyContextAll<'input> = ModuleBodyContext<'input>;


pub type ModuleBodyContext<'input> = BaseParserRuleContext<'input,ModuleBodyContextExt<'input>>;

#[derive(Clone)]
pub struct ModuleBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ModuleBodyContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ModuleBodyContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_moduleBody(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_moduleBody(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ModuleBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_moduleBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_moduleBody }
}
antlr_rust::tid!{ModuleBodyContextExt<'a>}

impl<'input> ModuleBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModuleBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModuleBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModuleBodyContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ModuleBodyContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn moduleDirective_all(&self) ->  Vec<Rc<ModuleDirectiveContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn moduleDirective(&self, i: usize) -> Option<Rc<ModuleDirectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ModuleBodyContextAttrs<'input> for ModuleBodyContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn moduleBody(&mut self,)
	-> Result<Rc<ModuleBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModuleBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_moduleBody);
        let mut _localctx: Rc<ModuleBodyContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(961);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(965);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & ((1usize << (REQUIRES - 53)) | (1usize << (EXPORTS - 53)) | (1usize << (OPENS - 53)) | (1usize << (USES - 53)) | (1usize << (PROVIDES - 53)))) != 0) {
				{
				{
				/*InvokeRule moduleDirective*/
				recog.base.set_state(962);
				recog.moduleDirective()?;

				}
				}
				recog.base.set_state(967);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(968);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- moduleDirective ----------------
pub type ModuleDirectiveContextAll<'input> = ModuleDirectiveContext<'input>;


pub type ModuleDirectiveContext<'input> = BaseParserRuleContext<'input,ModuleDirectiveContextExt<'input>>;

#[derive(Clone)]
pub struct ModuleDirectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ModuleDirectiveContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ModuleDirectiveContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_moduleDirective(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_moduleDirective(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ModuleDirectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_moduleDirective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_moduleDirective }
}
antlr_rust::tid!{ModuleDirectiveContextExt<'a>}

impl<'input> ModuleDirectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModuleDirectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModuleDirectiveContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModuleDirectiveContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ModuleDirectiveContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token REQUIRES
/// Returns `None` if there is no child corresponding to token REQUIRES
fn REQUIRES(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(REQUIRES, 0)
}
fn qualifiedName_all(&self) ->  Vec<Rc<QualifiedNameContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn qualifiedName(&self, i: usize) -> Option<Rc<QualifiedNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
fn requiresModifier_all(&self) ->  Vec<Rc<RequiresModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn requiresModifier(&self, i: usize) -> Option<Rc<RequiresModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token EXPORTS
/// Returns `None` if there is no child corresponding to token EXPORTS
fn EXPORTS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(EXPORTS, 0)
}
/// Retrieves first TerminalNode corresponding to token TO
/// Returns `None` if there is no child corresponding to token TO
fn TO(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(TO, 0)
}
/// Retrieves first TerminalNode corresponding to token OPENS
/// Returns `None` if there is no child corresponding to token OPENS
fn OPENS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(OPENS, 0)
}
/// Retrieves first TerminalNode corresponding to token USES
/// Returns `None` if there is no child corresponding to token USES
fn USES(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(USES, 0)
}
/// Retrieves first TerminalNode corresponding to token PROVIDES
/// Returns `None` if there is no child corresponding to token PROVIDES
fn PROVIDES(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(PROVIDES, 0)
}
/// Retrieves first TerminalNode corresponding to token WITH
/// Returns `None` if there is no child corresponding to token WITH
fn WITH(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(WITH, 0)
}

}

impl<'input> ModuleDirectiveContextAttrs<'input> for ModuleDirectiveContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn moduleDirective(&mut self,)
	-> Result<Rc<ModuleDirectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModuleDirectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 140, RULE_moduleDirective);
        let mut _localctx: Rc<ModuleDirectiveContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(1006);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 REQUIRES 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(970);
					recog.base.match_token(REQUIRES,&mut recog.err_handler)?;

					recog.base.set_state(974);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(112,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule requiresModifier*/
							recog.base.set_state(971);
							recog.requiresModifier()?;

							}
							} 
						}
						recog.base.set_state(976);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(112,&mut recog.base)?;
					}
					/*InvokeRule qualifiedName*/
					recog.base.set_state(977);
					recog.qualifiedName()?;

					recog.base.set_state(978);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}

			 EXPORTS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(980);
					recog.base.match_token(EXPORTS,&mut recog.err_handler)?;

					/*InvokeRule qualifiedName*/
					recog.base.set_state(981);
					recog.qualifiedName()?;

					recog.base.set_state(984);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==TO {
						{
						recog.base.set_state(982);
						recog.base.match_token(TO,&mut recog.err_handler)?;

						/*InvokeRule qualifiedName*/
						recog.base.set_state(983);
						recog.qualifiedName()?;

						}
					}

					recog.base.set_state(986);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}

			 OPENS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(988);
					recog.base.match_token(OPENS,&mut recog.err_handler)?;

					/*InvokeRule qualifiedName*/
					recog.base.set_state(989);
					recog.qualifiedName()?;

					recog.base.set_state(992);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==TO {
						{
						recog.base.set_state(990);
						recog.base.match_token(TO,&mut recog.err_handler)?;

						/*InvokeRule qualifiedName*/
						recog.base.set_state(991);
						recog.qualifiedName()?;

						}
					}

					recog.base.set_state(994);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}

			 USES 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(996);
					recog.base.match_token(USES,&mut recog.err_handler)?;

					/*InvokeRule qualifiedName*/
					recog.base.set_state(997);
					recog.qualifiedName()?;

					recog.base.set_state(998);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}

			 PROVIDES 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(1000);
					recog.base.match_token(PROVIDES,&mut recog.err_handler)?;

					/*InvokeRule qualifiedName*/
					recog.base.set_state(1001);
					recog.qualifiedName()?;

					recog.base.set_state(1002);
					recog.base.match_token(WITH,&mut recog.err_handler)?;

					/*InvokeRule qualifiedName*/
					recog.base.set_state(1003);
					recog.qualifiedName()?;

					recog.base.set_state(1004);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- requiresModifier ----------------
pub type RequiresModifierContextAll<'input> = RequiresModifierContext<'input>;


pub type RequiresModifierContext<'input> = BaseParserRuleContext<'input,RequiresModifierContextExt<'input>>;

#[derive(Clone)]
pub struct RequiresModifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for RequiresModifierContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for RequiresModifierContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_requiresModifier(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_requiresModifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for RequiresModifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_requiresModifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_requiresModifier }
}
antlr_rust::tid!{RequiresModifierContextExt<'a>}

impl<'input> RequiresModifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RequiresModifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RequiresModifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RequiresModifierContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<RequiresModifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TRANSITIVE
/// Returns `None` if there is no child corresponding to token TRANSITIVE
fn TRANSITIVE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(TRANSITIVE, 0)
}
/// Retrieves first TerminalNode corresponding to token STATIC
/// Returns `None` if there is no child corresponding to token STATIC
fn STATIC(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(STATIC, 0)
}

}

impl<'input> RequiresModifierContextAttrs<'input> for RequiresModifierContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn requiresModifier(&mut self,)
	-> Result<Rc<RequiresModifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RequiresModifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_requiresModifier);
        let mut _localctx: Rc<RequiresModifierContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1008);
			_la = recog.base.input.la(1);
			if { !(_la==STATIC || _la==TRANSITIVE) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- recordDeclaration ----------------
pub type RecordDeclarationContextAll<'input> = RecordDeclarationContext<'input>;


pub type RecordDeclarationContext<'input> = BaseParserRuleContext<'input,RecordDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct RecordDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for RecordDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for RecordDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_recordDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_recordDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for RecordDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_recordDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_recordDeclaration }
}
antlr_rust::tid!{RecordDeclarationContextExt<'a>}

impl<'input> RecordDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RecordDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RecordDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RecordDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<RecordDeclarationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RECORD
/// Returns `None` if there is no child corresponding to token RECORD
fn RECORD(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RECORD, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn recordHeader(&self) -> Option<Rc<RecordHeaderContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn recordBody(&self) -> Option<Rc<RecordBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeParameters(&self) -> Option<Rc<TypeParametersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IMPLEMENTS
/// Returns `None` if there is no child corresponding to token IMPLEMENTS
fn IMPLEMENTS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(IMPLEMENTS, 0)
}
fn typeList(&self) -> Option<Rc<TypeListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RecordDeclarationContextAttrs<'input> for RecordDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn recordDeclaration(&mut self,)
	-> Result<Rc<RecordDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RecordDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 144, RULE_recordDeclaration);
        let mut _localctx: Rc<RecordDeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1010);
			recog.base.match_token(RECORD,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(1011);
			recog.identifier()?;

			recog.base.set_state(1013);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LT {
				{
				/*InvokeRule typeParameters*/
				recog.base.set_state(1012);
				recog.typeParameters()?;

				}
			}

			/*InvokeRule recordHeader*/
			recog.base.set_state(1015);
			recog.recordHeader()?;

			recog.base.set_state(1018);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IMPLEMENTS {
				{
				recog.base.set_state(1016);
				recog.base.match_token(IMPLEMENTS,&mut recog.err_handler)?;

				/*InvokeRule typeList*/
				recog.base.set_state(1017);
				recog.typeList()?;

				}
			}

			/*InvokeRule recordBody*/
			recog.base.set_state(1020);
			recog.recordBody()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- recordHeader ----------------
pub type RecordHeaderContextAll<'input> = RecordHeaderContext<'input>;


pub type RecordHeaderContext<'input> = BaseParserRuleContext<'input,RecordHeaderContextExt<'input>>;

#[derive(Clone)]
pub struct RecordHeaderContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for RecordHeaderContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for RecordHeaderContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_recordHeader(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_recordHeader(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for RecordHeaderContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_recordHeader }
	//fn type_rule_index() -> usize where Self: Sized { RULE_recordHeader }
}
antlr_rust::tid!{RecordHeaderContextExt<'a>}

impl<'input> RecordHeaderContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RecordHeaderContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RecordHeaderContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RecordHeaderContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<RecordHeaderContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn recordComponentList(&self) -> Option<Rc<RecordComponentListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RecordHeaderContextAttrs<'input> for RecordHeaderContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn recordHeader(&mut self,)
	-> Result<Rc<RecordHeaderContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RecordHeaderContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_recordHeader);
        let mut _localctx: Rc<RecordHeaderContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1022);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(1024);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << DOUBLE) | (1usize << FLOAT) | (1usize << INT) | (1usize << LONG))) != 0) || ((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (SHORT - 37)) | (1usize << (MODULE - 37)) | (1usize << (OPEN - 37)) | (1usize << (REQUIRES - 37)) | (1usize << (EXPORTS - 37)) | (1usize << (OPENS - 37)) | (1usize << (TO - 37)) | (1usize << (USES - 37)) | (1usize << (PROVIDES - 37)) | (1usize << (WITH - 37)) | (1usize << (TRANSITIVE - 37)) | (1usize << (VAR - 37)) | (1usize << (YIELD - 37)) | (1usize << (RECORD - 37)) | (1usize << (SEALED - 37)) | (1usize << (PERMITS - 37)))) != 0) || _la==AT || _la==IDENTIFIER {
				{
				/*InvokeRule recordComponentList*/
				recog.base.set_state(1023);
				recog.recordComponentList()?;

				}
			}

			recog.base.set_state(1026);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- recordComponentList ----------------
pub type RecordComponentListContextAll<'input> = RecordComponentListContext<'input>;


pub type RecordComponentListContext<'input> = BaseParserRuleContext<'input,RecordComponentListContextExt<'input>>;

#[derive(Clone)]
pub struct RecordComponentListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for RecordComponentListContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for RecordComponentListContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_recordComponentList(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_recordComponentList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for RecordComponentListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_recordComponentList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_recordComponentList }
}
antlr_rust::tid!{RecordComponentListContextExt<'a>}

impl<'input> RecordComponentListContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RecordComponentListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RecordComponentListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RecordComponentListContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<RecordComponentListContextExt<'input>>{

fn recordComponent_all(&self) ->  Vec<Rc<RecordComponentContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn recordComponent(&self, i: usize) -> Option<Rc<RecordComponentContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> RecordComponentListContextAttrs<'input> for RecordComponentListContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn recordComponentList(&mut self,)
	-> Result<Rc<RecordComponentListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RecordComponentListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_recordComponentList);
        let mut _localctx: Rc<RecordComponentListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule recordComponent*/
			recog.base.set_state(1028);
			recog.recordComponent()?;

			recog.base.set_state(1033);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(1029);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule recordComponent*/
				recog.base.set_state(1030);
				recog.recordComponent()?;

				}
				}
				recog.base.set_state(1035);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- recordComponent ----------------
pub type RecordComponentContextAll<'input> = RecordComponentContext<'input>;


pub type RecordComponentContext<'input> = BaseParserRuleContext<'input,RecordComponentContextExt<'input>>;

#[derive(Clone)]
pub struct RecordComponentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for RecordComponentContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for RecordComponentContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_recordComponent(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_recordComponent(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for RecordComponentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_recordComponent }
	//fn type_rule_index() -> usize where Self: Sized { RULE_recordComponent }
}
antlr_rust::tid!{RecordComponentContextExt<'a>}

impl<'input> RecordComponentContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RecordComponentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RecordComponentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RecordComponentContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<RecordComponentContextExt<'input>>{

fn typeType(&self) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RecordComponentContextAttrs<'input> for RecordComponentContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn recordComponent(&mut self,)
	-> Result<Rc<RecordComponentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RecordComponentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_recordComponent);
        let mut _localctx: Rc<RecordComponentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typeType*/
			recog.base.set_state(1036);
			recog.typeType()?;

			/*InvokeRule identifier*/
			recog.base.set_state(1037);
			recog.identifier()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- recordBody ----------------
pub type RecordBodyContextAll<'input> = RecordBodyContext<'input>;


pub type RecordBodyContext<'input> = BaseParserRuleContext<'input,RecordBodyContextExt<'input>>;

#[derive(Clone)]
pub struct RecordBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for RecordBodyContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for RecordBodyContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_recordBody(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_recordBody(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for RecordBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_recordBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_recordBody }
}
antlr_rust::tid!{RecordBodyContextExt<'a>}

impl<'input> RecordBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RecordBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RecordBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RecordBodyContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<RecordBodyContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn classBodyDeclaration_all(&self) ->  Vec<Rc<ClassBodyDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn classBodyDeclaration(&self, i: usize) -> Option<Rc<ClassBodyDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn compactConstructorDeclaration_all(&self) ->  Vec<Rc<CompactConstructorDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn compactConstructorDeclaration(&self, i: usize) -> Option<Rc<CompactConstructorDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> RecordBodyContextAttrs<'input> for RecordBodyContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn recordBody(&mut self,)
	-> Result<Rc<RecordBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RecordBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 152, RULE_recordBody);
        let mut _localctx: Rc<RecordBodyContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1039);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(1044);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ABSTRACT) | (1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << CLASS) | (1usize << DOUBLE) | (1usize << ENUM) | (1usize << FINAL) | (1usize << FLOAT) | (1usize << INT) | (1usize << INTERFACE) | (1usize << LONG) | (1usize << NATIVE))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (PRIVATE - 33)) | (1usize << (PROTECTED - 33)) | (1usize << (PUBLIC - 33)) | (1usize << (SHORT - 33)) | (1usize << (STATIC - 33)) | (1usize << (STRICTFP - 33)) | (1usize << (SYNCHRONIZED - 33)) | (1usize << (TRANSIENT - 33)) | (1usize << (VOID - 33)) | (1usize << (VOLATILE - 33)) | (1usize << (MODULE - 33)) | (1usize << (OPEN - 33)) | (1usize << (REQUIRES - 33)) | (1usize << (EXPORTS - 33)) | (1usize << (OPENS - 33)) | (1usize << (TO - 33)) | (1usize << (USES - 33)) | (1usize << (PROVIDES - 33)) | (1usize << (WITH - 33)) | (1usize << (TRANSITIVE - 33)) | (1usize << (VAR - 33)) | (1usize << (YIELD - 33)) | (1usize << (RECORD - 33)) | (1usize << (SEALED - 33)))) != 0) || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (PERMITS - 65)) | (1usize << (NON_SEALED - 65)) | (1usize << (LBRACE - 65)) | (1usize << (SEMI - 65)) | (1usize << (LT - 65)))) != 0) || _la==AT || _la==IDENTIFIER {
				{
				recog.base.set_state(1042);
				recog.err_handler.sync(&mut recog.base)?;
				match  recog.interpreter.adaptive_predict(120,&mut recog.base)? {
					1 =>{
						{
						/*InvokeRule classBodyDeclaration*/
						recog.base.set_state(1040);
						recog.classBodyDeclaration()?;

						}
					}
				,
					2 =>{
						{
						/*InvokeRule compactConstructorDeclaration*/
						recog.base.set_state(1041);
						recog.compactConstructorDeclaration()?;

						}
					}

					_ => {}
				}
				}
				recog.base.set_state(1046);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1047);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- block ----------------
pub type BlockContextAll<'input> = BlockContext<'input>;


pub type BlockContext<'input> = BaseParserRuleContext<'input,BlockContextExt<'input>>;

#[derive(Clone)]
pub struct BlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for BlockContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for BlockContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_block(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_block(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for BlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_block }
	//fn type_rule_index() -> usize where Self: Sized { RULE_block }
}
antlr_rust::tid!{BlockContextExt<'a>}

impl<'input> BlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BlockContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<BlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn blockStatement_all(&self) ->  Vec<Rc<BlockStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn blockStatement(&self, i: usize) -> Option<Rc<BlockStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> BlockContextAttrs<'input> for BlockContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn block(&mut self,)
	-> Result<Rc<BlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 154, RULE_block);
        let mut _localctx: Rc<BlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1049);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(1053);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ABSTRACT) | (1usize << ASSERT) | (1usize << BOOLEAN) | (1usize << BREAK) | (1usize << BYTE) | (1usize << CHAR) | (1usize << CLASS) | (1usize << CONTINUE) | (1usize << DO) | (1usize << DOUBLE) | (1usize << FINAL) | (1usize << FLOAT) | (1usize << FOR) | (1usize << IF) | (1usize << INT) | (1usize << INTERFACE) | (1usize << LONG) | (1usize << NATIVE) | (1usize << NEW))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (PRIVATE - 33)) | (1usize << (PROTECTED - 33)) | (1usize << (PUBLIC - 33)) | (1usize << (RETURN - 33)) | (1usize << (SHORT - 33)) | (1usize << (STATIC - 33)) | (1usize << (STRICTFP - 33)) | (1usize << (SUPER - 33)) | (1usize << (SWITCH - 33)) | (1usize << (SYNCHRONIZED - 33)) | (1usize << (THIS - 33)) | (1usize << (THROW - 33)) | (1usize << (TRANSIENT - 33)) | (1usize << (TRY - 33)) | (1usize << (VOID - 33)) | (1usize << (VOLATILE - 33)) | (1usize << (WHILE - 33)) | (1usize << (MODULE - 33)) | (1usize << (OPEN - 33)) | (1usize << (REQUIRES - 33)) | (1usize << (EXPORTS - 33)) | (1usize << (OPENS - 33)) | (1usize << (TO - 33)) | (1usize << (USES - 33)) | (1usize << (PROVIDES - 33)) | (1usize << (WITH - 33)) | (1usize << (TRANSITIVE - 33)) | (1usize << (VAR - 33)) | (1usize << (YIELD - 33)) | (1usize << (RECORD - 33)) | (1usize << (SEALED - 33)))) != 0) || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (PERMITS - 65)) | (1usize << (NON_SEALED - 65)) | (1usize << (DECIMAL_LITERAL - 65)) | (1usize << (HEX_LITERAL - 65)) | (1usize << (OCT_LITERAL - 65)) | (1usize << (BINARY_LITERAL - 65)) | (1usize << (FLOAT_LITERAL - 65)) | (1usize << (HEX_FLOAT_LITERAL - 65)) | (1usize << (BOOL_LITERAL - 65)) | (1usize << (CHAR_LITERAL - 65)) | (1usize << (STRING_LITERAL - 65)) | (1usize << (TEXT_BLOCK - 65)) | (1usize << (NULL_LITERAL - 65)) | (1usize << (LPAREN - 65)) | (1usize << (LBRACE - 65)) | (1usize << (SEMI - 65)) | (1usize << (LT - 65)) | (1usize << (BANG - 65)) | (1usize << (TILDE - 65)))) != 0) || ((((_la - 100)) & !0x3f) == 0 && ((1usize << (_la - 100)) & ((1usize << (INC - 100)) | (1usize << (DEC - 100)) | (1usize << (ADD - 100)) | (1usize << (SUB - 100)) | (1usize << (AT - 100)) | (1usize << (IDENTIFIER - 100)))) != 0) {
				{
				{
				/*InvokeRule blockStatement*/
				recog.base.set_state(1050);
				recog.blockStatement()?;

				}
				}
				recog.base.set_state(1055);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1056);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- blockStatement ----------------
pub type BlockStatementContextAll<'input> = BlockStatementContext<'input>;


pub type BlockStatementContext<'input> = BaseParserRuleContext<'input,BlockStatementContextExt<'input>>;

#[derive(Clone)]
pub struct BlockStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for BlockStatementContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for BlockStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_blockStatement(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_blockStatement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for BlockStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_blockStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_blockStatement }
}
antlr_rust::tid!{BlockStatementContextExt<'a>}

impl<'input> BlockStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BlockStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlockStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BlockStatementContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<BlockStatementContextExt<'input>>{

fn localVariableDeclaration(&self) -> Option<Rc<LocalVariableDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
fn localTypeDeclaration(&self) -> Option<Rc<LocalTypeDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn statement(&self) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> BlockStatementContextAttrs<'input> for BlockStatementContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn blockStatement(&mut self,)
	-> Result<Rc<BlockStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BlockStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 156, RULE_blockStatement);
        let mut _localctx: Rc<BlockStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1063);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(123,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule localVariableDeclaration*/
					recog.base.set_state(1058);
					recog.localVariableDeclaration()?;

					recog.base.set_state(1059);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule localTypeDeclaration*/
					recog.base.set_state(1061);
					recog.localTypeDeclaration()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule statement*/
					recog.base.set_state(1062);
					recog.statement()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- localVariableDeclaration ----------------
pub type LocalVariableDeclarationContextAll<'input> = LocalVariableDeclarationContext<'input>;


pub type LocalVariableDeclarationContext<'input> = BaseParserRuleContext<'input,LocalVariableDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct LocalVariableDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for LocalVariableDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for LocalVariableDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_localVariableDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_localVariableDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for LocalVariableDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_localVariableDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_localVariableDeclaration }
}
antlr_rust::tid!{LocalVariableDeclarationContextExt<'a>}

impl<'input> LocalVariableDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LocalVariableDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LocalVariableDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LocalVariableDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<LocalVariableDeclarationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token VAR
/// Returns `None` if there is no child corresponding to token VAR
fn VAR(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(VAR, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN
/// Returns `None` if there is no child corresponding to token ASSIGN
fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeType(&self) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableDeclarators(&self) -> Option<Rc<VariableDeclaratorsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableModifier_all(&self) ->  Vec<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variableModifier(&self, i: usize) -> Option<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> LocalVariableDeclarationContextAttrs<'input> for LocalVariableDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn localVariableDeclaration(&mut self,)
	-> Result<Rc<LocalVariableDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LocalVariableDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 158, RULE_localVariableDeclaration);
        let mut _localctx: Rc<LocalVariableDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1068);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(124,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule variableModifier*/
					recog.base.set_state(1065);
					recog.variableModifier()?;

					}
					} 
				}
				recog.base.set_state(1070);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(124,&mut recog.base)?;
			}
			recog.base.set_state(1079);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(125,&mut recog.base)? {
				1 =>{
					{
					recog.base.set_state(1071);
					recog.base.match_token(VAR,&mut recog.err_handler)?;

					/*InvokeRule identifier*/
					recog.base.set_state(1072);
					recog.identifier()?;

					recog.base.set_state(1073);
					recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1074);
					recog.expression_rec(0)?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule typeType*/
					recog.base.set_state(1076);
					recog.typeType()?;

					/*InvokeRule variableDeclarators*/
					recog.base.set_state(1077);
					recog.variableDeclarators()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- identifier ----------------
pub type IdentifierContextAll<'input> = IdentifierContext<'input>;


pub type IdentifierContext<'input> = BaseParserRuleContext<'input,IdentifierContextExt<'input>>;

#[derive(Clone)]
pub struct IdentifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for IdentifierContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for IdentifierContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_identifier(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_identifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for IdentifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifier }
}
antlr_rust::tid!{IdentifierContextExt<'a>}

impl<'input> IdentifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdentifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdentifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdentifierContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<IdentifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token MODULE
/// Returns `None` if there is no child corresponding to token MODULE
fn MODULE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(MODULE, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN
/// Returns `None` if there is no child corresponding to token OPEN
fn OPEN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(OPEN, 0)
}
/// Retrieves first TerminalNode corresponding to token REQUIRES
/// Returns `None` if there is no child corresponding to token REQUIRES
fn REQUIRES(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(REQUIRES, 0)
}
/// Retrieves first TerminalNode corresponding to token EXPORTS
/// Returns `None` if there is no child corresponding to token EXPORTS
fn EXPORTS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(EXPORTS, 0)
}
/// Retrieves first TerminalNode corresponding to token OPENS
/// Returns `None` if there is no child corresponding to token OPENS
fn OPENS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(OPENS, 0)
}
/// Retrieves first TerminalNode corresponding to token TO
/// Returns `None` if there is no child corresponding to token TO
fn TO(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(TO, 0)
}
/// Retrieves first TerminalNode corresponding to token USES
/// Returns `None` if there is no child corresponding to token USES
fn USES(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(USES, 0)
}
/// Retrieves first TerminalNode corresponding to token PROVIDES
/// Returns `None` if there is no child corresponding to token PROVIDES
fn PROVIDES(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(PROVIDES, 0)
}
/// Retrieves first TerminalNode corresponding to token WITH
/// Returns `None` if there is no child corresponding to token WITH
fn WITH(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(WITH, 0)
}
/// Retrieves first TerminalNode corresponding to token TRANSITIVE
/// Returns `None` if there is no child corresponding to token TRANSITIVE
fn TRANSITIVE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(TRANSITIVE, 0)
}
/// Retrieves first TerminalNode corresponding to token YIELD
/// Returns `None` if there is no child corresponding to token YIELD
fn YIELD(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(YIELD, 0)
}
/// Retrieves first TerminalNode corresponding to token SEALED
/// Returns `None` if there is no child corresponding to token SEALED
fn SEALED(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEALED, 0)
}
/// Retrieves first TerminalNode corresponding to token PERMITS
/// Returns `None` if there is no child corresponding to token PERMITS
fn PERMITS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(PERMITS, 0)
}
/// Retrieves first TerminalNode corresponding to token RECORD
/// Returns `None` if there is no child corresponding to token RECORD
fn RECORD(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RECORD, 0)
}
/// Retrieves first TerminalNode corresponding to token VAR
/// Returns `None` if there is no child corresponding to token VAR
fn VAR(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(VAR, 0)
}

}

impl<'input> IdentifierContextAttrs<'input> for IdentifierContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn identifier(&mut self,)
	-> Result<Rc<IdentifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 160, RULE_identifier);
        let mut _localctx: Rc<IdentifierContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1081);
			_la = recog.base.input.la(1);
			if { !(((((_la - 51)) & !0x3f) == 0 && ((1usize << (_la - 51)) & ((1usize << (MODULE - 51)) | (1usize << (OPEN - 51)) | (1usize << (REQUIRES - 51)) | (1usize << (EXPORTS - 51)) | (1usize << (OPENS - 51)) | (1usize << (TO - 51)) | (1usize << (USES - 51)) | (1usize << (PROVIDES - 51)) | (1usize << (WITH - 51)) | (1usize << (TRANSITIVE - 51)) | (1usize << (VAR - 51)) | (1usize << (YIELD - 51)) | (1usize << (RECORD - 51)) | (1usize << (SEALED - 51)) | (1usize << (PERMITS - 51)))) != 0) || _la==IDENTIFIER) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeIdentifier ----------------
pub type TypeIdentifierContextAll<'input> = TypeIdentifierContext<'input>;


pub type TypeIdentifierContext<'input> = BaseParserRuleContext<'input,TypeIdentifierContextExt<'input>>;

#[derive(Clone)]
pub struct TypeIdentifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for TypeIdentifierContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for TypeIdentifierContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_typeIdentifier(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_typeIdentifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TypeIdentifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeIdentifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeIdentifier }
}
antlr_rust::tid!{TypeIdentifierContextExt<'a>}

impl<'input> TypeIdentifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeIdentifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeIdentifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeIdentifierContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<TypeIdentifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token MODULE
/// Returns `None` if there is no child corresponding to token MODULE
fn MODULE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(MODULE, 0)
}
/// Retrieves first TerminalNode corresponding to token OPEN
/// Returns `None` if there is no child corresponding to token OPEN
fn OPEN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(OPEN, 0)
}
/// Retrieves first TerminalNode corresponding to token REQUIRES
/// Returns `None` if there is no child corresponding to token REQUIRES
fn REQUIRES(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(REQUIRES, 0)
}
/// Retrieves first TerminalNode corresponding to token EXPORTS
/// Returns `None` if there is no child corresponding to token EXPORTS
fn EXPORTS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(EXPORTS, 0)
}
/// Retrieves first TerminalNode corresponding to token OPENS
/// Returns `None` if there is no child corresponding to token OPENS
fn OPENS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(OPENS, 0)
}
/// Retrieves first TerminalNode corresponding to token TO
/// Returns `None` if there is no child corresponding to token TO
fn TO(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(TO, 0)
}
/// Retrieves first TerminalNode corresponding to token USES
/// Returns `None` if there is no child corresponding to token USES
fn USES(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(USES, 0)
}
/// Retrieves first TerminalNode corresponding to token PROVIDES
/// Returns `None` if there is no child corresponding to token PROVIDES
fn PROVIDES(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(PROVIDES, 0)
}
/// Retrieves first TerminalNode corresponding to token WITH
/// Returns `None` if there is no child corresponding to token WITH
fn WITH(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(WITH, 0)
}
/// Retrieves first TerminalNode corresponding to token TRANSITIVE
/// Returns `None` if there is no child corresponding to token TRANSITIVE
fn TRANSITIVE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(TRANSITIVE, 0)
}
/// Retrieves first TerminalNode corresponding to token SEALED
/// Returns `None` if there is no child corresponding to token SEALED
fn SEALED(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEALED, 0)
}
/// Retrieves first TerminalNode corresponding to token PERMITS
/// Returns `None` if there is no child corresponding to token PERMITS
fn PERMITS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(PERMITS, 0)
}
/// Retrieves first TerminalNode corresponding to token RECORD
/// Returns `None` if there is no child corresponding to token RECORD
fn RECORD(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RECORD, 0)
}

}

impl<'input> TypeIdentifierContextAttrs<'input> for TypeIdentifierContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeIdentifier(&mut self,)
	-> Result<Rc<TypeIdentifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeIdentifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 162, RULE_typeIdentifier);
        let mut _localctx: Rc<TypeIdentifierContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1083);
			_la = recog.base.input.la(1);
			if { !(((((_la - 51)) & !0x3f) == 0 && ((1usize << (_la - 51)) & ((1usize << (MODULE - 51)) | (1usize << (OPEN - 51)) | (1usize << (REQUIRES - 51)) | (1usize << (EXPORTS - 51)) | (1usize << (OPENS - 51)) | (1usize << (TO - 51)) | (1usize << (USES - 51)) | (1usize << (PROVIDES - 51)) | (1usize << (WITH - 51)) | (1usize << (TRANSITIVE - 51)) | (1usize << (RECORD - 51)) | (1usize << (SEALED - 51)) | (1usize << (PERMITS - 51)))) != 0) || _la==IDENTIFIER) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- localTypeDeclaration ----------------
pub type LocalTypeDeclarationContextAll<'input> = LocalTypeDeclarationContext<'input>;


pub type LocalTypeDeclarationContext<'input> = BaseParserRuleContext<'input,LocalTypeDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct LocalTypeDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for LocalTypeDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for LocalTypeDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_localTypeDeclaration(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_localTypeDeclaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for LocalTypeDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_localTypeDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_localTypeDeclaration }
}
antlr_rust::tid!{LocalTypeDeclarationContextExt<'a>}

impl<'input> LocalTypeDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LocalTypeDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LocalTypeDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LocalTypeDeclarationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<LocalTypeDeclarationContextExt<'input>>{

fn classDeclaration(&self) -> Option<Rc<ClassDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn interfaceDeclaration(&self) -> Option<Rc<InterfaceDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn recordDeclaration(&self) -> Option<Rc<RecordDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn modifier_all(&self) ->  Vec<Rc<ModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn modifier(&self, i: usize) -> Option<Rc<ModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> LocalTypeDeclarationContextAttrs<'input> for LocalTypeDeclarationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn localTypeDeclaration(&mut self,)
	-> Result<Rc<LocalTypeDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LocalTypeDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 164, RULE_localTypeDeclaration);
        let mut _localctx: Rc<LocalTypeDeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1088);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(126,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule annotation*/
					recog.base.set_state(1085);
					recog.annotation()?;

					}
					} 
				}
				recog.base.set_state(1090);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(126,&mut recog.base)?;
			}
			recog.base.set_state(1094);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ABSTRACT) | (1usize << FINAL) | (1usize << NATIVE))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (PRIVATE - 33)) | (1usize << (PROTECTED - 33)) | (1usize << (PUBLIC - 33)) | (1usize << (STATIC - 33)) | (1usize << (STRICTFP - 33)) | (1usize << (SYNCHRONIZED - 33)) | (1usize << (TRANSIENT - 33)) | (1usize << (VOLATILE - 33)) | (1usize << (SEALED - 33)))) != 0) || _la==NON_SEALED {
				{
				{
				/*InvokeRule modifier*/
				recog.base.set_state(1091);
				recog.modifier()?;

				}
				}
				recog.base.set_state(1096);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1100);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 CLASS 
				=> {
					{
					/*InvokeRule classDeclaration*/
					recog.base.set_state(1097);
					recog.classDeclaration()?;

					}
				}

			 INTERFACE 
				=> {
					{
					/*InvokeRule interfaceDeclaration*/
					recog.base.set_state(1098);
					recog.interfaceDeclaration()?;

					}
				}

			 RECORD 
				=> {
					{
					/*InvokeRule recordDeclaration*/
					recog.base.set_state(1099);
					recog.recordDeclaration()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;


pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
	pub blockLabel: Option<Rc<BlockContextAll<'input>>>,
	pub statementExpression: Option<Rc<ExpressionContextAll<'input>>>,
	pub identifierLabel: Option<Rc<IdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for StatementContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_statement(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_statement(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::tid!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{
				blockLabel: None, statementExpression: None, identifierLabel: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<StatementContextExt<'input>>{

fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ASSERT
/// Returns `None` if there is no child corresponding to token ASSERT
fn ASSERT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(ASSERT, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token IF
/// Returns `None` if there is no child corresponding to token IF
fn IF(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(IF, 0)
}
fn parExpression(&self) -> Option<Rc<ParExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token ELSE
/// Returns `None` if there is no child corresponding to token ELSE
fn ELSE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(ELSE, 0)
}
/// Retrieves first TerminalNode corresponding to token FOR
/// Returns `None` if there is no child corresponding to token FOR
fn FOR(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(FOR, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn forControl(&self) -> Option<Rc<ForControlContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token WHILE
/// Returns `None` if there is no child corresponding to token WHILE
fn WHILE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(WHILE, 0)
}
/// Retrieves first TerminalNode corresponding to token DO
/// Returns `None` if there is no child corresponding to token DO
fn DO(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DO, 0)
}
/// Retrieves first TerminalNode corresponding to token TRY
/// Returns `None` if there is no child corresponding to token TRY
fn TRY(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(TRY, 0)
}
fn finallyBlock(&self) -> Option<Rc<FinallyBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn catchClause_all(&self) ->  Vec<Rc<CatchClauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn catchClause(&self, i: usize) -> Option<Rc<CatchClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn resourceSpecification(&self) -> Option<Rc<ResourceSpecificationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SWITCH
/// Returns `None` if there is no child corresponding to token SWITCH
fn SWITCH(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SWITCH, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn switchBlockStatementGroup_all(&self) ->  Vec<Rc<SwitchBlockStatementGroupContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn switchBlockStatementGroup(&self, i: usize) -> Option<Rc<SwitchBlockStatementGroupContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn switchLabel_all(&self) ->  Vec<Rc<SwitchLabelContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn switchLabel(&self, i: usize) -> Option<Rc<SwitchLabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token SYNCHRONIZED
/// Returns `None` if there is no child corresponding to token SYNCHRONIZED
fn SYNCHRONIZED(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SYNCHRONIZED, 0)
}
/// Retrieves first TerminalNode corresponding to token RETURN
/// Returns `None` if there is no child corresponding to token RETURN
fn RETURN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RETURN, 0)
}
/// Retrieves first TerminalNode corresponding to token THROW
/// Returns `None` if there is no child corresponding to token THROW
fn THROW(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(THROW, 0)
}
/// Retrieves first TerminalNode corresponding to token BREAK
/// Returns `None` if there is no child corresponding to token BREAK
fn BREAK(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(BREAK, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CONTINUE
/// Returns `None` if there is no child corresponding to token CONTINUE
fn CONTINUE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(CONTINUE, 0)
}
/// Retrieves first TerminalNode corresponding to token YIELD
/// Returns `None` if there is no child corresponding to token YIELD
fn YIELD(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(YIELD, 0)
}
fn switchExpression(&self) -> Option<Rc<SwitchExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 166, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(1215);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(142,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule block*/
					recog.base.set_state(1102);
					let tmp = recog.block()?;
					 cast_mut::<_,StatementContext >(&mut _localctx).blockLabel = Some(tmp.clone());
					  

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1103);
					recog.base.match_token(ASSERT,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1104);
					recog.expression_rec(0)?;

					recog.base.set_state(1107);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COLON {
						{
						recog.base.set_state(1105);
						recog.base.match_token(COLON,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(1106);
						recog.expression_rec(0)?;

						}
					}

					recog.base.set_state(1109);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(1111);
					recog.base.match_token(IF,&mut recog.err_handler)?;

					/*InvokeRule parExpression*/
					recog.base.set_state(1112);
					recog.parExpression()?;

					/*InvokeRule statement*/
					recog.base.set_state(1113);
					recog.statement()?;

					recog.base.set_state(1116);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(130,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(1114);
							recog.base.match_token(ELSE,&mut recog.err_handler)?;

							/*InvokeRule statement*/
							recog.base.set_state(1115);
							recog.statement()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1118);
					recog.base.match_token(FOR,&mut recog.err_handler)?;

					recog.base.set_state(1119);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule forControl*/
					recog.base.set_state(1120);
					recog.forControl()?;

					recog.base.set_state(1121);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(1122);
					recog.statement()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(1124);
					recog.base.match_token(WHILE,&mut recog.err_handler)?;

					/*InvokeRule parExpression*/
					recog.base.set_state(1125);
					recog.parExpression()?;

					/*InvokeRule statement*/
					recog.base.set_state(1126);
					recog.statement()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(1128);
					recog.base.match_token(DO,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(1129);
					recog.statement()?;

					recog.base.set_state(1130);
					recog.base.match_token(WHILE,&mut recog.err_handler)?;

					/*InvokeRule parExpression*/
					recog.base.set_state(1131);
					recog.parExpression()?;

					recog.base.set_state(1132);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					recog.base.set_state(1134);
					recog.base.match_token(TRY,&mut recog.err_handler)?;

					/*InvokeRule block*/
					recog.base.set_state(1135);
					recog.block()?;

					recog.base.set_state(1145);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 CATCH 
						=> {
							{
							recog.base.set_state(1137); 
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							loop {
								{
								{
								/*InvokeRule catchClause*/
								recog.base.set_state(1136);
								recog.catchClause()?;

								}
								}
								recog.base.set_state(1139); 
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								if !(_la==CATCH) {break}
							}
							recog.base.set_state(1142);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==FINALLY {
								{
								/*InvokeRule finallyBlock*/
								recog.base.set_state(1141);
								recog.finallyBlock()?;

								}
							}

							}
						}

					 FINALLY 
						=> {
							{
							/*InvokeRule finallyBlock*/
							recog.base.set_state(1144);
							recog.finallyBlock()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					recog.base.set_state(1147);
					recog.base.match_token(TRY,&mut recog.err_handler)?;

					/*InvokeRule resourceSpecification*/
					recog.base.set_state(1148);
					recog.resourceSpecification()?;

					/*InvokeRule block*/
					recog.base.set_state(1149);
					recog.block()?;

					recog.base.set_state(1153);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==CATCH {
						{
						{
						/*InvokeRule catchClause*/
						recog.base.set_state(1150);
						recog.catchClause()?;

						}
						}
						recog.base.set_state(1155);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(1157);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==FINALLY {
						{
						/*InvokeRule finallyBlock*/
						recog.base.set_state(1156);
						recog.finallyBlock()?;

						}
					}

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					recog.base.set_state(1159);
					recog.base.match_token(SWITCH,&mut recog.err_handler)?;

					/*InvokeRule parExpression*/
					recog.base.set_state(1160);
					recog.parExpression()?;

					recog.base.set_state(1161);
					recog.base.match_token(LBRACE,&mut recog.err_handler)?;

					recog.base.set_state(1165);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(136,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule switchBlockStatementGroup*/
							recog.base.set_state(1162);
							recog.switchBlockStatementGroup()?;

							}
							} 
						}
						recog.base.set_state(1167);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(136,&mut recog.base)?;
					}
					recog.base.set_state(1171);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==CASE || _la==DEFAULT {
						{
						{
						/*InvokeRule switchLabel*/
						recog.base.set_state(1168);
						recog.switchLabel()?;

						}
						}
						recog.base.set_state(1173);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(1174);
					recog.base.match_token(RBRACE,&mut recog.err_handler)?;

					}
				}
			,
				10 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					recog.base.set_state(1176);
					recog.base.match_token(SYNCHRONIZED,&mut recog.err_handler)?;

					/*InvokeRule parExpression*/
					recog.base.set_state(1177);
					recog.parExpression()?;

					/*InvokeRule block*/
					recog.base.set_state(1178);
					recog.block()?;

					}
				}
			,
				11 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 11);
					recog.base.enter_outer_alt(None, 11);
					{
					recog.base.set_state(1180);
					recog.base.match_token(RETURN,&mut recog.err_handler)?;

					recog.base.set_state(1182);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << DOUBLE) | (1usize << FLOAT) | (1usize << INT) | (1usize << LONG) | (1usize << NEW))) != 0) || ((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (SHORT - 37)) | (1usize << (SUPER - 37)) | (1usize << (SWITCH - 37)) | (1usize << (THIS - 37)) | (1usize << (VOID - 37)) | (1usize << (MODULE - 37)) | (1usize << (OPEN - 37)) | (1usize << (REQUIRES - 37)) | (1usize << (EXPORTS - 37)) | (1usize << (OPENS - 37)) | (1usize << (TO - 37)) | (1usize << (USES - 37)) | (1usize << (PROVIDES - 37)) | (1usize << (WITH - 37)) | (1usize << (TRANSITIVE - 37)) | (1usize << (VAR - 37)) | (1usize << (YIELD - 37)) | (1usize << (RECORD - 37)) | (1usize << (SEALED - 37)) | (1usize << (PERMITS - 37)) | (1usize << (DECIMAL_LITERAL - 37)) | (1usize << (HEX_LITERAL - 37)))) != 0) || ((((_la - 69)) & !0x3f) == 0 && ((1usize << (_la - 69)) & ((1usize << (OCT_LITERAL - 69)) | (1usize << (BINARY_LITERAL - 69)) | (1usize << (FLOAT_LITERAL - 69)) | (1usize << (HEX_FLOAT_LITERAL - 69)) | (1usize << (BOOL_LITERAL - 69)) | (1usize << (CHAR_LITERAL - 69)) | (1usize << (STRING_LITERAL - 69)) | (1usize << (TEXT_BLOCK - 69)) | (1usize << (NULL_LITERAL - 69)) | (1usize << (LPAREN - 69)) | (1usize << (LT - 69)) | (1usize << (BANG - 69)) | (1usize << (TILDE - 69)) | (1usize << (INC - 69)))) != 0) || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (DEC - 101)) | (1usize << (ADD - 101)) | (1usize << (SUB - 101)) | (1usize << (AT - 101)) | (1usize << (IDENTIFIER - 101)))) != 0) {
						{
						/*InvokeRule expression*/
						recog.base.set_state(1181);
						recog.expression_rec(0)?;

						}
					}

					recog.base.set_state(1184);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}
			,
				12 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 12);
					recog.base.enter_outer_alt(None, 12);
					{
					recog.base.set_state(1185);
					recog.base.match_token(THROW,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1186);
					recog.expression_rec(0)?;

					recog.base.set_state(1187);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}
			,
				13 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 13);
					recog.base.enter_outer_alt(None, 13);
					{
					recog.base.set_state(1189);
					recog.base.match_token(BREAK,&mut recog.err_handler)?;

					recog.base.set_state(1191);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 51)) & !0x3f) == 0 && ((1usize << (_la - 51)) & ((1usize << (MODULE - 51)) | (1usize << (OPEN - 51)) | (1usize << (REQUIRES - 51)) | (1usize << (EXPORTS - 51)) | (1usize << (OPENS - 51)) | (1usize << (TO - 51)) | (1usize << (USES - 51)) | (1usize << (PROVIDES - 51)) | (1usize << (WITH - 51)) | (1usize << (TRANSITIVE - 51)) | (1usize << (VAR - 51)) | (1usize << (YIELD - 51)) | (1usize << (RECORD - 51)) | (1usize << (SEALED - 51)) | (1usize << (PERMITS - 51)))) != 0) || _la==IDENTIFIER {
						{
						/*InvokeRule identifier*/
						recog.base.set_state(1190);
						recog.identifier()?;

						}
					}

					recog.base.set_state(1193);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}
			,
				14 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 14);
					recog.base.enter_outer_alt(None, 14);
					{
					recog.base.set_state(1194);
					recog.base.match_token(CONTINUE,&mut recog.err_handler)?;

					recog.base.set_state(1196);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 51)) & !0x3f) == 0 && ((1usize << (_la - 51)) & ((1usize << (MODULE - 51)) | (1usize << (OPEN - 51)) | (1usize << (REQUIRES - 51)) | (1usize << (EXPORTS - 51)) | (1usize << (OPENS - 51)) | (1usize << (TO - 51)) | (1usize << (USES - 51)) | (1usize << (PROVIDES - 51)) | (1usize << (WITH - 51)) | (1usize << (TRANSITIVE - 51)) | (1usize << (VAR - 51)) | (1usize << (YIELD - 51)) | (1usize << (RECORD - 51)) | (1usize << (SEALED - 51)) | (1usize << (PERMITS - 51)))) != 0) || _la==IDENTIFIER {
						{
						/*InvokeRule identifier*/
						recog.base.set_state(1195);
						recog.identifier()?;

						}
					}

					recog.base.set_state(1198);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}
			,
				15 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 15);
					recog.base.enter_outer_alt(None, 15);
					{
					recog.base.set_state(1199);
					recog.base.match_token(YIELD,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1200);
					recog.expression_rec(0)?;

					recog.base.set_state(1201);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}
			,
				16 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 16);
					recog.base.enter_outer_alt(None, 16);
					{
					recog.base.set_state(1203);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}
			,
				17 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 17);
					recog.base.enter_outer_alt(None, 17);
					{
					/*InvokeRule expression*/
					recog.base.set_state(1204);
					let tmp = recog.expression_rec(0)?;
					 cast_mut::<_,StatementContext >(&mut _localctx).statementExpression = Some(tmp.clone());
					  

					recog.base.set_state(1205);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					}
				}
			,
				18 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 18);
					recog.base.enter_outer_alt(None, 18);
					{
					/*InvokeRule switchExpression*/
					recog.base.set_state(1207);
					recog.switchExpression()?;

					recog.base.set_state(1209);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(141,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(1208);
							recog.base.match_token(SEMI,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				19 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 19);
					recog.base.enter_outer_alt(None, 19);
					{
					/*InvokeRule identifier*/
					recog.base.set_state(1211);
					let tmp = recog.identifier()?;
					 cast_mut::<_,StatementContext >(&mut _localctx).identifierLabel = Some(tmp.clone());
					  

					recog.base.set_state(1212);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(1213);
					recog.statement()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- catchClause ----------------
pub type CatchClauseContextAll<'input> = CatchClauseContext<'input>;


pub type CatchClauseContext<'input> = BaseParserRuleContext<'input,CatchClauseContextExt<'input>>;

#[derive(Clone)]
pub struct CatchClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for CatchClauseContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for CatchClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_catchClause(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_catchClause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for CatchClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_catchClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_catchClause }
}
antlr_rust::tid!{CatchClauseContextExt<'a>}

impl<'input> CatchClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CatchClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CatchClauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CatchClauseContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<CatchClauseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CATCH
/// Returns `None` if there is no child corresponding to token CATCH
fn CATCH(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(CATCH, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn catchType(&self) -> Option<Rc<CatchTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableModifier_all(&self) ->  Vec<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variableModifier(&self, i: usize) -> Option<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> CatchClauseContextAttrs<'input> for CatchClauseContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn catchClause(&mut self,)
	-> Result<Rc<CatchClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CatchClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 168, RULE_catchClause);
        let mut _localctx: Rc<CatchClauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1217);
			recog.base.match_token(CATCH,&mut recog.err_handler)?;

			recog.base.set_state(1218);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(1222);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(143,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule variableModifier*/
					recog.base.set_state(1219);
					recog.variableModifier()?;

					}
					} 
				}
				recog.base.set_state(1224);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(143,&mut recog.base)?;
			}
			/*InvokeRule catchType*/
			recog.base.set_state(1225);
			recog.catchType()?;

			/*InvokeRule identifier*/
			recog.base.set_state(1226);
			recog.identifier()?;

			recog.base.set_state(1227);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			/*InvokeRule block*/
			recog.base.set_state(1228);
			recog.block()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- catchType ----------------
pub type CatchTypeContextAll<'input> = CatchTypeContext<'input>;


pub type CatchTypeContext<'input> = BaseParserRuleContext<'input,CatchTypeContextExt<'input>>;

#[derive(Clone)]
pub struct CatchTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for CatchTypeContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for CatchTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_catchType(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_catchType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for CatchTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_catchType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_catchType }
}
antlr_rust::tid!{CatchTypeContextExt<'a>}

impl<'input> CatchTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CatchTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CatchTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CatchTypeContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<CatchTypeContextExt<'input>>{

fn qualifiedName_all(&self) ->  Vec<Rc<QualifiedNameContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn qualifiedName(&self, i: usize) -> Option<Rc<QualifiedNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token BITOR in current rule
fn BITOR_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token BITOR, starting from 0.
/// Returns `None` if number of children corresponding to token BITOR is less or equal than `i`.
fn BITOR(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(BITOR, i)
}

}

impl<'input> CatchTypeContextAttrs<'input> for CatchTypeContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn catchType(&mut self,)
	-> Result<Rc<CatchTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CatchTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 170, RULE_catchType);
        let mut _localctx: Rc<CatchTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule qualifiedName*/
			recog.base.set_state(1230);
			recog.qualifiedName()?;

			recog.base.set_state(1235);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==BITOR {
				{
				{
				recog.base.set_state(1231);
				recog.base.match_token(BITOR,&mut recog.err_handler)?;

				/*InvokeRule qualifiedName*/
				recog.base.set_state(1232);
				recog.qualifiedName()?;

				}
				}
				recog.base.set_state(1237);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- finallyBlock ----------------
pub type FinallyBlockContextAll<'input> = FinallyBlockContext<'input>;


pub type FinallyBlockContext<'input> = BaseParserRuleContext<'input,FinallyBlockContextExt<'input>>;

#[derive(Clone)]
pub struct FinallyBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for FinallyBlockContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for FinallyBlockContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_finallyBlock(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_finallyBlock(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for FinallyBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_finallyBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_finallyBlock }
}
antlr_rust::tid!{FinallyBlockContextExt<'a>}

impl<'input> FinallyBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FinallyBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FinallyBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FinallyBlockContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<FinallyBlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FINALLY
/// Returns `None` if there is no child corresponding to token FINALLY
fn FINALLY(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(FINALLY, 0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FinallyBlockContextAttrs<'input> for FinallyBlockContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn finallyBlock(&mut self,)
	-> Result<Rc<FinallyBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FinallyBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 172, RULE_finallyBlock);
        let mut _localctx: Rc<FinallyBlockContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1238);
			recog.base.match_token(FINALLY,&mut recog.err_handler)?;

			/*InvokeRule block*/
			recog.base.set_state(1239);
			recog.block()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- resourceSpecification ----------------
pub type ResourceSpecificationContextAll<'input> = ResourceSpecificationContext<'input>;


pub type ResourceSpecificationContext<'input> = BaseParserRuleContext<'input,ResourceSpecificationContextExt<'input>>;

#[derive(Clone)]
pub struct ResourceSpecificationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ResourceSpecificationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ResourceSpecificationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_resourceSpecification(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_resourceSpecification(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ResourceSpecificationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_resourceSpecification }
	//fn type_rule_index() -> usize where Self: Sized { RULE_resourceSpecification }
}
antlr_rust::tid!{ResourceSpecificationContextExt<'a>}

impl<'input> ResourceSpecificationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ResourceSpecificationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ResourceSpecificationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ResourceSpecificationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ResourceSpecificationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn resources(&self) -> Option<Rc<ResourcesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMI
/// Returns `None` if there is no child corresponding to token SEMI
fn SEMI(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEMI, 0)
}

}

impl<'input> ResourceSpecificationContextAttrs<'input> for ResourceSpecificationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn resourceSpecification(&mut self,)
	-> Result<Rc<ResourceSpecificationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ResourceSpecificationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 174, RULE_resourceSpecification);
        let mut _localctx: Rc<ResourceSpecificationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1241);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule resources*/
			recog.base.set_state(1242);
			recog.resources()?;

			recog.base.set_state(1244);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==SEMI {
				{
				recog.base.set_state(1243);
				recog.base.match_token(SEMI,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(1246);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- resources ----------------
pub type ResourcesContextAll<'input> = ResourcesContext<'input>;


pub type ResourcesContext<'input> = BaseParserRuleContext<'input,ResourcesContextExt<'input>>;

#[derive(Clone)]
pub struct ResourcesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ResourcesContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ResourcesContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_resources(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_resources(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ResourcesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_resources }
	//fn type_rule_index() -> usize where Self: Sized { RULE_resources }
}
antlr_rust::tid!{ResourcesContextExt<'a>}

impl<'input> ResourcesContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ResourcesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ResourcesContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ResourcesContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ResourcesContextExt<'input>>{

fn resource_all(&self) ->  Vec<Rc<ResourceContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn resource(&self, i: usize) -> Option<Rc<ResourceContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SEMI in current rule
fn SEMI_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SEMI, starting from 0.
/// Returns `None` if number of children corresponding to token SEMI is less or equal than `i`.
fn SEMI(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEMI, i)
}

}

impl<'input> ResourcesContextAttrs<'input> for ResourcesContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn resources(&mut self,)
	-> Result<Rc<ResourcesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ResourcesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 176, RULE_resources);
        let mut _localctx: Rc<ResourcesContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule resource*/
			recog.base.set_state(1248);
			recog.resource()?;

			recog.base.set_state(1253);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(146,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(1249);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					/*InvokeRule resource*/
					recog.base.set_state(1250);
					recog.resource()?;

					}
					} 
				}
				recog.base.set_state(1255);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(146,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- resource ----------------
pub type ResourceContextAll<'input> = ResourceContext<'input>;


pub type ResourceContext<'input> = BaseParserRuleContext<'input,ResourceContextExt<'input>>;

#[derive(Clone)]
pub struct ResourceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ResourceContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ResourceContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_resource(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_resource(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ResourceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_resource }
	//fn type_rule_index() -> usize where Self: Sized { RULE_resource }
}
antlr_rust::tid!{ResourceContextExt<'a>}

impl<'input> ResourceContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ResourceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ResourceContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ResourceContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ResourceContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ASSIGN
/// Returns `None` if there is no child corresponding to token ASSIGN
fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn classOrInterfaceType(&self) -> Option<Rc<ClassOrInterfaceTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableDeclaratorId(&self) -> Option<Rc<VariableDeclaratorIdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token VAR
/// Returns `None` if there is no child corresponding to token VAR
fn VAR(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(VAR, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableModifier_all(&self) ->  Vec<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variableModifier(&self, i: usize) -> Option<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ResourceContextAttrs<'input> for ResourceContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn resource(&mut self,)
	-> Result<Rc<ResourceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ResourceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 178, RULE_resource);
        let mut _localctx: Rc<ResourceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(1273);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(149,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1259);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(147,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule variableModifier*/
							recog.base.set_state(1256);
							recog.variableModifier()?;

							}
							} 
						}
						recog.base.set_state(1261);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(147,&mut recog.base)?;
					}
					recog.base.set_state(1267);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(148,&mut recog.base)? {
						1 =>{
							{
							/*InvokeRule classOrInterfaceType*/
							recog.base.set_state(1262);
							recog.classOrInterfaceType()?;

							/*InvokeRule variableDeclaratorId*/
							recog.base.set_state(1263);
							recog.variableDeclaratorId()?;

							}
						}
					,
						2 =>{
							{
							recog.base.set_state(1265);
							recog.base.match_token(VAR,&mut recog.err_handler)?;

							/*InvokeRule identifier*/
							recog.base.set_state(1266);
							recog.identifier()?;

							}
						}

						_ => {}
					}
					recog.base.set_state(1269);
					recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1270);
					recog.expression_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule identifier*/
					recog.base.set_state(1272);
					recog.identifier()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- switchBlockStatementGroup ----------------
pub type SwitchBlockStatementGroupContextAll<'input> = SwitchBlockStatementGroupContext<'input>;


pub type SwitchBlockStatementGroupContext<'input> = BaseParserRuleContext<'input,SwitchBlockStatementGroupContextExt<'input>>;

#[derive(Clone)]
pub struct SwitchBlockStatementGroupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for SwitchBlockStatementGroupContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for SwitchBlockStatementGroupContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_switchBlockStatementGroup(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_switchBlockStatementGroup(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for SwitchBlockStatementGroupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_switchBlockStatementGroup }
	//fn type_rule_index() -> usize where Self: Sized { RULE_switchBlockStatementGroup }
}
antlr_rust::tid!{SwitchBlockStatementGroupContextExt<'a>}

impl<'input> SwitchBlockStatementGroupContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SwitchBlockStatementGroupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SwitchBlockStatementGroupContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SwitchBlockStatementGroupContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<SwitchBlockStatementGroupContextExt<'input>>{

fn switchLabel_all(&self) ->  Vec<Rc<SwitchLabelContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn switchLabel(&self, i: usize) -> Option<Rc<SwitchLabelContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn blockStatement_all(&self) ->  Vec<Rc<BlockStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn blockStatement(&self, i: usize) -> Option<Rc<BlockStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SwitchBlockStatementGroupContextAttrs<'input> for SwitchBlockStatementGroupContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn switchBlockStatementGroup(&mut self,)
	-> Result<Rc<SwitchBlockStatementGroupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SwitchBlockStatementGroupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 180, RULE_switchBlockStatementGroup);
        let mut _localctx: Rc<SwitchBlockStatementGroupContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1276); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule switchLabel*/
				recog.base.set_state(1275);
				recog.switchLabel()?;

				}
				}
				recog.base.set_state(1278); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==CASE || _la==DEFAULT) {break}
			}
			recog.base.set_state(1281); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule blockStatement*/
				recog.base.set_state(1280);
				recog.blockStatement()?;

				}
				}
				recog.base.set_state(1283); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ABSTRACT) | (1usize << ASSERT) | (1usize << BOOLEAN) | (1usize << BREAK) | (1usize << BYTE) | (1usize << CHAR) | (1usize << CLASS) | (1usize << CONTINUE) | (1usize << DO) | (1usize << DOUBLE) | (1usize << FINAL) | (1usize << FLOAT) | (1usize << FOR) | (1usize << IF) | (1usize << INT) | (1usize << INTERFACE) | (1usize << LONG) | (1usize << NATIVE) | (1usize << NEW))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (PRIVATE - 33)) | (1usize << (PROTECTED - 33)) | (1usize << (PUBLIC - 33)) | (1usize << (RETURN - 33)) | (1usize << (SHORT - 33)) | (1usize << (STATIC - 33)) | (1usize << (STRICTFP - 33)) | (1usize << (SUPER - 33)) | (1usize << (SWITCH - 33)) | (1usize << (SYNCHRONIZED - 33)) | (1usize << (THIS - 33)) | (1usize << (THROW - 33)) | (1usize << (TRANSIENT - 33)) | (1usize << (TRY - 33)) | (1usize << (VOID - 33)) | (1usize << (VOLATILE - 33)) | (1usize << (WHILE - 33)) | (1usize << (MODULE - 33)) | (1usize << (OPEN - 33)) | (1usize << (REQUIRES - 33)) | (1usize << (EXPORTS - 33)) | (1usize << (OPENS - 33)) | (1usize << (TO - 33)) | (1usize << (USES - 33)) | (1usize << (PROVIDES - 33)) | (1usize << (WITH - 33)) | (1usize << (TRANSITIVE - 33)) | (1usize << (VAR - 33)) | (1usize << (YIELD - 33)) | (1usize << (RECORD - 33)) | (1usize << (SEALED - 33)))) != 0) || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (PERMITS - 65)) | (1usize << (NON_SEALED - 65)) | (1usize << (DECIMAL_LITERAL - 65)) | (1usize << (HEX_LITERAL - 65)) | (1usize << (OCT_LITERAL - 65)) | (1usize << (BINARY_LITERAL - 65)) | (1usize << (FLOAT_LITERAL - 65)) | (1usize << (HEX_FLOAT_LITERAL - 65)) | (1usize << (BOOL_LITERAL - 65)) | (1usize << (CHAR_LITERAL - 65)) | (1usize << (STRING_LITERAL - 65)) | (1usize << (TEXT_BLOCK - 65)) | (1usize << (NULL_LITERAL - 65)) | (1usize << (LPAREN - 65)) | (1usize << (LBRACE - 65)) | (1usize << (SEMI - 65)) | (1usize << (LT - 65)) | (1usize << (BANG - 65)) | (1usize << (TILDE - 65)))) != 0) || ((((_la - 100)) & !0x3f) == 0 && ((1usize << (_la - 100)) & ((1usize << (INC - 100)) | (1usize << (DEC - 100)) | (1usize << (ADD - 100)) | (1usize << (SUB - 100)) | (1usize << (AT - 100)) | (1usize << (IDENTIFIER - 100)))) != 0)) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- switchLabel ----------------
pub type SwitchLabelContextAll<'input> = SwitchLabelContext<'input>;


pub type SwitchLabelContext<'input> = BaseParserRuleContext<'input,SwitchLabelContextExt<'input>>;

#[derive(Clone)]
pub struct SwitchLabelContextExt<'input>{
	pub constantExpression: Option<Rc<ExpressionContextAll<'input>>>,
	pub enumConstantName: Option<TokenType<'input>>,
	pub varName: Option<Rc<IdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for SwitchLabelContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for SwitchLabelContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_switchLabel(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_switchLabel(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for SwitchLabelContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_switchLabel }
	//fn type_rule_index() -> usize where Self: Sized { RULE_switchLabel }
}
antlr_rust::tid!{SwitchLabelContextExt<'a>}

impl<'input> SwitchLabelContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SwitchLabelContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SwitchLabelContextExt{
				enumConstantName: None, 
				constantExpression: None, varName: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait SwitchLabelContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<SwitchLabelContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CASE
/// Returns `None` if there is no child corresponding to token CASE
fn CASE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(CASE, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn typeType(&self) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DEFAULT
/// Returns `None` if there is no child corresponding to token DEFAULT
fn DEFAULT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DEFAULT, 0)
}

}

impl<'input> SwitchLabelContextAttrs<'input> for SwitchLabelContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn switchLabel(&mut self,)
	-> Result<Rc<SwitchLabelContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SwitchLabelContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 182, RULE_switchLabel);
        let mut _localctx: Rc<SwitchLabelContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1296);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 CASE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1285);
					recog.base.match_token(CASE,&mut recog.err_handler)?;

					recog.base.set_state(1291);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(152,&mut recog.base)? {
						1 =>{
							{
							/*InvokeRule expression*/
							recog.base.set_state(1286);
							let tmp = recog.expression_rec(0)?;
							 cast_mut::<_,SwitchLabelContext >(&mut _localctx).constantExpression = Some(tmp.clone());
							  

							}
						}
					,
						2 =>{
							{
							recog.base.set_state(1287);
							let tmp = recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;
							 cast_mut::<_,SwitchLabelContext >(&mut _localctx).enumConstantName = Some(tmp.clone());
							  

							}
						}
					,
						3 =>{
							{
							/*InvokeRule typeType*/
							recog.base.set_state(1288);
							recog.typeType()?;

							/*InvokeRule identifier*/
							recog.base.set_state(1289);
							let tmp = recog.identifier()?;
							 cast_mut::<_,SwitchLabelContext >(&mut _localctx).varName = Some(tmp.clone());
							  

							}
						}

						_ => {}
					}
					recog.base.set_state(1293);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					}
				}

			 DEFAULT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1294);
					recog.base.match_token(DEFAULT,&mut recog.err_handler)?;

					recog.base.set_state(1295);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- forControl ----------------
pub type ForControlContextAll<'input> = ForControlContext<'input>;


pub type ForControlContext<'input> = BaseParserRuleContext<'input,ForControlContextExt<'input>>;

#[derive(Clone)]
pub struct ForControlContextExt<'input>{
	pub forUpdate: Option<Rc<ExpressionListContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ForControlContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ForControlContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_forControl(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_forControl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ForControlContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forControl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forControl }
}
antlr_rust::tid!{ForControlContextExt<'a>}

impl<'input> ForControlContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ForControlContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForControlContextExt{
				forUpdate: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ForControlContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ForControlContextExt<'input>>{

fn enhancedForControl(&self) -> Option<Rc<EnhancedForControlContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SEMI in current rule
fn SEMI_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SEMI, starting from 0.
/// Returns `None` if number of children corresponding to token SEMI is less or equal than `i`.
fn SEMI(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SEMI, i)
}
fn forInit(&self) -> Option<Rc<ForInitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expressionList(&self) -> Option<Rc<ExpressionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ForControlContextAttrs<'input> for ForControlContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn forControl(&mut self,)
	-> Result<Rc<ForControlContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForControlContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 184, RULE_forControl);
        let mut _localctx: Rc<ForControlContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1310);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(157,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule enhancedForControl*/
					recog.base.set_state(1298);
					recog.enhancedForControl()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1300);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << DOUBLE) | (1usize << FINAL) | (1usize << FLOAT) | (1usize << INT) | (1usize << LONG) | (1usize << NEW))) != 0) || ((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (SHORT - 37)) | (1usize << (SUPER - 37)) | (1usize << (SWITCH - 37)) | (1usize << (THIS - 37)) | (1usize << (VOID - 37)) | (1usize << (MODULE - 37)) | (1usize << (OPEN - 37)) | (1usize << (REQUIRES - 37)) | (1usize << (EXPORTS - 37)) | (1usize << (OPENS - 37)) | (1usize << (TO - 37)) | (1usize << (USES - 37)) | (1usize << (PROVIDES - 37)) | (1usize << (WITH - 37)) | (1usize << (TRANSITIVE - 37)) | (1usize << (VAR - 37)) | (1usize << (YIELD - 37)) | (1usize << (RECORD - 37)) | (1usize << (SEALED - 37)) | (1usize << (PERMITS - 37)) | (1usize << (DECIMAL_LITERAL - 37)) | (1usize << (HEX_LITERAL - 37)))) != 0) || ((((_la - 69)) & !0x3f) == 0 && ((1usize << (_la - 69)) & ((1usize << (OCT_LITERAL - 69)) | (1usize << (BINARY_LITERAL - 69)) | (1usize << (FLOAT_LITERAL - 69)) | (1usize << (HEX_FLOAT_LITERAL - 69)) | (1usize << (BOOL_LITERAL - 69)) | (1usize << (CHAR_LITERAL - 69)) | (1usize << (STRING_LITERAL - 69)) | (1usize << (TEXT_BLOCK - 69)) | (1usize << (NULL_LITERAL - 69)) | (1usize << (LPAREN - 69)) | (1usize << (LT - 69)) | (1usize << (BANG - 69)) | (1usize << (TILDE - 69)) | (1usize << (INC - 69)))) != 0) || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (DEC - 101)) | (1usize << (ADD - 101)) | (1usize << (SUB - 101)) | (1usize << (AT - 101)) | (1usize << (IDENTIFIER - 101)))) != 0) {
						{
						/*InvokeRule forInit*/
						recog.base.set_state(1299);
						recog.forInit()?;

						}
					}

					recog.base.set_state(1302);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					recog.base.set_state(1304);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << DOUBLE) | (1usize << FLOAT) | (1usize << INT) | (1usize << LONG) | (1usize << NEW))) != 0) || ((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (SHORT - 37)) | (1usize << (SUPER - 37)) | (1usize << (SWITCH - 37)) | (1usize << (THIS - 37)) | (1usize << (VOID - 37)) | (1usize << (MODULE - 37)) | (1usize << (OPEN - 37)) | (1usize << (REQUIRES - 37)) | (1usize << (EXPORTS - 37)) | (1usize << (OPENS - 37)) | (1usize << (TO - 37)) | (1usize << (USES - 37)) | (1usize << (PROVIDES - 37)) | (1usize << (WITH - 37)) | (1usize << (TRANSITIVE - 37)) | (1usize << (VAR - 37)) | (1usize << (YIELD - 37)) | (1usize << (RECORD - 37)) | (1usize << (SEALED - 37)) | (1usize << (PERMITS - 37)) | (1usize << (DECIMAL_LITERAL - 37)) | (1usize << (HEX_LITERAL - 37)))) != 0) || ((((_la - 69)) & !0x3f) == 0 && ((1usize << (_la - 69)) & ((1usize << (OCT_LITERAL - 69)) | (1usize << (BINARY_LITERAL - 69)) | (1usize << (FLOAT_LITERAL - 69)) | (1usize << (HEX_FLOAT_LITERAL - 69)) | (1usize << (BOOL_LITERAL - 69)) | (1usize << (CHAR_LITERAL - 69)) | (1usize << (STRING_LITERAL - 69)) | (1usize << (TEXT_BLOCK - 69)) | (1usize << (NULL_LITERAL - 69)) | (1usize << (LPAREN - 69)) | (1usize << (LT - 69)) | (1usize << (BANG - 69)) | (1usize << (TILDE - 69)) | (1usize << (INC - 69)))) != 0) || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (DEC - 101)) | (1usize << (ADD - 101)) | (1usize << (SUB - 101)) | (1usize << (AT - 101)) | (1usize << (IDENTIFIER - 101)))) != 0) {
						{
						/*InvokeRule expression*/
						recog.base.set_state(1303);
						recog.expression_rec(0)?;

						}
					}

					recog.base.set_state(1306);
					recog.base.match_token(SEMI,&mut recog.err_handler)?;

					recog.base.set_state(1308);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << DOUBLE) | (1usize << FLOAT) | (1usize << INT) | (1usize << LONG) | (1usize << NEW))) != 0) || ((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (SHORT - 37)) | (1usize << (SUPER - 37)) | (1usize << (SWITCH - 37)) | (1usize << (THIS - 37)) | (1usize << (VOID - 37)) | (1usize << (MODULE - 37)) | (1usize << (OPEN - 37)) | (1usize << (REQUIRES - 37)) | (1usize << (EXPORTS - 37)) | (1usize << (OPENS - 37)) | (1usize << (TO - 37)) | (1usize << (USES - 37)) | (1usize << (PROVIDES - 37)) | (1usize << (WITH - 37)) | (1usize << (TRANSITIVE - 37)) | (1usize << (VAR - 37)) | (1usize << (YIELD - 37)) | (1usize << (RECORD - 37)) | (1usize << (SEALED - 37)) | (1usize << (PERMITS - 37)) | (1usize << (DECIMAL_LITERAL - 37)) | (1usize << (HEX_LITERAL - 37)))) != 0) || ((((_la - 69)) & !0x3f) == 0 && ((1usize << (_la - 69)) & ((1usize << (OCT_LITERAL - 69)) | (1usize << (BINARY_LITERAL - 69)) | (1usize << (FLOAT_LITERAL - 69)) | (1usize << (HEX_FLOAT_LITERAL - 69)) | (1usize << (BOOL_LITERAL - 69)) | (1usize << (CHAR_LITERAL - 69)) | (1usize << (STRING_LITERAL - 69)) | (1usize << (TEXT_BLOCK - 69)) | (1usize << (NULL_LITERAL - 69)) | (1usize << (LPAREN - 69)) | (1usize << (LT - 69)) | (1usize << (BANG - 69)) | (1usize << (TILDE - 69)) | (1usize << (INC - 69)))) != 0) || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (DEC - 101)) | (1usize << (ADD - 101)) | (1usize << (SUB - 101)) | (1usize << (AT - 101)) | (1usize << (IDENTIFIER - 101)))) != 0) {
						{
						/*InvokeRule expressionList*/
						recog.base.set_state(1307);
						let tmp = recog.expressionList()?;
						 cast_mut::<_,ForControlContext >(&mut _localctx).forUpdate = Some(tmp.clone());
						  

						}
					}

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- forInit ----------------
pub type ForInitContextAll<'input> = ForInitContext<'input>;


pub type ForInitContext<'input> = BaseParserRuleContext<'input,ForInitContextExt<'input>>;

#[derive(Clone)]
pub struct ForInitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ForInitContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ForInitContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_forInit(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_forInit(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ForInitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forInit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forInit }
}
antlr_rust::tid!{ForInitContextExt<'a>}

impl<'input> ForInitContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ForInitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForInitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ForInitContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ForInitContextExt<'input>>{

fn localVariableDeclaration(&self) -> Option<Rc<LocalVariableDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expressionList(&self) -> Option<Rc<ExpressionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ForInitContextAttrs<'input> for ForInitContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn forInit(&mut self,)
	-> Result<Rc<ForInitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForInitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 186, RULE_forInit);
        let mut _localctx: Rc<ForInitContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1314);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(158,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule localVariableDeclaration*/
					recog.base.set_state(1312);
					recog.localVariableDeclaration()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expressionList*/
					recog.base.set_state(1313);
					recog.expressionList()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- enhancedForControl ----------------
pub type EnhancedForControlContextAll<'input> = EnhancedForControlContext<'input>;


pub type EnhancedForControlContext<'input> = BaseParserRuleContext<'input,EnhancedForControlContextExt<'input>>;

#[derive(Clone)]
pub struct EnhancedForControlContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for EnhancedForControlContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for EnhancedForControlContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_enhancedForControl(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_enhancedForControl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for EnhancedForControlContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enhancedForControl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enhancedForControl }
}
antlr_rust::tid!{EnhancedForControlContextExt<'a>}

impl<'input> EnhancedForControlContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnhancedForControlContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnhancedForControlContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnhancedForControlContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<EnhancedForControlContextExt<'input>>{

fn variableDeclaratorId(&self) -> Option<Rc<VariableDeclaratorIdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeType(&self) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token VAR
/// Returns `None` if there is no child corresponding to token VAR
fn VAR(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(VAR, 0)
}
fn variableModifier_all(&self) ->  Vec<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variableModifier(&self, i: usize) -> Option<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> EnhancedForControlContextAttrs<'input> for EnhancedForControlContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enhancedForControl(&mut self,)
	-> Result<Rc<EnhancedForControlContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnhancedForControlContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 188, RULE_enhancedForControl);
        let mut _localctx: Rc<EnhancedForControlContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1319);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(159,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule variableModifier*/
					recog.base.set_state(1316);
					recog.variableModifier()?;

					}
					} 
				}
				recog.base.set_state(1321);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(159,&mut recog.base)?;
			}
			recog.base.set_state(1324);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(160,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule typeType*/
					recog.base.set_state(1322);
					recog.typeType()?;

					}
				}
			,
				2 =>{
					{
					recog.base.set_state(1323);
					recog.base.match_token(VAR,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			/*InvokeRule variableDeclaratorId*/
			recog.base.set_state(1326);
			recog.variableDeclaratorId()?;

			recog.base.set_state(1327);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1328);
			recog.expression_rec(0)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parExpression ----------------
pub type ParExpressionContextAll<'input> = ParExpressionContext<'input>;


pub type ParExpressionContext<'input> = BaseParserRuleContext<'input,ParExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ParExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ParExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ParExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parExpression(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_parExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ParExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parExpression }
}
antlr_rust::tid!{ParExpressionContextExt<'a>}

impl<'input> ParExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParExpressionContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ParExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> ParExpressionContextAttrs<'input> for ParExpressionContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parExpression(&mut self,)
	-> Result<Rc<ParExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 190, RULE_parExpression);
        let mut _localctx: Rc<ParExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1330);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(1331);
			recog.expression_rec(0)?;

			recog.base.set_state(1332);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expressionList ----------------
pub type ExpressionListContextAll<'input> = ExpressionListContext<'input>;


pub type ExpressionListContext<'input> = BaseParserRuleContext<'input,ExpressionListContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ExpressionListContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ExpressionListContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expressionList(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_expressionList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ExpressionListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expressionList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expressionList }
}
antlr_rust::tid!{ExpressionListContextExt<'a>}

impl<'input> ExpressionListContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionListContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ExpressionListContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ExpressionListContextAttrs<'input> for ExpressionListContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expressionList(&mut self,)
	-> Result<Rc<ExpressionListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 192, RULE_expressionList);
        let mut _localctx: Rc<ExpressionListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(1334);
			recog.expression_rec(0)?;

			recog.base.set_state(1339);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(1335);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(1336);
				recog.expression_rec(0)?;

				}
				}
				recog.base.set_state(1341);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- methodCall ----------------
pub type MethodCallContextAll<'input> = MethodCallContext<'input>;


pub type MethodCallContext<'input> = BaseParserRuleContext<'input,MethodCallContextExt<'input>>;

#[derive(Clone)]
pub struct MethodCallContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for MethodCallContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for MethodCallContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_methodCall(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_methodCall(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for MethodCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_methodCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_methodCall }
}
antlr_rust::tid!{MethodCallContextExt<'a>}

impl<'input> MethodCallContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MethodCallContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MethodCallContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MethodCallContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<MethodCallContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn expressionList(&self) -> Option<Rc<ExpressionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token THIS
/// Returns `None` if there is no child corresponding to token THIS
fn THIS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(THIS, 0)
}
/// Retrieves first TerminalNode corresponding to token SUPER
/// Returns `None` if there is no child corresponding to token SUPER
fn SUPER(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SUPER, 0)
}

}

impl<'input> MethodCallContextAttrs<'input> for MethodCallContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn methodCall(&mut self,)
	-> Result<Rc<MethodCallContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MethodCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 194, RULE_methodCall);
        let mut _localctx: Rc<MethodCallContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1361);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 MODULE | OPEN | REQUIRES | EXPORTS | OPENS | TO | USES | PROVIDES | WITH |
			 TRANSITIVE | VAR | YIELD | RECORD | SEALED | PERMITS | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule identifier*/
					recog.base.set_state(1342);
					recog.identifier()?;

					recog.base.set_state(1343);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(1345);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << DOUBLE) | (1usize << FLOAT) | (1usize << INT) | (1usize << LONG) | (1usize << NEW))) != 0) || ((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (SHORT - 37)) | (1usize << (SUPER - 37)) | (1usize << (SWITCH - 37)) | (1usize << (THIS - 37)) | (1usize << (VOID - 37)) | (1usize << (MODULE - 37)) | (1usize << (OPEN - 37)) | (1usize << (REQUIRES - 37)) | (1usize << (EXPORTS - 37)) | (1usize << (OPENS - 37)) | (1usize << (TO - 37)) | (1usize << (USES - 37)) | (1usize << (PROVIDES - 37)) | (1usize << (WITH - 37)) | (1usize << (TRANSITIVE - 37)) | (1usize << (VAR - 37)) | (1usize << (YIELD - 37)) | (1usize << (RECORD - 37)) | (1usize << (SEALED - 37)) | (1usize << (PERMITS - 37)) | (1usize << (DECIMAL_LITERAL - 37)) | (1usize << (HEX_LITERAL - 37)))) != 0) || ((((_la - 69)) & !0x3f) == 0 && ((1usize << (_la - 69)) & ((1usize << (OCT_LITERAL - 69)) | (1usize << (BINARY_LITERAL - 69)) | (1usize << (FLOAT_LITERAL - 69)) | (1usize << (HEX_FLOAT_LITERAL - 69)) | (1usize << (BOOL_LITERAL - 69)) | (1usize << (CHAR_LITERAL - 69)) | (1usize << (STRING_LITERAL - 69)) | (1usize << (TEXT_BLOCK - 69)) | (1usize << (NULL_LITERAL - 69)) | (1usize << (LPAREN - 69)) | (1usize << (LT - 69)) | (1usize << (BANG - 69)) | (1usize << (TILDE - 69)) | (1usize << (INC - 69)))) != 0) || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (DEC - 101)) | (1usize << (ADD - 101)) | (1usize << (SUB - 101)) | (1usize << (AT - 101)) | (1usize << (IDENTIFIER - 101)))) != 0) {
						{
						/*InvokeRule expressionList*/
						recog.base.set_state(1344);
						recog.expressionList()?;

						}
					}

					recog.base.set_state(1347);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

			 THIS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1349);
					recog.base.match_token(THIS,&mut recog.err_handler)?;

					recog.base.set_state(1350);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(1352);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << DOUBLE) | (1usize << FLOAT) | (1usize << INT) | (1usize << LONG) | (1usize << NEW))) != 0) || ((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (SHORT - 37)) | (1usize << (SUPER - 37)) | (1usize << (SWITCH - 37)) | (1usize << (THIS - 37)) | (1usize << (VOID - 37)) | (1usize << (MODULE - 37)) | (1usize << (OPEN - 37)) | (1usize << (REQUIRES - 37)) | (1usize << (EXPORTS - 37)) | (1usize << (OPENS - 37)) | (1usize << (TO - 37)) | (1usize << (USES - 37)) | (1usize << (PROVIDES - 37)) | (1usize << (WITH - 37)) | (1usize << (TRANSITIVE - 37)) | (1usize << (VAR - 37)) | (1usize << (YIELD - 37)) | (1usize << (RECORD - 37)) | (1usize << (SEALED - 37)) | (1usize << (PERMITS - 37)) | (1usize << (DECIMAL_LITERAL - 37)) | (1usize << (HEX_LITERAL - 37)))) != 0) || ((((_la - 69)) & !0x3f) == 0 && ((1usize << (_la - 69)) & ((1usize << (OCT_LITERAL - 69)) | (1usize << (BINARY_LITERAL - 69)) | (1usize << (FLOAT_LITERAL - 69)) | (1usize << (HEX_FLOAT_LITERAL - 69)) | (1usize << (BOOL_LITERAL - 69)) | (1usize << (CHAR_LITERAL - 69)) | (1usize << (STRING_LITERAL - 69)) | (1usize << (TEXT_BLOCK - 69)) | (1usize << (NULL_LITERAL - 69)) | (1usize << (LPAREN - 69)) | (1usize << (LT - 69)) | (1usize << (BANG - 69)) | (1usize << (TILDE - 69)) | (1usize << (INC - 69)))) != 0) || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (DEC - 101)) | (1usize << (ADD - 101)) | (1usize << (SUB - 101)) | (1usize << (AT - 101)) | (1usize << (IDENTIFIER - 101)))) != 0) {
						{
						/*InvokeRule expressionList*/
						recog.base.set_state(1351);
						recog.expressionList()?;

						}
					}

					recog.base.set_state(1354);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

			 SUPER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(1355);
					recog.base.match_token(SUPER,&mut recog.err_handler)?;

					recog.base.set_state(1356);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(1358);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << DOUBLE) | (1usize << FLOAT) | (1usize << INT) | (1usize << LONG) | (1usize << NEW))) != 0) || ((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (SHORT - 37)) | (1usize << (SUPER - 37)) | (1usize << (SWITCH - 37)) | (1usize << (THIS - 37)) | (1usize << (VOID - 37)) | (1usize << (MODULE - 37)) | (1usize << (OPEN - 37)) | (1usize << (REQUIRES - 37)) | (1usize << (EXPORTS - 37)) | (1usize << (OPENS - 37)) | (1usize << (TO - 37)) | (1usize << (USES - 37)) | (1usize << (PROVIDES - 37)) | (1usize << (WITH - 37)) | (1usize << (TRANSITIVE - 37)) | (1usize << (VAR - 37)) | (1usize << (YIELD - 37)) | (1usize << (RECORD - 37)) | (1usize << (SEALED - 37)) | (1usize << (PERMITS - 37)) | (1usize << (DECIMAL_LITERAL - 37)) | (1usize << (HEX_LITERAL - 37)))) != 0) || ((((_la - 69)) & !0x3f) == 0 && ((1usize << (_la - 69)) & ((1usize << (OCT_LITERAL - 69)) | (1usize << (BINARY_LITERAL - 69)) | (1usize << (FLOAT_LITERAL - 69)) | (1usize << (HEX_FLOAT_LITERAL - 69)) | (1usize << (BOOL_LITERAL - 69)) | (1usize << (CHAR_LITERAL - 69)) | (1usize << (STRING_LITERAL - 69)) | (1usize << (TEXT_BLOCK - 69)) | (1usize << (NULL_LITERAL - 69)) | (1usize << (LPAREN - 69)) | (1usize << (LT - 69)) | (1usize << (BANG - 69)) | (1usize << (TILDE - 69)) | (1usize << (INC - 69)))) != 0) || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (DEC - 101)) | (1usize << (ADD - 101)) | (1usize << (SUB - 101)) | (1usize << (AT - 101)) | (1usize << (IDENTIFIER - 101)))) != 0) {
						{
						/*InvokeRule expressionList*/
						recog.base.set_state(1357);
						recog.expressionList()?;

						}
					}

					recog.base.set_state(1360);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
	pub prefix: Option<TokenType<'input>>,
	pub bop: Option<TokenType<'input>>,
	pub postfix: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				prefix: None, bop: None, postfix: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn primary(&self) -> Option<Rc<PrimaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn methodCall(&self) -> Option<Rc<MethodCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token NEW
/// Returns `None` if there is no child corresponding to token NEW
fn NEW(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(NEW, 0)
}
fn creator(&self) -> Option<Rc<CreatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn typeType_all(&self) ->  Vec<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeType(&self, i: usize) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token BITAND in current rule
fn BITAND_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token BITAND, starting from 0.
/// Returns `None` if number of children corresponding to token BITAND is less or equal than `i`.
fn BITAND(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(BITAND, i)
}
/// Retrieves first TerminalNode corresponding to token ADD
/// Returns `None` if there is no child corresponding to token ADD
fn ADD(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(ADD, 0)
}
/// Retrieves first TerminalNode corresponding to token SUB
/// Returns `None` if there is no child corresponding to token SUB
fn SUB(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SUB, 0)
}
/// Retrieves first TerminalNode corresponding to token INC
/// Returns `None` if there is no child corresponding to token INC
fn INC(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(INC, 0)
}
/// Retrieves first TerminalNode corresponding to token DEC
/// Returns `None` if there is no child corresponding to token DEC
fn DEC(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DEC, 0)
}
/// Retrieves first TerminalNode corresponding to token TILDE
/// Returns `None` if there is no child corresponding to token TILDE
fn TILDE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(TILDE, 0)
}
/// Retrieves first TerminalNode corresponding to token BANG
/// Returns `None` if there is no child corresponding to token BANG
fn BANG(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(BANG, 0)
}
fn lambdaExpression(&self) -> Option<Rc<LambdaExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn switchExpression(&self) -> Option<Rc<SwitchExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLONCOLON
/// Returns `None` if there is no child corresponding to token COLONCOLON
fn COLONCOLON(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COLONCOLON, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeArguments(&self) -> Option<Rc<TypeArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn classType(&self) -> Option<Rc<ClassTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token MUL
/// Returns `None` if there is no child corresponding to token MUL
fn MUL(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(MUL, 0)
}
/// Retrieves first TerminalNode corresponding to token DIV
/// Returns `None` if there is no child corresponding to token DIV
fn DIV(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DIV, 0)
}
/// Retrieves first TerminalNode corresponding to token MOD
/// Returns `None` if there is no child corresponding to token MOD
fn MOD(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(MOD, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token LT in current rule
fn LT_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LT, starting from 0.
/// Returns `None` if number of children corresponding to token LT is less or equal than `i`.
fn LT(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LT, i)
}
/// Retrieves all `TerminalNode`s corresponding to token GT in current rule
fn GT_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token GT, starting from 0.
/// Returns `None` if number of children corresponding to token GT is less or equal than `i`.
fn GT(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(GT, i)
}
/// Retrieves first TerminalNode corresponding to token LE
/// Returns `None` if there is no child corresponding to token LE
fn LE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LE, 0)
}
/// Retrieves first TerminalNode corresponding to token GE
/// Returns `None` if there is no child corresponding to token GE
fn GE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(GE, 0)
}
/// Retrieves first TerminalNode corresponding to token EQUAL
/// Returns `None` if there is no child corresponding to token EQUAL
fn EQUAL(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(EQUAL, 0)
}
/// Retrieves first TerminalNode corresponding to token NOTEQUAL
/// Returns `None` if there is no child corresponding to token NOTEQUAL
fn NOTEQUAL(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(NOTEQUAL, 0)
}
/// Retrieves first TerminalNode corresponding to token CARET
/// Returns `None` if there is no child corresponding to token CARET
fn CARET(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(CARET, 0)
}
/// Retrieves first TerminalNode corresponding to token BITOR
/// Returns `None` if there is no child corresponding to token BITOR
fn BITOR(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(BITOR, 0)
}
/// Retrieves first TerminalNode corresponding to token AND
/// Returns `None` if there is no child corresponding to token AND
fn AND(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(AND, 0)
}
/// Retrieves first TerminalNode corresponding to token OR
/// Returns `None` if there is no child corresponding to token OR
fn OR(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(OR, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token QUESTION
/// Returns `None` if there is no child corresponding to token QUESTION
fn QUESTION(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(QUESTION, 0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN
/// Returns `None` if there is no child corresponding to token ASSIGN
fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN, 0)
}
/// Retrieves first TerminalNode corresponding to token ADD_ASSIGN
/// Returns `None` if there is no child corresponding to token ADD_ASSIGN
fn ADD_ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(ADD_ASSIGN, 0)
}
/// Retrieves first TerminalNode corresponding to token SUB_ASSIGN
/// Returns `None` if there is no child corresponding to token SUB_ASSIGN
fn SUB_ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SUB_ASSIGN, 0)
}
/// Retrieves first TerminalNode corresponding to token MUL_ASSIGN
/// Returns `None` if there is no child corresponding to token MUL_ASSIGN
fn MUL_ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(MUL_ASSIGN, 0)
}
/// Retrieves first TerminalNode corresponding to token DIV_ASSIGN
/// Returns `None` if there is no child corresponding to token DIV_ASSIGN
fn DIV_ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DIV_ASSIGN, 0)
}
/// Retrieves first TerminalNode corresponding to token AND_ASSIGN
/// Returns `None` if there is no child corresponding to token AND_ASSIGN
fn AND_ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(AND_ASSIGN, 0)
}
/// Retrieves first TerminalNode corresponding to token OR_ASSIGN
/// Returns `None` if there is no child corresponding to token OR_ASSIGN
fn OR_ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(OR_ASSIGN, 0)
}
/// Retrieves first TerminalNode corresponding to token XOR_ASSIGN
/// Returns `None` if there is no child corresponding to token XOR_ASSIGN
fn XOR_ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(XOR_ASSIGN, 0)
}
/// Retrieves first TerminalNode corresponding to token RSHIFT_ASSIGN
/// Returns `None` if there is no child corresponding to token RSHIFT_ASSIGN
fn RSHIFT_ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RSHIFT_ASSIGN, 0)
}
/// Retrieves first TerminalNode corresponding to token URSHIFT_ASSIGN
/// Returns `None` if there is no child corresponding to token URSHIFT_ASSIGN
fn URSHIFT_ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(URSHIFT_ASSIGN, 0)
}
/// Retrieves first TerminalNode corresponding to token LSHIFT_ASSIGN
/// Returns `None` if there is no child corresponding to token LSHIFT_ASSIGN
fn LSHIFT_ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LSHIFT_ASSIGN, 0)
}
/// Retrieves first TerminalNode corresponding to token MOD_ASSIGN
/// Returns `None` if there is no child corresponding to token MOD_ASSIGN
fn MOD_ASSIGN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(MOD_ASSIGN, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
/// Retrieves first TerminalNode corresponding to token THIS
/// Returns `None` if there is no child corresponding to token THIS
fn THIS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(THIS, 0)
}
fn innerCreator(&self) -> Option<Rc<InnerCreatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SUPER
/// Returns `None` if there is no child corresponding to token SUPER
fn SUPER(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SUPER, 0)
}
fn superSuffix(&self) -> Option<Rc<SuperSuffixContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn explicitGenericInvocation(&self) -> Option<Rc<ExplicitGenericInvocationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nonWildcardTypeArguments(&self) -> Option<Rc<NonWildcardTypeArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LBRACK
/// Returns `None` if there is no child corresponding to token LBRACK
fn LBRACK(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACK, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACK
/// Returns `None` if there is no child corresponding to token RBRACK
fn RBRACK(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACK, 0)
}
/// Retrieves first TerminalNode corresponding to token INSTANCEOF
/// Returns `None` if there is no child corresponding to token INSTANCEOF
fn INSTANCEOF(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(INSTANCEOF, 0)
}
fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		self.expression_rec(0)
	}

	fn expression_rec(&mut self, _p: isize)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 196, RULE_expression, _p);
	    let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 196;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1408);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(171,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule primary*/
					recog.base.set_state(1364);
					recog.primary()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule methodCall*/
					recog.base.set_state(1365);
					recog.methodCall()?;

					}
				}
			,
				3 =>{
					{
					recog.base.set_state(1366);
					recog.base.match_token(NEW,&mut recog.err_handler)?;

					/*InvokeRule creator*/
					recog.base.set_state(1367);
					recog.creator()?;

					}
				}
			,
				4 =>{
					{
					recog.base.set_state(1368);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(1372);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(166,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule annotation*/
							recog.base.set_state(1369);
							recog.annotation()?;

							}
							} 
						}
						recog.base.set_state(1374);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(166,&mut recog.base)?;
					}
					/*InvokeRule typeType*/
					recog.base.set_state(1375);
					recog.typeType()?;

					recog.base.set_state(1380);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==BITAND {
						{
						{
						recog.base.set_state(1376);
						recog.base.match_token(BITAND,&mut recog.err_handler)?;

						/*InvokeRule typeType*/
						recog.base.set_state(1377);
						recog.typeType()?;

						}
						}
						recog.base.set_state(1382);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(1383);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1384);
					recog.expression_rec(22)?;

					}
				}
			,
				5 =>{
					{
					recog.base.set_state(1386);
					 cast_mut::<_,ExpressionContext >(&mut _localctx).prefix = recog.base.input.lt(1).cloned();
					 
					_la = recog.base.input.la(1);
					if { !(((((_la - 100)) & !0x3f) == 0 && ((1usize << (_la - 100)) & ((1usize << (INC - 100)) | (1usize << (DEC - 100)) | (1usize << (ADD - 100)) | (1usize << (SUB - 100)))) != 0)) } {
						let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
						 cast_mut::<_,ExpressionContext >(&mut _localctx).prefix = Some(tmp.clone());
						  

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule expression*/
					recog.base.set_state(1387);
					recog.expression_rec(20)?;

					}
				}
			,
				6 =>{
					{
					recog.base.set_state(1388);
					 cast_mut::<_,ExpressionContext >(&mut _localctx).prefix = recog.base.input.lt(1).cloned();
					 
					_la = recog.base.input.la(1);
					if { !(_la==BANG || _la==TILDE) } {
						let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
						 cast_mut::<_,ExpressionContext >(&mut _localctx).prefix = Some(tmp.clone());
						  

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule expression*/
					recog.base.set_state(1389);
					recog.expression_rec(19)?;

					}
				}
			,
				7 =>{
					{
					/*InvokeRule lambdaExpression*/
					recog.base.set_state(1390);
					recog.lambdaExpression()?;

					}
				}
			,
				8 =>{
					{
					/*InvokeRule switchExpression*/
					recog.base.set_state(1391);
					recog.switchExpression()?;

					}
				}
			,
				9 =>{
					{
					/*InvokeRule typeType*/
					recog.base.set_state(1392);
					recog.typeType()?;

					recog.base.set_state(1393);
					recog.base.match_token(COLONCOLON,&mut recog.err_handler)?;

					recog.base.set_state(1399);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 MODULE | OPEN | REQUIRES | EXPORTS | OPENS | TO | USES | PROVIDES |
					 WITH | TRANSITIVE | VAR | YIELD | RECORD | SEALED | PERMITS | LT |
					 IDENTIFIER 
						=> {
							{
							recog.base.set_state(1395);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==LT {
								{
								/*InvokeRule typeArguments*/
								recog.base.set_state(1394);
								recog.typeArguments()?;

								}
							}

							/*InvokeRule identifier*/
							recog.base.set_state(1397);
							recog.identifier()?;

							}
						}

					 NEW 
						=> {
							{
							recog.base.set_state(1398);
							recog.base.match_token(NEW,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}
			,
				10 =>{
					{
					/*InvokeRule classType*/
					recog.base.set_state(1401);
					recog.classType()?;

					recog.base.set_state(1402);
					recog.base.match_token(COLONCOLON,&mut recog.err_handler)?;

					recog.base.set_state(1404);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LT {
						{
						/*InvokeRule typeArguments*/
						recog.base.set_state(1403);
						recog.typeArguments()?;

						}
					}

					recog.base.set_state(1406);
					recog.base.match_token(NEW,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1493);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(178,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(1491);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(177,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1410);
							if !({recog.precpred(None, 18)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 18)".to_owned()), None))?;
							}
							recog.base.set_state(1411);
							 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = recog.base.input.lt(1).cloned();
							 
							_la = recog.base.input.la(1);
							if { !(((((_la - 104)) & !0x3f) == 0 && ((1usize << (_la - 104)) & ((1usize << (MUL - 104)) | (1usize << (DIV - 104)) | (1usize << (MOD - 104)))) != 0)) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = Some(tmp.clone());
								  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(1412);
							recog.expression_rec(19)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1413);
							if !({recog.precpred(None, 17)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 17)".to_owned()), None))?;
							}
							recog.base.set_state(1414);
							 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = recog.base.input.lt(1).cloned();
							 
							_la = recog.base.input.la(1);
							if { !(_la==ADD || _la==SUB) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = Some(tmp.clone());
								  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(1415);
							recog.expression_rec(18)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1416);
							if !({recog.precpred(None, 16)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 16)".to_owned()), None))?;
							}
							recog.base.set_state(1424);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(172,&mut recog.base)? {
								1 =>{
									{
									recog.base.set_state(1417);
									recog.base.match_token(LT,&mut recog.err_handler)?;

									recog.base.set_state(1418);
									recog.base.match_token(LT,&mut recog.err_handler)?;

									}
								}
							,
								2 =>{
									{
									recog.base.set_state(1419);
									recog.base.match_token(GT,&mut recog.err_handler)?;

									recog.base.set_state(1420);
									recog.base.match_token(GT,&mut recog.err_handler)?;

									recog.base.set_state(1421);
									recog.base.match_token(GT,&mut recog.err_handler)?;

									}
								}
							,
								3 =>{
									{
									recog.base.set_state(1422);
									recog.base.match_token(GT,&mut recog.err_handler)?;

									recog.base.set_state(1423);
									recog.base.match_token(GT,&mut recog.err_handler)?;

									}
								}

								_ => {}
							}
							/*InvokeRule expression*/
							recog.base.set_state(1426);
							recog.expression_rec(17)?;

							}
						}
					,
						4 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1427);
							if !({recog.precpred(None, 15)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 15)".to_owned()), None))?;
							}
							recog.base.set_state(1428);
							 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = recog.base.input.lt(1).cloned();
							 
							_la = recog.base.input.la(1);
							if { !(((((_la - 88)) & !0x3f) == 0 && ((1usize << (_la - 88)) & ((1usize << (GT - 88)) | (1usize << (LT - 88)) | (1usize << (LE - 88)) | (1usize << (GE - 88)))) != 0)) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = Some(tmp.clone());
								  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(1429);
							recog.expression_rec(16)?;

							}
						}
					,
						5 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1430);
							if !({recog.precpred(None, 13)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 13)".to_owned()), None))?;
							}
							recog.base.set_state(1431);
							 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = recog.base.input.lt(1).cloned();
							 
							_la = recog.base.input.la(1);
							if { !(_la==EQUAL || _la==NOTEQUAL) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = Some(tmp.clone());
								  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(1432);
							recog.expression_rec(14)?;

							}
						}
					,
						6 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1433);
							if !({recog.precpred(None, 12)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 12)".to_owned()), None))?;
							}
							recog.base.set_state(1434);
							let tmp = recog.base.match_token(BITAND,&mut recog.err_handler)?;
							 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = Some(tmp.clone());
							  

							/*InvokeRule expression*/
							recog.base.set_state(1435);
							recog.expression_rec(13)?;

							}
						}
					,
						7 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1436);
							if !({recog.precpred(None, 11)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 11)".to_owned()), None))?;
							}
							recog.base.set_state(1437);
							let tmp = recog.base.match_token(CARET,&mut recog.err_handler)?;
							 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = Some(tmp.clone());
							  

							/*InvokeRule expression*/
							recog.base.set_state(1438);
							recog.expression_rec(12)?;

							}
						}
					,
						8 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1439);
							if !({recog.precpred(None, 10)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 10)".to_owned()), None))?;
							}
							recog.base.set_state(1440);
							let tmp = recog.base.match_token(BITOR,&mut recog.err_handler)?;
							 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = Some(tmp.clone());
							  

							/*InvokeRule expression*/
							recog.base.set_state(1441);
							recog.expression_rec(11)?;

							}
						}
					,
						9 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1442);
							if !({recog.precpred(None, 9)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 9)".to_owned()), None))?;
							}
							recog.base.set_state(1443);
							let tmp = recog.base.match_token(AND,&mut recog.err_handler)?;
							 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = Some(tmp.clone());
							  

							/*InvokeRule expression*/
							recog.base.set_state(1444);
							recog.expression_rec(10)?;

							}
						}
					,
						10 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1445);
							if !({recog.precpred(None, 8)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 8)".to_owned()), None))?;
							}
							recog.base.set_state(1446);
							let tmp = recog.base.match_token(OR,&mut recog.err_handler)?;
							 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = Some(tmp.clone());
							  

							/*InvokeRule expression*/
							recog.base.set_state(1447);
							recog.expression_rec(9)?;

							}
						}
					,
						11 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1448);
							if !({recog.precpred(None, 7)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
							}
							recog.base.set_state(1449);
							let tmp = recog.base.match_token(QUESTION,&mut recog.err_handler)?;
							 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = Some(tmp.clone());
							  

							/*InvokeRule expression*/
							recog.base.set_state(1450);
							recog.expression_rec(0)?;

							recog.base.set_state(1451);
							recog.base.match_token(COLON,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(1452);
							recog.expression_rec(7)?;

							}
						}
					,
						12 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1454);
							if !({recog.precpred(None, 6)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
							}
							recog.base.set_state(1455);
							 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = recog.base.input.lt(1).cloned();
							 
							_la = recog.base.input.la(1);
							if { !(((((_la - 87)) & !0x3f) == 0 && ((1usize << (_la - 87)) & ((1usize << (ASSIGN - 87)) | (1usize << (ADD_ASSIGN - 87)) | (1usize << (SUB_ASSIGN - 87)) | (1usize << (MUL_ASSIGN - 87)) | (1usize << (DIV_ASSIGN - 87)) | (1usize << (AND_ASSIGN - 87)) | (1usize << (OR_ASSIGN - 87)) | (1usize << (XOR_ASSIGN - 87)) | (1usize << (MOD_ASSIGN - 87)) | (1usize << (LSHIFT_ASSIGN - 87)))) != 0) || _la==RSHIFT_ASSIGN || _la==URSHIFT_ASSIGN) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = Some(tmp.clone());
								  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(1456);
							recog.expression_rec(6)?;

							}
						}
					,
						13 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1457);
							if !({recog.precpred(None, 26)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 26)".to_owned()), None))?;
							}
							recog.base.set_state(1458);
							let tmp = recog.base.match_token(DOT,&mut recog.err_handler)?;
							 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = Some(tmp.clone());
							  

							recog.base.set_state(1470);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(174,&mut recog.base)? {
								1 =>{
									{
									/*InvokeRule identifier*/
									recog.base.set_state(1459);
									recog.identifier()?;

									}
								}
							,
								2 =>{
									{
									/*InvokeRule methodCall*/
									recog.base.set_state(1460);
									recog.methodCall()?;

									}
								}
							,
								3 =>{
									{
									recog.base.set_state(1461);
									recog.base.match_token(THIS,&mut recog.err_handler)?;

									}
								}
							,
								4 =>{
									{
									recog.base.set_state(1462);
									recog.base.match_token(NEW,&mut recog.err_handler)?;

									recog.base.set_state(1464);
									recog.err_handler.sync(&mut recog.base)?;
									_la = recog.base.input.la(1);
									if _la==LT {
										{
										/*InvokeRule nonWildcardTypeArguments*/
										recog.base.set_state(1463);
										recog.nonWildcardTypeArguments()?;

										}
									}

									/*InvokeRule innerCreator*/
									recog.base.set_state(1466);
									recog.innerCreator()?;

									}
								}
							,
								5 =>{
									{
									recog.base.set_state(1467);
									recog.base.match_token(SUPER,&mut recog.err_handler)?;

									/*InvokeRule superSuffix*/
									recog.base.set_state(1468);
									recog.superSuffix()?;

									}
								}
							,
								6 =>{
									{
									/*InvokeRule explicitGenericInvocation*/
									recog.base.set_state(1469);
									recog.explicitGenericInvocation()?;

									}
								}

								_ => {}
							}
							}
						}
					,
						14 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1472);
							if !({recog.precpred(None, 25)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 25)".to_owned()), None))?;
							}
							recog.base.set_state(1473);
							recog.base.match_token(LBRACK,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(1474);
							recog.expression_rec(0)?;

							recog.base.set_state(1475);
							recog.base.match_token(RBRACK,&mut recog.err_handler)?;

							}
						}
					,
						15 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1477);
							if !({recog.precpred(None, 21)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 21)".to_owned()), None))?;
							}
							recog.base.set_state(1478);
							 cast_mut::<_,ExpressionContext >(&mut _localctx).postfix = recog.base.input.lt(1).cloned();
							 
							_la = recog.base.input.la(1);
							if { !(_la==INC || _la==DEC) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								 cast_mut::<_,ExpressionContext >(&mut _localctx).postfix = Some(tmp.clone());
								  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							}
						}
					,
						16 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1479);
							if !({recog.precpred(None, 14)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 14)".to_owned()), None))?;
							}
							recog.base.set_state(1480);
							let tmp = recog.base.match_token(INSTANCEOF,&mut recog.err_handler)?;
							 cast_mut::<_,ExpressionContext >(&mut _localctx).bop = Some(tmp.clone());
							  

							recog.base.set_state(1483);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(175,&mut recog.base)? {
								1 =>{
									{
									/*InvokeRule typeType*/
									recog.base.set_state(1481);
									recog.typeType()?;

									}
								}
							,
								2 =>{
									{
									/*InvokeRule pattern*/
									recog.base.set_state(1482);
									recog.pattern()?;

									}
								}

								_ => {}
							}
							}
						}
					,
						17 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(1485);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(1486);
							recog.base.match_token(COLONCOLON,&mut recog.err_handler)?;

							recog.base.set_state(1488);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==LT {
								{
								/*InvokeRule typeArguments*/
								recog.base.set_state(1487);
								recog.typeArguments()?;

								}
							}

							/*InvokeRule identifier*/
							recog.base.set_state(1490);
							recog.identifier()?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(1495);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(178,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- pattern ----------------
pub type PatternContextAll<'input> = PatternContext<'input>;


pub type PatternContext<'input> = BaseParserRuleContext<'input,PatternContextExt<'input>>;

#[derive(Clone)]
pub struct PatternContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for PatternContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for PatternContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pattern(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_pattern(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pattern }
}
antlr_rust::tid!{PatternContextExt<'a>}

impl<'input> PatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PatternContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<PatternContextExt<'input>>{

fn typeType(&self) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableModifier_all(&self) ->  Vec<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variableModifier(&self, i: usize) -> Option<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PatternContextAttrs<'input> for PatternContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pattern(&mut self,)
	-> Result<Rc<PatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 198, RULE_pattern);
        let mut _localctx: Rc<PatternContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1499);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(179,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule variableModifier*/
					recog.base.set_state(1496);
					recog.variableModifier()?;

					}
					} 
				}
				recog.base.set_state(1501);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(179,&mut recog.base)?;
			}
			/*InvokeRule typeType*/
			recog.base.set_state(1502);
			recog.typeType()?;

			recog.base.set_state(1506);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(180,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule annotation*/
					recog.base.set_state(1503);
					recog.annotation()?;

					}
					} 
				}
				recog.base.set_state(1508);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(180,&mut recog.base)?;
			}
			/*InvokeRule identifier*/
			recog.base.set_state(1509);
			recog.identifier()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- lambdaExpression ----------------
pub type LambdaExpressionContextAll<'input> = LambdaExpressionContext<'input>;


pub type LambdaExpressionContext<'input> = BaseParserRuleContext<'input,LambdaExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct LambdaExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for LambdaExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for LambdaExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lambdaExpression(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_lambdaExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for LambdaExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lambdaExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lambdaExpression }
}
antlr_rust::tid!{LambdaExpressionContextExt<'a>}

impl<'input> LambdaExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LambdaExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LambdaExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LambdaExpressionContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<LambdaExpressionContextExt<'input>>{

fn lambdaParameters(&self) -> Option<Rc<LambdaParametersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ARROW
/// Returns `None` if there is no child corresponding to token ARROW
fn ARROW(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(ARROW, 0)
}
fn lambdaBody(&self) -> Option<Rc<LambdaBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LambdaExpressionContextAttrs<'input> for LambdaExpressionContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lambdaExpression(&mut self,)
	-> Result<Rc<LambdaExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LambdaExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 200, RULE_lambdaExpression);
        let mut _localctx: Rc<LambdaExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule lambdaParameters*/
			recog.base.set_state(1511);
			recog.lambdaParameters()?;

			recog.base.set_state(1512);
			recog.base.match_token(ARROW,&mut recog.err_handler)?;

			/*InvokeRule lambdaBody*/
			recog.base.set_state(1513);
			recog.lambdaBody()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- lambdaParameters ----------------
pub type LambdaParametersContextAll<'input> = LambdaParametersContext<'input>;


pub type LambdaParametersContext<'input> = BaseParserRuleContext<'input,LambdaParametersContextExt<'input>>;

#[derive(Clone)]
pub struct LambdaParametersContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for LambdaParametersContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for LambdaParametersContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lambdaParameters(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_lambdaParameters(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for LambdaParametersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lambdaParameters }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lambdaParameters }
}
antlr_rust::tid!{LambdaParametersContextExt<'a>}

impl<'input> LambdaParametersContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LambdaParametersContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LambdaParametersContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LambdaParametersContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<LambdaParametersContextExt<'input>>{

fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn formalParameterList(&self) -> Option<Rc<FormalParameterListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}
fn lambdaLVTIList(&self) -> Option<Rc<LambdaLVTIListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LambdaParametersContextAttrs<'input> for LambdaParametersContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lambdaParameters(&mut self,)
	-> Result<Rc<LambdaParametersContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LambdaParametersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 202, RULE_lambdaParameters);
        let mut _localctx: Rc<LambdaParametersContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1537);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(184,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule identifier*/
					recog.base.set_state(1515);
					recog.identifier()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1516);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(1518);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << DOUBLE) | (1usize << FINAL) | (1usize << FLOAT) | (1usize << INT) | (1usize << LONG))) != 0) || ((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (SHORT - 37)) | (1usize << (MODULE - 37)) | (1usize << (OPEN - 37)) | (1usize << (REQUIRES - 37)) | (1usize << (EXPORTS - 37)) | (1usize << (OPENS - 37)) | (1usize << (TO - 37)) | (1usize << (USES - 37)) | (1usize << (PROVIDES - 37)) | (1usize << (WITH - 37)) | (1usize << (TRANSITIVE - 37)) | (1usize << (VAR - 37)) | (1usize << (YIELD - 37)) | (1usize << (RECORD - 37)) | (1usize << (SEALED - 37)) | (1usize << (PERMITS - 37)))) != 0) || _la==AT || _la==IDENTIFIER {
						{
						/*InvokeRule formalParameterList*/
						recog.base.set_state(1517);
						recog.formalParameterList()?;

						}
					}

					recog.base.set_state(1520);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(1521);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule identifier*/
					recog.base.set_state(1522);
					recog.identifier()?;

					recog.base.set_state(1527);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==COMMA {
						{
						{
						recog.base.set_state(1523);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule identifier*/
						recog.base.set_state(1524);
						recog.identifier()?;

						}
						}
						recog.base.set_state(1529);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(1530);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1532);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(1534);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==FINAL || ((((_la - 51)) & !0x3f) == 0 && ((1usize << (_la - 51)) & ((1usize << (MODULE - 51)) | (1usize << (OPEN - 51)) | (1usize << (REQUIRES - 51)) | (1usize << (EXPORTS - 51)) | (1usize << (OPENS - 51)) | (1usize << (TO - 51)) | (1usize << (USES - 51)) | (1usize << (PROVIDES - 51)) | (1usize << (WITH - 51)) | (1usize << (TRANSITIVE - 51)) | (1usize << (VAR - 51)) | (1usize << (YIELD - 51)) | (1usize << (RECORD - 51)) | (1usize << (SEALED - 51)) | (1usize << (PERMITS - 51)))) != 0) || _la==AT || _la==IDENTIFIER {
						{
						/*InvokeRule lambdaLVTIList*/
						recog.base.set_state(1533);
						recog.lambdaLVTIList()?;

						}
					}

					recog.base.set_state(1536);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- lambdaBody ----------------
pub type LambdaBodyContextAll<'input> = LambdaBodyContext<'input>;


pub type LambdaBodyContext<'input> = BaseParserRuleContext<'input,LambdaBodyContextExt<'input>>;

#[derive(Clone)]
pub struct LambdaBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for LambdaBodyContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for LambdaBodyContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_lambdaBody(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_lambdaBody(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for LambdaBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lambdaBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lambdaBody }
}
antlr_rust::tid!{LambdaBodyContextExt<'a>}

impl<'input> LambdaBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LambdaBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LambdaBodyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LambdaBodyContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<LambdaBodyContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LambdaBodyContextAttrs<'input> for LambdaBodyContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn lambdaBody(&mut self,)
	-> Result<Rc<LambdaBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LambdaBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 204, RULE_lambdaBody);
        let mut _localctx: Rc<LambdaBodyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1541);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BOOLEAN | BYTE | CHAR | DOUBLE | FLOAT | INT | LONG | NEW | SHORT | SUPER |
			 SWITCH | THIS | VOID | MODULE | OPEN | REQUIRES | EXPORTS | OPENS | TO |
			 USES | PROVIDES | WITH | TRANSITIVE | VAR | YIELD | RECORD | SEALED |
			 PERMITS | DECIMAL_LITERAL | HEX_LITERAL | OCT_LITERAL | BINARY_LITERAL |
			 FLOAT_LITERAL | HEX_FLOAT_LITERAL | BOOL_LITERAL | CHAR_LITERAL | STRING_LITERAL |
			 TEXT_BLOCK | NULL_LITERAL | LPAREN | LT | BANG | TILDE | INC | DEC |
			 ADD | SUB | AT | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule expression*/
					recog.base.set_state(1539);
					recog.expression_rec(0)?;

					}
				}

			 LBRACE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule block*/
					recog.base.set_state(1540);
					recog.block()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- primary ----------------
pub type PrimaryContextAll<'input> = PrimaryContext<'input>;


pub type PrimaryContext<'input> = BaseParserRuleContext<'input,PrimaryContextExt<'input>>;

#[derive(Clone)]
pub struct PrimaryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for PrimaryContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for PrimaryContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_primary(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_primary(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PrimaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary }
}
antlr_rust::tid!{PrimaryContextExt<'a>}

impl<'input> PrimaryContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PrimaryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrimaryContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PrimaryContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<PrimaryContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token THIS
/// Returns `None` if there is no child corresponding to token THIS
fn THIS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(THIS, 0)
}
/// Retrieves first TerminalNode corresponding to token SUPER
/// Returns `None` if there is no child corresponding to token SUPER
fn SUPER(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SUPER, 0)
}
fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeTypeOrVoid(&self) -> Option<Rc<TypeTypeOrVoidContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
/// Retrieves first TerminalNode corresponding to token CLASS
/// Returns `None` if there is no child corresponding to token CLASS
fn CLASS(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(CLASS, 0)
}
fn nonWildcardTypeArguments(&self) -> Option<Rc<NonWildcardTypeArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn explicitGenericInvocationSuffix(&self) -> Option<Rc<ExplicitGenericInvocationSuffixContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PrimaryContextAttrs<'input> for PrimaryContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn primary(&mut self,)
	-> Result<Rc<PrimaryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrimaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 206, RULE_primary);
        let mut _localctx: Rc<PrimaryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1561);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(187,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1543);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1544);
					recog.expression_rec(0)?;

					recog.base.set_state(1545);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1547);
					recog.base.match_token(THIS,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(1548);
					recog.base.match_token(SUPER,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule literal*/
					recog.base.set_state(1549);
					recog.literal()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule identifier*/
					recog.base.set_state(1550);
					recog.identifier()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule typeTypeOrVoid*/
					recog.base.set_state(1551);
					recog.typeTypeOrVoid()?;

					recog.base.set_state(1552);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					recog.base.set_state(1553);
					recog.base.match_token(CLASS,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule nonWildcardTypeArguments*/
					recog.base.set_state(1555);
					recog.nonWildcardTypeArguments()?;

					recog.base.set_state(1559);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 SUPER | MODULE | OPEN | REQUIRES | EXPORTS | OPENS | TO | USES | PROVIDES |
					 WITH | TRANSITIVE | VAR | YIELD | RECORD | SEALED | PERMITS | IDENTIFIER 
						=> {
							{
							/*InvokeRule explicitGenericInvocationSuffix*/
							recog.base.set_state(1556);
							recog.explicitGenericInvocationSuffix()?;

							}
						}

					 THIS 
						=> {
							{
							recog.base.set_state(1557);
							recog.base.match_token(THIS,&mut recog.err_handler)?;

							/*InvokeRule arguments*/
							recog.base.set_state(1558);
							recog.arguments()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- switchExpression ----------------
pub type SwitchExpressionContextAll<'input> = SwitchExpressionContext<'input>;


pub type SwitchExpressionContext<'input> = BaseParserRuleContext<'input,SwitchExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct SwitchExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for SwitchExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for SwitchExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_switchExpression(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_switchExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for SwitchExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_switchExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_switchExpression }
}
antlr_rust::tid!{SwitchExpressionContextExt<'a>}

impl<'input> SwitchExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SwitchExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SwitchExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SwitchExpressionContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<SwitchExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SWITCH
/// Returns `None` if there is no child corresponding to token SWITCH
fn SWITCH(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SWITCH, 0)
}
fn parExpression(&self) -> Option<Rc<ParExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn switchLabeledRule_all(&self) ->  Vec<Rc<SwitchLabeledRuleContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn switchLabeledRule(&self, i: usize) -> Option<Rc<SwitchLabeledRuleContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SwitchExpressionContextAttrs<'input> for SwitchExpressionContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn switchExpression(&mut self,)
	-> Result<Rc<SwitchExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SwitchExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 208, RULE_switchExpression);
        let mut _localctx: Rc<SwitchExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1563);
			recog.base.match_token(SWITCH,&mut recog.err_handler)?;

			/*InvokeRule parExpression*/
			recog.base.set_state(1564);
			recog.parExpression()?;

			recog.base.set_state(1565);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(1569);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==CASE || _la==DEFAULT {
				{
				{
				/*InvokeRule switchLabeledRule*/
				recog.base.set_state(1566);
				recog.switchLabeledRule()?;

				}
				}
				recog.base.set_state(1571);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1572);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- switchLabeledRule ----------------
pub type SwitchLabeledRuleContextAll<'input> = SwitchLabeledRuleContext<'input>;


pub type SwitchLabeledRuleContext<'input> = BaseParserRuleContext<'input,SwitchLabeledRuleContextExt<'input>>;

#[derive(Clone)]
pub struct SwitchLabeledRuleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for SwitchLabeledRuleContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for SwitchLabeledRuleContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_switchLabeledRule(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_switchLabeledRule(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for SwitchLabeledRuleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_switchLabeledRule }
	//fn type_rule_index() -> usize where Self: Sized { RULE_switchLabeledRule }
}
antlr_rust::tid!{SwitchLabeledRuleContextExt<'a>}

impl<'input> SwitchLabeledRuleContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SwitchLabeledRuleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SwitchLabeledRuleContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SwitchLabeledRuleContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<SwitchLabeledRuleContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CASE
/// Returns `None` if there is no child corresponding to token CASE
fn CASE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(CASE, 0)
}
fn switchRuleOutcome(&self) -> Option<Rc<SwitchRuleOutcomeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ARROW
/// Returns `None` if there is no child corresponding to token ARROW
fn ARROW(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(ARROW, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn expressionList(&self) -> Option<Rc<ExpressionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token NULL_LITERAL
/// Returns `None` if there is no child corresponding to token NULL_LITERAL
fn NULL_LITERAL(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(NULL_LITERAL, 0)
}
fn guardedPattern(&self) -> Option<Rc<GuardedPatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DEFAULT
/// Returns `None` if there is no child corresponding to token DEFAULT
fn DEFAULT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DEFAULT, 0)
}

}

impl<'input> SwitchLabeledRuleContextAttrs<'input> for SwitchLabeledRuleContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn switchLabeledRule(&mut self,)
	-> Result<Rc<SwitchLabeledRuleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SwitchLabeledRuleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 210, RULE_switchLabeledRule);
        let mut _localctx: Rc<SwitchLabeledRuleContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1585);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 CASE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1574);
					recog.base.match_token(CASE,&mut recog.err_handler)?;

					recog.base.set_state(1578);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(189,&mut recog.base)? {
						1 =>{
							{
							/*InvokeRule expressionList*/
							recog.base.set_state(1575);
							recog.expressionList()?;

							}
						}
					,
						2 =>{
							{
							recog.base.set_state(1576);
							recog.base.match_token(NULL_LITERAL,&mut recog.err_handler)?;

							}
						}
					,
						3 =>{
							{
							/*InvokeRule guardedPattern*/
							recog.base.set_state(1577);
							recog.guardedPattern_rec(0)?;

							}
						}

						_ => {}
					}
					recog.base.set_state(1580);
					_la = recog.base.input.la(1);
					if { !(_la==COLON || _la==ARROW) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule switchRuleOutcome*/
					recog.base.set_state(1581);
					recog.switchRuleOutcome()?;

					}
				}

			 DEFAULT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1582);
					recog.base.match_token(DEFAULT,&mut recog.err_handler)?;

					recog.base.set_state(1583);
					_la = recog.base.input.la(1);
					if { !(_la==COLON || _la==ARROW) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule switchRuleOutcome*/
					recog.base.set_state(1584);
					recog.switchRuleOutcome()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- guardedPattern ----------------
pub type GuardedPatternContextAll<'input> = GuardedPatternContext<'input>;


pub type GuardedPatternContext<'input> = BaseParserRuleContext<'input,GuardedPatternContextExt<'input>>;

#[derive(Clone)]
pub struct GuardedPatternContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for GuardedPatternContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for GuardedPatternContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_guardedPattern(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_guardedPattern(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for GuardedPatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_guardedPattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_guardedPattern }
}
antlr_rust::tid!{GuardedPatternContextExt<'a>}

impl<'input> GuardedPatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GuardedPatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GuardedPatternContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GuardedPatternContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<GuardedPatternContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn guardedPattern(&self) -> Option<Rc<GuardedPatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn typeType(&self) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableModifier_all(&self) ->  Vec<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variableModifier(&self, i: usize) -> Option<Rc<VariableModifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token AND in current rule
fn AND_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token AND, starting from 0.
/// Returns `None` if number of children corresponding to token AND is less or equal than `i`.
fn AND(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(AND, i)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> GuardedPatternContextAttrs<'input> for GuardedPatternContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  guardedPattern(&mut self,)
	-> Result<Rc<GuardedPatternContextAll<'input>>,ANTLRError> {
		self.guardedPattern_rec(0)
	}

	fn guardedPattern_rec(&mut self, _p: isize)
	-> Result<Rc<GuardedPatternContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = GuardedPatternContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 212, RULE_guardedPattern, _p);
	    let mut _localctx: Rc<GuardedPatternContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 212;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1613);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LPAREN 
				=> {
					{
					recog.base.set_state(1588);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule guardedPattern*/
					recog.base.set_state(1589);
					recog.guardedPattern_rec(0)?;

					recog.base.set_state(1590);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

			 BOOLEAN | BYTE | CHAR | DOUBLE | FINAL | FLOAT | INT | LONG | SHORT |
			 MODULE | OPEN | REQUIRES | EXPORTS | OPENS | TO | USES | PROVIDES | WITH |
			 TRANSITIVE | VAR | YIELD | RECORD | SEALED | PERMITS | AT | IDENTIFIER 
				=> {
					{
					recog.base.set_state(1595);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(191,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule variableModifier*/
							recog.base.set_state(1592);
							recog.variableModifier()?;

							}
							} 
						}
						recog.base.set_state(1597);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(191,&mut recog.base)?;
					}
					/*InvokeRule typeType*/
					recog.base.set_state(1598);
					recog.typeType()?;

					recog.base.set_state(1602);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(192,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule annotation*/
							recog.base.set_state(1599);
							recog.annotation()?;

							}
							} 
						}
						recog.base.set_state(1604);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(192,&mut recog.base)?;
					}
					/*InvokeRule identifier*/
					recog.base.set_state(1605);
					recog.identifier()?;

					recog.base.set_state(1610);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(193,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(1606);
							recog.base.match_token(AND,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(1607);
							recog.expression_rec(0)?;

							}
							} 
						}
						recog.base.set_state(1612);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(193,&mut recog.base)?;
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(1620);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(195,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = GuardedPatternContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_guardedPattern);
					_localctx = tmp;
					recog.base.set_state(1615);
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(1616);
					recog.base.match_token(AND,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1617);
					recog.expression_rec(0)?;

					}
					} 
				}
				recog.base.set_state(1622);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(195,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- switchRuleOutcome ----------------
pub type SwitchRuleOutcomeContextAll<'input> = SwitchRuleOutcomeContext<'input>;


pub type SwitchRuleOutcomeContext<'input> = BaseParserRuleContext<'input,SwitchRuleOutcomeContextExt<'input>>;

#[derive(Clone)]
pub struct SwitchRuleOutcomeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for SwitchRuleOutcomeContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for SwitchRuleOutcomeContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_switchRuleOutcome(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_switchRuleOutcome(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for SwitchRuleOutcomeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_switchRuleOutcome }
	//fn type_rule_index() -> usize where Self: Sized { RULE_switchRuleOutcome }
}
antlr_rust::tid!{SwitchRuleOutcomeContextExt<'a>}

impl<'input> SwitchRuleOutcomeContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SwitchRuleOutcomeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SwitchRuleOutcomeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SwitchRuleOutcomeContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<SwitchRuleOutcomeContextExt<'input>>{

fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn blockStatement_all(&self) ->  Vec<Rc<BlockStatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn blockStatement(&self, i: usize) -> Option<Rc<BlockStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SwitchRuleOutcomeContextAttrs<'input> for SwitchRuleOutcomeContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn switchRuleOutcome(&mut self,)
	-> Result<Rc<SwitchRuleOutcomeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SwitchRuleOutcomeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 214, RULE_switchRuleOutcome);
        let mut _localctx: Rc<SwitchRuleOutcomeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1630);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(197,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule block*/
					recog.base.set_state(1623);
					recog.block()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1627);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ABSTRACT) | (1usize << ASSERT) | (1usize << BOOLEAN) | (1usize << BREAK) | (1usize << BYTE) | (1usize << CHAR) | (1usize << CLASS) | (1usize << CONTINUE) | (1usize << DO) | (1usize << DOUBLE) | (1usize << FINAL) | (1usize << FLOAT) | (1usize << FOR) | (1usize << IF) | (1usize << INT) | (1usize << INTERFACE) | (1usize << LONG) | (1usize << NATIVE) | (1usize << NEW))) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & ((1usize << (PRIVATE - 33)) | (1usize << (PROTECTED - 33)) | (1usize << (PUBLIC - 33)) | (1usize << (RETURN - 33)) | (1usize << (SHORT - 33)) | (1usize << (STATIC - 33)) | (1usize << (STRICTFP - 33)) | (1usize << (SUPER - 33)) | (1usize << (SWITCH - 33)) | (1usize << (SYNCHRONIZED - 33)) | (1usize << (THIS - 33)) | (1usize << (THROW - 33)) | (1usize << (TRANSIENT - 33)) | (1usize << (TRY - 33)) | (1usize << (VOID - 33)) | (1usize << (VOLATILE - 33)) | (1usize << (WHILE - 33)) | (1usize << (MODULE - 33)) | (1usize << (OPEN - 33)) | (1usize << (REQUIRES - 33)) | (1usize << (EXPORTS - 33)) | (1usize << (OPENS - 33)) | (1usize << (TO - 33)) | (1usize << (USES - 33)) | (1usize << (PROVIDES - 33)) | (1usize << (WITH - 33)) | (1usize << (TRANSITIVE - 33)) | (1usize << (VAR - 33)) | (1usize << (YIELD - 33)) | (1usize << (RECORD - 33)) | (1usize << (SEALED - 33)))) != 0) || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (PERMITS - 65)) | (1usize << (NON_SEALED - 65)) | (1usize << (DECIMAL_LITERAL - 65)) | (1usize << (HEX_LITERAL - 65)) | (1usize << (OCT_LITERAL - 65)) | (1usize << (BINARY_LITERAL - 65)) | (1usize << (FLOAT_LITERAL - 65)) | (1usize << (HEX_FLOAT_LITERAL - 65)) | (1usize << (BOOL_LITERAL - 65)) | (1usize << (CHAR_LITERAL - 65)) | (1usize << (STRING_LITERAL - 65)) | (1usize << (TEXT_BLOCK - 65)) | (1usize << (NULL_LITERAL - 65)) | (1usize << (LPAREN - 65)) | (1usize << (LBRACE - 65)) | (1usize << (SEMI - 65)) | (1usize << (LT - 65)) | (1usize << (BANG - 65)) | (1usize << (TILDE - 65)))) != 0) || ((((_la - 100)) & !0x3f) == 0 && ((1usize << (_la - 100)) & ((1usize << (INC - 100)) | (1usize << (DEC - 100)) | (1usize << (ADD - 100)) | (1usize << (SUB - 100)) | (1usize << (AT - 100)) | (1usize << (IDENTIFIER - 100)))) != 0) {
						{
						{
						/*InvokeRule blockStatement*/
						recog.base.set_state(1624);
						recog.blockStatement()?;

						}
						}
						recog.base.set_state(1629);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- classType ----------------
pub type ClassTypeContextAll<'input> = ClassTypeContext<'input>;


pub type ClassTypeContext<'input> = BaseParserRuleContext<'input,ClassTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ClassTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ClassTypeContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ClassTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_classType(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_classType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ClassTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_classType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_classType }
}
antlr_rust::tid!{ClassTypeContextExt<'a>}

impl<'input> ClassTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ClassTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ClassTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ClassTypeContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ClassTypeContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn classOrInterfaceType(&self) -> Option<Rc<ClassOrInterfaceTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn typeArguments(&self) -> Option<Rc<TypeArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ClassTypeContextAttrs<'input> for ClassTypeContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn classType(&mut self,)
	-> Result<Rc<ClassTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ClassTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 216, RULE_classType);
        let mut _localctx: Rc<ClassTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1635);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(198,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule classOrInterfaceType*/
					recog.base.set_state(1632);
					recog.classOrInterfaceType()?;

					recog.base.set_state(1633);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			recog.base.set_state(1640);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(199,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule annotation*/
					recog.base.set_state(1637);
					recog.annotation()?;

					}
					} 
				}
				recog.base.set_state(1642);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(199,&mut recog.base)?;
			}
			/*InvokeRule identifier*/
			recog.base.set_state(1643);
			recog.identifier()?;

			recog.base.set_state(1645);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LT {
				{
				/*InvokeRule typeArguments*/
				recog.base.set_state(1644);
				recog.typeArguments()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- creator ----------------
pub type CreatorContextAll<'input> = CreatorContext<'input>;


pub type CreatorContext<'input> = BaseParserRuleContext<'input,CreatorContextExt<'input>>;

#[derive(Clone)]
pub struct CreatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for CreatorContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for CreatorContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_creator(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_creator(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for CreatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_creator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_creator }
}
antlr_rust::tid!{CreatorContextExt<'a>}

impl<'input> CreatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CreatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CreatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CreatorContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<CreatorContextExt<'input>>{

fn nonWildcardTypeArguments(&self) -> Option<Rc<NonWildcardTypeArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn createdName(&self) -> Option<Rc<CreatedNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn classCreatorRest(&self) -> Option<Rc<ClassCreatorRestContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn arrayCreatorRest(&self) -> Option<Rc<ArrayCreatorRestContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CreatorContextAttrs<'input> for CreatorContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn creator(&mut self,)
	-> Result<Rc<CreatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CreatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 218, RULE_creator);
        let mut _localctx: Rc<CreatorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1656);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule nonWildcardTypeArguments*/
					recog.base.set_state(1647);
					recog.nonWildcardTypeArguments()?;

					/*InvokeRule createdName*/
					recog.base.set_state(1648);
					recog.createdName()?;

					/*InvokeRule classCreatorRest*/
					recog.base.set_state(1649);
					recog.classCreatorRest()?;

					}
				}

			 BOOLEAN | BYTE | CHAR | DOUBLE | FLOAT | INT | LONG | SHORT | MODULE |
			 OPEN | REQUIRES | EXPORTS | OPENS | TO | USES | PROVIDES | WITH | TRANSITIVE |
			 VAR | YIELD | RECORD | SEALED | PERMITS | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule createdName*/
					recog.base.set_state(1651);
					recog.createdName()?;

					recog.base.set_state(1654);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 LBRACK 
						=> {
							{
							/*InvokeRule arrayCreatorRest*/
							recog.base.set_state(1652);
							recog.arrayCreatorRest()?;

							}
						}

					 LPAREN 
						=> {
							{
							/*InvokeRule classCreatorRest*/
							recog.base.set_state(1653);
							recog.classCreatorRest()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- createdName ----------------
pub type CreatedNameContextAll<'input> = CreatedNameContext<'input>;


pub type CreatedNameContext<'input> = BaseParserRuleContext<'input,CreatedNameContextExt<'input>>;

#[derive(Clone)]
pub struct CreatedNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for CreatedNameContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for CreatedNameContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_createdName(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_createdName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for CreatedNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_createdName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_createdName }
}
antlr_rust::tid!{CreatedNameContextExt<'a>}

impl<'input> CreatedNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CreatedNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CreatedNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CreatedNameContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<CreatedNameContextExt<'input>>{

fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn typeArgumentsOrDiamond_all(&self) ->  Vec<Rc<TypeArgumentsOrDiamondContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeArgumentsOrDiamond(&self, i: usize) -> Option<Rc<TypeArgumentsOrDiamondContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
fn DOT_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DOT, starting from 0.
/// Returns `None` if number of children corresponding to token DOT is less or equal than `i`.
fn DOT(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DOT, i)
}
fn primitiveType(&self) -> Option<Rc<PrimitiveTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CreatedNameContextAttrs<'input> for CreatedNameContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn createdName(&mut self,)
	-> Result<Rc<CreatedNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CreatedNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 220, RULE_createdName);
        let mut _localctx: Rc<CreatedNameContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1673);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 MODULE | OPEN | REQUIRES | EXPORTS | OPENS | TO | USES | PROVIDES | WITH |
			 TRANSITIVE | VAR | YIELD | RECORD | SEALED | PERMITS | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule identifier*/
					recog.base.set_state(1658);
					recog.identifier()?;

					recog.base.set_state(1660);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LT {
						{
						/*InvokeRule typeArgumentsOrDiamond*/
						recog.base.set_state(1659);
						recog.typeArgumentsOrDiamond()?;

						}
					}

					recog.base.set_state(1669);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==DOT {
						{
						{
						recog.base.set_state(1662);
						recog.base.match_token(DOT,&mut recog.err_handler)?;

						/*InvokeRule identifier*/
						recog.base.set_state(1663);
						recog.identifier()?;

						recog.base.set_state(1665);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if _la==LT {
							{
							/*InvokeRule typeArgumentsOrDiamond*/
							recog.base.set_state(1664);
							recog.typeArgumentsOrDiamond()?;

							}
						}

						}
						}
						recog.base.set_state(1671);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

			 BOOLEAN | BYTE | CHAR | DOUBLE | FLOAT | INT | LONG | SHORT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule primitiveType*/
					recog.base.set_state(1672);
					recog.primitiveType()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- innerCreator ----------------
pub type InnerCreatorContextAll<'input> = InnerCreatorContext<'input>;


pub type InnerCreatorContext<'input> = BaseParserRuleContext<'input,InnerCreatorContextExt<'input>>;

#[derive(Clone)]
pub struct InnerCreatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for InnerCreatorContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for InnerCreatorContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_innerCreator(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_innerCreator(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for InnerCreatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_innerCreator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_innerCreator }
}
antlr_rust::tid!{InnerCreatorContextExt<'a>}

impl<'input> InnerCreatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InnerCreatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InnerCreatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InnerCreatorContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<InnerCreatorContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn classCreatorRest(&self) -> Option<Rc<ClassCreatorRestContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nonWildcardTypeArgumentsOrDiamond(&self) -> Option<Rc<NonWildcardTypeArgumentsOrDiamondContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InnerCreatorContextAttrs<'input> for InnerCreatorContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn innerCreator(&mut self,)
	-> Result<Rc<InnerCreatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InnerCreatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 222, RULE_innerCreator);
        let mut _localctx: Rc<InnerCreatorContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(1675);
			recog.identifier()?;

			recog.base.set_state(1677);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LT {
				{
				/*InvokeRule nonWildcardTypeArgumentsOrDiamond*/
				recog.base.set_state(1676);
				recog.nonWildcardTypeArgumentsOrDiamond()?;

				}
			}

			/*InvokeRule classCreatorRest*/
			recog.base.set_state(1679);
			recog.classCreatorRest()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- arrayCreatorRest ----------------
pub type ArrayCreatorRestContextAll<'input> = ArrayCreatorRestContext<'input>;


pub type ArrayCreatorRestContext<'input> = BaseParserRuleContext<'input,ArrayCreatorRestContextExt<'input>>;

#[derive(Clone)]
pub struct ArrayCreatorRestContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ArrayCreatorRestContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ArrayCreatorRestContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_arrayCreatorRest(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_arrayCreatorRest(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ArrayCreatorRestContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arrayCreatorRest }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arrayCreatorRest }
}
antlr_rust::tid!{ArrayCreatorRestContextExt<'a>}

impl<'input> ArrayCreatorRestContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArrayCreatorRestContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArrayCreatorRestContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArrayCreatorRestContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ArrayCreatorRestContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token LBRACK in current rule
fn LBRACK_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LBRACK, starting from 0.
/// Returns `None` if number of children corresponding to token LBRACK is less or equal than `i`.
fn LBRACK(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACK, i)
}
/// Retrieves all `TerminalNode`s corresponding to token RBRACK in current rule
fn RBRACK_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RBRACK, starting from 0.
/// Returns `None` if number of children corresponding to token RBRACK is less or equal than `i`.
fn RBRACK(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACK, i)
}
fn arrayInitializer(&self) -> Option<Rc<ArrayInitializerContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ArrayCreatorRestContextAttrs<'input> for ArrayCreatorRestContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn arrayCreatorRest(&mut self,)
	-> Result<Rc<ArrayCreatorRestContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArrayCreatorRestContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 224, RULE_arrayCreatorRest);
        let mut _localctx: Rc<ArrayCreatorRestContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1681);
			recog.base.match_token(LBRACK,&mut recog.err_handler)?;

			recog.base.set_state(1709);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 RBRACK 
				=> {
					{
					recog.base.set_state(1682);
					recog.base.match_token(RBRACK,&mut recog.err_handler)?;

					recog.base.set_state(1687);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==LBRACK {
						{
						{
						recog.base.set_state(1683);
						recog.base.match_token(LBRACK,&mut recog.err_handler)?;

						recog.base.set_state(1684);
						recog.base.match_token(RBRACK,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(1689);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					/*InvokeRule arrayInitializer*/
					recog.base.set_state(1690);
					recog.arrayInitializer()?;

					}
				}

			 BOOLEAN | BYTE | CHAR | DOUBLE | FLOAT | INT | LONG | NEW | SHORT | SUPER |
			 SWITCH | THIS | VOID | MODULE | OPEN | REQUIRES | EXPORTS | OPENS | TO |
			 USES | PROVIDES | WITH | TRANSITIVE | VAR | YIELD | RECORD | SEALED |
			 PERMITS | DECIMAL_LITERAL | HEX_LITERAL | OCT_LITERAL | BINARY_LITERAL |
			 FLOAT_LITERAL | HEX_FLOAT_LITERAL | BOOL_LITERAL | CHAR_LITERAL | STRING_LITERAL |
			 TEXT_BLOCK | NULL_LITERAL | LPAREN | LT | BANG | TILDE | INC | DEC |
			 ADD | SUB | AT | IDENTIFIER 
				=> {
					{
					/*InvokeRule expression*/
					recog.base.set_state(1691);
					recog.expression_rec(0)?;

					recog.base.set_state(1692);
					recog.base.match_token(RBRACK,&mut recog.err_handler)?;

					recog.base.set_state(1699);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(209,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(1693);
							recog.base.match_token(LBRACK,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(1694);
							recog.expression_rec(0)?;

							recog.base.set_state(1695);
							recog.base.match_token(RBRACK,&mut recog.err_handler)?;

							}
							} 
						}
						recog.base.set_state(1701);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(209,&mut recog.base)?;
					}
					recog.base.set_state(1706);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(210,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(1702);
							recog.base.match_token(LBRACK,&mut recog.err_handler)?;

							recog.base.set_state(1703);
							recog.base.match_token(RBRACK,&mut recog.err_handler)?;

							}
							} 
						}
						recog.base.set_state(1708);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(210,&mut recog.base)?;
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- classCreatorRest ----------------
pub type ClassCreatorRestContextAll<'input> = ClassCreatorRestContext<'input>;


pub type ClassCreatorRestContext<'input> = BaseParserRuleContext<'input,ClassCreatorRestContextExt<'input>>;

#[derive(Clone)]
pub struct ClassCreatorRestContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ClassCreatorRestContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ClassCreatorRestContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_classCreatorRest(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_classCreatorRest(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ClassCreatorRestContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_classCreatorRest }
	//fn type_rule_index() -> usize where Self: Sized { RULE_classCreatorRest }
}
antlr_rust::tid!{ClassCreatorRestContextExt<'a>}

impl<'input> ClassCreatorRestContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ClassCreatorRestContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ClassCreatorRestContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ClassCreatorRestContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ClassCreatorRestContextExt<'input>>{

fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn classBody(&self) -> Option<Rc<ClassBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ClassCreatorRestContextAttrs<'input> for ClassCreatorRestContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn classCreatorRest(&mut self,)
	-> Result<Rc<ClassCreatorRestContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ClassCreatorRestContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 226, RULE_classCreatorRest);
        let mut _localctx: Rc<ClassCreatorRestContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule arguments*/
			recog.base.set_state(1711);
			recog.arguments()?;

			recog.base.set_state(1713);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(212,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule classBody*/
					recog.base.set_state(1712);
					recog.classBody()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- explicitGenericInvocation ----------------
pub type ExplicitGenericInvocationContextAll<'input> = ExplicitGenericInvocationContext<'input>;


pub type ExplicitGenericInvocationContext<'input> = BaseParserRuleContext<'input,ExplicitGenericInvocationContextExt<'input>>;

#[derive(Clone)]
pub struct ExplicitGenericInvocationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ExplicitGenericInvocationContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ExplicitGenericInvocationContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_explicitGenericInvocation(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_explicitGenericInvocation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ExplicitGenericInvocationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_explicitGenericInvocation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_explicitGenericInvocation }
}
antlr_rust::tid!{ExplicitGenericInvocationContextExt<'a>}

impl<'input> ExplicitGenericInvocationContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExplicitGenericInvocationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExplicitGenericInvocationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExplicitGenericInvocationContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ExplicitGenericInvocationContextExt<'input>>{

fn nonWildcardTypeArguments(&self) -> Option<Rc<NonWildcardTypeArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn explicitGenericInvocationSuffix(&self) -> Option<Rc<ExplicitGenericInvocationSuffixContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExplicitGenericInvocationContextAttrs<'input> for ExplicitGenericInvocationContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn explicitGenericInvocation(&mut self,)
	-> Result<Rc<ExplicitGenericInvocationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExplicitGenericInvocationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 228, RULE_explicitGenericInvocation);
        let mut _localctx: Rc<ExplicitGenericInvocationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule nonWildcardTypeArguments*/
			recog.base.set_state(1715);
			recog.nonWildcardTypeArguments()?;

			/*InvokeRule explicitGenericInvocationSuffix*/
			recog.base.set_state(1716);
			recog.explicitGenericInvocationSuffix()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeArgumentsOrDiamond ----------------
pub type TypeArgumentsOrDiamondContextAll<'input> = TypeArgumentsOrDiamondContext<'input>;


pub type TypeArgumentsOrDiamondContext<'input> = BaseParserRuleContext<'input,TypeArgumentsOrDiamondContextExt<'input>>;

#[derive(Clone)]
pub struct TypeArgumentsOrDiamondContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for TypeArgumentsOrDiamondContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for TypeArgumentsOrDiamondContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_typeArgumentsOrDiamond(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_typeArgumentsOrDiamond(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TypeArgumentsOrDiamondContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeArgumentsOrDiamond }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeArgumentsOrDiamond }
}
antlr_rust::tid!{TypeArgumentsOrDiamondContextExt<'a>}

impl<'input> TypeArgumentsOrDiamondContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeArgumentsOrDiamondContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeArgumentsOrDiamondContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeArgumentsOrDiamondContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<TypeArgumentsOrDiamondContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LT
/// Returns `None` if there is no child corresponding to token LT
fn LT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LT, 0)
}
/// Retrieves first TerminalNode corresponding to token GT
/// Returns `None` if there is no child corresponding to token GT
fn GT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(GT, 0)
}
fn typeArguments(&self) -> Option<Rc<TypeArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TypeArgumentsOrDiamondContextAttrs<'input> for TypeArgumentsOrDiamondContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeArgumentsOrDiamond(&mut self,)
	-> Result<Rc<TypeArgumentsOrDiamondContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeArgumentsOrDiamondContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 230, RULE_typeArgumentsOrDiamond);
        let mut _localctx: Rc<TypeArgumentsOrDiamondContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1721);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(213,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1718);
					recog.base.match_token(LT,&mut recog.err_handler)?;

					recog.base.set_state(1719);
					recog.base.match_token(GT,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule typeArguments*/
					recog.base.set_state(1720);
					recog.typeArguments()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- nonWildcardTypeArgumentsOrDiamond ----------------
pub type NonWildcardTypeArgumentsOrDiamondContextAll<'input> = NonWildcardTypeArgumentsOrDiamondContext<'input>;


pub type NonWildcardTypeArgumentsOrDiamondContext<'input> = BaseParserRuleContext<'input,NonWildcardTypeArgumentsOrDiamondContextExt<'input>>;

#[derive(Clone)]
pub struct NonWildcardTypeArgumentsOrDiamondContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for NonWildcardTypeArgumentsOrDiamondContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for NonWildcardTypeArgumentsOrDiamondContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_nonWildcardTypeArgumentsOrDiamond(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_nonWildcardTypeArgumentsOrDiamond(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for NonWildcardTypeArgumentsOrDiamondContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nonWildcardTypeArgumentsOrDiamond }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nonWildcardTypeArgumentsOrDiamond }
}
antlr_rust::tid!{NonWildcardTypeArgumentsOrDiamondContextExt<'a>}

impl<'input> NonWildcardTypeArgumentsOrDiamondContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NonWildcardTypeArgumentsOrDiamondContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NonWildcardTypeArgumentsOrDiamondContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NonWildcardTypeArgumentsOrDiamondContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<NonWildcardTypeArgumentsOrDiamondContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LT
/// Returns `None` if there is no child corresponding to token LT
fn LT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LT, 0)
}
/// Retrieves first TerminalNode corresponding to token GT
/// Returns `None` if there is no child corresponding to token GT
fn GT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(GT, 0)
}
fn nonWildcardTypeArguments(&self) -> Option<Rc<NonWildcardTypeArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> NonWildcardTypeArgumentsOrDiamondContextAttrs<'input> for NonWildcardTypeArgumentsOrDiamondContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn nonWildcardTypeArgumentsOrDiamond(&mut self,)
	-> Result<Rc<NonWildcardTypeArgumentsOrDiamondContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NonWildcardTypeArgumentsOrDiamondContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 232, RULE_nonWildcardTypeArgumentsOrDiamond);
        let mut _localctx: Rc<NonWildcardTypeArgumentsOrDiamondContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1726);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(214,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1723);
					recog.base.match_token(LT,&mut recog.err_handler)?;

					recog.base.set_state(1724);
					recog.base.match_token(GT,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule nonWildcardTypeArguments*/
					recog.base.set_state(1725);
					recog.nonWildcardTypeArguments()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- nonWildcardTypeArguments ----------------
pub type NonWildcardTypeArgumentsContextAll<'input> = NonWildcardTypeArgumentsContext<'input>;


pub type NonWildcardTypeArgumentsContext<'input> = BaseParserRuleContext<'input,NonWildcardTypeArgumentsContextExt<'input>>;

#[derive(Clone)]
pub struct NonWildcardTypeArgumentsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for NonWildcardTypeArgumentsContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for NonWildcardTypeArgumentsContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_nonWildcardTypeArguments(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_nonWildcardTypeArguments(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for NonWildcardTypeArgumentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nonWildcardTypeArguments }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nonWildcardTypeArguments }
}
antlr_rust::tid!{NonWildcardTypeArgumentsContextExt<'a>}

impl<'input> NonWildcardTypeArgumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NonWildcardTypeArgumentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NonWildcardTypeArgumentsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NonWildcardTypeArgumentsContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<NonWildcardTypeArgumentsContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LT
/// Returns `None` if there is no child corresponding to token LT
fn LT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LT, 0)
}
fn typeList(&self) -> Option<Rc<TypeListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token GT
/// Returns `None` if there is no child corresponding to token GT
fn GT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(GT, 0)
}

}

impl<'input> NonWildcardTypeArgumentsContextAttrs<'input> for NonWildcardTypeArgumentsContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn nonWildcardTypeArguments(&mut self,)
	-> Result<Rc<NonWildcardTypeArgumentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NonWildcardTypeArgumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 234, RULE_nonWildcardTypeArguments);
        let mut _localctx: Rc<NonWildcardTypeArgumentsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1728);
			recog.base.match_token(LT,&mut recog.err_handler)?;

			/*InvokeRule typeList*/
			recog.base.set_state(1729);
			recog.typeList()?;

			recog.base.set_state(1730);
			recog.base.match_token(GT,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeList ----------------
pub type TypeListContextAll<'input> = TypeListContext<'input>;


pub type TypeListContext<'input> = BaseParserRuleContext<'input,TypeListContextExt<'input>>;

#[derive(Clone)]
pub struct TypeListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for TypeListContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for TypeListContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_typeList(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_typeList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TypeListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeList }
}
antlr_rust::tid!{TypeListContextExt<'a>}

impl<'input> TypeListContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeListContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<TypeListContextExt<'input>>{

fn typeType_all(&self) ->  Vec<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeType(&self, i: usize) -> Option<Rc<TypeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> TypeListContextAttrs<'input> for TypeListContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeList(&mut self,)
	-> Result<Rc<TypeListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 236, RULE_typeList);
        let mut _localctx: Rc<TypeListContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typeType*/
			recog.base.set_state(1732);
			recog.typeType()?;

			recog.base.set_state(1737);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(1733);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule typeType*/
				recog.base.set_state(1734);
				recog.typeType()?;

				}
				}
				recog.base.set_state(1739);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeType ----------------
pub type TypeTypeContextAll<'input> = TypeTypeContext<'input>;


pub type TypeTypeContext<'input> = BaseParserRuleContext<'input,TypeTypeContextExt<'input>>;

#[derive(Clone)]
pub struct TypeTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for TypeTypeContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for TypeTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_typeType(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_typeType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TypeTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeType }
}
antlr_rust::tid!{TypeTypeContextExt<'a>}

impl<'input> TypeTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeTypeContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<TypeTypeContextExt<'input>>{

fn classOrInterfaceType(&self) -> Option<Rc<ClassOrInterfaceTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn primitiveType(&self) -> Option<Rc<PrimitiveTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token LBRACK in current rule
fn LBRACK_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LBRACK, starting from 0.
/// Returns `None` if number of children corresponding to token LBRACK is less or equal than `i`.
fn LBRACK(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LBRACK, i)
}
/// Retrieves all `TerminalNode`s corresponding to token RBRACK in current rule
fn RBRACK_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RBRACK, starting from 0.
/// Returns `None` if number of children corresponding to token RBRACK is less or equal than `i`.
fn RBRACK(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RBRACK, i)
}

}

impl<'input> TypeTypeContextAttrs<'input> for TypeTypeContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeType(&mut self,)
	-> Result<Rc<TypeTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 238, RULE_typeType);
        let mut _localctx: Rc<TypeTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1743);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(216,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule annotation*/
					recog.base.set_state(1740);
					recog.annotation()?;

					}
					} 
				}
				recog.base.set_state(1745);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(216,&mut recog.base)?;
			}
			recog.base.set_state(1748);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 MODULE | OPEN | REQUIRES | EXPORTS | OPENS | TO | USES | PROVIDES | WITH |
			 TRANSITIVE | VAR | YIELD | RECORD | SEALED | PERMITS | IDENTIFIER 
				=> {
					{
					/*InvokeRule classOrInterfaceType*/
					recog.base.set_state(1746);
					recog.classOrInterfaceType()?;

					}
				}

			 BOOLEAN | BYTE | CHAR | DOUBLE | FLOAT | INT | LONG | SHORT 
				=> {
					{
					/*InvokeRule primitiveType*/
					recog.base.set_state(1747);
					recog.primitiveType()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(1760);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(219,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(1753);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 51)) & !0x3f) == 0 && ((1usize << (_la - 51)) & ((1usize << (MODULE - 51)) | (1usize << (OPEN - 51)) | (1usize << (REQUIRES - 51)) | (1usize << (EXPORTS - 51)) | (1usize << (OPENS - 51)) | (1usize << (TO - 51)) | (1usize << (USES - 51)) | (1usize << (PROVIDES - 51)) | (1usize << (WITH - 51)) | (1usize << (TRANSITIVE - 51)) | (1usize << (VAR - 51)) | (1usize << (YIELD - 51)) | (1usize << (RECORD - 51)) | (1usize << (SEALED - 51)) | (1usize << (PERMITS - 51)))) != 0) || _la==AT || _la==IDENTIFIER {
						{
						{
						/*InvokeRule annotation*/
						recog.base.set_state(1750);
						recog.annotation()?;

						}
						}
						recog.base.set_state(1755);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(1756);
					recog.base.match_token(LBRACK,&mut recog.err_handler)?;

					recog.base.set_state(1757);
					recog.base.match_token(RBRACK,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(1762);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(219,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- primitiveType ----------------
pub type PrimitiveTypeContextAll<'input> = PrimitiveTypeContext<'input>;


pub type PrimitiveTypeContext<'input> = BaseParserRuleContext<'input,PrimitiveTypeContextExt<'input>>;

#[derive(Clone)]
pub struct PrimitiveTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for PrimitiveTypeContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for PrimitiveTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_primitiveType(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_primitiveType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PrimitiveTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primitiveType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primitiveType }
}
antlr_rust::tid!{PrimitiveTypeContextExt<'a>}

impl<'input> PrimitiveTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PrimitiveTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrimitiveTypeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PrimitiveTypeContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<PrimitiveTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BOOLEAN
/// Returns `None` if there is no child corresponding to token BOOLEAN
fn BOOLEAN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(BOOLEAN, 0)
}
/// Retrieves first TerminalNode corresponding to token CHAR
/// Returns `None` if there is no child corresponding to token CHAR
fn CHAR(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token BYTE
/// Returns `None` if there is no child corresponding to token BYTE
fn BYTE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(BYTE, 0)
}
/// Retrieves first TerminalNode corresponding to token SHORT
/// Returns `None` if there is no child corresponding to token SHORT
fn SHORT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SHORT, 0)
}
/// Retrieves first TerminalNode corresponding to token INT
/// Returns `None` if there is no child corresponding to token INT
fn INT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(INT, 0)
}
/// Retrieves first TerminalNode corresponding to token LONG
/// Returns `None` if there is no child corresponding to token LONG
fn LONG(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LONG, 0)
}
/// Retrieves first TerminalNode corresponding to token FLOAT
/// Returns `None` if there is no child corresponding to token FLOAT
fn FLOAT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(FLOAT, 0)
}
/// Retrieves first TerminalNode corresponding to token DOUBLE
/// Returns `None` if there is no child corresponding to token DOUBLE
fn DOUBLE(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DOUBLE, 0)
}

}

impl<'input> PrimitiveTypeContextAttrs<'input> for PrimitiveTypeContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn primitiveType(&mut self,)
	-> Result<Rc<PrimitiveTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrimitiveTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 240, RULE_primitiveType);
        let mut _localctx: Rc<PrimitiveTypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1763);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << DOUBLE) | (1usize << FLOAT) | (1usize << INT) | (1usize << LONG))) != 0) || _la==SHORT) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeArguments ----------------
pub type TypeArgumentsContextAll<'input> = TypeArgumentsContext<'input>;


pub type TypeArgumentsContext<'input> = BaseParserRuleContext<'input,TypeArgumentsContextExt<'input>>;

#[derive(Clone)]
pub struct TypeArgumentsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for TypeArgumentsContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for TypeArgumentsContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_typeArguments(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_typeArguments(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TypeArgumentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeArguments }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeArguments }
}
antlr_rust::tid!{TypeArgumentsContextExt<'a>}

impl<'input> TypeArgumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeArgumentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeArgumentsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeArgumentsContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<TypeArgumentsContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LT
/// Returns `None` if there is no child corresponding to token LT
fn LT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LT, 0)
}
fn typeArgument_all(&self) ->  Vec<Rc<TypeArgumentContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeArgument(&self, i: usize) -> Option<Rc<TypeArgumentContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token GT
/// Returns `None` if there is no child corresponding to token GT
fn GT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(GT, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,JavaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> TypeArgumentsContextAttrs<'input> for TypeArgumentsContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeArguments(&mut self,)
	-> Result<Rc<TypeArgumentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeArgumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 242, RULE_typeArguments);
        let mut _localctx: Rc<TypeArgumentsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1765);
			recog.base.match_token(LT,&mut recog.err_handler)?;

			/*InvokeRule typeArgument*/
			recog.base.set_state(1766);
			recog.typeArgument()?;

			recog.base.set_state(1771);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(1767);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule typeArgument*/
				recog.base.set_state(1768);
				recog.typeArgument()?;

				}
				}
				recog.base.set_state(1773);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(1774);
			recog.base.match_token(GT,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- superSuffix ----------------
pub type SuperSuffixContextAll<'input> = SuperSuffixContext<'input>;


pub type SuperSuffixContext<'input> = BaseParserRuleContext<'input,SuperSuffixContextExt<'input>>;

#[derive(Clone)]
pub struct SuperSuffixContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for SuperSuffixContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for SuperSuffixContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_superSuffix(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_superSuffix(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for SuperSuffixContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_superSuffix }
	//fn type_rule_index() -> usize where Self: Sized { RULE_superSuffix }
}
antlr_rust::tid!{SuperSuffixContextExt<'a>}

impl<'input> SuperSuffixContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SuperSuffixContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SuperSuffixContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SuperSuffixContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<SuperSuffixContextExt<'input>>{

fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeArguments(&self) -> Option<Rc<TypeArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SuperSuffixContextAttrs<'input> for SuperSuffixContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn superSuffix(&mut self,)
	-> Result<Rc<SuperSuffixContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SuperSuffixContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 244, RULE_superSuffix);
        let mut _localctx: Rc<SuperSuffixContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1785);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LPAREN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule arguments*/
					recog.base.set_state(1776);
					recog.arguments()?;

					}
				}

			 DOT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1777);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					recog.base.set_state(1779);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LT {
						{
						/*InvokeRule typeArguments*/
						recog.base.set_state(1778);
						recog.typeArguments()?;

						}
					}

					/*InvokeRule identifier*/
					recog.base.set_state(1781);
					recog.identifier()?;

					recog.base.set_state(1783);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(222,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule arguments*/
							recog.base.set_state(1782);
							recog.arguments()?;

							}
						}

						_ => {}
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- explicitGenericInvocationSuffix ----------------
pub type ExplicitGenericInvocationSuffixContextAll<'input> = ExplicitGenericInvocationSuffixContext<'input>;


pub type ExplicitGenericInvocationSuffixContext<'input> = BaseParserRuleContext<'input,ExplicitGenericInvocationSuffixContextExt<'input>>;

#[derive(Clone)]
pub struct ExplicitGenericInvocationSuffixContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ExplicitGenericInvocationSuffixContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ExplicitGenericInvocationSuffixContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_explicitGenericInvocationSuffix(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_explicitGenericInvocationSuffix(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ExplicitGenericInvocationSuffixContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_explicitGenericInvocationSuffix }
	//fn type_rule_index() -> usize where Self: Sized { RULE_explicitGenericInvocationSuffix }
}
antlr_rust::tid!{ExplicitGenericInvocationSuffixContextExt<'a>}

impl<'input> ExplicitGenericInvocationSuffixContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExplicitGenericInvocationSuffixContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExplicitGenericInvocationSuffixContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExplicitGenericInvocationSuffixContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ExplicitGenericInvocationSuffixContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SUPER
/// Returns `None` if there is no child corresponding to token SUPER
fn SUPER(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(SUPER, 0)
}
fn superSuffix(&self) -> Option<Rc<SuperSuffixContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExplicitGenericInvocationSuffixContextAttrs<'input> for ExplicitGenericInvocationSuffixContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn explicitGenericInvocationSuffix(&mut self,)
	-> Result<Rc<ExplicitGenericInvocationSuffixContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExplicitGenericInvocationSuffixContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 246, RULE_explicitGenericInvocationSuffix);
        let mut _localctx: Rc<ExplicitGenericInvocationSuffixContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1792);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 SUPER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1787);
					recog.base.match_token(SUPER,&mut recog.err_handler)?;

					/*InvokeRule superSuffix*/
					recog.base.set_state(1788);
					recog.superSuffix()?;

					}
				}

			 MODULE | OPEN | REQUIRES | EXPORTS | OPENS | TO | USES | PROVIDES | WITH |
			 TRANSITIVE | VAR | YIELD | RECORD | SEALED | PERMITS | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule identifier*/
					recog.base.set_state(1789);
					recog.identifier()?;

					/*InvokeRule arguments*/
					recog.base.set_state(1790);
					recog.arguments()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- arguments ----------------
pub type ArgumentsContextAll<'input> = ArgumentsContext<'input>;


pub type ArgumentsContext<'input> = BaseParserRuleContext<'input,ArgumentsContextExt<'input>>;

#[derive(Clone)]
pub struct ArgumentsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> JavaParserContext<'input> for ArgumentsContext<'input>{}

impl<'input,'a> Listenable<dyn JavaParserListener<'input> + 'a> for ArgumentsContext<'input>{
		fn enter(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_arguments(self);
		}fn exit(&self,listener: &mut (dyn JavaParserListener<'input> + 'a)) {
			listener.exit_arguments(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ArgumentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = JavaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arguments }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arguments }
}
antlr_rust::tid!{ArgumentsContextExt<'a>}

impl<'input> ArgumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn JavaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArgumentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArgumentsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArgumentsContextAttrs<'input>: JavaParserContext<'input> + BorrowMut<ArgumentsContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,JavaParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
fn expressionList(&self) -> Option<Rc<ExpressionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ArgumentsContextAttrs<'input> for ArgumentsContext<'input>{}

impl<'input, I, H> JavaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn arguments(&mut self,)
	-> Result<Rc<ArgumentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArgumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 248, RULE_arguments);
        let mut _localctx: Rc<ArgumentsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1794);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(1796);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BOOLEAN) | (1usize << BYTE) | (1usize << CHAR) | (1usize << DOUBLE) | (1usize << FLOAT) | (1usize << INT) | (1usize << LONG) | (1usize << NEW))) != 0) || ((((_la - 37)) & !0x3f) == 0 && ((1usize << (_la - 37)) & ((1usize << (SHORT - 37)) | (1usize << (SUPER - 37)) | (1usize << (SWITCH - 37)) | (1usize << (THIS - 37)) | (1usize << (VOID - 37)) | (1usize << (MODULE - 37)) | (1usize << (OPEN - 37)) | (1usize << (REQUIRES - 37)) | (1usize << (EXPORTS - 37)) | (1usize << (OPENS - 37)) | (1usize << (TO - 37)) | (1usize << (USES - 37)) | (1usize << (PROVIDES - 37)) | (1usize << (WITH - 37)) | (1usize << (TRANSITIVE - 37)) | (1usize << (VAR - 37)) | (1usize << (YIELD - 37)) | (1usize << (RECORD - 37)) | (1usize << (SEALED - 37)) | (1usize << (PERMITS - 37)) | (1usize << (DECIMAL_LITERAL - 37)) | (1usize << (HEX_LITERAL - 37)))) != 0) || ((((_la - 69)) & !0x3f) == 0 && ((1usize << (_la - 69)) & ((1usize << (OCT_LITERAL - 69)) | (1usize << (BINARY_LITERAL - 69)) | (1usize << (FLOAT_LITERAL - 69)) | (1usize << (HEX_FLOAT_LITERAL - 69)) | (1usize << (BOOL_LITERAL - 69)) | (1usize << (CHAR_LITERAL - 69)) | (1usize << (STRING_LITERAL - 69)) | (1usize << (TEXT_BLOCK - 69)) | (1usize << (NULL_LITERAL - 69)) | (1usize << (LPAREN - 69)) | (1usize << (LT - 69)) | (1usize << (BANG - 69)) | (1usize << (TILDE - 69)) | (1usize << (INC - 69)))) != 0) || ((((_la - 101)) & !0x3f) == 0 && ((1usize << (_la - 101)) & ((1usize << (DEC - 101)) | (1usize << (ADD - 101)) | (1usize << (SUB - 101)) | (1usize << (AT - 101)) | (1usize << (IDENTIFIER - 101)))) != 0) {
				{
				/*InvokeRule expressionList*/
				recog.base.set_state(1795);
				recog.expressionList()?;

				}
			}

			recog.base.set_state(1798);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\u{82}\u{70b}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x04\x32\x09\
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x04\x36\x09\x36\x04\
	\x37\x09\x37\x04\x38\x09\x38\x04\x39\x09\x39\x04\x3a\x09\x3a\x04\x3b\x09\
	\x3b\x04\x3c\x09\x3c\x04\x3d\x09\x3d\x04\x3e\x09\x3e\x04\x3f\x09\x3f\x04\
	\x40\x09\x40\x04\x41\x09\x41\x04\x42\x09\x42\x04\x43\x09\x43\x04\x44\x09\
	\x44\x04\x45\x09\x45\x04\x46\x09\x46\x04\x47\x09\x47\x04\x48\x09\x48\x04\
	\x49\x09\x49\x04\x4a\x09\x4a\x04\x4b\x09\x4b\x04\x4c\x09\x4c\x04\x4d\x09\
	\x4d\x04\x4e\x09\x4e\x04\x4f\x09\x4f\x04\x50\x09\x50\x04\x51\x09\x51\x04\
	\x52\x09\x52\x04\x53\x09\x53\x04\x54\x09\x54\x04\x55\x09\x55\x04\x56\x09\
	\x56\x04\x57\x09\x57\x04\x58\x09\x58\x04\x59\x09\x59\x04\x5a\x09\x5a\x04\
	\x5b\x09\x5b\x04\x5c\x09\x5c\x04\x5d\x09\x5d\x04\x5e\x09\x5e\x04\x5f\x09\
	\x5f\x04\x60\x09\x60\x04\x61\x09\x61\x04\x62\x09\x62\x04\x63\x09\x63\x04\
	\x64\x09\x64\x04\x65\x09\x65\x04\x66\x09\x66\x04\x67\x09\x67\x04\x68\x09\
	\x68\x04\x69\x09\x69\x04\x6a\x09\x6a\x04\x6b\x09\x6b\x04\x6c\x09\x6c\x04\
	\x6d\x09\x6d\x04\x6e\x09\x6e\x04\x6f\x09\x6f\x04\x70\x09\x70\x04\x71\x09\
	\x71\x04\x72\x09\x72\x04\x73\x09\x73\x04\x74\x09\x74\x04\x75\x09\x75\x04\
	\x76\x09\x76\x04\x77\x09\x77\x04\x78\x09\x78\x04\x79\x09\x79\x04\x7a\x09\
	\x7a\x04\x7b\x09\x7b\x04\x7c\x09\x7c\x04\x7d\x09\x7d\x04\x7e\x09\x7e\x03\
	\x02\x07\x02\u{fe}\x0a\x02\x0c\x02\x0e\x02\u{101}\x0b\x02\x03\x02\x05\x02\
	\u{104}\x0a\x02\x03\x02\x07\x02\u{107}\x0a\x02\x0c\x02\x0e\x02\u{10a}\x0b\
	\x02\x03\x02\x07\x02\u{10d}\x0a\x02\x0c\x02\x0e\x02\u{110}\x0b\x02\x03\x02\
	\x03\x02\x03\x02\x05\x02\u{115}\x0a\x02\x03\x03\x07\x03\u{118}\x0a\x03\x0c\
	\x03\x0e\x03\u{11b}\x0b\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x04\x03\
	\x04\x05\x04\u{123}\x0a\x04\x03\x04\x03\x04\x03\x04\x05\x04\u{128}\x0a\x04\
	\x03\x04\x03\x04\x03\x05\x07\x05\u{12d}\x0a\x05\x0c\x05\x0e\x05\u{130}\x0b\
	\x05\x03\x05\x07\x05\u{133}\x0a\x05\x0c\x05\x0e\x05\u{136}\x0b\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x03\x05\x05\x05\u{13d}\x0a\x05\x03\x05\x05\x05\
	\u{140}\x0a\x05\x03\x06\x03\x06\x03\x07\x03\x07\x05\x07\u{146}\x0a\x07\x03\
	\x08\x03\x08\x03\x08\x05\x08\u{14b}\x0a\x08\x03\x08\x03\x08\x05\x08\u{14f}\
	\x0a\x08\x03\x08\x03\x08\x05\x08\u{153}\x0a\x08\x03\x08\x03\x08\x05\x08\
	\u{157}\x0a\x08\x03\x08\x03\x08\x03\x09\x03\x09\x03\x09\x03\x09\x07\x09\
	\u{15f}\x0a\x09\x0c\x09\x0e\x09\u{162}\x0b\x09\x03\x09\x03\x09\x03\x0a\x07\
	\x0a\u{167}\x0a\x0a\x0c\x0a\x0e\x0a\u{16a}\x0b\x0a\x03\x0a\x03\x0a\x03\x0a\
	\x07\x0a\u{16f}\x0a\x0a\x0c\x0a\x0e\x0a\u{172}\x0b\x0a\x03\x0a\x05\x0a\u{175}\
	\x0a\x0a\x03\x0b\x03\x0b\x03\x0b\x07\x0b\u{17a}\x0a\x0b\x0c\x0b\x0e\x0b\
	\u{17d}\x0b\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x05\x0c\u{183}\x0a\x0c\x03\
	\x0c\x03\x0c\x05\x0c\u{187}\x0a\x0c\x03\x0c\x05\x0c\u{18a}\x0a\x0c\x03\x0c\
	\x05\x0c\u{18d}\x0a\x0c\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0d\x07\x0d\
	\u{194}\x0a\x0d\x0c\x0d\x0e\x0d\u{197}\x0b\x0d\x03\x0e\x07\x0e\u{19a}\x0a\
	\x0e\x0c\x0e\x0e\x0e\u{19d}\x0b\x0e\x03\x0e\x03\x0e\x05\x0e\u{1a1}\x0a\x0e\
	\x03\x0e\x05\x0e\u{1a4}\x0a\x0e\x03\x0f\x03\x0f\x07\x0f\u{1a8}\x0a\x0f\x0c\
	\x0f\x0e\x0f\u{1ab}\x0b\x0f\x03\x10\x03\x10\x03\x10\x05\x10\u{1b0}\x0a\x10\
	\x03\x10\x03\x10\x05\x10\u{1b4}\x0a\x10\x03\x10\x03\x10\x05\x10\u{1b8}\x0a\
	\x10\x03\x10\x03\x10\x03\x11\x03\x11\x07\x11\u{1be}\x0a\x11\x0c\x11\x0e\
	\x11\u{1c1}\x0b\x11\x03\x11\x03\x11\x03\x12\x03\x12\x07\x12\u{1c7}\x0a\x12\
	\x0c\x12\x0e\x12\u{1ca}\x0b\x12\x03\x12\x03\x12\x03\x13\x03\x13\x05\x13\
	\u{1d0}\x0a\x13\x03\x13\x03\x13\x07\x13\u{1d4}\x0a\x13\x0c\x13\x0e\x13\u{1d7}\
	\x0b\x13\x03\x13\x07\x13\u{1da}\x0a\x13\x0c\x13\x0e\x13\u{1dd}\x0b\x13\x03\
	\x13\x05\x13\u{1e0}\x0a\x13\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\
	\x14\x03\x14\x03\x14\x03\x14\x03\x14\x05\x14\u{1ec}\x0a\x14\x03\x15\x03\
	\x15\x03\x15\x03\x15\x03\x15\x07\x15\u{1f3}\x0a\x15\x0c\x15\x0e\x15\u{1f6}\
	\x0b\x15\x03\x15\x03\x15\x05\x15\u{1fa}\x0a\x15\x03\x15\x03\x15\x03\x16\
	\x03\x16\x05\x16\u{200}\x0a\x16\x03\x17\x03\x17\x05\x17\u{204}\x0a\x17\x03\
	\x18\x03\x18\x03\x18\x03\x19\x03\x19\x03\x19\x03\x1a\x03\x1a\x03\x1a\x03\
	\x1a\x05\x1a\u{210}\x0a\x1a\x03\x1a\x03\x1a\x03\x1b\x07\x1b\u{215}\x0a\x1b\
	\x0c\x1b\x0e\x1b\u{218}\x0b\x1b\x03\x1b\x07\x1b\u{21b}\x0a\x1b\x0c\x1b\x0e\
	\x1b\u{21e}\x0b\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1c\x03\x1c\x03\x1c\x03\
	\x1c\x03\x1d\x07\x1d\u{228}\x0a\x1d\x0c\x1d\x0e\x1d\u{22b}\x0b\x1d\x03\x1d\
	\x07\x1d\u{22e}\x0a\x1d\x0c\x1d\x0e\x1d\u{231}\x0b\x1d\x03\x1d\x03\x1d\x05\
	\x1d\u{235}\x0a\x1d\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\
	\x1e\x03\x1e\x05\x1e\u{23f}\x0a\x1e\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x07\
	\x1f\u{245}\x0a\x1f\x0c\x1f\x0e\x1f\u{248}\x0b\x1f\x03\x1f\x03\x1f\x03\x20\
	\x03\x20\x03\x20\x07\x20\u{24f}\x0a\x20\x0c\x20\x0e\x20\u{252}\x0b\x20\x03\
	\x20\x03\x20\x03\x20\x03\x21\x07\x21\u{258}\x0a\x21\x0c\x21\x0e\x21\u{25b}\
	\x0b\x21\x03\x21\x03\x21\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\
	\x05\x22\u{265}\x0a\x22\x03\x23\x07\x23\u{268}\x0a\x23\x0c\x23\x0e\x23\u{26b}\
	\x0b\x23\x03\x23\x03\x23\x03\x23\x03\x24\x07\x24\u{271}\x0a\x24\x0c\x24\
	\x0e\x24\u{274}\x0b\x24\x03\x24\x03\x24\x03\x24\x03\x24\x03\x24\x07\x24\
	\u{27b}\x0a\x24\x0c\x24\x0e\x24\u{27e}\x0b\x24\x03\x24\x03\x24\x05\x24\u{282}\
	\x0a\x24\x03\x24\x03\x24\x03\x25\x03\x25\x03\x25\x07\x25\u{289}\x0a\x25\
	\x0c\x25\x0e\x25\u{28c}\x0b\x25\x03\x26\x03\x26\x03\x26\x05\x26\u{291}\x0a\
	\x26\x03\x27\x03\x27\x03\x27\x07\x27\u{296}\x0a\x27\x0c\x27\x0e\x27\u{299}\
	\x0b\x27\x03\x28\x03\x28\x05\x28\u{29d}\x0a\x28\x03\x29\x03\x29\x03\x29\
	\x03\x29\x07\x29\u{2a3}\x0a\x29\x0c\x29\x0e\x29\u{2a6}\x0b\x29\x03\x29\x05\
	\x29\u{2a9}\x0a\x29\x05\x29\u{2ab}\x0a\x29\x03\x29\x03\x29\x03\x2a\x03\x2a\
	\x05\x2a\u{2b1}\x0a\x2a\x03\x2a\x03\x2a\x07\x2a\u{2b5}\x0a\x2a\x0c\x2a\x0e\
	\x2a\u{2b8}\x0b\x2a\x03\x2a\x03\x2a\x05\x2a\u{2bc}\x0a\x2a\x03\x2b\x03\x2b\
	\x07\x2b\u{2c0}\x0a\x2b\x0c\x2b\x0e\x2b\u{2c3}\x0b\x2b\x03\x2b\x03\x2b\x03\
	\x2b\x05\x2b\u{2c8}\x0a\x2b\x05\x2b\u{2ca}\x0a\x2b\x03\x2c\x03\x2c\x03\x2c\
	\x07\x2c\u{2cf}\x0a\x2c\x0c\x2c\x0e\x2c\u{2d2}\x0b\x2c\x03\x2d\x03\x2d\x05\
	\x2d\u{2d6}\x0a\x2d\x03\x2d\x03\x2d\x03\x2d\x05\x2d\u{2db}\x0a\x2d\x03\x2d\
	\x05\x2d\u{2de}\x0a\x2d\x05\x2d\u{2e0}\x0a\x2d\x03\x2d\x03\x2d\x03\x2e\x03\
	\x2e\x03\x2e\x03\x2e\x07\x2e\u{2e8}\x0a\x2e\x0c\x2e\x0e\x2e\u{2eb}\x0b\x2e\
	\x03\x2e\x03\x2e\x03\x2f\x03\x2f\x03\x2f\x07\x2f\u{2f2}\x0a\x2f\x0c\x2f\
	\x0e\x2f\u{2f5}\x0b\x2f\x03\x2f\x03\x2f\x05\x2f\u{2f9}\x0a\x2f\x03\x2f\x05\
	\x2f\u{2fc}\x0a\x2f\x03\x30\x07\x30\u{2ff}\x0a\x30\x0c\x30\x0e\x30\u{302}\
	\x0b\x30\x03\x30\x03\x30\x03\x30\x03\x31\x07\x31\u{308}\x0a\x31\x0c\x31\
	\x0e\x31\u{30b}\x0b\x31\x03\x31\x03\x31\x07\x31\u{30f}\x0a\x31\x0c\x31\x0e\
	\x31\u{312}\x0b\x31\x03\x31\x03\x31\x03\x31\x03\x32\x03\x32\x03\x32\x07\
	\x32\u{31a}\x0a\x32\x0c\x32\x0e\x32\u{31d}\x0b\x32\x03\x33\x07\x33\u{320}\
	\x0a\x33\x0c\x33\x0e\x33\u{323}\x0b\x33\x03\x33\x03\x33\x03\x33\x03\x34\
	\x03\x34\x03\x34\x07\x34\u{32b}\x0a\x34\x0c\x34\x0e\x34\u{32e}\x0b\x34\x03\
	\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x05\x35\u{337}\x0a\
	\x35\x03\x36\x03\x36\x03\x37\x03\x37\x03\x38\x03\x38\x03\x38\x07\x38\u{340}\
	\x0a\x38\x0c\x38\x0e\x38\u{343}\x0b\x38\x03\x38\x03\x38\x03\x38\x03\x39\
	\x03\x39\x03\x39\x05\x39\u{34b}\x0a\x39\x03\x39\x03\x39\x03\x39\x05\x39\
	\u{350}\x0a\x39\x03\x39\x05\x39\u{353}\x0a\x39\x03\x3a\x03\x3a\x03\x3a\x07\
	\x3a\u{358}\x0a\x3a\x0c\x3a\x0e\x3a\u{35b}\x0b\x3a\x03\x3b\x03\x3b\x03\x3b\
	\x03\x3b\x03\x3c\x03\x3c\x03\x3c\x05\x3c\u{364}\x0a\x3c\x03\x3d\x03\x3d\
	\x03\x3d\x03\x3d\x07\x3d\u{36a}\x0a\x3d\x0c\x3d\x0e\x3d\u{36d}\x0b\x3d\x05\
	\x3d\u{36f}\x0a\x3d\x03\x3d\x05\x3d\u{372}\x0a\x3d\x03\x3d\x03\x3d\x03\x3e\
	\x03\x3e\x03\x3e\x03\x3e\x03\x3e\x03\x3f\x03\x3f\x07\x3f\u{37d}\x0a\x3f\
	\x0c\x3f\x0e\x3f\u{380}\x0b\x3f\x03\x3f\x03\x3f\x03\x40\x07\x40\u{385}\x0a\
	\x40\x0c\x40\x0e\x40\u{388}\x0b\x40\x03\x40\x07\x40\u{38b}\x0a\x40\x0c\x40\
	\x0e\x40\u{38e}\x0b\x40\x03\x40\x03\x40\x05\x40\u{392}\x0a\x40\x03\x41\x03\
	\x41\x03\x41\x03\x41\x03\x41\x03\x41\x05\x41\u{39a}\x0a\x41\x03\x41\x03\
	\x41\x05\x41\u{39e}\x0a\x41\x03\x41\x03\x41\x05\x41\u{3a2}\x0a\x41\x03\x41\
	\x03\x41\x05\x41\u{3a6}\x0a\x41\x03\x41\x03\x41\x05\x41\u{3aa}\x0a\x41\x05\
	\x41\u{3ac}\x0a\x41\x03\x42\x03\x42\x05\x42\u{3b0}\x0a\x42\x03\x43\x03\x43\
	\x03\x43\x03\x43\x05\x43\u{3b6}\x0a\x43\x03\x44\x03\x44\x03\x45\x03\x45\
	\x03\x45\x03\x46\x05\x46\u{3be}\x0a\x46\x03\x46\x03\x46\x03\x46\x03\x46\
	\x03\x47\x03\x47\x07\x47\u{3c6}\x0a\x47\x0c\x47\x0e\x47\u{3c9}\x0b\x47\x03\
	\x47\x03\x47\x03\x48\x03\x48\x07\x48\u{3cf}\x0a\x48\x0c\x48\x0e\x48\u{3d2}\
	\x0b\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x05\x48\
	\u{3db}\x0a\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x05\x48\
	\u{3e3}\x0a\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\
	\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x05\x48\u{3f1}\x0a\x48\x03\x49\
	\x03\x49\x03\x4a\x03\x4a\x03\x4a\x05\x4a\u{3f8}\x0a\x4a\x03\x4a\x03\x4a\
	\x03\x4a\x05\x4a\u{3fd}\x0a\x4a\x03\x4a\x03\x4a\x03\x4b\x03\x4b\x05\x4b\
	\u{403}\x0a\x4b\x03\x4b\x03\x4b\x03\x4c\x03\x4c\x03\x4c\x07\x4c\u{40a}\x0a\
	\x4c\x0c\x4c\x0e\x4c\u{40d}\x0b\x4c\x03\x4d\x03\x4d\x03\x4d\x03\x4e\x03\
	\x4e\x03\x4e\x07\x4e\u{415}\x0a\x4e\x0c\x4e\x0e\x4e\u{418}\x0b\x4e\x03\x4e\
	\x03\x4e\x03\x4f\x03\x4f\x07\x4f\u{41e}\x0a\x4f\x0c\x4f\x0e\x4f\u{421}\x0b\
	\x4f\x03\x4f\x03\x4f\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x05\x50\u{42a}\
	\x0a\x50\x03\x51\x07\x51\u{42d}\x0a\x51\x0c\x51\x0e\x51\u{430}\x0b\x51\x03\
	\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x05\x51\u{43a}\
	\x0a\x51\x03\x52\x03\x52\x03\x53\x03\x53\x03\x54\x07\x54\u{441}\x0a\x54\
	\x0c\x54\x0e\x54\u{444}\x0b\x54\x03\x54\x07\x54\u{447}\x0a\x54\x0c\x54\x0e\
	\x54\u{44a}\x0b\x54\x03\x54\x03\x54\x03\x54\x05\x54\u{44f}\x0a\x54\x03\x55\
	\x03\x55\x03\x55\x03\x55\x03\x55\x05\x55\u{456}\x0a\x55\x03\x55\x03\x55\
	\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x05\x55\u{45f}\x0a\x55\x03\x55\
	\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\
	\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\
	\x06\x55\u{474}\x0a\x55\x0d\x55\x0e\x55\u{475}\x03\x55\x05\x55\u{479}\x0a\
	\x55\x03\x55\x05\x55\u{47c}\x0a\x55\x03\x55\x03\x55\x03\x55\x03\x55\x07\
	\x55\u{482}\x0a\x55\x0c\x55\x0e\x55\u{485}\x0b\x55\x03\x55\x05\x55\u{488}\
	\x0a\x55\x03\x55\x03\x55\x03\x55\x03\x55\x07\x55\u{48e}\x0a\x55\x0c\x55\
	\x0e\x55\u{491}\x0b\x55\x03\x55\x07\x55\u{494}\x0a\x55\x0c\x55\x0e\x55\u{497}\
	\x0b\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\
	\x05\x55\u{4a1}\x0a\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\
	\x03\x55\x05\x55\u{4aa}\x0a\x55\x03\x55\x03\x55\x03\x55\x05\x55\u{4af}\x0a\
	\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x03\
	\x55\x03\x55\x03\x55\x05\x55\u{4bc}\x0a\x55\x03\x55\x03\x55\x03\x55\x03\
	\x55\x05\x55\u{4c2}\x0a\x55\x03\x56\x03\x56\x03\x56\x07\x56\u{4c7}\x0a\x56\
	\x0c\x56\x0e\x56\u{4ca}\x0b\x56\x03\x56\x03\x56\x03\x56\x03\x56\x03\x56\
	\x03\x57\x03\x57\x03\x57\x07\x57\u{4d4}\x0a\x57\x0c\x57\x0e\x57\u{4d7}\x0b\
	\x57\x03\x58\x03\x58\x03\x58\x03\x59\x03\x59\x03\x59\x05\x59\u{4df}\x0a\
	\x59\x03\x59\x03\x59\x03\x5a\x03\x5a\x03\x5a\x07\x5a\u{4e6}\x0a\x5a\x0c\
	\x5a\x0e\x5a\u{4e9}\x0b\x5a\x03\x5b\x07\x5b\u{4ec}\x0a\x5b\x0c\x5b\x0e\x5b\
	\u{4ef}\x0b\x5b\x03\x5b\x03\x5b\x03\x5b\x03\x5b\x03\x5b\x05\x5b\u{4f6}\x0a\
	\x5b\x03\x5b\x03\x5b\x03\x5b\x03\x5b\x05\x5b\u{4fc}\x0a\x5b\x03\x5c\x06\
	\x5c\u{4ff}\x0a\x5c\x0d\x5c\x0e\x5c\u{500}\x03\x5c\x06\x5c\u{504}\x0a\x5c\
	\x0d\x5c\x0e\x5c\u{505}\x03\x5d\x03\x5d\x03\x5d\x03\x5d\x03\x5d\x03\x5d\
	\x05\x5d\u{50e}\x0a\x5d\x03\x5d\x03\x5d\x03\x5d\x05\x5d\u{513}\x0a\x5d\x03\
	\x5e\x03\x5e\x05\x5e\u{517}\x0a\x5e\x03\x5e\x03\x5e\x05\x5e\u{51b}\x0a\x5e\
	\x03\x5e\x03\x5e\x05\x5e\u{51f}\x0a\x5e\x05\x5e\u{521}\x0a\x5e\x03\x5f\x03\
	\x5f\x05\x5f\u{525}\x0a\x5f\x03\x60\x07\x60\u{528}\x0a\x60\x0c\x60\x0e\x60\
	\u{52b}\x0b\x60\x03\x60\x03\x60\x05\x60\u{52f}\x0a\x60\x03\x60\x03\x60\x03\
	\x60\x03\x60\x03\x61\x03\x61\x03\x61\x03\x61\x03\x62\x03\x62\x03\x62\x07\
	\x62\u{53c}\x0a\x62\x0c\x62\x0e\x62\u{53f}\x0b\x62\x03\x63\x03\x63\x03\x63\
	\x05\x63\u{544}\x0a\x63\x03\x63\x03\x63\x03\x63\x03\x63\x03\x63\x05\x63\
	\u{54b}\x0a\x63\x03\x63\x03\x63\x03\x63\x03\x63\x05\x63\u{551}\x0a\x63\x03\
	\x63\x05\x63\u{554}\x0a\x63\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\
	\x64\x03\x64\x07\x64\u{55d}\x0a\x64\x0c\x64\x0e\x64\u{560}\x0b\x64\x03\x64\
	\x03\x64\x03\x64\x07\x64\u{565}\x0a\x64\x0c\x64\x0e\x64\u{568}\x0b\x64\x03\
	\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\
	\x64\x03\x64\x03\x64\x05\x64\u{576}\x0a\x64\x03\x64\x03\x64\x05\x64\u{57a}\
	\x0a\x64\x03\x64\x03\x64\x03\x64\x05\x64\u{57f}\x0a\x64\x03\x64\x03\x64\
	\x05\x64\u{583}\x0a\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\
	\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x05\x64\
	\u{593}\x0a\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\
	\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\
	\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\
	\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\
	\x03\x64\x03\x64\x03\x64\x03\x64\x05\x64\u{5bb}\x0a\x64\x03\x64\x03\x64\
	\x03\x64\x03\x64\x05\x64\u{5c1}\x0a\x64\x03\x64\x03\x64\x03\x64\x03\x64\
	\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x03\x64\x05\x64\u{5ce}\
	\x0a\x64\x03\x64\x03\x64\x03\x64\x05\x64\u{5d3}\x0a\x64\x03\x64\x07\x64\
	\u{5d6}\x0a\x64\x0c\x64\x0e\x64\u{5d9}\x0b\x64\x03\x65\x07\x65\u{5dc}\x0a\
	\x65\x0c\x65\x0e\x65\u{5df}\x0b\x65\x03\x65\x03\x65\x07\x65\u{5e3}\x0a\x65\
	\x0c\x65\x0e\x65\u{5e6}\x0b\x65\x03\x65\x03\x65\x03\x66\x03\x66\x03\x66\
	\x03\x66\x03\x67\x03\x67\x03\x67\x05\x67\u{5f1}\x0a\x67\x03\x67\x03\x67\
	\x03\x67\x03\x67\x03\x67\x07\x67\u{5f8}\x0a\x67\x0c\x67\x0e\x67\u{5fb}\x0b\
	\x67\x03\x67\x03\x67\x03\x67\x03\x67\x05\x67\u{601}\x0a\x67\x03\x67\x05\
	\x67\u{604}\x0a\x67\x03\x68\x03\x68\x05\x68\u{608}\x0a\x68\x03\x69\x03\x69\
	\x03\x69\x03\x69\x03\x69\x03\x69\x03\x69\x03\x69\x03\x69\x03\x69\x03\x69\
	\x03\x69\x03\x69\x03\x69\x03\x69\x03\x69\x05\x69\u{61a}\x0a\x69\x05\x69\
	\u{61c}\x0a\x69\x03\x6a\x03\x6a\x03\x6a\x03\x6a\x07\x6a\u{622}\x0a\x6a\x0c\
	\x6a\x0e\x6a\u{625}\x0b\x6a\x03\x6a\x03\x6a\x03\x6b\x03\x6b\x03\x6b\x03\
	\x6b\x05\x6b\u{62d}\x0a\x6b\x03\x6b\x03\x6b\x03\x6b\x03\x6b\x03\x6b\x05\
	\x6b\u{634}\x0a\x6b\x03\x6c\x03\x6c\x03\x6c\x03\x6c\x03\x6c\x03\x6c\x07\
	\x6c\u{63c}\x0a\x6c\x0c\x6c\x0e\x6c\u{63f}\x0b\x6c\x03\x6c\x03\x6c\x07\x6c\
	\u{643}\x0a\x6c\x0c\x6c\x0e\x6c\u{646}\x0b\x6c\x03\x6c\x03\x6c\x03\x6c\x07\
	\x6c\u{64b}\x0a\x6c\x0c\x6c\x0e\x6c\u{64e}\x0b\x6c\x05\x6c\u{650}\x0a\x6c\
	\x03\x6c\x03\x6c\x03\x6c\x07\x6c\u{655}\x0a\x6c\x0c\x6c\x0e\x6c\u{658}\x0b\
	\x6c\x03\x6d\x03\x6d\x07\x6d\u{65c}\x0a\x6d\x0c\x6d\x0e\x6d\u{65f}\x0b\x6d\
	\x05\x6d\u{661}\x0a\x6d\x03\x6e\x03\x6e\x03\x6e\x05\x6e\u{666}\x0a\x6e\x03\
	\x6e\x07\x6e\u{669}\x0a\x6e\x0c\x6e\x0e\x6e\u{66c}\x0b\x6e\x03\x6e\x03\x6e\
	\x05\x6e\u{670}\x0a\x6e\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\x03\x6f\
	\x03\x6f\x05\x6f\u{679}\x0a\x6f\x05\x6f\u{67b}\x0a\x6f\x03\x70\x03\x70\x05\
	\x70\u{67f}\x0a\x70\x03\x70\x03\x70\x03\x70\x05\x70\u{684}\x0a\x70\x07\x70\
	\u{686}\x0a\x70\x0c\x70\x0e\x70\u{689}\x0b\x70\x03\x70\x05\x70\u{68c}\x0a\
	\x70\x03\x71\x03\x71\x05\x71\u{690}\x0a\x71\x03\x71\x03\x71\x03\x72\x03\
	\x72\x03\x72\x03\x72\x07\x72\u{698}\x0a\x72\x0c\x72\x0e\x72\u{69b}\x0b\x72\
	\x03\x72\x03\x72\x03\x72\x03\x72\x03\x72\x03\x72\x03\x72\x07\x72\u{6a4}\
	\x0a\x72\x0c\x72\x0e\x72\u{6a7}\x0b\x72\x03\x72\x03\x72\x07\x72\u{6ab}\x0a\
	\x72\x0c\x72\x0e\x72\u{6ae}\x0b\x72\x05\x72\u{6b0}\x0a\x72\x03\x73\x03\x73\
	\x05\x73\u{6b4}\x0a\x73\x03\x74\x03\x74\x03\x74\x03\x75\x03\x75\x03\x75\
	\x05\x75\u{6bc}\x0a\x75\x03\x76\x03\x76\x03\x76\x05\x76\u{6c1}\x0a\x76\x03\
	\x77\x03\x77\x03\x77\x03\x77\x03\x78\x03\x78\x03\x78\x07\x78\u{6ca}\x0a\
	\x78\x0c\x78\x0e\x78\u{6cd}\x0b\x78\x03\x79\x07\x79\u{6d0}\x0a\x79\x0c\x79\
	\x0e\x79\u{6d3}\x0b\x79\x03\x79\x03\x79\x05\x79\u{6d7}\x0a\x79\x03\x79\x07\
	\x79\u{6da}\x0a\x79\x0c\x79\x0e\x79\u{6dd}\x0b\x79\x03\x79\x03\x79\x07\x79\
	\u{6e1}\x0a\x79\x0c\x79\x0e\x79\u{6e4}\x0b\x79\x03\x7a\x03\x7a\x03\x7b\x03\
	\x7b\x03\x7b\x03\x7b\x07\x7b\u{6ec}\x0a\x7b\x0c\x7b\x0e\x7b\u{6ef}\x0b\x7b\
	\x03\x7b\x03\x7b\x03\x7c\x03\x7c\x03\x7c\x05\x7c\u{6f6}\x0a\x7c\x03\x7c\
	\x03\x7c\x05\x7c\u{6fa}\x0a\x7c\x05\x7c\u{6fc}\x0a\x7c\x03\x7d\x03\x7d\x03\
	\x7d\x03\x7d\x03\x7d\x05\x7d\u{703}\x0a\x7d\x03\x7e\x03\x7e\x05\x7e\u{707}\
	\x0a\x7e\x03\x7e\x03\x7e\x03\x7e\x02\x04\u{c6}\u{d6}\x7f\x02\x04\x06\x08\
	\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\
	\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\x4e\x50\
	\x52\x54\x56\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\x6a\x6c\x6e\x70\x72\x74\
	\x76\x78\x7a\x7c\x7e\u{80}\u{82}\u{84}\u{86}\u{88}\u{8a}\u{8c}\u{8e}\u{90}\
	\u{92}\u{94}\u{96}\u{98}\u{9a}\u{9c}\u{9e}\u{a0}\u{a2}\u{a4}\u{a6}\u{a8}\
	\u{aa}\u{ac}\u{ae}\u{b0}\u{b2}\u{b4}\u{b6}\u{b8}\u{ba}\u{bc}\u{be}\u{c0}\
	\u{c2}\u{c4}\u{c6}\u{c8}\u{ca}\u{cc}\u{ce}\u{d0}\u{d2}\u{d4}\u{d6}\u{d8}\
	\u{da}\u{dc}\u{de}\u{e0}\u{e2}\u{e4}\u{e6}\u{e8}\u{ea}\u{ec}\u{ee}\u{f0}\
	\u{f2}\u{f4}\u{f6}\u{f8}\u{fa}\x02\x13\x0c\x02\x03\x03\x14\x14\x20\x20\x23\
	\x25\x28\x29\x2c\x2c\x30\x30\x33\x33\x42\x42\x44\x44\x04\x02\x13\x13\x2a\
	\x2a\x03\x02\x45\x48\x03\x02\x49\x4a\x04\x02\x28\x28\x3e\x3e\x04\x02\x35\
	\x43\u{82}\u{82}\x05\x02\x35\x3e\x41\x43\u{82}\u{82}\x03\x02\x66\x69\x03\
	\x02\x5c\x5d\x04\x02\x6a\x6b\x6f\x6f\x03\x02\x68\x69\x04\x02\x5a\x5b\x61\
	\x62\x04\x02\x60\x60\x63\x63\x04\x02\x59\x59\x70\x7a\x03\x02\x66\x67\x04\
	\x02\x5f\x5f\x7b\x7b\x0a\x02\x05\x05\x07\x07\x0a\x0a\x10\x10\x16\x16\x1d\
	\x1d\x1f\x1f\x27\x27\x02\u{7cd}\x02\u{114}\x03\x02\x02\x02\x04\u{119}\x03\
	\x02\x02\x02\x06\u{120}\x03\x02\x02\x02\x08\u{13f}\x03\x02\x02\x02\x0a\u{141}\
	\x03\x02\x02\x02\x0c\u{145}\x03\x02\x02\x02\x0e\u{147}\x03\x02\x02\x02\x10\
	\u{15a}\x03\x02\x02\x02\x12\u{168}\x03\x02\x02\x02\x14\u{176}\x03\x02\x02\
	\x02\x16\u{17e}\x03\x02\x02\x02\x18\u{190}\x03\x02\x02\x02\x1a\u{19b}\x03\
	\x02\x02\x02\x1c\u{1a5}\x03\x02\x02\x02\x1e\u{1ac}\x03\x02\x02\x02\x20\u{1bb}\
	\x03\x02\x02\x02\x22\u{1c4}\x03\x02\x02\x02\x24\u{1df}\x03\x02\x02\x02\x26\
	\u{1eb}\x03\x02\x02\x02\x28\u{1ed}\x03\x02\x02\x02\x2a\u{1ff}\x03\x02\x02\
	\x02\x2c\u{203}\x03\x02\x02\x02\x2e\u{205}\x03\x02\x02\x02\x30\u{208}\x03\
	\x02\x02\x02\x32\u{20b}\x03\x02\x02\x02\x34\u{216}\x03\x02\x02\x02\x36\u{222}\
	\x03\x02\x02\x02\x38\u{234}\x03\x02\x02\x02\x3a\u{23e}\x03\x02\x02\x02\x3c\
	\u{240}\x03\x02\x02\x02\x3e\u{24b}\x03\x02\x02\x02\x40\u{259}\x03\x02\x02\
	\x02\x42\u{264}\x03\x02\x02\x02\x44\u{269}\x03\x02\x02\x02\x46\u{272}\x03\
	\x02\x02\x02\x48\u{285}\x03\x02\x02\x02\x4a\u{28d}\x03\x02\x02\x02\x4c\u{292}\
	\x03\x02\x02\x02\x4e\u{29c}\x03\x02\x02\x02\x50\u{29e}\x03\x02\x02\x02\x52\
	\u{2b6}\x03\x02\x02\x02\x54\u{2c9}\x03\x02\x02\x02\x56\u{2cb}\x03\x02\x02\
	\x02\x58\u{2d3}\x03\x02\x02\x02\x5a\u{2e3}\x03\x02\x02\x02\x5c\u{2fb}\x03\
	\x02\x02\x02\x5e\u{300}\x03\x02\x02\x02\x60\u{309}\x03\x02\x02\x02\x62\u{316}\
	\x03\x02\x02\x02\x64\u{321}\x03\x02\x02\x02\x66\u{327}\x03\x02\x02\x02\x68\
	\u{336}\x03\x02\x02\x02\x6a\u{338}\x03\x02\x02\x02\x6c\u{33a}\x03\x02\x02\
	\x02\x6e\u{341}\x03\x02\x02\x02\x70\u{34a}\x03\x02\x02\x02\x72\u{354}\x03\
	\x02\x02\x02\x74\u{35c}\x03\x02\x02\x02\x76\u{363}\x03\x02\x02\x02\x78\u{365}\
	\x03\x02\x02\x02\x7a\u{375}\x03\x02\x02\x02\x7c\u{37a}\x03\x02\x02\x02\x7e\
	\u{391}\x03\x02\x02\x02\u{80}\u{3ab}\x03\x02\x02\x02\u{82}\u{3af}\x03\x02\
	\x02\x02\u{84}\u{3b1}\x03\x02\x02\x02\u{86}\u{3b7}\x03\x02\x02\x02\u{88}\
	\u{3b9}\x03\x02\x02\x02\u{8a}\u{3bd}\x03\x02\x02\x02\u{8c}\u{3c3}\x03\x02\
	\x02\x02\u{8e}\u{3f0}\x03\x02\x02\x02\u{90}\u{3f2}\x03\x02\x02\x02\u{92}\
	\u{3f4}\x03\x02\x02\x02\u{94}\u{400}\x03\x02\x02\x02\u{96}\u{406}\x03\x02\
	\x02\x02\u{98}\u{40e}\x03\x02\x02\x02\u{9a}\u{411}\x03\x02\x02\x02\u{9c}\
	\u{41b}\x03\x02\x02\x02\u{9e}\u{429}\x03\x02\x02\x02\u{a0}\u{42e}\x03\x02\
	\x02\x02\u{a2}\u{43b}\x03\x02\x02\x02\u{a4}\u{43d}\x03\x02\x02\x02\u{a6}\
	\u{442}\x03\x02\x02\x02\u{a8}\u{4c1}\x03\x02\x02\x02\u{aa}\u{4c3}\x03\x02\
	\x02\x02\u{ac}\u{4d0}\x03\x02\x02\x02\u{ae}\u{4d8}\x03\x02\x02\x02\u{b0}\
	\u{4db}\x03\x02\x02\x02\u{b2}\u{4e2}\x03\x02\x02\x02\u{b4}\u{4fb}\x03\x02\
	\x02\x02\u{b6}\u{4fe}\x03\x02\x02\x02\u{b8}\u{512}\x03\x02\x02\x02\u{ba}\
	\u{520}\x03\x02\x02\x02\u{bc}\u{524}\x03\x02\x02\x02\u{be}\u{529}\x03\x02\
	\x02\x02\u{c0}\u{534}\x03\x02\x02\x02\u{c2}\u{538}\x03\x02\x02\x02\u{c4}\
	\u{553}\x03\x02\x02\x02\u{c6}\u{582}\x03\x02\x02\x02\u{c8}\u{5dd}\x03\x02\
	\x02\x02\u{ca}\u{5e9}\x03\x02\x02\x02\u{cc}\u{603}\x03\x02\x02\x02\u{ce}\
	\u{607}\x03\x02\x02\x02\u{d0}\u{61b}\x03\x02\x02\x02\u{d2}\u{61d}\x03\x02\
	\x02\x02\u{d4}\u{633}\x03\x02\x02\x02\u{d6}\u{64f}\x03\x02\x02\x02\u{d8}\
	\u{660}\x03\x02\x02\x02\u{da}\u{665}\x03\x02\x02\x02\u{dc}\u{67a}\x03\x02\
	\x02\x02\u{de}\u{68b}\x03\x02\x02\x02\u{e0}\u{68d}\x03\x02\x02\x02\u{e2}\
	\u{693}\x03\x02\x02\x02\u{e4}\u{6b1}\x03\x02\x02\x02\u{e6}\u{6b5}\x03\x02\
	\x02\x02\u{e8}\u{6bb}\x03\x02\x02\x02\u{ea}\u{6c0}\x03\x02\x02\x02\u{ec}\
	\u{6c2}\x03\x02\x02\x02\u{ee}\u{6c6}\x03\x02\x02\x02\u{f0}\u{6d1}\x03\x02\
	\x02\x02\u{f2}\u{6e5}\x03\x02\x02\x02\u{f4}\u{6e7}\x03\x02\x02\x02\u{f6}\
	\u{6fb}\x03\x02\x02\x02\u{f8}\u{702}\x03\x02\x02\x02\u{fa}\u{704}\x03\x02\
	\x02\x02\u{fc}\u{fe}\x07\u{80}\x02\x02\u{fd}\u{fc}\x03\x02\x02\x02\u{fe}\
	\u{101}\x03\x02\x02\x02\u{ff}\u{fd}\x03\x02\x02\x02\u{ff}\u{100}\x03\x02\
	\x02\x02\u{100}\u{103}\x03\x02\x02\x02\u{101}\u{ff}\x03\x02\x02\x02\u{102}\
	\u{104}\x05\x04\x03\x02\u{103}\u{102}\x03\x02\x02\x02\u{103}\u{104}\x03\
	\x02\x02\x02\u{104}\u{108}\x03\x02\x02\x02\u{105}\u{107}\x05\x06\x04\x02\
	\u{106}\u{105}\x03\x02\x02\x02\u{107}\u{10a}\x03\x02\x02\x02\u{108}\u{106}\
	\x03\x02\x02\x02\u{108}\u{109}\x03\x02\x02\x02\u{109}\u{10e}\x03\x02\x02\
	\x02\u{10a}\u{108}\x03\x02\x02\x02\u{10b}\u{10d}\x05\x08\x05\x02\u{10c}\
	\u{10b}\x03\x02\x02\x02\u{10d}\u{110}\x03\x02\x02\x02\u{10e}\u{10c}\x03\
	\x02\x02\x02\u{10e}\u{10f}\x03\x02\x02\x02\u{10f}\u{115}\x03\x02\x02\x02\
	\u{110}\u{10e}\x03\x02\x02\x02\u{111}\u{112}\x05\u{8a}\x46\x02\u{112}\u{113}\
	\x07\x02\x02\x03\u{113}\u{115}\x03\x02\x02\x02\u{114}\u{ff}\x03\x02\x02\
	\x02\u{114}\u{111}\x03\x02\x02\x02\u{115}\x03\x03\x02\x02\x02\u{116}\u{118}\
	\x05\x70\x39\x02\u{117}\u{116}\x03\x02\x02\x02\u{118}\u{11b}\x03\x02\x02\
	\x02\u{119}\u{117}\x03\x02\x02\x02\u{119}\u{11a}\x03\x02\x02\x02\u{11a}\
	\u{11c}\x03\x02\x02\x02\u{11b}\u{119}\x03\x02\x02\x02\u{11c}\u{11d}\x07\
	\x22\x02\x02\u{11d}\u{11e}\x05\x66\x34\x02\u{11e}\u{11f}\x07\x56\x02\x02\
	\u{11f}\x05\x03\x02\x02\x02\u{120}\u{122}\x07\x1b\x02\x02\u{121}\u{123}\
	\x07\x28\x02\x02\u{122}\u{121}\x03\x02\x02\x02\u{122}\u{123}\x03\x02\x02\
	\x02\u{123}\u{124}\x03\x02\x02\x02\u{124}\u{127}\x05\x66\x34\x02\u{125}\
	\u{126}\x07\x58\x02\x02\u{126}\u{128}\x07\x6a\x02\x02\u{127}\u{125}\x03\
	\x02\x02\x02\u{127}\u{128}\x03\x02\x02\x02\u{128}\u{129}\x03\x02\x02\x02\
	\u{129}\u{12a}\x07\x56\x02\x02\u{12a}\x07\x03\x02\x02\x02\u{12b}\u{12d}\
	\x05\x70\x39\x02\u{12c}\u{12b}\x03\x02\x02\x02\u{12d}\u{130}\x03\x02\x02\
	\x02\u{12e}\u{12c}\x03\x02\x02\x02\u{12e}\u{12f}\x03\x02\x02\x02\u{12f}\
	\u{134}\x03\x02\x02\x02\u{130}\u{12e}\x03\x02\x02\x02\u{131}\u{133}\x05\
	\x0a\x06\x02\u{132}\u{131}\x03\x02\x02\x02\u{133}\u{136}\x03\x02\x02\x02\
	\u{134}\u{132}\x03\x02\x02\x02\u{134}\u{135}\x03\x02\x02\x02\u{135}\u{13c}\
	\x03\x02\x02\x02\u{136}\u{134}\x03\x02\x02\x02\u{137}\u{13d}\x05\x0e\x08\
	\x02\u{138}\u{13d}\x05\x16\x0c\x02\u{139}\u{13d}\x05\x1e\x10\x02\u{13a}\
	\u{13d}\x05\x7a\x3e\x02\u{13b}\u{13d}\x05\u{92}\x4a\x02\u{13c}\u{137}\x03\
	\x02\x02\x02\u{13c}\u{138}\x03\x02\x02\x02\u{13c}\u{139}\x03\x02\x02\x02\
	\u{13c}\u{13a}\x03\x02\x02\x02\u{13c}\u{13b}\x03\x02\x02\x02\u{13d}\u{140}\
	\x03\x02\x02\x02\u{13e}\u{140}\x07\x56\x02\x02\u{13f}\u{12e}\x03\x02\x02\
	\x02\u{13f}\u{13e}\x03\x02\x02\x02\u{140}\x09\x03\x02\x02\x02\u{141}\u{142}\
	\x09\x02\x02\x02\u{142}\x0b\x03\x02\x02\x02\u{143}\u{146}\x07\x14\x02\x02\
	\u{144}\u{146}\x05\x70\x39\x02\u{145}\u{143}\x03\x02\x02\x02\u{145}\u{144}\
	\x03\x02\x02\x02\u{146}\x0d\x03\x02\x02\x02\u{147}\u{148}\x07\x0b\x02\x02\
	\u{148}\u{14a}\x05\u{a2}\x52\x02\u{149}\u{14b}\x05\x10\x09\x02\u{14a}\u{149}\
	\x03\x02\x02\x02\u{14a}\u{14b}\x03\x02\x02\x02\u{14b}\u{14e}\x03\x02\x02\
	\x02\u{14c}\u{14d}\x07\x13\x02\x02\u{14d}\u{14f}\x05\u{f0}\x79\x02\u{14e}\
	\u{14c}\x03\x02\x02\x02\u{14e}\u{14f}\x03\x02\x02\x02\u{14f}\u{152}\x03\
	\x02\x02\x02\u{150}\u{151}\x07\x1a\x02\x02\u{151}\u{153}\x05\u{ee}\x78\x02\
	\u{152}\u{150}\x03\x02\x02\x02\u{152}\u{153}\x03\x02\x02\x02\u{153}\u{156}\
	\x03\x02\x02\x02\u{154}\u{155}\x07\x43\x02\x02\u{155}\u{157}\x05\u{ee}\x78\
	\x02\u{156}\u{154}\x03\x02\x02\x02\u{156}\u{157}\x03\x02\x02\x02\u{157}\
	\u{158}\x03\x02\x02\x02\u{158}\u{159}\x05\x20\x11\x02\u{159}\x0f\x03\x02\
	\x02\x02\u{15a}\u{15b}\x07\x5b\x02\x02\u{15b}\u{160}\x05\x12\x0a\x02\u{15c}\
	\u{15d}\x07\x57\x02\x02\u{15d}\u{15f}\x05\x12\x0a\x02\u{15e}\u{15c}\x03\
	\x02\x02\x02\u{15f}\u{162}\x03\x02\x02\x02\u{160}\u{15e}\x03\x02\x02\x02\
	\u{160}\u{161}\x03\x02\x02\x02\u{161}\u{163}\x03\x02\x02\x02\u{162}\u{160}\
	\x03\x02\x02\x02\u{163}\u{164}\x07\x5a\x02\x02\u{164}\x11\x03\x02\x02\x02\
	\u{165}\u{167}\x05\x70\x39\x02\u{166}\u{165}\x03\x02\x02\x02\u{167}\u{16a}\
	\x03\x02\x02\x02\u{168}\u{166}\x03\x02\x02\x02\u{168}\u{169}\x03\x02\x02\
	\x02\u{169}\u{16b}\x03\x02\x02\x02\u{16a}\u{168}\x03\x02\x02\x02\u{16b}\
	\u{174}\x05\u{a2}\x52\x02\u{16c}\u{170}\x07\x13\x02\x02\u{16d}\u{16f}\x05\
	\x70\x39\x02\u{16e}\u{16d}\x03\x02\x02\x02\u{16f}\u{172}\x03\x02\x02\x02\
	\u{170}\u{16e}\x03\x02\x02\x02\u{170}\u{171}\x03\x02\x02\x02\u{171}\u{173}\
	\x03\x02\x02\x02\u{172}\u{170}\x03\x02\x02\x02\u{173}\u{175}\x05\x14\x0b\
	\x02\u{174}\u{16c}\x03\x02\x02\x02\u{174}\u{175}\x03\x02\x02\x02\u{175}\
	\x13\x03\x02\x02\x02\u{176}\u{17b}\x05\u{f0}\x79\x02\u{177}\u{178}\x07\x6c\
	\x02\x02\u{178}\u{17a}\x05\u{f0}\x79\x02\u{179}\u{177}\x03\x02\x02\x02\u{17a}\
	\u{17d}\x03\x02\x02\x02\u{17b}\u{179}\x03\x02\x02\x02\u{17b}\u{17c}\x03\
	\x02\x02\x02\u{17c}\x15\x03\x02\x02\x02\u{17d}\u{17b}\x03\x02\x02\x02\u{17e}\
	\u{17f}\x07\x12\x02\x02\u{17f}\u{182}\x05\u{a2}\x52\x02\u{180}\u{181}\x07\
	\x1a\x02\x02\u{181}\u{183}\x05\u{ee}\x78\x02\u{182}\u{180}\x03\x02\x02\x02\
	\u{182}\u{183}\x03\x02\x02\x02\u{183}\u{184}\x03\x02\x02\x02\u{184}\u{186}\
	\x07\x52\x02\x02\u{185}\u{187}\x05\x18\x0d\x02\u{186}\u{185}\x03\x02\x02\
	\x02\u{186}\u{187}\x03\x02\x02\x02\u{187}\u{189}\x03\x02\x02\x02\u{188}\
	\u{18a}\x07\x57\x02\x02\u{189}\u{188}\x03\x02\x02\x02\u{189}\u{18a}\x03\
	\x02\x02\x02\u{18a}\u{18c}\x03\x02\x02\x02\u{18b}\u{18d}\x05\x1c\x0f\x02\
	\u{18c}\u{18b}\x03\x02\x02\x02\u{18c}\u{18d}\x03\x02\x02\x02\u{18d}\u{18e}\
	\x03\x02\x02\x02\u{18e}\u{18f}\x07\x53\x02\x02\u{18f}\x17\x03\x02\x02\x02\
	\u{190}\u{195}\x05\x1a\x0e\x02\u{191}\u{192}\x07\x57\x02\x02\u{192}\u{194}\
	\x05\x1a\x0e\x02\u{193}\u{191}\x03\x02\x02\x02\u{194}\u{197}\x03\x02\x02\
	\x02\u{195}\u{193}\x03\x02\x02\x02\u{195}\u{196}\x03\x02\x02\x02\u{196}\
	\x19\x03\x02\x02\x02\u{197}\u{195}\x03\x02\x02\x02\u{198}\u{19a}\x05\x70\
	\x39\x02\u{199}\u{198}\x03\x02\x02\x02\u{19a}\u{19d}\x03\x02\x02\x02\u{19b}\
	\u{199}\x03\x02\x02\x02\u{19b}\u{19c}\x03\x02\x02\x02\u{19c}\u{19e}\x03\
	\x02\x02\x02\u{19d}\u{19b}\x03\x02\x02\x02\u{19e}\u{1a0}\x05\u{a2}\x52\x02\
	\u{19f}\u{1a1}\x05\u{fa}\x7e\x02\u{1a0}\u{19f}\x03\x02\x02\x02\u{1a0}\u{1a1}\
	\x03\x02\x02\x02\u{1a1}\u{1a3}\x03\x02\x02\x02\u{1a2}\u{1a4}\x05\x20\x11\
	\x02\u{1a3}\u{1a2}\x03\x02\x02\x02\u{1a3}\u{1a4}\x03\x02\x02\x02\u{1a4}\
	\x1b\x03\x02\x02\x02\u{1a5}\u{1a9}\x07\x56\x02\x02\u{1a6}\u{1a8}\x05\x24\
	\x13\x02\u{1a7}\u{1a6}\x03\x02\x02\x02\u{1a8}\u{1ab}\x03\x02\x02\x02\u{1a9}\
	\u{1a7}\x03\x02\x02\x02\u{1a9}\u{1aa}\x03\x02\x02\x02\u{1aa}\x1d\x03\x02\
	\x02\x02\u{1ab}\u{1a9}\x03\x02\x02\x02\u{1ac}\u{1ad}\x07\x1e\x02\x02\u{1ad}\
	\u{1af}\x05\u{a2}\x52\x02\u{1ae}\u{1b0}\x05\x10\x09\x02\u{1af}\u{1ae}\x03\
	\x02\x02\x02\u{1af}\u{1b0}\x03\x02\x02\x02\u{1b0}\u{1b3}\x03\x02\x02\x02\
	\u{1b1}\u{1b2}\x07\x13\x02\x02\u{1b2}\u{1b4}\x05\u{ee}\x78\x02\u{1b3}\u{1b1}\
	\x03\x02\x02\x02\u{1b3}\u{1b4}\x03\x02\x02\x02\u{1b4}\u{1b7}\x03\x02\x02\
	\x02\u{1b5}\u{1b6}\x07\x43\x02\x02\u{1b6}\u{1b8}\x05\u{ee}\x78\x02\u{1b7}\
	\u{1b5}\x03\x02\x02\x02\u{1b7}\u{1b8}\x03\x02\x02\x02\u{1b8}\u{1b9}\x03\
	\x02\x02\x02\u{1b9}\u{1ba}\x05\x22\x12\x02\u{1ba}\x1f\x03\x02\x02\x02\u{1bb}\
	\u{1bf}\x07\x52\x02\x02\u{1bc}\u{1be}\x05\x24\x13\x02\u{1bd}\u{1bc}\x03\
	\x02\x02\x02\u{1be}\u{1c1}\x03\x02\x02\x02\u{1bf}\u{1bd}\x03\x02\x02\x02\
	\u{1bf}\u{1c0}\x03\x02\x02\x02\u{1c0}\u{1c2}\x03\x02\x02\x02\u{1c1}\u{1bf}\
	\x03\x02\x02\x02\u{1c2}\u{1c3}\x07\x53\x02\x02\u{1c3}\x21\x03\x02\x02\x02\
	\u{1c4}\u{1c8}\x07\x52\x02\x02\u{1c5}\u{1c7}\x05\x38\x1d\x02\u{1c6}\u{1c5}\
	\x03\x02\x02\x02\u{1c7}\u{1ca}\x03\x02\x02\x02\u{1c8}\u{1c6}\x03\x02\x02\
	\x02\u{1c8}\u{1c9}\x03\x02\x02\x02\u{1c9}\u{1cb}\x03\x02\x02\x02\u{1ca}\
	\u{1c8}\x03\x02\x02\x02\u{1cb}\u{1cc}\x07\x53\x02\x02\u{1cc}\x23\x03\x02\
	\x02\x02\u{1cd}\u{1e0}\x07\x56\x02\x02\u{1ce}\u{1d0}\x07\x28\x02\x02\u{1cf}\
	\u{1ce}\x03\x02\x02\x02\u{1cf}\u{1d0}\x03\x02\x02\x02\u{1d0}\u{1d1}\x03\
	\x02\x02\x02\u{1d1}\u{1e0}\x05\u{9c}\x4f\x02\u{1d2}\u{1d4}\x05\x70\x39\x02\
	\u{1d3}\u{1d2}\x03\x02\x02\x02\u{1d4}\u{1d7}\x03\x02\x02\x02\u{1d5}\u{1d3}\
	\x03\x02\x02\x02\u{1d5}\u{1d6}\x03\x02\x02\x02\u{1d6}\u{1db}\x03\x02\x02\
	\x02\u{1d7}\u{1d5}\x03\x02\x02\x02\u{1d8}\u{1da}\x05\x0a\x06\x02\u{1d9}\
	\u{1d8}\x03\x02\x02\x02\u{1da}\u{1dd}\x03\x02\x02\x02\u{1db}\u{1d9}\x03\
	\x02\x02\x02\u{1db}\u{1dc}\x03\x02\x02\x02\u{1dc}\u{1de}\x03\x02\x02\x02\
	\u{1dd}\u{1db}\x03\x02\x02\x02\u{1de}\u{1e0}\x05\x26\x14\x02\u{1df}\u{1cd}\
	\x03\x02\x02\x02\u{1df}\u{1cf}\x03\x02\x02\x02\u{1df}\u{1d5}\x03\x02\x02\
	\x02\u{1e0}\x25\x03\x02\x02\x02\u{1e1}\u{1ec}\x05\u{92}\x4a\x02\u{1e2}\u{1ec}\
	\x05\x28\x15\x02\u{1e3}\u{1ec}\x05\x2e\x18\x02\u{1e4}\u{1ec}\x05\x36\x1c\
	\x02\u{1e5}\u{1ec}\x05\x32\x1a\x02\u{1e6}\u{1ec}\x05\x30\x19\x02\u{1e7}\
	\u{1ec}\x05\x1e\x10\x02\u{1e8}\u{1ec}\x05\x7a\x3e\x02\u{1e9}\u{1ec}\x05\
	\x0e\x08\x02\u{1ea}\u{1ec}\x05\x16\x0c\x02\u{1eb}\u{1e1}\x03\x02\x02\x02\
	\u{1eb}\u{1e2}\x03\x02\x02\x02\u{1eb}\u{1e3}\x03\x02\x02\x02\u{1eb}\u{1e4}\
	\x03\x02\x02\x02\u{1eb}\u{1e5}\x03\x02\x02\x02\u{1eb}\u{1e6}\x03\x02\x02\
	\x02\u{1eb}\u{1e7}\x03\x02\x02\x02\u{1eb}\u{1e8}\x03\x02\x02\x02\u{1eb}\
	\u{1e9}\x03\x02\x02\x02\u{1eb}\u{1ea}\x03\x02\x02\x02\u{1ec}\x27\x03\x02\
	\x02\x02\u{1ed}\u{1ee}\x05\x2c\x17\x02\u{1ee}\u{1ef}\x05\u{a2}\x52\x02\u{1ef}\
	\u{1f4}\x05\x58\x2d\x02\u{1f0}\u{1f1}\x07\x54\x02\x02\u{1f1}\u{1f3}\x07\
	\x55\x02\x02\u{1f2}\u{1f0}\x03\x02\x02\x02\u{1f3}\u{1f6}\x03\x02\x02\x02\
	\u{1f4}\u{1f2}\x03\x02\x02\x02\u{1f4}\u{1f5}\x03\x02\x02\x02\u{1f5}\u{1f9}\
	\x03\x02\x02\x02\u{1f6}\u{1f4}\x03\x02\x02\x02\u{1f7}\u{1f8}\x07\x2f\x02\
	\x02\u{1f8}\u{1fa}\x05\x56\x2c\x02\u{1f9}\u{1f7}\x03\x02\x02\x02\u{1f9}\
	\u{1fa}\x03\x02\x02\x02\u{1fa}\u{1fb}\x03\x02\x02\x02\u{1fb}\u{1fc}\x05\
	\x2a\x16\x02\u{1fc}\x29\x03\x02\x02\x02\u{1fd}\u{200}\x05\u{9c}\x4f\x02\
	\u{1fe}\u{200}\x07\x56\x02\x02\u{1ff}\u{1fd}\x03\x02\x02\x02\u{1ff}\u{1fe}\
	\x03\x02\x02\x02\u{200}\x2b\x03\x02\x02\x02\u{201}\u{204}\x05\u{f0}\x79\
	\x02\u{202}\u{204}\x07\x32\x02\x02\u{203}\u{201}\x03\x02\x02\x02\u{203}\
	\u{202}\x03\x02\x02\x02\u{204}\x2d\x03\x02\x02\x02\u{205}\u{206}\x05\x10\
	\x09\x02\u{206}\u{207}\x05\x28\x15\x02\u{207}\x2f\x03\x02\x02\x02\u{208}\
	\u{209}\x05\x10\x09\x02\u{209}\u{20a}\x05\x32\x1a\x02\u{20a}\x31\x03\x02\
	\x02\x02\u{20b}\u{20c}\x05\u{a2}\x52\x02\u{20c}\u{20f}\x05\x58\x2d\x02\u{20d}\
	\u{20e}\x07\x2f\x02\x02\u{20e}\u{210}\x05\x56\x2c\x02\u{20f}\u{20d}\x03\
	\x02\x02\x02\u{20f}\u{210}\x03\x02\x02\x02\u{210}\u{211}\x03\x02\x02\x02\
	\u{211}\u{212}\x05\u{9c}\x4f\x02\u{212}\x33\x03\x02\x02\x02\u{213}\u{215}\
	\x05\x70\x39\x02\u{214}\u{213}\x03\x02\x02\x02\u{215}\u{218}\x03\x02\x02\
	\x02\u{216}\u{214}\x03\x02\x02\x02\u{216}\u{217}\x03\x02\x02\x02\u{217}\
	\u{21c}\x03\x02\x02\x02\u{218}\u{216}\x03\x02\x02\x02\u{219}\u{21b}\x05\
	\x0a\x06\x02\u{21a}\u{219}\x03\x02\x02\x02\u{21b}\u{21e}\x03\x02\x02\x02\
	\u{21c}\u{21a}\x03\x02\x02\x02\u{21c}\u{21d}\x03\x02\x02\x02\u{21d}\u{21f}\
	\x03\x02\x02\x02\u{21e}\u{21c}\x03\x02\x02\x02\u{21f}\u{220}\x05\u{a2}\x52\
	\x02\u{220}\u{221}\x05\u{9c}\x4f\x02\u{221}\x35\x03\x02\x02\x02\u{222}\u{223}\
	\x05\u{f0}\x79\x02\u{223}\u{224}\x05\x48\x25\x02\u{224}\u{225}\x07\x56\x02\
	\x02\u{225}\x37\x03\x02\x02\x02\u{226}\u{228}\x05\x70\x39\x02\u{227}\u{226}\
	\x03\x02\x02\x02\u{228}\u{22b}\x03\x02\x02\x02\u{229}\u{227}\x03\x02\x02\
	\x02\u{229}\u{22a}\x03\x02\x02\x02\u{22a}\u{22f}\x03\x02\x02\x02\u{22b}\
	\u{229}\x03\x02\x02\x02\u{22c}\u{22e}\x05\x0a\x06\x02\u{22d}\u{22c}\x03\
	\x02\x02\x02\u{22e}\u{231}\x03\x02\x02\x02\u{22f}\u{22d}\x03\x02\x02\x02\
	\u{22f}\u{230}\x03\x02\x02\x02\u{230}\u{232}\x03\x02\x02\x02\u{231}\u{22f}\
	\x03\x02\x02\x02\u{232}\u{235}\x05\x3a\x1e\x02\u{233}\u{235}\x07\x56\x02\
	\x02\u{234}\u{229}\x03\x02\x02\x02\u{234}\u{233}\x03\x02\x02\x02\u{235}\
	\x39\x03\x02\x02\x02\u{236}\u{23f}\x05\x3c\x1f\x02\u{237}\u{23f}\x05\x40\
	\x21\x02\u{238}\u{23f}\x05\x44\x23\x02\u{239}\u{23f}\x05\x1e\x10\x02\u{23a}\
	\u{23f}\x05\x7a\x3e\x02\u{23b}\u{23f}\x05\x0e\x08\x02\u{23c}\u{23f}\x05\
	\x16\x0c\x02\u{23d}\u{23f}\x05\u{92}\x4a\x02\u{23e}\u{236}\x03\x02\x02\x02\
	\u{23e}\u{237}\x03\x02\x02\x02\u{23e}\u{238}\x03\x02\x02\x02\u{23e}\u{239}\
	\x03\x02\x02\x02\u{23e}\u{23a}\x03\x02\x02\x02\u{23e}\u{23b}\x03\x02\x02\
	\x02\u{23e}\u{23c}\x03\x02\x02\x02\u{23e}\u{23d}\x03\x02\x02\x02\u{23f}\
	\x3b\x03\x02\x02\x02\u{240}\u{241}\x05\u{f0}\x79\x02\u{241}\u{246}\x05\x3e\
	\x20\x02\u{242}\u{243}\x07\x57\x02\x02\u{243}\u{245}\x05\x3e\x20\x02\u{244}\
	\u{242}\x03\x02\x02\x02\u{245}\u{248}\x03\x02\x02\x02\u{246}\u{244}\x03\
	\x02\x02\x02\u{246}\u{247}\x03\x02\x02\x02\u{247}\u{249}\x03\x02\x02\x02\
	\u{248}\u{246}\x03\x02\x02\x02\u{249}\u{24a}\x07\x56\x02\x02\u{24a}\x3d\
	\x03\x02\x02\x02\u{24b}\u{250}\x05\u{a2}\x52\x02\u{24c}\u{24d}\x07\x54\x02\
	\x02\u{24d}\u{24f}\x07\x55\x02\x02\u{24e}\u{24c}\x03\x02\x02\x02\u{24f}\
	\u{252}\x03\x02\x02\x02\u{250}\u{24e}\x03\x02\x02\x02\u{250}\u{251}\x03\
	\x02\x02\x02\u{251}\u{253}\x03\x02\x02\x02\u{252}\u{250}\x03\x02\x02\x02\
	\u{253}\u{254}\x07\x59\x02\x02\u{254}\u{255}\x05\x4e\x28\x02\u{255}\x3f\
	\x03\x02\x02\x02\u{256}\u{258}\x05\x42\x22\x02\u{257}\u{256}\x03\x02\x02\
	\x02\u{258}\u{25b}\x03\x02\x02\x02\u{259}\u{257}\x03\x02\x02\x02\u{259}\
	\u{25a}\x03\x02\x02\x02\u{25a}\u{25c}\x03\x02\x02\x02\u{25b}\u{259}\x03\
	\x02\x02\x02\u{25c}\u{25d}\x05\x46\x24\x02\u{25d}\x41\x03\x02\x02\x02\u{25e}\
	\u{265}\x05\x70\x39\x02\u{25f}\u{265}\x07\x25\x02\x02\u{260}\u{265}\x07\
	\x03\x02\x02\u{261}\u{265}\x07\x0e\x02\x02\u{262}\u{265}\x07\x28\x02\x02\
	\u{263}\u{265}\x07\x29\x02\x02\u{264}\u{25e}\x03\x02\x02\x02\u{264}\u{25f}\
	\x03\x02\x02\x02\u{264}\u{260}\x03\x02\x02\x02\u{264}\u{261}\x03\x02\x02\
	\x02\u{264}\u{262}\x03\x02\x02\x02\u{264}\u{263}\x03\x02\x02\x02\u{265}\
	\x43\x03\x02\x02\x02\u{266}\u{268}\x05\x42\x22\x02\u{267}\u{266}\x03\x02\
	\x02\x02\u{268}\u{26b}\x03\x02\x02\x02\u{269}\u{267}\x03\x02\x02\x02\u{269}\
	\u{26a}\x03\x02\x02\x02\u{26a}\u{26c}\x03\x02\x02\x02\u{26b}\u{269}\x03\
	\x02\x02\x02\u{26c}\u{26d}\x05\x10\x09\x02\u{26d}\u{26e}\x05\x46\x24\x02\
	\u{26e}\x45\x03\x02\x02\x02\u{26f}\u{271}\x05\x70\x39\x02\u{270}\u{26f}\
	\x03\x02\x02\x02\u{271}\u{274}\x03\x02\x02\x02\u{272}\u{270}\x03\x02\x02\
	\x02\u{272}\u{273}\x03\x02\x02\x02\u{273}\u{275}\x03\x02\x02\x02\u{274}\
	\u{272}\x03\x02\x02\x02\u{275}\u{276}\x05\x2c\x17\x02\u{276}\u{277}\x05\
	\u{a2}\x52\x02\u{277}\u{27c}\x05\x58\x2d\x02\u{278}\u{279}\x07\x54\x02\x02\
	\u{279}\u{27b}\x07\x55\x02\x02\u{27a}\u{278}\x03\x02\x02\x02\u{27b}\u{27e}\
	\x03\x02\x02\x02\u{27c}\u{27a}\x03\x02\x02\x02\u{27c}\u{27d}\x03\x02\x02\
	\x02\u{27d}\u{281}\x03\x02\x02\x02\u{27e}\u{27c}\x03\x02\x02\x02\u{27f}\
	\u{280}\x07\x2f\x02\x02\u{280}\u{282}\x05\x56\x2c\x02\u{281}\u{27f}\x03\
	\x02\x02\x02\u{281}\u{282}\x03\x02\x02\x02\u{282}\u{283}\x03\x02\x02\x02\
	\u{283}\u{284}\x05\x2a\x16\x02\u{284}\x47\x03\x02\x02\x02\u{285}\u{28a}\
	\x05\x4a\x26\x02\u{286}\u{287}\x07\x57\x02\x02\u{287}\u{289}\x05\x4a\x26\
	\x02\u{288}\u{286}\x03\x02\x02\x02\u{289}\u{28c}\x03\x02\x02\x02\u{28a}\
	\u{288}\x03\x02\x02\x02\u{28a}\u{28b}\x03\x02\x02\x02\u{28b}\x49\x03\x02\
	\x02\x02\u{28c}\u{28a}\x03\x02\x02\x02\u{28d}\u{290}\x05\x4c\x27\x02\u{28e}\
	\u{28f}\x07\x59\x02\x02\u{28f}\u{291}\x05\x4e\x28\x02\u{290}\u{28e}\x03\
	\x02\x02\x02\u{290}\u{291}\x03\x02\x02\x02\u{291}\x4b\x03\x02\x02\x02\u{292}\
	\u{297}\x05\u{a2}\x52\x02\u{293}\u{294}\x07\x54\x02\x02\u{294}\u{296}\x07\
	\x55\x02\x02\u{295}\u{293}\x03\x02\x02\x02\u{296}\u{299}\x03\x02\x02\x02\
	\u{297}\u{295}\x03\x02\x02\x02\u{297}\u{298}\x03\x02\x02\x02\u{298}\x4d\
	\x03\x02\x02\x02\u{299}\u{297}\x03\x02\x02\x02\u{29a}\u{29d}\x05\x50\x29\
	\x02\u{29b}\u{29d}\x05\u{c6}\x64\x02\u{29c}\u{29a}\x03\x02\x02\x02\u{29c}\
	\u{29b}\x03\x02\x02\x02\u{29d}\x4f\x03\x02\x02\x02\u{29e}\u{2aa}\x07\x52\
	\x02\x02\u{29f}\u{2a4}\x05\x4e\x28\x02\u{2a0}\u{2a1}\x07\x57\x02\x02\u{2a1}\
	\u{2a3}\x05\x4e\x28\x02\u{2a2}\u{2a0}\x03\x02\x02\x02\u{2a3}\u{2a6}\x03\
	\x02\x02\x02\u{2a4}\u{2a2}\x03\x02\x02\x02\u{2a4}\u{2a5}\x03\x02\x02\x02\
	\u{2a5}\u{2a8}\x03\x02\x02\x02\u{2a6}\u{2a4}\x03\x02\x02\x02\u{2a7}\u{2a9}\
	\x07\x57\x02\x02\u{2a8}\u{2a7}\x03\x02\x02\x02\u{2a8}\u{2a9}\x03\x02\x02\
	\x02\u{2a9}\u{2ab}\x03\x02\x02\x02\u{2aa}\u{29f}\x03\x02\x02\x02\u{2aa}\
	\u{2ab}\x03\x02\x02\x02\u{2ab}\u{2ac}\x03\x02\x02\x02\u{2ac}\u{2ad}\x07\
	\x53\x02\x02\u{2ad}\x51\x03\x02\x02\x02\u{2ae}\u{2b0}\x05\u{a2}\x52\x02\
	\u{2af}\u{2b1}\x05\u{f4}\x7b\x02\u{2b0}\u{2af}\x03\x02\x02\x02\u{2b0}\u{2b1}\
	\x03\x02\x02\x02\u{2b1}\u{2b2}\x03\x02\x02\x02\u{2b2}\u{2b3}\x07\x58\x02\
	\x02\u{2b3}\u{2b5}\x03\x02\x02\x02\u{2b4}\u{2ae}\x03\x02\x02\x02\u{2b5}\
	\u{2b8}\x03\x02\x02\x02\u{2b6}\u{2b4}\x03\x02\x02\x02\u{2b6}\u{2b7}\x03\
	\x02\x02\x02\u{2b7}\u{2b9}\x03\x02\x02\x02\u{2b8}\u{2b6}\x03\x02\x02\x02\
	\u{2b9}\u{2bb}\x05\u{a4}\x53\x02\u{2ba}\u{2bc}\x05\u{f4}\x7b\x02\u{2bb}\
	\u{2ba}\x03\x02\x02\x02\u{2bb}\u{2bc}\x03\x02\x02\x02\u{2bc}\x53\x03\x02\
	\x02\x02\u{2bd}\u{2ca}\x05\u{f0}\x79\x02\u{2be}\u{2c0}\x05\x70\x39\x02\u{2bf}\
	\u{2be}\x03\x02\x02\x02\u{2c0}\u{2c3}\x03\x02\x02\x02\u{2c1}\u{2bf}\x03\
	\x02\x02\x02\u{2c1}\u{2c2}\x03\x02\x02\x02\u{2c2}\u{2c4}\x03\x02\x02\x02\
	\u{2c3}\u{2c1}\x03\x02\x02\x02\u{2c4}\u{2c7}\x07\x5e\x02\x02\u{2c5}\u{2c6}\
	\x09\x03\x02\x02\u{2c6}\u{2c8}\x05\u{f0}\x79\x02\u{2c7}\u{2c5}\x03\x02\x02\
	\x02\u{2c7}\u{2c8}\x03\x02\x02\x02\u{2c8}\u{2ca}\x03\x02\x02\x02\u{2c9}\
	\u{2bd}\x03\x02\x02\x02\u{2c9}\u{2c1}\x03\x02\x02\x02\u{2ca}\x55\x03\x02\
	\x02\x02\u{2cb}\u{2d0}\x05\x66\x34\x02\u{2cc}\u{2cd}\x07\x57\x02\x02\u{2cd}\
	\u{2cf}\x05\x66\x34\x02\u{2ce}\u{2cc}\x03\x02\x02\x02\u{2cf}\u{2d2}\x03\
	\x02\x02\x02\u{2d0}\u{2ce}\x03\x02\x02\x02\u{2d0}\u{2d1}\x03\x02\x02\x02\
	\u{2d1}\x57\x03\x02\x02\x02\u{2d2}\u{2d0}\x03\x02\x02\x02\u{2d3}\u{2df}\
	\x07\x50\x02\x02\u{2d4}\u{2d6}\x05\x5a\x2e\x02\u{2d5}\u{2d4}\x03\x02\x02\
	\x02\u{2d5}\u{2d6}\x03\x02\x02\x02\u{2d6}\u{2e0}\x03\x02\x02\x02\u{2d7}\
	\u{2da}\x05\x5a\x2e\x02\u{2d8}\u{2d9}\x07\x57\x02\x02\u{2d9}\u{2db}\x05\
	\x5c\x2f\x02\u{2da}\u{2d8}\x03\x02\x02\x02\u{2da}\u{2db}\x03\x02\x02\x02\
	\u{2db}\u{2e0}\x03\x02\x02\x02\u{2dc}\u{2de}\x05\x5c\x2f\x02\u{2dd}\u{2dc}\
	\x03\x02\x02\x02\u{2dd}\u{2de}\x03\x02\x02\x02\u{2de}\u{2e0}\x03\x02\x02\
	\x02\u{2df}\u{2d5}\x03\x02\x02\x02\u{2df}\u{2d7}\x03\x02\x02\x02\u{2df}\
	\u{2dd}\x03\x02\x02\x02\u{2e0}\u{2e1}\x03\x02\x02\x02\u{2e1}\u{2e2}\x07\
	\x51\x02\x02\u{2e2}\x59\x03\x02\x02\x02\u{2e3}\u{2e9}\x05\u{f0}\x79\x02\
	\u{2e4}\u{2e5}\x05\u{a2}\x52\x02\u{2e5}\u{2e6}\x07\x58\x02\x02\u{2e6}\u{2e8}\
	\x03\x02\x02\x02\u{2e7}\u{2e4}\x03\x02\x02\x02\u{2e8}\u{2eb}\x03\x02\x02\
	\x02\u{2e9}\u{2e7}\x03\x02\x02\x02\u{2e9}\u{2ea}\x03\x02\x02\x02\u{2ea}\
	\u{2ec}\x03\x02\x02\x02\u{2eb}\u{2e9}\x03\x02\x02\x02\u{2ec}\u{2ed}\x07\
	\x2d\x02\x02\u{2ed}\x5b\x03\x02\x02\x02\u{2ee}\u{2f3}\x05\x5e\x30\x02\u{2ef}\
	\u{2f0}\x07\x57\x02\x02\u{2f0}\u{2f2}\x05\x5e\x30\x02\u{2f1}\u{2ef}\x03\
	\x02\x02\x02\u{2f2}\u{2f5}\x03\x02\x02\x02\u{2f3}\u{2f1}\x03\x02\x02\x02\
	\u{2f3}\u{2f4}\x03\x02\x02\x02\u{2f4}\u{2f8}\x03\x02\x02\x02\u{2f5}\u{2f3}\
	\x03\x02\x02\x02\u{2f6}\u{2f7}\x07\x57\x02\x02\u{2f7}\u{2f9}\x05\x60\x31\
	\x02\u{2f8}\u{2f6}\x03\x02\x02\x02\u{2f8}\u{2f9}\x03\x02\x02\x02\u{2f9}\
	\u{2fc}\x03\x02\x02\x02\u{2fa}\u{2fc}\x05\x60\x31\x02\u{2fb}\u{2ee}\x03\
	\x02\x02\x02\u{2fb}\u{2fa}\x03\x02\x02\x02\u{2fc}\x5d\x03\x02\x02\x02\u{2fd}\
	\u{2ff}\x05\x0c\x07\x02\u{2fe}\u{2fd}\x03\x02\x02\x02\u{2ff}\u{302}\x03\
	\x02\x02\x02\u{300}\u{2fe}\x03\x02\x02\x02\u{300}\u{301}\x03\x02\x02\x02\
	\u{301}\u{303}\x03\x02\x02\x02\u{302}\u{300}\x03\x02\x02\x02\u{303}\u{304}\
	\x05\u{f0}\x79\x02\u{304}\u{305}\x05\x4c\x27\x02\u{305}\x5f\x03\x02\x02\
	\x02\u{306}\u{308}\x05\x0c\x07\x02\u{307}\u{306}\x03\x02\x02\x02\u{308}\
	\u{30b}\x03\x02\x02\x02\u{309}\u{307}\x03\x02\x02\x02\u{309}\u{30a}\x03\
	\x02\x02\x02\u{30a}\u{30c}\x03\x02\x02\x02\u{30b}\u{309}\x03\x02\x02\x02\
	\u{30c}\u{310}\x05\u{f0}\x79\x02\u{30d}\u{30f}\x05\x70\x39\x02\u{30e}\u{30d}\
	\x03\x02\x02\x02\u{30f}\u{312}\x03\x02\x02\x02\u{310}\u{30e}\x03\x02\x02\
	\x02\u{310}\u{311}\x03\x02\x02\x02\u{311}\u{313}\x03\x02\x02\x02\u{312}\
	\u{310}\x03\x02\x02\x02\u{313}\u{314}\x07\x7e\x02\x02\u{314}\u{315}\x05\
	\x4c\x27\x02\u{315}\x61\x03\x02\x02\x02\u{316}\u{31b}\x05\x64\x33\x02\u{317}\
	\u{318}\x07\x57\x02\x02\u{318}\u{31a}\x05\x64\x33\x02\u{319}\u{317}\x03\
	\x02\x02\x02\u{31a}\u{31d}\x03\x02\x02\x02\u{31b}\u{319}\x03\x02\x02\x02\
	\u{31b}\u{31c}\x03\x02\x02\x02\u{31c}\x63\x03\x02\x02\x02\u{31d}\u{31b}\
	\x03\x02\x02\x02\u{31e}\u{320}\x05\x0c\x07\x02\u{31f}\u{31e}\x03\x02\x02\
	\x02\u{320}\u{323}\x03\x02\x02\x02\u{321}\u{31f}\x03\x02\x02\x02\u{321}\
	\u{322}\x03\x02\x02\x02\u{322}\u{324}\x03\x02\x02\x02\u{323}\u{321}\x03\
	\x02\x02\x02\u{324}\u{325}\x07\x3f\x02\x02\u{325}\u{326}\x05\u{a2}\x52\x02\
	\u{326}\x65\x03\x02\x02\x02\u{327}\u{32c}\x05\u{a2}\x52\x02\u{328}\u{329}\
	\x07\x58\x02\x02\u{329}\u{32b}\x05\u{a2}\x52\x02\u{32a}\u{328}\x03\x02\x02\
	\x02\u{32b}\u{32e}\x03\x02\x02\x02\u{32c}\u{32a}\x03\x02\x02\x02\u{32c}\
	\u{32d}\x03\x02\x02\x02\u{32d}\x67\x03\x02\x02\x02\u{32e}\u{32c}\x03\x02\
	\x02\x02\u{32f}\u{337}\x05\x6a\x36\x02\u{330}\u{337}\x05\x6c\x37\x02\u{331}\
	\u{337}\x07\x4c\x02\x02\u{332}\u{337}\x07\x4d\x02\x02\u{333}\u{337}\x07\
	\x4b\x02\x02\u{334}\u{337}\x07\x4f\x02\x02\u{335}\u{337}\x07\x4e\x02\x02\
	\u{336}\u{32f}\x03\x02\x02\x02\u{336}\u{330}\x03\x02\x02\x02\u{336}\u{331}\
	\x03\x02\x02\x02\u{336}\u{332}\x03\x02\x02\x02\u{336}\u{333}\x03\x02\x02\
	\x02\u{336}\u{334}\x03\x02\x02\x02\u{336}\u{335}\x03\x02\x02\x02\u{337}\
	\x69\x03\x02\x02\x02\u{338}\u{339}\x09\x04\x02\x02\u{339}\x6b\x03\x02\x02\
	\x02\u{33a}\u{33b}\x09\x05\x02\x02\u{33b}\x6d\x03\x02\x02\x02\u{33c}\u{33d}\
	\x05\u{a2}\x52\x02\u{33d}\u{33e}\x07\x58\x02\x02\u{33e}\u{340}\x03\x02\x02\
	\x02\u{33f}\u{33c}\x03\x02\x02\x02\u{340}\u{343}\x03\x02\x02\x02\u{341}\
	\u{33f}\x03\x02\x02\x02\u{341}\u{342}\x03\x02\x02\x02\u{342}\u{344}\x03\
	\x02\x02\x02\u{343}\u{341}\x03\x02\x02\x02\u{344}\u{345}\x07\x7d\x02\x02\
	\u{345}\u{346}\x05\u{a2}\x52\x02\u{346}\x6f\x03\x02\x02\x02\u{347}\u{348}\
	\x07\x7d\x02\x02\u{348}\u{34b}\x05\x66\x34\x02\u{349}\u{34b}\x05\x6e\x38\
	\x02\u{34a}\u{347}\x03\x02\x02\x02\u{34a}\u{349}\x03\x02\x02\x02\u{34b}\
	\u{352}\x03\x02\x02\x02\u{34c}\u{34f}\x07\x50\x02\x02\u{34d}\u{350}\x05\
	\x72\x3a\x02\u{34e}\u{350}\x05\x76\x3c\x02\u{34f}\u{34d}\x03\x02\x02\x02\
	\u{34f}\u{34e}\x03\x02\x02\x02\u{34f}\u{350}\x03\x02\x02\x02\u{350}\u{351}\
	\x03\x02\x02\x02\u{351}\u{353}\x07\x51\x02\x02\u{352}\u{34c}\x03\x02\x02\
	\x02\u{352}\u{353}\x03\x02\x02\x02\u{353}\x71\x03\x02\x02\x02\u{354}\u{359}\
	\x05\x74\x3b\x02\u{355}\u{356}\x07\x57\x02\x02\u{356}\u{358}\x05\x74\x3b\
	\x02\u{357}\u{355}\x03\x02\x02\x02\u{358}\u{35b}\x03\x02\x02\x02\u{359}\
	\u{357}\x03\x02\x02\x02\u{359}\u{35a}\x03\x02\x02\x02\u{35a}\x73\x03\x02\
	\x02\x02\u{35b}\u{359}\x03\x02\x02\x02\u{35c}\u{35d}\x05\u{a2}\x52\x02\u{35d}\
	\u{35e}\x07\x59\x02\x02\u{35e}\u{35f}\x05\x76\x3c\x02\u{35f}\x75\x03\x02\
	\x02\x02\u{360}\u{364}\x05\u{c6}\x64\x02\u{361}\u{364}\x05\x70\x39\x02\u{362}\
	\u{364}\x05\x78\x3d\x02\u{363}\u{360}\x03\x02\x02\x02\u{363}\u{361}\x03\
	\x02\x02\x02\u{363}\u{362}\x03\x02\x02\x02\u{364}\x77\x03\x02\x02\x02\u{365}\
	\u{36e}\x07\x52\x02\x02\u{366}\u{36b}\x05\x76\x3c\x02\u{367}\u{368}\x07\
	\x57\x02\x02\u{368}\u{36a}\x05\x76\x3c\x02\u{369}\u{367}\x03\x02\x02\x02\
	\u{36a}\u{36d}\x03\x02\x02\x02\u{36b}\u{369}\x03\x02\x02\x02\u{36b}\u{36c}\
	\x03\x02\x02\x02\u{36c}\u{36f}\x03\x02\x02\x02\u{36d}\u{36b}\x03\x02\x02\
	\x02\u{36e}\u{366}\x03\x02\x02\x02\u{36e}\u{36f}\x03\x02\x02\x02\u{36f}\
	\u{371}\x03\x02\x02\x02\u{370}\u{372}\x07\x57\x02\x02\u{371}\u{370}\x03\
	\x02\x02\x02\u{371}\u{372}\x03\x02\x02\x02\u{372}\u{373}\x03\x02\x02\x02\
	\u{373}\u{374}\x07\x53\x02\x02\u{374}\x79\x03\x02\x02\x02\u{375}\u{376}\
	\x07\x7d\x02\x02\u{376}\u{377}\x07\x1e\x02\x02\u{377}\u{378}\x05\u{a2}\x52\
	\x02\u{378}\u{379}\x05\x7c\x3f\x02\u{379}\x7b\x03\x02\x02\x02\u{37a}\u{37e}\
	\x07\x52\x02\x02\u{37b}\u{37d}\x05\x7e\x40\x02\u{37c}\u{37b}\x03\x02\x02\
	\x02\u{37d}\u{380}\x03\x02\x02\x02\u{37e}\u{37c}\x03\x02\x02\x02\u{37e}\
	\u{37f}\x03\x02\x02\x02\u{37f}\u{381}\x03\x02\x02\x02\u{380}\u{37e}\x03\
	\x02\x02\x02\u{381}\u{382}\x07\x53\x02\x02\u{382}\x7d\x03\x02\x02\x02\u{383}\
	\u{385}\x05\x70\x39\x02\u{384}\u{383}\x03\x02\x02\x02\u{385}\u{388}\x03\
	\x02\x02\x02\u{386}\u{384}\x03\x02\x02\x02\u{386}\u{387}\x03\x02\x02\x02\
	\u{387}\u{38c}\x03\x02\x02\x02\u{388}\u{386}\x03\x02\x02\x02\u{389}\u{38b}\
	\x05\x0a\x06\x02\u{38a}\u{389}\x03\x02\x02\x02\u{38b}\u{38e}\x03\x02\x02\
	\x02\u{38c}\u{38a}\x03\x02\x02\x02\u{38c}\u{38d}\x03\x02\x02\x02\u{38d}\
	\u{38f}\x03\x02\x02\x02\u{38e}\u{38c}\x03\x02\x02\x02\u{38f}\u{392}\x05\
	\u{80}\x41\x02\u{390}\u{392}\x07\x56\x02\x02\u{391}\u{386}\x03\x02\x02\x02\
	\u{391}\u{390}\x03\x02\x02\x02\u{392}\x7f\x03\x02\x02\x02\u{393}\u{394}\
	\x05\u{f0}\x79\x02\u{394}\u{395}\x05\u{82}\x42\x02\u{395}\u{396}\x07\x56\
	\x02\x02\u{396}\u{3ac}\x03\x02\x02\x02\u{397}\u{399}\x05\x0e\x08\x02\u{398}\
	\u{39a}\x07\x56\x02\x02\u{399}\u{398}\x03\x02\x02\x02\u{399}\u{39a}\x03\
	\x02\x02\x02\u{39a}\u{3ac}\x03\x02\x02\x02\u{39b}\u{39d}\x05\x1e\x10\x02\
	\u{39c}\u{39e}\x07\x56\x02\x02\u{39d}\u{39c}\x03\x02\x02\x02\u{39d}\u{39e}\
	\x03\x02\x02\x02\u{39e}\u{3ac}\x03\x02\x02\x02\u{39f}\u{3a1}\x05\x16\x0c\
	\x02\u{3a0}\u{3a2}\x07\x56\x02\x02\u{3a1}\u{3a0}\x03\x02\x02\x02\u{3a1}\
	\u{3a2}\x03\x02\x02\x02\u{3a2}\u{3ac}\x03\x02\x02\x02\u{3a3}\u{3a5}\x05\
	\x7a\x3e\x02\u{3a4}\u{3a6}\x07\x56\x02\x02\u{3a5}\u{3a4}\x03\x02\x02\x02\
	\u{3a5}\u{3a6}\x03\x02\x02\x02\u{3a6}\u{3ac}\x03\x02\x02\x02\u{3a7}\u{3a9}\
	\x05\u{92}\x4a\x02\u{3a8}\u{3aa}\x07\x56\x02\x02\u{3a9}\u{3a8}\x03\x02\x02\
	\x02\u{3a9}\u{3aa}\x03\x02\x02\x02\u{3aa}\u{3ac}\x03\x02\x02\x02\u{3ab}\
	\u{393}\x03\x02\x02\x02\u{3ab}\u{397}\x03\x02\x02\x02\u{3ab}\u{39b}\x03\
	\x02\x02\x02\u{3ab}\u{39f}\x03\x02\x02\x02\u{3ab}\u{3a3}\x03\x02\x02\x02\
	\u{3ab}\u{3a7}\x03\x02\x02\x02\u{3ac}\u{81}\x03\x02\x02\x02\u{3ad}\u{3b0}\
	\x05\u{84}\x43\x02\u{3ae}\u{3b0}\x05\u{86}\x44\x02\u{3af}\u{3ad}\x03\x02\
	\x02\x02\u{3af}\u{3ae}\x03\x02\x02\x02\u{3b0}\u{83}\x03\x02\x02\x02\u{3b1}\
	\u{3b2}\x05\u{a2}\x52\x02\u{3b2}\u{3b3}\x07\x50\x02\x02\u{3b3}\u{3b5}\x07\
	\x51\x02\x02\u{3b4}\u{3b6}\x05\u{88}\x45\x02\u{3b5}\u{3b4}\x03\x02\x02\x02\
	\u{3b5}\u{3b6}\x03\x02\x02\x02\u{3b6}\u{85}\x03\x02\x02\x02\u{3b7}\u{3b8}\
	\x05\x48\x25\x02\u{3b8}\u{87}\x03\x02\x02\x02\u{3b9}\u{3ba}\x07\x0e\x02\
	\x02\u{3ba}\u{3bb}\x05\x76\x3c\x02\u{3bb}\u{89}\x03\x02\x02\x02\u{3bc}\u{3be}\
	\x07\x36\x02\x02\u{3bd}\u{3bc}\x03\x02\x02\x02\u{3bd}\u{3be}\x03\x02\x02\
	\x02\u{3be}\u{3bf}\x03\x02\x02\x02\u{3bf}\u{3c0}\x07\x35\x02\x02\u{3c0}\
	\u{3c1}\x05\x66\x34\x02\u{3c1}\u{3c2}\x05\u{8c}\x47\x02\u{3c2}\u{8b}\x03\
	\x02\x02\x02\u{3c3}\u{3c7}\x07\x52\x02\x02\u{3c4}\u{3c6}\x05\u{8e}\x48\x02\
	\u{3c5}\u{3c4}\x03\x02\x02\x02\u{3c6}\u{3c9}\x03\x02\x02\x02\u{3c7}\u{3c5}\
	\x03\x02\x02\x02\u{3c7}\u{3c8}\x03\x02\x02\x02\u{3c8}\u{3ca}\x03\x02\x02\
	\x02\u{3c9}\u{3c7}\x03\x02\x02\x02\u{3ca}\u{3cb}\x07\x53\x02\x02\u{3cb}\
	\u{8d}\x03\x02\x02\x02\u{3cc}\u{3d0}\x07\x37\x02\x02\u{3cd}\u{3cf}\x05\u{90}\
	\x49\x02\u{3ce}\u{3cd}\x03\x02\x02\x02\u{3cf}\u{3d2}\x03\x02\x02\x02\u{3d0}\
	\u{3ce}\x03\x02\x02\x02\u{3d0}\u{3d1}\x03\x02\x02\x02\u{3d1}\u{3d3}\x03\
	\x02\x02\x02\u{3d2}\u{3d0}\x03\x02\x02\x02\u{3d3}\u{3d4}\x05\x66\x34\x02\
	\u{3d4}\u{3d5}\x07\x56\x02\x02\u{3d5}\u{3f1}\x03\x02\x02\x02\u{3d6}\u{3d7}\
	\x07\x38\x02\x02\u{3d7}\u{3da}\x05\x66\x34\x02\u{3d8}\u{3d9}\x07\x3a\x02\
	\x02\u{3d9}\u{3db}\x05\x66\x34\x02\u{3da}\u{3d8}\x03\x02\x02\x02\u{3da}\
	\u{3db}\x03\x02\x02\x02\u{3db}\u{3dc}\x03\x02\x02\x02\u{3dc}\u{3dd}\x07\
	\x56\x02\x02\u{3dd}\u{3f1}\x03\x02\x02\x02\u{3de}\u{3df}\x07\x39\x02\x02\
	\u{3df}\u{3e2}\x05\x66\x34\x02\u{3e0}\u{3e1}\x07\x3a\x02\x02\u{3e1}\u{3e3}\
	\x05\x66\x34\x02\u{3e2}\u{3e0}\x03\x02\x02\x02\u{3e2}\u{3e3}\x03\x02\x02\
	\x02\u{3e3}\u{3e4}\x03\x02\x02\x02\u{3e4}\u{3e5}\x07\x56\x02\x02\u{3e5}\
	\u{3f1}\x03\x02\x02\x02\u{3e6}\u{3e7}\x07\x3b\x02\x02\u{3e7}\u{3e8}\x05\
	\x66\x34\x02\u{3e8}\u{3e9}\x07\x56\x02\x02\u{3e9}\u{3f1}\x03\x02\x02\x02\
	\u{3ea}\u{3eb}\x07\x3c\x02\x02\u{3eb}\u{3ec}\x05\x66\x34\x02\u{3ec}\u{3ed}\
	\x07\x3d\x02\x02\u{3ed}\u{3ee}\x05\x66\x34\x02\u{3ee}\u{3ef}\x07\x56\x02\
	\x02\u{3ef}\u{3f1}\x03\x02\x02\x02\u{3f0}\u{3cc}\x03\x02\x02\x02\u{3f0}\
	\u{3d6}\x03\x02\x02\x02\u{3f0}\u{3de}\x03\x02\x02\x02\u{3f0}\u{3e6}\x03\
	\x02\x02\x02\u{3f0}\u{3ea}\x03\x02\x02\x02\u{3f1}\u{8f}\x03\x02\x02\x02\
	\u{3f2}\u{3f3}\x09\x06\x02\x02\u{3f3}\u{91}\x03\x02\x02\x02\u{3f4}\u{3f5}\
	\x07\x41\x02\x02\u{3f5}\u{3f7}\x05\u{a2}\x52\x02\u{3f6}\u{3f8}\x05\x10\x09\
	\x02\u{3f7}\u{3f6}\x03\x02\x02\x02\u{3f7}\u{3f8}\x03\x02\x02\x02\u{3f8}\
	\u{3f9}\x03\x02\x02\x02\u{3f9}\u{3fc}\x05\u{94}\x4b\x02\u{3fa}\u{3fb}\x07\
	\x1a\x02\x02\u{3fb}\u{3fd}\x05\u{ee}\x78\x02\u{3fc}\u{3fa}\x03\x02\x02\x02\
	\u{3fc}\u{3fd}\x03\x02\x02\x02\u{3fd}\u{3fe}\x03\x02\x02\x02\u{3fe}\u{3ff}\
	\x05\u{9a}\x4e\x02\u{3ff}\u{93}\x03\x02\x02\x02\u{400}\u{402}\x07\x50\x02\
	\x02\u{401}\u{403}\x05\u{96}\x4c\x02\u{402}\u{401}\x03\x02\x02\x02\u{402}\
	\u{403}\x03\x02\x02\x02\u{403}\u{404}\x03\x02\x02\x02\u{404}\u{405}\x07\
	\x51\x02\x02\u{405}\u{95}\x03\x02\x02\x02\u{406}\u{40b}\x05\u{98}\x4d\x02\
	\u{407}\u{408}\x07\x57\x02\x02\u{408}\u{40a}\x05\u{98}\x4d\x02\u{409}\u{407}\
	\x03\x02\x02\x02\u{40a}\u{40d}\x03\x02\x02\x02\u{40b}\u{409}\x03\x02\x02\
	\x02\u{40b}\u{40c}\x03\x02\x02\x02\u{40c}\u{97}\x03\x02\x02\x02\u{40d}\u{40b}\
	\x03\x02\x02\x02\u{40e}\u{40f}\x05\u{f0}\x79\x02\u{40f}\u{410}\x05\u{a2}\
	\x52\x02\u{410}\u{99}\x03\x02\x02\x02\u{411}\u{416}\x07\x52\x02\x02\u{412}\
	\u{415}\x05\x24\x13\x02\u{413}\u{415}\x05\x34\x1b\x02\u{414}\u{412}\x03\
	\x02\x02\x02\u{414}\u{413}\x03\x02\x02\x02\u{415}\u{418}\x03\x02\x02\x02\
	\u{416}\u{414}\x03\x02\x02\x02\u{416}\u{417}\x03\x02\x02\x02\u{417}\u{419}\
	\x03\x02\x02\x02\u{418}\u{416}\x03\x02\x02\x02\u{419}\u{41a}\x07\x53\x02\
	\x02\u{41a}\u{9b}\x03\x02\x02\x02\u{41b}\u{41f}\x07\x52\x02\x02\u{41c}\u{41e}\
	\x05\u{9e}\x50\x02\u{41d}\u{41c}\x03\x02\x02\x02\u{41e}\u{421}\x03\x02\x02\
	\x02\u{41f}\u{41d}\x03\x02\x02\x02\u{41f}\u{420}\x03\x02\x02\x02\u{420}\
	\u{422}\x03\x02\x02\x02\u{421}\u{41f}\x03\x02\x02\x02\u{422}\u{423}\x07\
	\x53\x02\x02\u{423}\u{9d}\x03\x02\x02\x02\u{424}\u{425}\x05\u{a0}\x51\x02\
	\u{425}\u{426}\x07\x56\x02\x02\u{426}\u{42a}\x03\x02\x02\x02\u{427}\u{42a}\
	\x05\u{a6}\x54\x02\u{428}\u{42a}\x05\u{a8}\x55\x02\u{429}\u{424}\x03\x02\
	\x02\x02\u{429}\u{427}\x03\x02\x02\x02\u{429}\u{428}\x03\x02\x02\x02\u{42a}\
	\u{9f}\x03\x02\x02\x02\u{42b}\u{42d}\x05\x0c\x07\x02\u{42c}\u{42b}\x03\x02\
	\x02\x02\u{42d}\u{430}\x03\x02\x02\x02\u{42e}\u{42c}\x03\x02\x02\x02\u{42e}\
	\u{42f}\x03\x02\x02\x02\u{42f}\u{439}\x03\x02\x02\x02\u{430}\u{42e}\x03\
	\x02\x02\x02\u{431}\u{432}\x07\x3f\x02\x02\u{432}\u{433}\x05\u{a2}\x52\x02\
	\u{433}\u{434}\x07\x59\x02\x02\u{434}\u{435}\x05\u{c6}\x64\x02\u{435}\u{43a}\
	\x03\x02\x02\x02\u{436}\u{437}\x05\u{f0}\x79\x02\u{437}\u{438}\x05\x48\x25\
	\x02\u{438}\u{43a}\x03\x02\x02\x02\u{439}\u{431}\x03\x02\x02\x02\u{439}\
	\u{436}\x03\x02\x02\x02\u{43a}\u{a1}\x03\x02\x02\x02\u{43b}\u{43c}\x09\x07\
	\x02\x02\u{43c}\u{a3}\x03\x02\x02\x02\u{43d}\u{43e}\x09\x08\x02\x02\u{43e}\
	\u{a5}\x03\x02\x02\x02\u{43f}\u{441}\x05\x70\x39\x02\u{440}\u{43f}\x03\x02\
	\x02\x02\u{441}\u{444}\x03\x02\x02\x02\u{442}\u{440}\x03\x02\x02\x02\u{442}\
	\u{443}\x03\x02\x02\x02\u{443}\u{448}\x03\x02\x02\x02\u{444}\u{442}\x03\
	\x02\x02\x02\u{445}\u{447}\x05\x0a\x06\x02\u{446}\u{445}\x03\x02\x02\x02\
	\u{447}\u{44a}\x03\x02\x02\x02\u{448}\u{446}\x03\x02\x02\x02\u{448}\u{449}\
	\x03\x02\x02\x02\u{449}\u{44e}\x03\x02\x02\x02\u{44a}\u{448}\x03\x02\x02\
	\x02\u{44b}\u{44f}\x05\x0e\x08\x02\u{44c}\u{44f}\x05\x1e\x10\x02\u{44d}\
	\u{44f}\x05\u{92}\x4a\x02\u{44e}\u{44b}\x03\x02\x02\x02\u{44e}\u{44c}\x03\
	\x02\x02\x02\u{44e}\u{44d}\x03\x02\x02\x02\u{44f}\u{a7}\x03\x02\x02\x02\
	\u{450}\u{4c2}\x05\u{9c}\x4f\x02\u{451}\u{452}\x07\x04\x02\x02\u{452}\u{455}\
	\x05\u{c6}\x64\x02\u{453}\u{454}\x07\x5f\x02\x02\u{454}\u{456}\x05\u{c6}\
	\x64\x02\u{455}\u{453}\x03\x02\x02\x02\u{455}\u{456}\x03\x02\x02\x02\u{456}\
	\u{457}\x03\x02\x02\x02\u{457}\u{458}\x07\x56\x02\x02\u{458}\u{4c2}\x03\
	\x02\x02\x02\u{459}\u{45a}\x07\x18\x02\x02\u{45a}\u{45b}\x05\u{c0}\x61\x02\
	\u{45b}\u{45e}\x05\u{a8}\x55\x02\u{45c}\u{45d}\x07\x11\x02\x02\u{45d}\u{45f}\
	\x05\u{a8}\x55\x02\u{45e}\u{45c}\x03\x02\x02\x02\u{45e}\u{45f}\x03\x02\x02\
	\x02\u{45f}\u{4c2}\x03\x02\x02\x02\u{460}\u{461}\x07\x17\x02\x02\u{461}\
	\u{462}\x07\x50\x02\x02\u{462}\u{463}\x05\u{ba}\x5e\x02\u{463}\u{464}\x07\
	\x51\x02\x02\u{464}\u{465}\x05\u{a8}\x55\x02\u{465}\u{4c2}\x03\x02\x02\x02\
	\u{466}\u{467}\x07\x34\x02\x02\u{467}\u{468}\x05\u{c0}\x61\x02\u{468}\u{469}\
	\x05\u{a8}\x55\x02\u{469}\u{4c2}\x03\x02\x02\x02\u{46a}\u{46b}\x07\x0f\x02\
	\x02\u{46b}\u{46c}\x05\u{a8}\x55\x02\u{46c}\u{46d}\x07\x34\x02\x02\u{46d}\
	\u{46e}\x05\u{c0}\x61\x02\u{46e}\u{46f}\x07\x56\x02\x02\u{46f}\u{4c2}\x03\
	\x02\x02\x02\u{470}\u{471}\x07\x31\x02\x02\u{471}\u{47b}\x05\u{9c}\x4f\x02\
	\u{472}\u{474}\x05\u{aa}\x56\x02\u{473}\u{472}\x03\x02\x02\x02\u{474}\u{475}\
	\x03\x02\x02\x02\u{475}\u{473}\x03\x02\x02\x02\u{475}\u{476}\x03\x02\x02\
	\x02\u{476}\u{478}\x03\x02\x02\x02\u{477}\u{479}\x05\u{ae}\x58\x02\u{478}\
	\u{477}\x03\x02\x02\x02\u{478}\u{479}\x03\x02\x02\x02\u{479}\u{47c}\x03\
	\x02\x02\x02\u{47a}\u{47c}\x05\u{ae}\x58\x02\u{47b}\u{473}\x03\x02\x02\x02\
	\u{47b}\u{47a}\x03\x02\x02\x02\u{47c}\u{4c2}\x03\x02\x02\x02\u{47d}\u{47e}\
	\x07\x31\x02\x02\u{47e}\u{47f}\x05\u{b0}\x59\x02\u{47f}\u{483}\x05\u{9c}\
	\x4f\x02\u{480}\u{482}\x05\u{aa}\x56\x02\u{481}\u{480}\x03\x02\x02\x02\u{482}\
	\u{485}\x03\x02\x02\x02\u{483}\u{481}\x03\x02\x02\x02\u{483}\u{484}\x03\
	\x02\x02\x02\u{484}\u{487}\x03\x02\x02\x02\u{485}\u{483}\x03\x02\x02\x02\
	\u{486}\u{488}\x05\u{ae}\x58\x02\u{487}\u{486}\x03\x02\x02\x02\u{487}\u{488}\
	\x03\x02\x02\x02\u{488}\u{4c2}\x03\x02\x02\x02\u{489}\u{48a}\x07\x2b\x02\
	\x02\u{48a}\u{48b}\x05\u{c0}\x61\x02\u{48b}\u{48f}\x07\x52\x02\x02\u{48c}\
	\u{48e}\x05\u{b6}\x5c\x02\u{48d}\u{48c}\x03\x02\x02\x02\u{48e}\u{491}\x03\
	\x02\x02\x02\u{48f}\u{48d}\x03\x02\x02\x02\u{48f}\u{490}\x03\x02\x02\x02\
	\u{490}\u{495}\x03\x02\x02\x02\u{491}\u{48f}\x03\x02\x02\x02\u{492}\u{494}\
	\x05\u{b8}\x5d\x02\u{493}\u{492}\x03\x02\x02\x02\u{494}\u{497}\x03\x02\x02\
	\x02\u{495}\u{493}\x03\x02\x02\x02\u{495}\u{496}\x03\x02\x02\x02\u{496}\
	\u{498}\x03\x02\x02\x02\u{497}\u{495}\x03\x02\x02\x02\u{498}\u{499}\x07\
	\x53\x02\x02\u{499}\u{4c2}\x03\x02\x02\x02\u{49a}\u{49b}\x07\x2c\x02\x02\
	\u{49b}\u{49c}\x05\u{c0}\x61\x02\u{49c}\u{49d}\x05\u{9c}\x4f\x02\u{49d}\
	\u{4c2}\x03\x02\x02\x02\u{49e}\u{4a0}\x07\x26\x02\x02\u{49f}\u{4a1}\x05\
	\u{c6}\x64\x02\u{4a0}\u{49f}\x03\x02\x02\x02\u{4a0}\u{4a1}\x03\x02\x02\x02\
	\u{4a1}\u{4a2}\x03\x02\x02\x02\u{4a2}\u{4c2}\x07\x56\x02\x02\u{4a3}\u{4a4}\
	\x07\x2e\x02\x02\u{4a4}\u{4a5}\x05\u{c6}\x64\x02\u{4a5}\u{4a6}\x07\x56\x02\
	\x02\u{4a6}\u{4c2}\x03\x02\x02\x02\u{4a7}\u{4a9}\x07\x06\x02\x02\u{4a8}\
	\u{4aa}\x05\u{a2}\x52\x02\u{4a9}\u{4a8}\x03\x02\x02\x02\u{4a9}\u{4aa}\x03\
	\x02\x02\x02\u{4aa}\u{4ab}\x03\x02\x02\x02\u{4ab}\u{4c2}\x07\x56\x02\x02\
	\u{4ac}\u{4ae}\x07\x0d\x02\x02\u{4ad}\u{4af}\x05\u{a2}\x52\x02\u{4ae}\u{4ad}\
	\x03\x02\x02\x02\u{4ae}\u{4af}\x03\x02\x02\x02\u{4af}\u{4b0}\x03\x02\x02\
	\x02\u{4b0}\u{4c2}\x07\x56\x02\x02\u{4b1}\u{4b2}\x07\x40\x02\x02\u{4b2}\
	\u{4b3}\x05\u{c6}\x64\x02\u{4b3}\u{4b4}\x07\x56\x02\x02\u{4b4}\u{4c2}\x03\
	\x02\x02\x02\u{4b5}\u{4c2}\x07\x56\x02\x02\u{4b6}\u{4b7}\x05\u{c6}\x64\x02\
	\u{4b7}\u{4b8}\x07\x56\x02\x02\u{4b8}\u{4c2}\x03\x02\x02\x02\u{4b9}\u{4bb}\
	\x05\u{d2}\x6a\x02\u{4ba}\u{4bc}\x07\x56\x02\x02\u{4bb}\u{4ba}\x03\x02\x02\
	\x02\u{4bb}\u{4bc}\x03\x02\x02\x02\u{4bc}\u{4c2}\x03\x02\x02\x02\u{4bd}\
	\u{4be}\x05\u{a2}\x52\x02\u{4be}\u{4bf}\x07\x5f\x02\x02\u{4bf}\u{4c0}\x05\
	\u{a8}\x55\x02\u{4c0}\u{4c2}\x03\x02\x02\x02\u{4c1}\u{450}\x03\x02\x02\x02\
	\u{4c1}\u{451}\x03\x02\x02\x02\u{4c1}\u{459}\x03\x02\x02\x02\u{4c1}\u{460}\
	\x03\x02\x02\x02\u{4c1}\u{466}\x03\x02\x02\x02\u{4c1}\u{46a}\x03\x02\x02\
	\x02\u{4c1}\u{470}\x03\x02\x02\x02\u{4c1}\u{47d}\x03\x02\x02\x02\u{4c1}\
	\u{489}\x03\x02\x02\x02\u{4c1}\u{49a}\x03\x02\x02\x02\u{4c1}\u{49e}\x03\
	\x02\x02\x02\u{4c1}\u{4a3}\x03\x02\x02\x02\u{4c1}\u{4a7}\x03\x02\x02\x02\
	\u{4c1}\u{4ac}\x03\x02\x02\x02\u{4c1}\u{4b1}\x03\x02\x02\x02\u{4c1}\u{4b5}\
	\x03\x02\x02\x02\u{4c1}\u{4b6}\x03\x02\x02\x02\u{4c1}\u{4b9}\x03\x02\x02\
	\x02\u{4c1}\u{4bd}\x03\x02\x02\x02\u{4c2}\u{a9}\x03\x02\x02\x02\u{4c3}\u{4c4}\
	\x07\x09\x02\x02\u{4c4}\u{4c8}\x07\x50\x02\x02\u{4c5}\u{4c7}\x05\x0c\x07\
	\x02\u{4c6}\u{4c5}\x03\x02\x02\x02\u{4c7}\u{4ca}\x03\x02\x02\x02\u{4c8}\
	\u{4c6}\x03\x02\x02\x02\u{4c8}\u{4c9}\x03\x02\x02\x02\u{4c9}\u{4cb}\x03\
	\x02\x02\x02\u{4ca}\u{4c8}\x03\x02\x02\x02\u{4cb}\u{4cc}\x05\u{ac}\x57\x02\
	\u{4cc}\u{4cd}\x05\u{a2}\x52\x02\u{4cd}\u{4ce}\x07\x51\x02\x02\u{4ce}\u{4cf}\
	\x05\u{9c}\x4f\x02\u{4cf}\u{ab}\x03\x02\x02\x02\u{4d0}\u{4d5}\x05\x66\x34\
	\x02\u{4d1}\u{4d2}\x07\x6d\x02\x02\u{4d2}\u{4d4}\x05\x66\x34\x02\u{4d3}\
	\u{4d1}\x03\x02\x02\x02\u{4d4}\u{4d7}\x03\x02\x02\x02\u{4d5}\u{4d3}\x03\
	\x02\x02\x02\u{4d5}\u{4d6}\x03\x02\x02\x02\u{4d6}\u{ad}\x03\x02\x02\x02\
	\u{4d7}\u{4d5}\x03\x02\x02\x02\u{4d8}\u{4d9}\x07\x15\x02\x02\u{4d9}\u{4da}\
	\x05\u{9c}\x4f\x02\u{4da}\u{af}\x03\x02\x02\x02\u{4db}\u{4dc}\x07\x50\x02\
	\x02\u{4dc}\u{4de}\x05\u{b2}\x5a\x02\u{4dd}\u{4df}\x07\x56\x02\x02\u{4de}\
	\u{4dd}\x03\x02\x02\x02\u{4de}\u{4df}\x03\x02\x02\x02\u{4df}\u{4e0}\x03\
	\x02\x02\x02\u{4e0}\u{4e1}\x07\x51\x02\x02\u{4e1}\u{b1}\x03\x02\x02\x02\
	\u{4e2}\u{4e7}\x05\u{b4}\x5b\x02\u{4e3}\u{4e4}\x07\x56\x02\x02\u{4e4}\u{4e6}\
	\x05\u{b4}\x5b\x02\u{4e5}\u{4e3}\x03\x02\x02\x02\u{4e6}\u{4e9}\x03\x02\x02\
	\x02\u{4e7}\u{4e5}\x03\x02\x02\x02\u{4e7}\u{4e8}\x03\x02\x02\x02\u{4e8}\
	\u{b3}\x03\x02\x02\x02\u{4e9}\u{4e7}\x03\x02\x02\x02\u{4ea}\u{4ec}\x05\x0c\
	\x07\x02\u{4eb}\u{4ea}\x03\x02\x02\x02\u{4ec}\u{4ef}\x03\x02\x02\x02\u{4ed}\
	\u{4eb}\x03\x02\x02\x02\u{4ed}\u{4ee}\x03\x02\x02\x02\u{4ee}\u{4f5}\x03\
	\x02\x02\x02\u{4ef}\u{4ed}\x03\x02\x02\x02\u{4f0}\u{4f1}\x05\x52\x2a\x02\
	\u{4f1}\u{4f2}\x05\x4c\x27\x02\u{4f2}\u{4f6}\x03\x02\x02\x02\u{4f3}\u{4f4}\
	\x07\x3f\x02\x02\u{4f4}\u{4f6}\x05\u{a2}\x52\x02\u{4f5}\u{4f0}\x03\x02\x02\
	\x02\u{4f5}\u{4f3}\x03\x02\x02\x02\u{4f6}\u{4f7}\x03\x02\x02\x02\u{4f7}\
	\u{4f8}\x07\x59\x02\x02\u{4f8}\u{4f9}\x05\u{c6}\x64\x02\u{4f9}\u{4fc}\x03\
	\x02\x02\x02\u{4fa}\u{4fc}\x05\u{a2}\x52\x02\u{4fb}\u{4ed}\x03\x02\x02\x02\
	\u{4fb}\u{4fa}\x03\x02\x02\x02\u{4fc}\u{b5}\x03\x02\x02\x02\u{4fd}\u{4ff}\
	\x05\u{b8}\x5d\x02\u{4fe}\u{4fd}\x03\x02\x02\x02\u{4ff}\u{500}\x03\x02\x02\
	\x02\u{500}\u{4fe}\x03\x02\x02\x02\u{500}\u{501}\x03\x02\x02\x02\u{501}\
	\u{503}\x03\x02\x02\x02\u{502}\u{504}\x05\u{9e}\x50\x02\u{503}\u{502}\x03\
	\x02\x02\x02\u{504}\u{505}\x03\x02\x02\x02\u{505}\u{503}\x03\x02\x02\x02\
	\u{505}\u{506}\x03\x02\x02\x02\u{506}\u{b7}\x03\x02\x02\x02\u{507}\u{50d}\
	\x07\x08\x02\x02\u{508}\u{50e}\x05\u{c6}\x64\x02\u{509}\u{50e}\x07\u{82}\
	\x02\x02\u{50a}\u{50b}\x05\u{f0}\x79\x02\u{50b}\u{50c}\x05\u{a2}\x52\x02\
	\u{50c}\u{50e}\x03\x02\x02\x02\u{50d}\u{508}\x03\x02\x02\x02\u{50d}\u{509}\
	\x03\x02\x02\x02\u{50d}\u{50a}\x03\x02\x02\x02\u{50e}\u{50f}\x03\x02\x02\
	\x02\u{50f}\u{513}\x07\x5f\x02\x02\u{510}\u{511}\x07\x0e\x02\x02\u{511}\
	\u{513}\x07\x5f\x02\x02\u{512}\u{507}\x03\x02\x02\x02\u{512}\u{510}\x03\
	\x02\x02\x02\u{513}\u{b9}\x03\x02\x02\x02\u{514}\u{521}\x05\u{be}\x60\x02\
	\u{515}\u{517}\x05\u{bc}\x5f\x02\u{516}\u{515}\x03\x02\x02\x02\u{516}\u{517}\
	\x03\x02\x02\x02\u{517}\u{518}\x03\x02\x02\x02\u{518}\u{51a}\x07\x56\x02\
	\x02\u{519}\u{51b}\x05\u{c6}\x64\x02\u{51a}\u{519}\x03\x02\x02\x02\u{51a}\
	\u{51b}\x03\x02\x02\x02\u{51b}\u{51c}\x03\x02\x02\x02\u{51c}\u{51e}\x07\
	\x56\x02\x02\u{51d}\u{51f}\x05\u{c2}\x62\x02\u{51e}\u{51d}\x03\x02\x02\x02\
	\u{51e}\u{51f}\x03\x02\x02\x02\u{51f}\u{521}\x03\x02\x02\x02\u{520}\u{514}\
	\x03\x02\x02\x02\u{520}\u{516}\x03\x02\x02\x02\u{521}\u{bb}\x03\x02\x02\
	\x02\u{522}\u{525}\x05\u{a0}\x51\x02\u{523}\u{525}\x05\u{c2}\x62\x02\u{524}\
	\u{522}\x03\x02\x02\x02\u{524}\u{523}\x03\x02\x02\x02\u{525}\u{bd}\x03\x02\
	\x02\x02\u{526}\u{528}\x05\x0c\x07\x02\u{527}\u{526}\x03\x02\x02\x02\u{528}\
	\u{52b}\x03\x02\x02\x02\u{529}\u{527}\x03\x02\x02\x02\u{529}\u{52a}\x03\
	\x02\x02\x02\u{52a}\u{52e}\x03\x02\x02\x02\u{52b}\u{529}\x03\x02\x02\x02\
	\u{52c}\u{52f}\x05\u{f0}\x79\x02\u{52d}\u{52f}\x07\x3f\x02\x02\u{52e}\u{52c}\
	\x03\x02\x02\x02\u{52e}\u{52d}\x03\x02\x02\x02\u{52f}\u{530}\x03\x02\x02\
	\x02\u{530}\u{531}\x05\x4c\x27\x02\u{531}\u{532}\x07\x5f\x02\x02\u{532}\
	\u{533}\x05\u{c6}\x64\x02\u{533}\u{bf}\x03\x02\x02\x02\u{534}\u{535}\x07\
	\x50\x02\x02\u{535}\u{536}\x05\u{c6}\x64\x02\u{536}\u{537}\x07\x51\x02\x02\
	\u{537}\u{c1}\x03\x02\x02\x02\u{538}\u{53d}\x05\u{c6}\x64\x02\u{539}\u{53a}\
	\x07\x57\x02\x02\u{53a}\u{53c}\x05\u{c6}\x64\x02\u{53b}\u{539}\x03\x02\x02\
	\x02\u{53c}\u{53f}\x03\x02\x02\x02\u{53d}\u{53b}\x03\x02\x02\x02\u{53d}\
	\u{53e}\x03\x02\x02\x02\u{53e}\u{c3}\x03\x02\x02\x02\u{53f}\u{53d}\x03\x02\
	\x02\x02\u{540}\u{541}\x05\u{a2}\x52\x02\u{541}\u{543}\x07\x50\x02\x02\u{542}\
	\u{544}\x05\u{c2}\x62\x02\u{543}\u{542}\x03\x02\x02\x02\u{543}\u{544}\x03\
	\x02\x02\x02\u{544}\u{545}\x03\x02\x02\x02\u{545}\u{546}\x07\x51\x02\x02\
	\u{546}\u{554}\x03\x02\x02\x02\u{547}\u{548}\x07\x2d\x02\x02\u{548}\u{54a}\
	\x07\x50\x02\x02\u{549}\u{54b}\x05\u{c2}\x62\x02\u{54a}\u{549}\x03\x02\x02\
	\x02\u{54a}\u{54b}\x03\x02\x02\x02\u{54b}\u{54c}\x03\x02\x02\x02\u{54c}\
	\u{554}\x07\x51\x02\x02\u{54d}\u{54e}\x07\x2a\x02\x02\u{54e}\u{550}\x07\
	\x50\x02\x02\u{54f}\u{551}\x05\u{c2}\x62\x02\u{550}\u{54f}\x03\x02\x02\x02\
	\u{550}\u{551}\x03\x02\x02\x02\u{551}\u{552}\x03\x02\x02\x02\u{552}\u{554}\
	\x07\x51\x02\x02\u{553}\u{540}\x03\x02\x02\x02\u{553}\u{547}\x03\x02\x02\
	\x02\u{553}\u{54d}\x03\x02\x02\x02\u{554}\u{c5}\x03\x02\x02\x02\u{555}\u{556}\
	\x08\x64\x01\x02\u{556}\u{583}\x05\u{d0}\x69\x02\u{557}\u{583}\x05\u{c4}\
	\x63\x02\u{558}\u{559}\x07\x21\x02\x02\u{559}\u{583}\x05\u{dc}\x6f\x02\u{55a}\
	\u{55e}\x07\x50\x02\x02\u{55b}\u{55d}\x05\x70\x39\x02\u{55c}\u{55b}\x03\
	\x02\x02\x02\u{55d}\u{560}\x03\x02\x02\x02\u{55e}\u{55c}\x03\x02\x02\x02\
	\u{55e}\u{55f}\x03\x02\x02\x02\u{55f}\u{561}\x03\x02\x02\x02\u{560}\u{55e}\
	\x03\x02\x02\x02\u{561}\u{566}\x05\u{f0}\x79\x02\u{562}\u{563}\x07\x6c\x02\
	\x02\u{563}\u{565}\x05\u{f0}\x79\x02\u{564}\u{562}\x03\x02\x02\x02\u{565}\
	\u{568}\x03\x02\x02\x02\u{566}\u{564}\x03\x02\x02\x02\u{566}\u{567}\x03\
	\x02\x02\x02\u{567}\u{569}\x03\x02\x02\x02\u{568}\u{566}\x03\x02\x02\x02\
	\u{569}\u{56a}\x07\x51\x02\x02\u{56a}\u{56b}\x05\u{c6}\x64\x18\u{56b}\u{583}\
	\x03\x02\x02\x02\u{56c}\u{56d}\x09\x09\x02\x02\u{56d}\u{583}\x05\u{c6}\x64\
	\x16\u{56e}\u{56f}\x09\x0a\x02\x02\u{56f}\u{583}\x05\u{c6}\x64\x15\u{570}\
	\u{583}\x05\u{ca}\x66\x02\u{571}\u{583}\x05\u{d2}\x6a\x02\u{572}\u{573}\
	\x05\u{f0}\x79\x02\u{573}\u{579}\x07\x7c\x02\x02\u{574}\u{576}\x05\u{f4}\
	\x7b\x02\u{575}\u{574}\x03\x02\x02\x02\u{575}\u{576}\x03\x02\x02\x02\u{576}\
	\u{577}\x03\x02\x02\x02\u{577}\u{57a}\x05\u{a2}\x52\x02\u{578}\u{57a}\x07\
	\x21\x02\x02\u{579}\u{575}\x03\x02\x02\x02\u{579}\u{578}\x03\x02\x02\x02\
	\u{57a}\u{583}\x03\x02\x02\x02\u{57b}\u{57c}\x05\u{da}\x6e\x02\u{57c}\u{57e}\
	\x07\x7c\x02\x02\u{57d}\u{57f}\x05\u{f4}\x7b\x02\u{57e}\u{57d}\x03\x02\x02\
	\x02\u{57e}\u{57f}\x03\x02\x02\x02\u{57f}\u{580}\x03\x02\x02\x02\u{580}\
	\u{581}\x07\x21\x02\x02\u{581}\u{583}\x03\x02\x02\x02\u{582}\u{555}\x03\
	\x02\x02\x02\u{582}\u{557}\x03\x02\x02\x02\u{582}\u{558}\x03\x02\x02\x02\
	\u{582}\u{55a}\x03\x02\x02\x02\u{582}\u{56c}\x03\x02\x02\x02\u{582}\u{56e}\
	\x03\x02\x02\x02\u{582}\u{570}\x03\x02\x02\x02\u{582}\u{571}\x03\x02\x02\
	\x02\u{582}\u{572}\x03\x02\x02\x02\u{582}\u{57b}\x03\x02\x02\x02\u{583}\
	\u{5d7}\x03\x02\x02\x02\u{584}\u{585}\x0c\x14\x02\x02\u{585}\u{586}\x09\
	\x0b\x02\x02\u{586}\u{5d6}\x05\u{c6}\x64\x15\u{587}\u{588}\x0c\x13\x02\x02\
	\u{588}\u{589}\x09\x0c\x02\x02\u{589}\u{5d6}\x05\u{c6}\x64\x14\u{58a}\u{592}\
	\x0c\x12\x02\x02\u{58b}\u{58c}\x07\x5b\x02\x02\u{58c}\u{593}\x07\x5b\x02\
	\x02\u{58d}\u{58e}\x07\x5a\x02\x02\u{58e}\u{58f}\x07\x5a\x02\x02\u{58f}\
	\u{593}\x07\x5a\x02\x02\u{590}\u{591}\x07\x5a\x02\x02\u{591}\u{593}\x07\
	\x5a\x02\x02\u{592}\u{58b}\x03\x02\x02\x02\u{592}\u{58d}\x03\x02\x02\x02\
	\u{592}\u{590}\x03\x02\x02\x02\u{593}\u{594}\x03\x02\x02\x02\u{594}\u{5d6}\
	\x05\u{c6}\x64\x13\u{595}\u{596}\x0c\x11\x02\x02\u{596}\u{597}\x09\x0d\x02\
	\x02\u{597}\u{5d6}\x05\u{c6}\x64\x12\u{598}\u{599}\x0c\x0f\x02\x02\u{599}\
	\u{59a}\x09\x0e\x02\x02\u{59a}\u{5d6}\x05\u{c6}\x64\x10\u{59b}\u{59c}\x0c\
	\x0e\x02\x02\u{59c}\u{59d}\x07\x6c\x02\x02\u{59d}\u{5d6}\x05\u{c6}\x64\x0f\
	\u{59e}\u{59f}\x0c\x0d\x02\x02\u{59f}\u{5a0}\x07\x6e\x02\x02\u{5a0}\u{5d6}\
	\x05\u{c6}\x64\x0e\u{5a1}\u{5a2}\x0c\x0c\x02\x02\u{5a2}\u{5a3}\x07\x6d\x02\
	\x02\u{5a3}\u{5d6}\x05\u{c6}\x64\x0d\u{5a4}\u{5a5}\x0c\x0b\x02\x02\u{5a5}\
	\u{5a6}\x07\x64\x02\x02\u{5a6}\u{5d6}\x05\u{c6}\x64\x0c\u{5a7}\u{5a8}\x0c\
	\x0a\x02\x02\u{5a8}\u{5a9}\x07\x65\x02\x02\u{5a9}\u{5d6}\x05\u{c6}\x64\x0b\
	\u{5aa}\u{5ab}\x0c\x09\x02\x02\u{5ab}\u{5ac}\x07\x5e\x02\x02\u{5ac}\u{5ad}\
	\x05\u{c6}\x64\x02\u{5ad}\u{5ae}\x07\x5f\x02\x02\u{5ae}\u{5af}\x05\u{c6}\
	\x64\x09\u{5af}\u{5d6}\x03\x02\x02\x02\u{5b0}\u{5b1}\x0c\x08\x02\x02\u{5b1}\
	\u{5b2}\x09\x0f\x02\x02\u{5b2}\u{5d6}\x05\u{c6}\x64\x08\u{5b3}\u{5b4}\x0c\
	\x1c\x02\x02\u{5b4}\u{5c0}\x07\x58\x02\x02\u{5b5}\u{5c1}\x05\u{a2}\x52\x02\
	\u{5b6}\u{5c1}\x05\u{c4}\x63\x02\u{5b7}\u{5c1}\x07\x2d\x02\x02\u{5b8}\u{5ba}\
	\x07\x21\x02\x02\u{5b9}\u{5bb}\x05\u{ec}\x77\x02\u{5ba}\u{5b9}\x03\x02\x02\
	\x02\u{5ba}\u{5bb}\x03\x02\x02\x02\u{5bb}\u{5bc}\x03\x02\x02\x02\u{5bc}\
	\u{5c1}\x05\u{e0}\x71\x02\u{5bd}\u{5be}\x07\x2a\x02\x02\u{5be}\u{5c1}\x05\
	\u{f6}\x7c\x02\u{5bf}\u{5c1}\x05\u{e6}\x74\x02\u{5c0}\u{5b5}\x03\x02\x02\
	\x02\u{5c0}\u{5b6}\x03\x02\x02\x02\u{5c0}\u{5b7}\x03\x02\x02\x02\u{5c0}\
	\u{5b8}\x03\x02\x02\x02\u{5c0}\u{5bd}\x03\x02\x02\x02\u{5c0}\u{5bf}\x03\
	\x02\x02\x02\u{5c1}\u{5d6}\x03\x02\x02\x02\u{5c2}\u{5c3}\x0c\x1b\x02\x02\
	\u{5c3}\u{5c4}\x07\x54\x02\x02\u{5c4}\u{5c5}\x05\u{c6}\x64\x02\u{5c5}\u{5c6}\
	\x07\x55\x02\x02\u{5c6}\u{5d6}\x03\x02\x02\x02\u{5c7}\u{5c8}\x0c\x17\x02\
	\x02\u{5c8}\u{5d6}\x09\x10\x02\x02\u{5c9}\u{5ca}\x0c\x10\x02\x02\u{5ca}\
	\u{5cd}\x07\x1c\x02\x02\u{5cb}\u{5ce}\x05\u{f0}\x79\x02\u{5cc}\u{5ce}\x05\
	\u{c8}\x65\x02\u{5cd}\u{5cb}\x03\x02\x02\x02\u{5cd}\u{5cc}\x03\x02\x02\x02\
	\u{5ce}\u{5d6}\x03\x02\x02\x02\u{5cf}\u{5d0}\x0c\x05\x02\x02\u{5d0}\u{5d2}\
	\x07\x7c\x02\x02\u{5d1}\u{5d3}\x05\u{f4}\x7b\x02\u{5d2}\u{5d1}\x03\x02\x02\
	\x02\u{5d2}\u{5d3}\x03\x02\x02\x02\u{5d3}\u{5d4}\x03\x02\x02\x02\u{5d4}\
	\u{5d6}\x05\u{a2}\x52\x02\u{5d5}\u{584}\x03\x02\x02\x02\u{5d5}\u{587}\x03\
	\x02\x02\x02\u{5d5}\u{58a}\x03\x02\x02\x02\u{5d5}\u{595}\x03\x02\x02\x02\
	\u{5d5}\u{598}\x03\x02\x02\x02\u{5d5}\u{59b}\x03\x02\x02\x02\u{5d5}\u{59e}\
	\x03\x02\x02\x02\u{5d5}\u{5a1}\x03\x02\x02\x02\u{5d5}\u{5a4}\x03\x02\x02\
	\x02\u{5d5}\u{5a7}\x03\x02\x02\x02\u{5d5}\u{5aa}\x03\x02\x02\x02\u{5d5}\
	\u{5b0}\x03\x02\x02\x02\u{5d5}\u{5b3}\x03\x02\x02\x02\u{5d5}\u{5c2}\x03\
	\x02\x02\x02\u{5d5}\u{5c7}\x03\x02\x02\x02\u{5d5}\u{5c9}\x03\x02\x02\x02\
	\u{5d5}\u{5cf}\x03\x02\x02\x02\u{5d6}\u{5d9}\x03\x02\x02\x02\u{5d7}\u{5d5}\
	\x03\x02\x02\x02\u{5d7}\u{5d8}\x03\x02\x02\x02\u{5d8}\u{c7}\x03\x02\x02\
	\x02\u{5d9}\u{5d7}\x03\x02\x02\x02\u{5da}\u{5dc}\x05\x0c\x07\x02\u{5db}\
	\u{5da}\x03\x02\x02\x02\u{5dc}\u{5df}\x03\x02\x02\x02\u{5dd}\u{5db}\x03\
	\x02\x02\x02\u{5dd}\u{5de}\x03\x02\x02\x02\u{5de}\u{5e0}\x03\x02\x02\x02\
	\u{5df}\u{5dd}\x03\x02\x02\x02\u{5e0}\u{5e4}\x05\u{f0}\x79\x02\u{5e1}\u{5e3}\
	\x05\x70\x39\x02\u{5e2}\u{5e1}\x03\x02\x02\x02\u{5e3}\u{5e6}\x03\x02\x02\
	\x02\u{5e4}\u{5e2}\x03\x02\x02\x02\u{5e4}\u{5e5}\x03\x02\x02\x02\u{5e5}\
	\u{5e7}\x03\x02\x02\x02\u{5e6}\u{5e4}\x03\x02\x02\x02\u{5e7}\u{5e8}\x05\
	\u{a2}\x52\x02\u{5e8}\u{c9}\x03\x02\x02\x02\u{5e9}\u{5ea}\x05\u{cc}\x67\
	\x02\u{5ea}\u{5eb}\x07\x7b\x02\x02\u{5eb}\u{5ec}\x05\u{ce}\x68\x02\u{5ec}\
	\u{cb}\x03\x02\x02\x02\u{5ed}\u{604}\x05\u{a2}\x52\x02\u{5ee}\u{5f0}\x07\
	\x50\x02\x02\u{5ef}\u{5f1}\x05\x5c\x2f\x02\u{5f0}\u{5ef}\x03\x02\x02\x02\
	\u{5f0}\u{5f1}\x03\x02\x02\x02\u{5f1}\u{5f2}\x03\x02\x02\x02\u{5f2}\u{604}\
	\x07\x51\x02\x02\u{5f3}\u{5f4}\x07\x50\x02\x02\u{5f4}\u{5f9}\x05\u{a2}\x52\
	\x02\u{5f5}\u{5f6}\x07\x57\x02\x02\u{5f6}\u{5f8}\x05\u{a2}\x52\x02\u{5f7}\
	\u{5f5}\x03\x02\x02\x02\u{5f8}\u{5fb}\x03\x02\x02\x02\u{5f9}\u{5f7}\x03\
	\x02\x02\x02\u{5f9}\u{5fa}\x03\x02\x02\x02\u{5fa}\u{5fc}\x03\x02\x02\x02\
	\u{5fb}\u{5f9}\x03\x02\x02\x02\u{5fc}\u{5fd}\x07\x51\x02\x02\u{5fd}\u{604}\
	\x03\x02\x02\x02\u{5fe}\u{600}\x07\x50\x02\x02\u{5ff}\u{601}\x05\x62\x32\
	\x02\u{600}\u{5ff}\x03\x02\x02\x02\u{600}\u{601}\x03\x02\x02\x02\u{601}\
	\u{602}\x03\x02\x02\x02\u{602}\u{604}\x07\x51\x02\x02\u{603}\u{5ed}\x03\
	\x02\x02\x02\u{603}\u{5ee}\x03\x02\x02\x02\u{603}\u{5f3}\x03\x02\x02\x02\
	\u{603}\u{5fe}\x03\x02\x02\x02\u{604}\u{cd}\x03\x02\x02\x02\u{605}\u{608}\
	\x05\u{c6}\x64\x02\u{606}\u{608}\x05\u{9c}\x4f\x02\u{607}\u{605}\x03\x02\
	\x02\x02\u{607}\u{606}\x03\x02\x02\x02\u{608}\u{cf}\x03\x02\x02\x02\u{609}\
	\u{60a}\x07\x50\x02\x02\u{60a}\u{60b}\x05\u{c6}\x64\x02\u{60b}\u{60c}\x07\
	\x51\x02\x02\u{60c}\u{61c}\x03\x02\x02\x02\u{60d}\u{61c}\x07\x2d\x02\x02\
	\u{60e}\u{61c}\x07\x2a\x02\x02\u{60f}\u{61c}\x05\x68\x35\x02\u{610}\u{61c}\
	\x05\u{a2}\x52\x02\u{611}\u{612}\x05\x2c\x17\x02\u{612}\u{613}\x07\x58\x02\
	\x02\u{613}\u{614}\x07\x0b\x02\x02\u{614}\u{61c}\x03\x02\x02\x02\u{615}\
	\u{619}\x05\u{ec}\x77\x02\u{616}\u{61a}\x05\u{f8}\x7d\x02\u{617}\u{618}\
	\x07\x2d\x02\x02\u{618}\u{61a}\x05\u{fa}\x7e\x02\u{619}\u{616}\x03\x02\x02\
	\x02\u{619}\u{617}\x03\x02\x02\x02\u{61a}\u{61c}\x03\x02\x02\x02\u{61b}\
	\u{609}\x03\x02\x02\x02\u{61b}\u{60d}\x03\x02\x02\x02\u{61b}\u{60e}\x03\
	\x02\x02\x02\u{61b}\u{60f}\x03\x02\x02\x02\u{61b}\u{610}\x03\x02\x02\x02\
	\u{61b}\u{611}\x03\x02\x02\x02\u{61b}\u{615}\x03\x02\x02\x02\u{61c}\u{d1}\
	\x03\x02\x02\x02\u{61d}\u{61e}\x07\x2b\x02\x02\u{61e}\u{61f}\x05\u{c0}\x61\
	\x02\u{61f}\u{623}\x07\x52\x02\x02\u{620}\u{622}\x05\u{d4}\x6b\x02\u{621}\
	\u{620}\x03\x02\x02\x02\u{622}\u{625}\x03\x02\x02\x02\u{623}\u{621}\x03\
	\x02\x02\x02\u{623}\u{624}\x03\x02\x02\x02\u{624}\u{626}\x03\x02\x02\x02\
	\u{625}\u{623}\x03\x02\x02\x02\u{626}\u{627}\x07\x53\x02\x02\u{627}\u{d3}\
	\x03\x02\x02\x02\u{628}\u{62c}\x07\x08\x02\x02\u{629}\u{62d}\x05\u{c2}\x62\
	\x02\u{62a}\u{62d}\x07\x4f\x02\x02\u{62b}\u{62d}\x05\u{d6}\x6c\x02\u{62c}\
	\u{629}\x03\x02\x02\x02\u{62c}\u{62a}\x03\x02\x02\x02\u{62c}\u{62b}\x03\
	\x02\x02\x02\u{62d}\u{62e}\x03\x02\x02\x02\u{62e}\u{62f}\x09\x11\x02\x02\
	\u{62f}\u{634}\x05\u{d8}\x6d\x02\u{630}\u{631}\x07\x0e\x02\x02\u{631}\u{632}\
	\x09\x11\x02\x02\u{632}\u{634}\x05\u{d8}\x6d\x02\u{633}\u{628}\x03\x02\x02\
	\x02\u{633}\u{630}\x03\x02\x02\x02\u{634}\u{d5}\x03\x02\x02\x02\u{635}\u{636}\
	\x08\x6c\x01\x02\u{636}\u{637}\x07\x50\x02\x02\u{637}\u{638}\x05\u{d6}\x6c\
	\x02\u{638}\u{639}\x07\x51\x02\x02\u{639}\u{650}\x03\x02\x02\x02\u{63a}\
	\u{63c}\x05\x0c\x07\x02\u{63b}\u{63a}\x03\x02\x02\x02\u{63c}\u{63f}\x03\
	\x02\x02\x02\u{63d}\u{63b}\x03\x02\x02\x02\u{63d}\u{63e}\x03\x02\x02\x02\
	\u{63e}\u{640}\x03\x02\x02\x02\u{63f}\u{63d}\x03\x02\x02\x02\u{640}\u{644}\
	\x05\u{f0}\x79\x02\u{641}\u{643}\x05\x70\x39\x02\u{642}\u{641}\x03\x02\x02\
	\x02\u{643}\u{646}\x03\x02\x02\x02\u{644}\u{642}\x03\x02\x02\x02\u{644}\
	\u{645}\x03\x02\x02\x02\u{645}\u{647}\x03\x02\x02\x02\u{646}\u{644}\x03\
	\x02\x02\x02\u{647}\u{64c}\x05\u{a2}\x52\x02\u{648}\u{649}\x07\x64\x02\x02\
	\u{649}\u{64b}\x05\u{c6}\x64\x02\u{64a}\u{648}\x03\x02\x02\x02\u{64b}\u{64e}\
	\x03\x02\x02\x02\u{64c}\u{64a}\x03\x02\x02\x02\u{64c}\u{64d}\x03\x02\x02\
	\x02\u{64d}\u{650}\x03\x02\x02\x02\u{64e}\u{64c}\x03\x02\x02\x02\u{64f}\
	\u{635}\x03\x02\x02\x02\u{64f}\u{63d}\x03\x02\x02\x02\u{650}\u{656}\x03\
	\x02\x02\x02\u{651}\u{652}\x0c\x03\x02\x02\u{652}\u{653}\x07\x64\x02\x02\
	\u{653}\u{655}\x05\u{c6}\x64\x02\u{654}\u{651}\x03\x02\x02\x02\u{655}\u{658}\
	\x03\x02\x02\x02\u{656}\u{654}\x03\x02\x02\x02\u{656}\u{657}\x03\x02\x02\
	\x02\u{657}\u{d7}\x03\x02\x02\x02\u{658}\u{656}\x03\x02\x02\x02\u{659}\u{661}\
	\x05\u{9c}\x4f\x02\u{65a}\u{65c}\x05\u{9e}\x50\x02\u{65b}\u{65a}\x03\x02\
	\x02\x02\u{65c}\u{65f}\x03\x02\x02\x02\u{65d}\u{65b}\x03\x02\x02\x02\u{65d}\
	\u{65e}\x03\x02\x02\x02\u{65e}\u{661}\x03\x02\x02\x02\u{65f}\u{65d}\x03\
	\x02\x02\x02\u{660}\u{659}\x03\x02\x02\x02\u{660}\u{65d}\x03\x02\x02\x02\
	\u{661}\u{d9}\x03\x02\x02\x02\u{662}\u{663}\x05\x52\x2a\x02\u{663}\u{664}\
	\x07\x58\x02\x02\u{664}\u{666}\x03\x02\x02\x02\u{665}\u{662}\x03\x02\x02\
	\x02\u{665}\u{666}\x03\x02\x02\x02\u{666}\u{66a}\x03\x02\x02\x02\u{667}\
	\u{669}\x05\x70\x39\x02\u{668}\u{667}\x03\x02\x02\x02\u{669}\u{66c}\x03\
	\x02\x02\x02\u{66a}\u{668}\x03\x02\x02\x02\u{66a}\u{66b}\x03\x02\x02\x02\
	\u{66b}\u{66d}\x03\x02\x02\x02\u{66c}\u{66a}\x03\x02\x02\x02\u{66d}\u{66f}\
	\x05\u{a2}\x52\x02\u{66e}\u{670}\x05\u{f4}\x7b\x02\u{66f}\u{66e}\x03\x02\
	\x02\x02\u{66f}\u{670}\x03\x02\x02\x02\u{670}\u{db}\x03\x02\x02\x02\u{671}\
	\u{672}\x05\u{ec}\x77\x02\u{672}\u{673}\x05\u{de}\x70\x02\u{673}\u{674}\
	\x05\u{e4}\x73\x02\u{674}\u{67b}\x03\x02\x02\x02\u{675}\u{678}\x05\u{de}\
	\x70\x02\u{676}\u{679}\x05\u{e2}\x72\x02\u{677}\u{679}\x05\u{e4}\x73\x02\
	\u{678}\u{676}\x03\x02\x02\x02\u{678}\u{677}\x03\x02\x02\x02\u{679}\u{67b}\
	\x03\x02\x02\x02\u{67a}\u{671}\x03\x02\x02\x02\u{67a}\u{675}\x03\x02\x02\
	\x02\u{67b}\u{dd}\x03\x02\x02\x02\u{67c}\u{67e}\x05\u{a2}\x52\x02\u{67d}\
	\u{67f}\x05\u{e8}\x75\x02\u{67e}\u{67d}\x03\x02\x02\x02\u{67e}\u{67f}\x03\
	\x02\x02\x02\u{67f}\u{687}\x03\x02\x02\x02\u{680}\u{681}\x07\x58\x02\x02\
	\u{681}\u{683}\x05\u{a2}\x52\x02\u{682}\u{684}\x05\u{e8}\x75\x02\u{683}\
	\u{682}\x03\x02\x02\x02\u{683}\u{684}\x03\x02\x02\x02\u{684}\u{686}\x03\
	\x02\x02\x02\u{685}\u{680}\x03\x02\x02\x02\u{686}\u{689}\x03\x02\x02\x02\
	\u{687}\u{685}\x03\x02\x02\x02\u{687}\u{688}\x03\x02\x02\x02\u{688}\u{68c}\
	\x03\x02\x02\x02\u{689}\u{687}\x03\x02\x02\x02\u{68a}\u{68c}\x05\u{f2}\x7a\
	\x02\u{68b}\u{67c}\x03\x02\x02\x02\u{68b}\u{68a}\x03\x02\x02\x02\u{68c}\
	\u{df}\x03\x02\x02\x02\u{68d}\u{68f}\x05\u{a2}\x52\x02\u{68e}\u{690}\x05\
	\u{ea}\x76\x02\u{68f}\u{68e}\x03\x02\x02\x02\u{68f}\u{690}\x03\x02\x02\x02\
	\u{690}\u{691}\x03\x02\x02\x02\u{691}\u{692}\x05\u{e4}\x73\x02\u{692}\u{e1}\
	\x03\x02\x02\x02\u{693}\u{6af}\x07\x54\x02\x02\u{694}\u{699}\x07\x55\x02\
	\x02\u{695}\u{696}\x07\x54\x02\x02\u{696}\u{698}\x07\x55\x02\x02\u{697}\
	\u{695}\x03\x02\x02\x02\u{698}\u{69b}\x03\x02\x02\x02\u{699}\u{697}\x03\
	\x02\x02\x02\u{699}\u{69a}\x03\x02\x02\x02\u{69a}\u{69c}\x03\x02\x02\x02\
	\u{69b}\u{699}\x03\x02\x02\x02\u{69c}\u{6b0}\x05\x50\x29\x02\u{69d}\u{69e}\
	\x05\u{c6}\x64\x02\u{69e}\u{6a5}\x07\x55\x02\x02\u{69f}\u{6a0}\x07\x54\x02\
	\x02\u{6a0}\u{6a1}\x05\u{c6}\x64\x02\u{6a1}\u{6a2}\x07\x55\x02\x02\u{6a2}\
	\u{6a4}\x03\x02\x02\x02\u{6a3}\u{69f}\x03\x02\x02\x02\u{6a4}\u{6a7}\x03\
	\x02\x02\x02\u{6a5}\u{6a3}\x03\x02\x02\x02\u{6a5}\u{6a6}\x03\x02\x02\x02\
	\u{6a6}\u{6ac}\x03\x02\x02\x02\u{6a7}\u{6a5}\x03\x02\x02\x02\u{6a8}\u{6a9}\
	\x07\x54\x02\x02\u{6a9}\u{6ab}\x07\x55\x02\x02\u{6aa}\u{6a8}\x03\x02\x02\
	\x02\u{6ab}\u{6ae}\x03\x02\x02\x02\u{6ac}\u{6aa}\x03\x02\x02\x02\u{6ac}\
	\u{6ad}\x03\x02\x02\x02\u{6ad}\u{6b0}\x03\x02\x02\x02\u{6ae}\u{6ac}\x03\
	\x02\x02\x02\u{6af}\u{694}\x03\x02\x02\x02\u{6af}\u{69d}\x03\x02\x02\x02\
	\u{6b0}\u{e3}\x03\x02\x02\x02\u{6b1}\u{6b3}\x05\u{fa}\x7e\x02\u{6b2}\u{6b4}\
	\x05\x20\x11\x02\u{6b3}\u{6b2}\x03\x02\x02\x02\u{6b3}\u{6b4}\x03\x02\x02\
	\x02\u{6b4}\u{e5}\x03\x02\x02\x02\u{6b5}\u{6b6}\x05\u{ec}\x77\x02\u{6b6}\
	\u{6b7}\x05\u{f8}\x7d\x02\u{6b7}\u{e7}\x03\x02\x02\x02\u{6b8}\u{6b9}\x07\
	\x5b\x02\x02\u{6b9}\u{6bc}\x07\x5a\x02\x02\u{6ba}\u{6bc}\x05\u{f4}\x7b\x02\
	\u{6bb}\u{6b8}\x03\x02\x02\x02\u{6bb}\u{6ba}\x03\x02\x02\x02\u{6bc}\u{e9}\
	\x03\x02\x02\x02\u{6bd}\u{6be}\x07\x5b\x02\x02\u{6be}\u{6c1}\x07\x5a\x02\
	\x02\u{6bf}\u{6c1}\x05\u{ec}\x77\x02\u{6c0}\u{6bd}\x03\x02\x02\x02\u{6c0}\
	\u{6bf}\x03\x02\x02\x02\u{6c1}\u{eb}\x03\x02\x02\x02\u{6c2}\u{6c3}\x07\x5b\
	\x02\x02\u{6c3}\u{6c4}\x05\u{ee}\x78\x02\u{6c4}\u{6c5}\x07\x5a\x02\x02\u{6c5}\
	\u{ed}\x03\x02\x02\x02\u{6c6}\u{6cb}\x05\u{f0}\x79\x02\u{6c7}\u{6c8}\x07\
	\x57\x02\x02\u{6c8}\u{6ca}\x05\u{f0}\x79\x02\u{6c9}\u{6c7}\x03\x02\x02\x02\
	\u{6ca}\u{6cd}\x03\x02\x02\x02\u{6cb}\u{6c9}\x03\x02\x02\x02\u{6cb}\u{6cc}\
	\x03\x02\x02\x02\u{6cc}\u{ef}\x03\x02\x02\x02\u{6cd}\u{6cb}\x03\x02\x02\
	\x02\u{6ce}\u{6d0}\x05\x70\x39\x02\u{6cf}\u{6ce}\x03\x02\x02\x02\u{6d0}\
	\u{6d3}\x03\x02\x02\x02\u{6d1}\u{6cf}\x03\x02\x02\x02\u{6d1}\u{6d2}\x03\
	\x02\x02\x02\u{6d2}\u{6d6}\x03\x02\x02\x02\u{6d3}\u{6d1}\x03\x02\x02\x02\
	\u{6d4}\u{6d7}\x05\x52\x2a\x02\u{6d5}\u{6d7}\x05\u{f2}\x7a\x02\u{6d6}\u{6d4}\
	\x03\x02\x02\x02\u{6d6}\u{6d5}\x03\x02\x02\x02\u{6d7}\u{6e2}\x03\x02\x02\
	\x02\u{6d8}\u{6da}\x05\x70\x39\x02\u{6d9}\u{6d8}\x03\x02\x02\x02\u{6da}\
	\u{6dd}\x03\x02\x02\x02\u{6db}\u{6d9}\x03\x02\x02\x02\u{6db}\u{6dc}\x03\
	\x02\x02\x02\u{6dc}\u{6de}\x03\x02\x02\x02\u{6dd}\u{6db}\x03\x02\x02\x02\
	\u{6de}\u{6df}\x07\x54\x02\x02\u{6df}\u{6e1}\x07\x55\x02\x02\u{6e0}\u{6db}\
	\x03\x02\x02\x02\u{6e1}\u{6e4}\x03\x02\x02\x02\u{6e2}\u{6e0}\x03\x02\x02\
	\x02\u{6e2}\u{6e3}\x03\x02\x02\x02\u{6e3}\u{f1}\x03\x02\x02\x02\u{6e4}\u{6e2}\
	\x03\x02\x02\x02\u{6e5}\u{6e6}\x09\x12\x02\x02\u{6e6}\u{f3}\x03\x02\x02\
	\x02\u{6e7}\u{6e8}\x07\x5b\x02\x02\u{6e8}\u{6ed}\x05\x54\x2b\x02\u{6e9}\
	\u{6ea}\x07\x57\x02\x02\u{6ea}\u{6ec}\x05\x54\x2b\x02\u{6eb}\u{6e9}\x03\
	\x02\x02\x02\u{6ec}\u{6ef}\x03\x02\x02\x02\u{6ed}\u{6eb}\x03\x02\x02\x02\
	\u{6ed}\u{6ee}\x03\x02\x02\x02\u{6ee}\u{6f0}\x03\x02\x02\x02\u{6ef}\u{6ed}\
	\x03\x02\x02\x02\u{6f0}\u{6f1}\x07\x5a\x02\x02\u{6f1}\u{f5}\x03\x02\x02\
	\x02\u{6f2}\u{6fc}\x05\u{fa}\x7e\x02\u{6f3}\u{6f5}\x07\x58\x02\x02\u{6f4}\
	\u{6f6}\x05\u{f4}\x7b\x02\u{6f5}\u{6f4}\x03\x02\x02\x02\u{6f5}\u{6f6}\x03\
	\x02\x02\x02\u{6f6}\u{6f7}\x03\x02\x02\x02\u{6f7}\u{6f9}\x05\u{a2}\x52\x02\
	\u{6f8}\u{6fa}\x05\u{fa}\x7e\x02\u{6f9}\u{6f8}\x03\x02\x02\x02\u{6f9}\u{6fa}\
	\x03\x02\x02\x02\u{6fa}\u{6fc}\x03\x02\x02\x02\u{6fb}\u{6f2}\x03\x02\x02\
	\x02\u{6fb}\u{6f3}\x03\x02\x02\x02\u{6fc}\u{f7}\x03\x02\x02\x02\u{6fd}\u{6fe}\
	\x07\x2a\x02\x02\u{6fe}\u{703}\x05\u{f6}\x7c\x02\u{6ff}\u{700}\x05\u{a2}\
	\x52\x02\u{700}\u{701}\x05\u{fa}\x7e\x02\u{701}\u{703}\x03\x02\x02\x02\u{702}\
	\u{6fd}\x03\x02\x02\x02\u{702}\u{6ff}\x03\x02\x02\x02\u{703}\u{f9}\x03\x02\
	\x02\x02\u{704}\u{706}\x07\x50\x02\x02\u{705}\u{707}\x05\u{c2}\x62\x02\u{706}\
	\u{705}\x03\x02\x02\x02\u{706}\u{707}\x03\x02\x02\x02\u{707}\u{708}\x03\
	\x02\x02\x02\u{708}\u{709}\x07\x51\x02\x02\u{709}\u{fb}\x03\x02\x02\x02\
	\u{e4}\u{ff}\u{103}\u{108}\u{10e}\u{114}\u{119}\u{122}\u{127}\u{12e}\u{134}\
	\u{13c}\u{13f}\u{145}\u{14a}\u{14e}\u{152}\u{156}\u{160}\u{168}\u{170}\u{174}\
	\u{17b}\u{182}\u{186}\u{189}\u{18c}\u{195}\u{19b}\u{1a0}\u{1a3}\u{1a9}\u{1af}\
	\u{1b3}\u{1b7}\u{1bf}\u{1c8}\u{1cf}\u{1d5}\u{1db}\u{1df}\u{1eb}\u{1f4}\u{1f9}\
	\u{1ff}\u{203}\u{20f}\u{216}\u{21c}\u{229}\u{22f}\u{234}\u{23e}\u{246}\u{250}\
	\u{259}\u{264}\u{269}\u{272}\u{27c}\u{281}\u{28a}\u{290}\u{297}\u{29c}\u{2a4}\
	\u{2a8}\u{2aa}\u{2b0}\u{2b6}\u{2bb}\u{2c1}\u{2c7}\u{2c9}\u{2d0}\u{2d5}\u{2da}\
	\u{2dd}\u{2df}\u{2e9}\u{2f3}\u{2f8}\u{2fb}\u{300}\u{309}\u{310}\u{31b}\u{321}\
	\u{32c}\u{336}\u{341}\u{34a}\u{34f}\u{352}\u{359}\u{363}\u{36b}\u{36e}\u{371}\
	\u{37e}\u{386}\u{38c}\u{391}\u{399}\u{39d}\u{3a1}\u{3a5}\u{3a9}\u{3ab}\u{3af}\
	\u{3b5}\u{3bd}\u{3c7}\u{3d0}\u{3da}\u{3e2}\u{3f0}\u{3f7}\u{3fc}\u{402}\u{40b}\
	\u{414}\u{416}\u{41f}\u{429}\u{42e}\u{439}\u{442}\u{448}\u{44e}\u{455}\u{45e}\
	\u{475}\u{478}\u{47b}\u{483}\u{487}\u{48f}\u{495}\u{4a0}\u{4a9}\u{4ae}\u{4bb}\
	\u{4c1}\u{4c8}\u{4d5}\u{4de}\u{4e7}\u{4ed}\u{4f5}\u{4fb}\u{500}\u{505}\u{50d}\
	\u{512}\u{516}\u{51a}\u{51e}\u{520}\u{524}\u{529}\u{52e}\u{53d}\u{543}\u{54a}\
	\u{550}\u{553}\u{55e}\u{566}\u{575}\u{579}\u{57e}\u{582}\u{592}\u{5ba}\u{5c0}\
	\u{5cd}\u{5d2}\u{5d5}\u{5d7}\u{5dd}\u{5e4}\u{5f0}\u{5f9}\u{600}\u{603}\u{607}\
	\u{619}\u{61b}\u{623}\u{62c}\u{633}\u{63d}\u{644}\u{64c}\u{64f}\u{656}\u{65d}\
	\u{660}\u{665}\u{66a}\u{66f}\u{678}\u{67a}\u{67e}\u{683}\u{687}\u{68b}\u{68f}\
	\u{699}\u{6a5}\u{6ac}\u{6af}\u{6b3}\u{6bb}\u{6c0}\u{6cb}\u{6d1}\u{6d6}\u{6db}\
	\u{6e2}\u{6ed}\u{6f5}\u{6f9}\u{6fb}\u{702}\u{706}";

