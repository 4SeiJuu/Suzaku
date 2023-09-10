# Suzaku
Source code optimization tool with Rust.

## Features and Roadmap
### Functions
 - [ ] source code scan (doing)
 - [ ] architecture analysis (doing)
 - [ ] refactor
 - [ ] unit testing generation 
 - [ ] decouple
 - [ ] injection
 - [ ] multiple and mixing languages support

### Languages Supports
 - [ ] Java (doing)
 - [ ] C
 - [ ] C++
 - [ ] Rust

### Usability
 - [ ] parallel processing
 - [ ] incremental processing

## Usages
### all usages
```shell
$ suzaku-cli -h
```
```shell
Usage: suzaku-cli [COMMAND]

Commands:
  parse     parse source code
  extract   data extraction
  map       data mapping, will generates a file contains combined elements
  analysis  analysing, will generates a file contains analysed data
  report    generates report
  pipeline  pipeline
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
### for parsing
```shell
$ suzaku-cli parse -h
```
the usages
```shell
parse source code

Usage: suzaku-cli parse [OPTIONS] --output <OUTPUT>

Options:
  -s, --src <SOURCES>       Sets source code folder path which will be parsed
  -o, --output <OUTPUT>     Sets output folder path which will store the output files
  -x, --exclude <EXCLUDES>  Sets paths which need be excluded folder or file
  -h, --help                Print help
```

### for data extracting
```shell
$ suzaku-cli extract -h
```
the usages
```shell
data extraction

Usage: suzaku-cli extract [OPTIONS] --output <OUTPUT>

Options:
  -m, --metadata <METADATAS>  Sets metadata file folder path which is generated by 'Parse' command
  -o, --output <OUTPUT>       Sets output folder path which will store the output files
  -x, --exclude <EXCLUDES>    Sets paths which need be excluded folder or file
  -h, --help                  Print help
```

### for data mapping
```shell
$ suzaku-cli map -h
```
the usages
```shell
data mapping, will generates a file contains combined elements

Usage: suzaku-cli map [OPTIONS] --output <OUTPUT>

Options:
  -e, --element <ELEMENTS>  Sets metadata file folder path which is generated by 'Extraction' command
  -o, --output <OUTPUT>     Sets output folder path which will store the output files
  -x, --exclude <EXCLUDES>  Sets paths which need be excluded folder or file
  -h, --help                Print help
```

### for analysing
```shell
$ suzaku-cli analysis -h
```
the usages
```shell
analysing, will generates a file contains analysed data

Usage: suzaku-cli analysis --data <DATA> --output <OUTPUT>

Options:
  -d, --data <DATA>      Sets data file path which is generated by 'Map' command
  -o, --output <OUTPUT>  Sets output folder path which will store the output files
  -h, --help             Print help
```

### for reporting
```shell
$ suzaku-cli report -h
```
the usages
```shell
generates report

Usage: suzaku-cli report --data <DATA> --output <OUTPUT>

Options:
  -d, --data <DATA>      Sets data file path which is generated by 'Analysis' command
  -o, --output <OUTPUT>  Sets output folder path which will store the output files
  -h, --help             Print help
```

## Dependencies
### ![logo](doc/imgs/anltr.jpeg) [ANTLR](https://www.antlr.org/) ([Github](https://github.com/antlr))

#### Requirements
- Java SE
    - [Java SE 11 Archive Downloads](https://www.oracle.com/jp/java/technologies/javase/jdk11-archive-downloads.html)
    - [Java SE 17 / 19 Downloads](https://www.oracle.com/java/technologies/downloads/)
- Maven
    - [Downloading Apache Maven](https://maven.apache.org/download.cgi)

#### Install from jar
check the release versons, choose one and download it.
for example, could use following command to download the version of 4.13.0
```shell
$ curl -O https://www.antlr.org/download/antlr-4.13.0-complete.jar
```
*NOTE: the official release version still cannot generate Rust code*

### ANTLR 4 Rust
Git Repository: [rrevenantt / antlr4rust](https://github.com/rrevenantt/antlr4rust)

#### Download Jar of latest version
- [Releases](https://github.com/rrevenantt/antlr4rust/releases)

#### Install from source code
```shell
$ git clone -b rust-target git@github.com:rrevenantt/antlr4.git
$ cd antlr4
$ git submodule update --init --recursive --remote
```

then, run following command to build and install tool
```shell
$ mvn -DskipTests install
```
when succeed, the jars will be installed to
```shell
$HOME/.m2/repository/org/antlr/antlr4/4.8-2-SNAPSHOT/antlr4-4.8-2-SNAPSHOT-complete.jar
```

##### Known Issues for Building
- JDK version: Should be 11

- if you build it on macOS, there are some linked files may not be found. you need to run following command to replace the symbolic links with real files
```shell
$ rm tool/resources/org/antlr/v4/tool/templates/codegen/Rust/Rust.stg
$ ln -s runtime/Rust/templates/Rust.stg tool/resources/org/antlr/v4/tool/templates/codegen/Rust/Rust.stg
$ rm runtime-testsuite/resources/org/antlr/v4/test/runtime/templates/Rust.test.stg
$ ln -s runtime/Rust/templates/Rust.test.stg runtime-testsuite/resources/org/antlr/v4/test/runtime/templates/Rust.test.stg
$ rm runtime-testsuite/test/org/antlr/v4/test/runtime/rust/BaseRustTest.java
$ ln -s runtime/Rust/templates/BaseRustTest.java runtime-testsuite/test/org/antlr/v4/test/runtime/rust/BaseRustTest.java
```

- broken symbolic link to   
  - runtime-testsuite/test/org/antlr/v4/test/runtime/rust/BaseRustTest.java  
    broken symbolic link to runtime/Rust/templates/BaseRustTest.java
    ```shell
    $ rm runtime-testsuite/test/org/antlr/v4/test/runtime/rust/BaseRustTest.java
    $ cp runtime/Rust/templates/BaseRustTest.java runtime-testsuite/test/org/antlr/v4/test/runtime/rust/BaseRustTest.java
    ```

- Failed to parse plugin descriptor for org.antlr:antlr4-maven-plugin  
  see [Failed to parse plugin descriptor](https://github.com/antlr/antlr4/issues/1940)

#### Generates Lexer and Parser
##### Lexer
```shell
$ java -jar <antlr4rust.jar> -Dlanguage=Rust ./language-extensions/src/java/generated/JavaLexer.g4
```

##### Parser
```shell
$ java -jar <antlr4rust.jar> -Dlanguage=Rust ./language-extensions/src/java/generated/JavaParser.g4
```
