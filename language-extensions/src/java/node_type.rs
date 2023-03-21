use std::str::FromStr;

use serde::{Serialize, Deserialize};

pub struct JavaNodeTypeConvertError {}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum JavaNodeType {
    File,
    PackageDeclaration,
    ImportDeclaration,
    TypeDeclaration,
    Modifier,
    // ClassOrInterfaceModifier,
    VariableModifier,
    ClassDeclaration,
    TypeParameters,
    TypeParameter,
    TypeBound,
    EnumDeclaration,
    EnumConstants,
    EnumConstant,
    EnumBodyDeclarations,
    InterfaceDeclaration,
    ClassBody,
    InterfaceBody,
    ClassBodyDeclaration,
    MemberDeclaration,
    MethodDeclaration,
    MethodBody,
    TypeTypeOrVoid,
    GenericMethodDeclaration,
    GenericConstructorDeclaration,
    ConstructorDeclaration,
    CompactConstructorDeclaration,
    FieldDeclaration,
    InterfaceBodyDeclaration,
    InterfaceMemberDeclaration,
    ConstDeclaration,
    ConstantDeclarator,
    InterfaceMethodDeclaration,
    InterfaceMethodModifier,
    GenericInterfaceMethodDeclaration,
    InterfaceCommonBodyDeclaration,
    VariableDeclarators,
    VariableDeclarator,
    VariableDeclaratorId,
    VariableInitializer,
    ArrayInitializer,
    ClassOrInterfaceType,
    TypeArgument,
    QualifiedNameList,
    FormalParameters,
    ReceiverParameter,
    FormalParameterList,
    FormalParameter,
    LastFormalParameter,
    LambdaLVTIList,
    LambdaLVTIParameter,
    QualifiedName,
    Literal,
    IntegerLiteral,
    FloatLiteral,
    AltAnnotationQualifiedName,
    Annotation,
    ElementValuePairs,
    ElementValuePair,
    ElementValue,
    ElementValueArrayInitializer,
    AnnotationTypeDeclaration,
    AnnotationTypeBody,
    AnnotationTypeElementDeclaration,
    AnnotationTypeElementRest,
    AnnotationMethodOrConstantRest,
    AnnotationMethodRest,
    AnnotationConstantRest,
    DefaultValue,
    ModuleDeclaration,
    ModuleBody,
    ModuleDirective,
    RequiresModifier,
    RecordDeclaration,
    RecordHeader,
    RecordComponentList,
    RecordComponent,
    RecordBody,
    Block,
    BlockStatement,
    LocalVariableDeclaration,
    Identifier,
    TypeIdentifier,
    LocalTypeDeclaration,
    Statement,
    CatchClause,
    CatchType,
    FinallyBlock,
    ResourceSpecification,
    Resources,
    Resource,
    SwitchBlockStatementGroup,
    SwitchLabel,
    ForControl,
    ForInit,
    EnhancedForControl,
    ParExpression,
    ExpressionList,
    MethodCall,
    Expression,
    Pattern,
    LambdaExpression,
    LambdaParameters,
    LambdaBody,
    Primary,
    SwitchExpression,
    SwitchLabeledRule,
    GuardedPattern,
    SwitchRuleOutcome,
    ClassType,
    Creator,
    CreatedName,
    InnerCreator,
    ArrayCreatorRest,
    ClassCreatorRest,
    ExplicitGenericInvocation,
    TypeArgumentsOrDiamond,
    NonWildcardTypeArgumentsOrDiamond,
    NonWildcardTypeArguments,
    TypeList,
    TypeType,
    PrimitiveType,
    TypeArguments,
    SuperSuffix,
    ExplicitGenericInvocationSuffix,
    Arguments,

    Operator,
    Keyword,
}

impl ToString for JavaNodeType {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl FromStr for JavaNodeType {
    type Err = JavaNodeTypeConvertError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "file" => Ok(JavaNodeType::File),
            "packagedeclaration" => Ok(JavaNodeType::PackageDeclaration),
            "importdeclaration" => Ok(JavaNodeType::ImportDeclaration),
            "typedeclaration" => Ok(JavaNodeType::TypeDeclaration),
            "modifier" => Ok(JavaNodeType::Modifier),
            // "classorinterfacemodifier" => Ok(JavaNodeType::ClassOrInterfaceModifier),
            "variablemodifier" => Ok(JavaNodeType::VariableModifier),
            "classdeclaration" => Ok(JavaNodeType::ClassDeclaration),
            "typeparameters" => Ok(JavaNodeType::TypeParameters),
            "typeparameter" => Ok(JavaNodeType::TypeParameter),
            "typebound" => Ok(JavaNodeType::TypeBound),
            "enumdeclaration" => Ok(JavaNodeType::EnumDeclaration),
            "enumconstants" => Ok(JavaNodeType::EnumConstants),
            "enumconstant" => Ok(JavaNodeType::EnumConstant),
            "enumbodydeclarations" => Ok(JavaNodeType::EnumBodyDeclarations),
            "interfacedeclaration" => Ok(JavaNodeType::InterfaceDeclaration),
            "classbody" => Ok(JavaNodeType::ClassBody),
            "interfacebody" => Ok(JavaNodeType::InterfaceBody),
            "classbodydeclaration" => Ok(JavaNodeType::ClassBodyDeclaration),
            "memberdeclaration" => Ok(JavaNodeType::MemberDeclaration),
            "methoddeclaration" => Ok(JavaNodeType::MethodDeclaration),
            "methodbody" => Ok(JavaNodeType::MethodBody),
            "typetypeorvoid" => Ok(JavaNodeType::TypeTypeOrVoid),
            "genericmethoddeclaration" => Ok(JavaNodeType::GenericMethodDeclaration),
            "genericconstructordeclaration" => Ok(JavaNodeType::GenericConstructorDeclaration),
            "constructordeclaration" => Ok(JavaNodeType::ConstructorDeclaration),
            "compactconstructordeclaration" => Ok(JavaNodeType::CompactConstructorDeclaration),
            "fielddeclaration" => Ok(JavaNodeType::FieldDeclaration),
            "interfacebodydeclaration" => Ok(JavaNodeType::InterfaceBodyDeclaration),
            "interfacememberdeclaration" => Ok(JavaNodeType::InterfaceMemberDeclaration),
            "constdeclaration" => Ok(JavaNodeType::ConstDeclaration),
            "constantdeclarator" => Ok(JavaNodeType::ConstantDeclarator),
            "interfacemethoddeclaration" => Ok(JavaNodeType::InterfaceMethodDeclaration),
            "interfacemethodmodifier" => Ok(JavaNodeType::InterfaceMethodModifier),
            "genericinterfacemethoddeclaration" => Ok(JavaNodeType::GenericInterfaceMethodDeclaration),
            "interfacecommonbodydeclaration" => Ok(JavaNodeType::InterfaceCommonBodyDeclaration),
            "variabledeclarators" => Ok(JavaNodeType::VariableDeclarators),
            "variabledeclarator" => Ok(JavaNodeType::VariableDeclarator),
            "variabledeclaratorid" => Ok(JavaNodeType::VariableDeclaratorId),
            "variableinitializer" => Ok(JavaNodeType::VariableInitializer),
            "arrayinitializer" => Ok(JavaNodeType::ArrayInitializer),
            "classorinterfacetype" => Ok(JavaNodeType::ClassOrInterfaceType),
            "typeargument" => Ok(JavaNodeType::TypeArgument),
            "qualifiednamelist" => Ok(JavaNodeType::QualifiedNameList),
            "formalparameters" => Ok(JavaNodeType::FormalParameters),
            "receiverparameter" => Ok(JavaNodeType::ReceiverParameter),
            "formalparameterlist" => Ok(JavaNodeType::FormalParameterList),
            "formalparameter" => Ok(JavaNodeType::FormalParameter),
            "lastformalparameter" => Ok(JavaNodeType::LastFormalParameter),
            "lambdalvtilist" => Ok(JavaNodeType::LambdaLVTIList),
            "lambdalvtiparameter" => Ok(JavaNodeType::LambdaLVTIParameter),
            "qualifiedname" => Ok(JavaNodeType::QualifiedName),
            "literal" => Ok(JavaNodeType::Literal),
            "integerliteral" => Ok(JavaNodeType::IntegerLiteral),
            "floatliteral" => Ok(JavaNodeType::FloatLiteral),
            "altannotationqualifiedname" => Ok(JavaNodeType::AltAnnotationQualifiedName),
            "annotation" => Ok(JavaNodeType::Annotation),
            "elementvaluepairs" => Ok(JavaNodeType::ElementValuePairs),
            "elementvaluepair" => Ok(JavaNodeType::ElementValuePair),
            "elementvalue" => Ok(JavaNodeType::ElementValue),
            "elementvaluearrayinitializer" => Ok(JavaNodeType::ElementValueArrayInitializer),
            "annotationtypedeclaration" => Ok(JavaNodeType::AnnotationTypeDeclaration),
            "annotationtypebody" => Ok(JavaNodeType::AnnotationTypeBody),
            "annotationtypeelementdeclaration" => Ok(JavaNodeType::AnnotationTypeElementDeclaration),
            "annotationtypeelementrest" => Ok(JavaNodeType::AnnotationTypeElementRest),
            "annotationmethodorconstantrest" => Ok(JavaNodeType::AnnotationMethodOrConstantRest),
            "annotationmethodrest" => Ok(JavaNodeType::AnnotationMethodRest),
            "annotationconstantrest" => Ok(JavaNodeType::AnnotationConstantRest),
            "defaultvalue" => Ok(JavaNodeType::DefaultValue),
            "moduledeclaration" => Ok(JavaNodeType::ModuleDeclaration),
            "modulebody" => Ok(JavaNodeType::ModuleBody),
            "moduledirective" => Ok(JavaNodeType::ModuleDirective),
            "requiresmodifier" => Ok(JavaNodeType::RequiresModifier),
            "recorddeclaration" => Ok(JavaNodeType::RecordDeclaration),
            "recordheader" => Ok(JavaNodeType::RecordHeader),
            "recordcomponentlist" => Ok(JavaNodeType::RecordComponentList),
            "recordcomponent" => Ok(JavaNodeType::RecordComponent),
            "recordbody" => Ok(JavaNodeType::RecordBody),
            "block" => Ok(JavaNodeType::Block),
            "blockstatement" => Ok(JavaNodeType::BlockStatement),
            "localvariabledeclaration" => Ok(JavaNodeType::LocalVariableDeclaration),
            "identifier" => Ok(JavaNodeType::Identifier),
            "typeidentifier" => Ok(JavaNodeType::TypeIdentifier),
            "localtypedeclaration" => Ok(JavaNodeType::LocalTypeDeclaration),
            "statement" => Ok(JavaNodeType::Statement),
            "catchclause" => Ok(JavaNodeType::CatchClause),
            "catchtype" => Ok(JavaNodeType::CatchType),
            "finallyblock" => Ok(JavaNodeType::FinallyBlock),
            "resourcespecification" => Ok(JavaNodeType::ResourceSpecification),
            "resources" => Ok(JavaNodeType::Resources),
            "resource" => Ok(JavaNodeType::Resource),
            "switchblockstatementgroup" => Ok(JavaNodeType::SwitchBlockStatementGroup),
            "switchlabel" => Ok(JavaNodeType::SwitchLabel),
            "forcontrol" => Ok(JavaNodeType::ForControl),
            "forinit" => Ok(JavaNodeType::ForInit),
            "enhancedforcontrol" => Ok(JavaNodeType::EnhancedForControl),
            "parexpression" => Ok(JavaNodeType::ParExpression),
            "expressionlist" => Ok(JavaNodeType::ExpressionList),
            "methodcall" => Ok(JavaNodeType::MethodCall),
            "expression" => Ok(JavaNodeType::Expression),
            "pattern" => Ok(JavaNodeType::Pattern),
            "lambdaexpression" => Ok(JavaNodeType::LambdaExpression),
            "lambdaparameters" => Ok(JavaNodeType::LambdaParameters),
            "lambdabody" => Ok(JavaNodeType::LambdaBody),
            "primary" => Ok(JavaNodeType::Primary),
            "switchexpression" => Ok(JavaNodeType::SwitchExpression),
            "switchlabeledrule" => Ok(JavaNodeType::SwitchLabeledRule),
            "guardedpattern" => Ok(JavaNodeType::GuardedPattern),
            "switchruleoutcome" => Ok(JavaNodeType::SwitchRuleOutcome),
            "classtype" => Ok(JavaNodeType::ClassType),
            "creator" => Ok(JavaNodeType::Creator),
            "createdname" => Ok(JavaNodeType::CreatedName),
            "innercreator" => Ok(JavaNodeType::InnerCreator),
            "arraycreatorrest" => Ok(JavaNodeType::ArrayCreatorRest),
            "classcreatorrest" => Ok(JavaNodeType::ClassCreatorRest),
            "explicitgenericinvocation" => Ok(JavaNodeType::ExplicitGenericInvocation),
            "typeargumentsordiamond" => Ok(JavaNodeType::TypeArgumentsOrDiamond),
            "nonwildcardtypeargumentsordiamond" => Ok(JavaNodeType::NonWildcardTypeArgumentsOrDiamond),
            "nonwildcardtypearguments" => Ok(JavaNodeType::NonWildcardTypeArguments),
            "typelist" => Ok(JavaNodeType::TypeList),
            "typetype" => Ok(JavaNodeType::TypeType),
            "primitivetype" => Ok(JavaNodeType::PrimitiveType),
            "typearguments" => Ok(JavaNodeType::TypeArguments),
            "supersuffix" => Ok(JavaNodeType::SuperSuffix),
            "explicitgenericinvocationsuffix" => Ok(JavaNodeType::ExplicitGenericInvocationSuffix),
            "arguments" => Ok(JavaNodeType::Arguments),
            "operator" => Ok(JavaNodeType::Operator),
            "keyword" => Ok(JavaNodeType::Keyword),
            _ => Err(JavaNodeTypeConvertError{})
        }
    }
}