# Strict Mark 

> (Translated using Qwen)

The core idea is the separation of structured text and style. It provides the ability to export to Word using a specified style and the results of all calculations, while ensuring maximum file portability. It also includes an extension system that allows executing blocks of different code, merging them into a single space when necessary. The result of such computations is always either text or files that this text must reference.
The style system can not only specify formatting parameters for blocks, but also modify them using Lua, and automatically configure references to elements, tracking numbering.

# Program Operation Principle
When a project is created, its file and a hidden folder are created; the folder opens as a project in the selected editor (VS Code is recommended for beginners, or any other code editor for advanced users; some of them will be downloadable directly from the program).
The program will continuously recreate DOCX and PDF. The PDF can be displayed for preview.

The original AST will be embedded in the DOCX file for further decompilation.

The program will also support a portable file format (there will be no save button—saving is always automatic). It will embed all project files, including cache, DOCX, and PDF, so that if sent to a person without my program, it can be viewed (optionally, all dependencies can also be included separately in case the other PC may not have internet).
Also, all SM files in it will be formatted, and text in all paragraphs will be wrapped so that line length does not exceed 200 characters; when needed, it will be easier to read and modify SM files outside special editors. And all new line breaks will be marked with a special character.

The file structure looks like this:
- Folder where the user placed the document
    - project_name.sm.zip
    - .project_name
        - project_name.docx
        - project_name.pdf
        - project
            - .git
            - .gitignore
            - extensions.yaml
            - main.sm
            other files that the user will embed (for example, splitting the work into several SM files)
            - style
                - style.yaml
                - style.lock
                - style_lock.zip
                    other system files that the style depends on (for example, local fonts), if any
            - builds
                - cache
                    - style
                        compiled versions of Lua scripts
                    - sm_cache.cbor (cache of all results of the extension system)

Every user's project will have Git, which will create commits both on request and every half hour, as well as on any save. When creating a project, a repository can be specified, and all commits will be pushed there. Moreover, for popular Git hosting services, systems for automatic access configuration and repository creation will be implemented, which can be extended in settings to other platforms. `builds` is listed in `.gitignore`.

# Style
The style directly converts my AST into the AST from the `rdocx` crate via the `rdocx_decl` layer.

## Contents
A style may have external dependencies; they must be specified either by link or by path to a file in the list in the `inputs` field.

A style is a YAML file in which, for each element type, formatting parameters are described or a Lua transformation script is provided for more complex parts.

The style also contains Lua scripts for preliminary AST editing (this system will be used to implement automatic formula numbering) and validation.

A style can depend on another style either via a link or by taking it from a system file. In this case, element descriptions recursively override each other, and editing and validation scripts, as well as external dependencies, are all applied together (the more basic ones come first).
If a style is specified by link, the link must point to a ZIP archive containing the style and its lock data, or to a Git repository (or a folder in it) containing the style and lock data. If the style is on the user's PC, it must be either the same kind of ZIP archive or have a folder with lock data nearby.

Lock data means hashes of linked files and the files themselves if they come from the user's system.

# Structured Text
Text structure is described using modified Markdown without HTML support.

# Structural Elements
## Names
To refer to any element, it must be named. This is necessary because numbering in such cases may shift.
To name an element, above it, on a separate line, write `[element name]`, and to reference it, specify `@(element name)`. If an element has named subelements, they are specified with `:`, for example `@(element name 1):(element name 1.1)`.
They have a boolean parameter `namespace`, which turns it into a namespace for child objects; by default it is false, but for lists and tables it is true.

## Parameters
Some structural elements have parameters; they are specified using `{}` and written in the format `name = value`, but boolean values may omit an explicit value if they are optional and default to false, for example: `{bool_param, int_param = 6}`. Parameters are written on a separate line above the block they apply to, but below the name if one is present.

## Comments
Everything after `%` is a comment and has no significance.

## Headings
Headings are marked with `#`; the greater their number, the lower the heading level.

## Paragraphs
A block of text; for better readability of the plain file in editors without automatic wrapping, line breaks without blank lines are replaced with spaces.
Paragraphs can be of different types, and to change them there is a `type` parameter, which can take the following values:
- `text` - ordinary text paragraph, default value;
- `footnote` - formatted as a footnote (most styles should insert it at the end of the page on which it is mentioned).

## Quote
A quote can contain any text enclosed by `<<<` and `>>>`.

## Enumerations
Enumerations can be created using `-`. To control appearance, there is a `{mark_type}` parameter with several values:
- `number` - numbers are used as markers; this is the default value;
- `mark` - a constant marker chosen by the style is used as markers;
- `char` - letters are used as markers;
- `bibliography` - the entire list and references to it are formatted as a bibliography list;
- `definitions` - the entire list and references to it are formatted as a definitions list.
To refer to a specific item, you can use its number or name if specified (the latter is preferred). The text inside is aligned with spaces to the beginning of the line and written as ordinary text.
If the text is a footnote, the `type` parameter can be set to `footnote`.

In lists, each item is full SM text with all capabilities. When writing more than one line, each subsequent line can use either a tab or two spaces. Since some text editors remove whitespace characters on empty lines, it is recommended to wrap text in `{}` for multiple paragraphs; in this mode, indentation is only a recommendation.

## Formatting
Text can be made bold `**bold text**`, italic `*italic text*`, underlined `__underlined text__`, strikethrough `~~strikethrough text~~`, and code (monospaced) using backticks.

## Formulas
Single `$` can be used to write inline LaTeX formulas, and double `$` for formulas that will be displayed outside the text and numbered by many styles.

## Insertion
Document insertion is divided into 4 types: page insertion, resource insertion, SM or SMZ file insertion, and standard element insertion.

If the file is not located within the project folders, it is copied to the cache.

### Page Insertion
Inserts images and PDFs as pages using the `#(path)` syntax.

### Resource Insertion
Inserts images and adds captions to them. If multiple such images are specified consecutively without empty lines between them, they will be merged into a single group.
```
#(path1.png)"
text1
"
#(path2.png)"
text2
"
```

### SM or SMZ File Insertion
Inserts the specified file by embedding its data list directly at the invocation point `#(path.sm)`.

### Standard Element Insertion
Inserts an element with the specified name `#[name]`. The implementation and the list of available names are determined by the active style, but a subset of names is predefined for standardization purposes.

Standard names:

- `#[contents]` — inserts the table of contents
- `#[lists_count]` — inserts the total number of sheets
- `#[pictures_count]` — inserts the total number of pictures
- `#[formulas_count]` — inserts the total number of formulas
- `#[tables_count]` — inserts the total number of tables

## Tables
Tables can be created in a Markdown-like way, but it is extended to allow merging cells and specifying how text should be positioned in cells.

Unfortunately, tables in documents need a very large number of functions, but usually not all of them are used at once, so this should not look too complicated.

Between each row there is a separator clearly indicating how many cells are in the next row and how data is positioned in them. All such separators must exactly match in character count, and column separators must stand in the same column to indicate column boundaries. This is the only place with such a requirement; in all other cases it is not necessary, but the formatter will fix it.
Columns are separated by characters indicating how text in the cell below and to the right of the separator is positioned. Three variants are available:

- `|` - centered
- `^` - top
- `v` - bottom

Colons may appear near separators; they indicate which edge to align text to in the cell below them; if absent, text is centered (placing colons on both edges is forbidden).
The remaining space between separators must be filled with `-`. If the row above is a header, then after the row there must be a `<<` sign.

If cells need to be merged vertically, use only `|` as the left separator of the column and, making an indent with spaces inside the cell, write the text.
In this case, the number of characters in such a part of the separator does not matter, but the rest must be placed as if everything in this part were correct. This approach is needed so that text in an already created structure can be easily edited; using it for a lazy description of the structure will often be inconvenient.

Rows are separated by `|`, at that the number of columns must match the markup of the given row, but the positions of the separators themselves may not match - the formatter will fix this. Text inside can be multiline; the same systems apply to it as to the whole file.

Also, any column can be given a name and a type so that it can be accessed from extensions. To do this, add to the row separator that formats the top cell a new row in which the left separator of the initial cell will be `|`, and `[name]type` will be specified through `-`; the remaining space until the next separator should be filled with `-`; the number of columns must match the row separator above, but the column separators need not match - the formatter will fix this. At the same time, all separators of this column must consist only of `|`, because cell formatting is inherited from the first. And the cells themselves no longer require per-row separators, because their data occupies 1 row (empty rows are ignored). But important: although it is recommended not to use this system for all columns within the same rows, it is possible, but requires strict adherence to the number of rows. Also, unnamed data can be created; in this case, the first constant `-` character in the separator above the first cell must be replaced with `'d'`.

The table has a `size` parameter, which specifies a list of column sizes (the sum of its elements must equal 1, and their number must match the number of columns).

After the table, in quotes, comes the table description; text inside has mandatory indentation if it is multiline.

In the example below, the first letter indicates how text will be aligned vertically, and the second horizontally. The cells referenced by the caption contain their numbers.
```
[table name]
{size = [50%, 20%, 30%]}
^:-------------|------v-----:|
| tl           | cc   | br 1 |
|---------------------| br 1 |
| cc 2                | br 1 |
|-------------:^-------------|
| cr           | tc          |
|-d-----------:|-------------|<<
|-[name]type---|-------------|
| cr 3         | cc 5        |
|              |:------------|
| cr 4         | cl 6        |
v-------------:|             |
| br           |             |
"
    Table with the fullest example of syntax (in reality such tables will not exist, because it is an absurd pile-up of all features).

    First the table name is specified, then the size ratio via parameter.
    Cell 1 is merged vertically because the row separator below it does not consist of `-`; its text: "br 1\nbr 1\nbr 1".
    Cell 2 is merged horizontally because the row separator above it does not contain one column separator.
    Cells 3 and 4 are 2 cells of a named set. There must be 2, because beside them are 2 cells 5 and 6, despite cell 6 being merged vertically with the cell below.
"

[table name 2]
|---|---------------|
| Names | measurement stages |
|   |---|---|---|---|
|   | 1 | 2 | 3 | 4 |
|-d:|-d-|-d-|-d-|-d-|<<
| Anedew | 30 | 40 | 20 | 50 |
| Bob | 30 | 40 | 20 | 50 |
| Alex | 30 | 40 | 20 | 50 |
| Jon | 30 | 40 | 20 | 50 |"
    How a person will write the table before formatter processing.
"

[table name 2 (result)]
|--------|---------------------------------------|
| Names  | measurement stages                    |
|        |---------|---------|---------|---------|
|        | 1       | 2       | 3       | 4       |
|-d-----:|-d-------|-d-------|-d-------|-d-------|<<
| Anedew | 30      | 40      | 20      | 50      |
| Bob    | 30      | 40      | 20      | 50      |
| Alex   | 30      | 40      | 20      | 50      |
| Jon    | 30      | 40      | 20      | 50      |"
    What the formatter will output.
"
```

## Code Blocks
Code blocks are the main way to extend the editor's capabilities.
In a special file, a Lua script can be written that will either perform the work itself or call third-party software for processing (up to compiling code in other languages). But ultimately, the result will always be text in this editor's format and a set of resources (for example, images) that this text must insert.

Code blocks are written as in Markdown, but instead of a language, the name of the extension in which execution of this code is configured is specified.

For example, diagrams, automatic math calculation, Typst, and LaTeX will be implemented this way.

Also, these extensions have access to all variables of the file.
If an extension relies on variables that have not yet been generated and cannot derive them from its own code, they must be specified explicitly. In this case, execution of the extension will be deferred until the new variables appear in the context.

It is often necessary to combine extensions into chains so that all information between them is passed natively. For this, the required extensions must specify a chain name using `@`.

```
    ```@chain_name exception_name (@(variable name 1), @(variable name 2))
        code
    ```
```

After execution, the extension must report which variables it used; this is necessary for optimizing recalculations.

Extensions may rely on external dependencies, and in this case they must either install and configure them themselves or give the user instructions. And before starting work, they must collect all information for reproducibility and record it in a lock file.

Some extensions may partially compile their code to speed up operation.

All extension blocks can be assembled into a C-like library. If an extension does not provide such functionality, its call pipeline is assembled in Zig, and its dependencies are specified in a special YAML dependency file for this library, with an environment reproduction pipeline and a lock file, if any.

# Calling `sm` and `smz` Files as Programs
`sm` and `smz` can be assembled into a program and executed. To do this, it is necessary to specify which variables will change and which variable needs to be computed. Then all necessary extension blocks will be assembled into C-like libraries, and afterwards Zig code will be generated and assembled that unites them into a single program, and also checks the environment and reproduces what is necessary if possible (if not, it outputs an error with a description of how to set up the environment).

```sh
sm main.sm build fn_name(@(variable name 1), @(variable name 2)) -> @(variable name 3)
```

```sh
sm main.sm run fn_name(@(variable name 1) = 3, @(variable name 2) = 4) -> @(variable name 3)
```

# Build Algorithm
0. Style assembly (loading all dependencies, assembling Lua scripts)
1. Parsing
2. Executing extensions
    1. Assembly of all extensions that need it (with caching)
    2. Computing the required input variables for each block
    3. All blocks begin assembling as much as possible in parallel and asynchronously; after assembly, a block is inserted into the text, and it is computed which variables it added; if, with their addition, some blocks become computable, they are computed. The result of executing all blocks is cached (with recording of all its dependencies).
3. Document validation.
4. Editing the document by style.
5. Calculating formatting of each block with caching (conversion to DOCX).
6. Converting DOCX to PDF.

# Ecosystem and Development Plan
1. Compiler to DOCX.
2. Creating a set of styles.
3. Creating a set of extensions.
4. Graphical program for automatic installation and configuration of VS Code and other editors.
5. Embedding Git support in the previous program.
6. Support for full program building from `sm` and `smz`.
7. Decompilation of DOCX (both those created by my program and ordinary ones) and embedding a change-loading system (if the final DOCX was modified by a reviewer, it can be loaded, and the system will show the changes; if it cannot reproduce them or they are stylistic, they will be noted in comments).
8. Creating a full standalone editor for both SM and styles (it will not try to look like the final file, because within the system implementing such a thing is absurdly complex, and it also contradicts the idea that a person should not have to think about formatting).