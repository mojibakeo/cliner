# Cliner

A command line tool for managing Cline rules and modes.

## Overview

Cliner is a utility for generating configuration files for Cline from markdown files. It manages two main components:

- **Modes**: Custom modes for Roo Code
- **Rules**: Context and instructions for Claude

## Installation

### From Source

```bash
cargo install --path .
```

## Usage

Cliner has two main commands:

### Initialize

Creates a new `.cline` directory structure in the current directory.

```bash
cliner init
```

This command:

- Creates a `.cline` directory with `modes` and `rules` subdirectories
- Attempts to copy mode and rule files from global config directories if they exist

### Generate

Generates configuration files from the contents of `.cline` directory.

```bash
cliner generate
```

This command:

- Creates `.roomodes` file from markdown files in `.cline/modes/`
- Creates `.clinerules` file from markdown files in `.cline/rules/`
- Files are processed in alphabetical order by filename

## Directory Structure

```
project/
├── .cline/
│   ├── modes/
│   │   ├── 00_mode1.md
│   │   └── 01_mode2.md
│   └── rules/
│       ├── 00_rule1.md
│       └── 01_rule2.md
├── .roomodes       # Generated JSON file for custom modes
└── .clinerules     # Generated concatenated text file for rules
```

## Mode File Format

Mode files in `.cline/modes/` should be markdown files with YAML frontmatter followed by markdown content:

```markdown
slug: designer
mode_name: Designer
groups:

- read
- edit
- browser
- command
  custom_instructions: Optional additional instructions for this mode.

---

# Designer Mode

You are Roo, a UI/UX expert specializing in design systems and frontend development.
Your expertise includes creating and maintaining design systems, implementing responsive
and accessible web interfaces, and ensuring consistent user experiences across platforms.
```

### Required Fields

- **slug**: A unique identifier using lowercase letters, numbers, and hyphens (shorter is better)
- **mode_name**: The display name for the mode shown in the UI
- **groups**: Array of allowed tool groups (can be empty)

### Optional Fields

- **custom_instructions**: Additional instructions specific to this mode

### Role Definition

The markdown content after the frontmatter separator (`---`) defines the role for the mode. This should be a detailed description of the mode's capabilities and responsibilities.

### Tool Groups

Available tool groups:

- **read**: File reading operations (`read_file`, `search_files`, `list_files`, `list_code_definition_names`)
- **edit**: File editing operations (`apply_diff`, `write_to_file`)
- **browser**: Browser control (`browser_action`)
- **command**: Terminal command execution (`execute_command`)
- **mcp**: Model Context Protocol tools (`use_mcp_tool`, `access_mcp_resource`)

### Restricting File Operations

To restrict which files a mode can edit, use this format:

```markdown
groups:

- read
- ["edit", { "fileRegex": "\\.md$", "description": "Markdown files only" }]
- browser
```

This example allows the mode to edit only markdown files (with `.md` extension).

````

## Rule File Format

Rule files in `.cline/rules/` are plain markdown files that will be concatenated in order:

```markdown
# Rule Title

This is a rule for the Claude assistant.
````

## License

MIT

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
