# Suzaku
Sources scan and analyzing tool with antlr4 and Rust


## ANTLR tool

### Requirements
- Java SE
    - [Java SE 11 Archive Downloads](https://www.oracle.com/jp/java/technologies/javase/jdk11-archive-downloads.html)
    - [Java SE 17 / 19 Downloads](https://www.oracle.com/java/technologies/downloads/)
- Maven
    - [Downloading Apache Maven](https://maven.apache.org/download.cgi)

### Build
```shell
$ git clone -b rust-target git@github.com:rrevenantt/antlr4.git
$ cd antlr4
$ git submodule update --init --recursive --remote
```

** if you build it on macOS, there are some linked files may not be found. you need to run following command to replace the symbolic links with real files **
```shell
$ cp runtime/Rust/templates/Rust.stg tool/resources/org/antlr/v4/tool/templates/codegen/Rust/Rust.stg
$ cp runtime/Rust/templates/Rust.test.stg runtime-testsuite/resources/org/antlr/v4/test/runtime/templates/Rust.test.stg
$ cp runtime/Rust/templates/BaseRustTest.java runtime-testsuite/test/org/antlr/v4/test/runtime/rust/BaseRustTest.java
```

then, run following command to build and install tool
```shell
$ mvn -DskipTests install
```
when succeed, the jars will be installed to
```shell
$HOME/.m2/repository/org/antlr/antlr4
```

## Generates Lexer and Parser
### Lexer
```shell
$ java -jar $HOME/.m2/repository/org/antlr/antlr4/4.8-2-SNAPSHOT/antlr4-4.8-2-SNAPSHOT-complete.jar -Dlanguage=Rust ./language-extensions/src/java/generated/JavaLexer.g4
```

### Parser
```shell
$ java -jar $HOME/.m2/repository/org/antlr/antlr4/4.8-2-SNAPSHOT/antlr4-4.8-2-SNAPSHOT-complete.jar -Dlanguage=Rust ./language-extensions/src/java/generated/JavaParser.g4
```
