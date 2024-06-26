use std::cmp::PartialEq;
use std::fmt::Display;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

lazy_static! {
	static ref INLINE_LATEX_REGEX: Regex = Regex::new(r"(?m)^\$\$.*?\$\$").unwrap();
	static ref INLINE_CODE_REGEX: Regex = Regex::new(r"(?m)^```.*?```").unwrap();
	static ref HEADING_PATTERN: Regex = Regex::new(r"(?m)^(?P<level>#+) (?P<text>.*?)$").unwrap();
	static ref BULLET_POINT_PATTERN: Regex = Regex::new(r"(?m)^(?P<indent>[ \r\t]*)?- (?P<text>.*?)$").unwrap();
	static ref LIST_ITEM_PATTERN: Regex = Regex::new(r"(?m)^(?P<indent>[ \r\t]*)?(?P<number>\d+)\. (?P<text>.*?)$").unwrap();
	static ref LINEBAR_PATTERN: Regex = Regex::new(r"(?m)^---\s*?$").unwrap();
	static ref YAML_PATTERN: Regex = Regex::new(r"(?m)^---([\s\S]*?)---").unwrap();
	static ref BOLD_ITALIC_PATTERN: Regex = Regex::new(r"^\*\*\*(.*?)\*\*\*").unwrap();
	static ref BOLD_PATTERN: Regex = Regex::new(r"^\*\*(.*?)\*\*").unwrap();
	static ref ITALIC_PATTERN: Regex = Regex::new(r"^\*(.*?)\*").unwrap();
	static ref INLINE_CODE_PATTERN: Regex = Regex::new(r"^`(.*?)`").unwrap();
	static ref INLINE_BLOCK_LATEX_PATTERN: Regex = Regex::new(r"^\$\$(.*?)\$\$").unwrap();
	static ref INLINE_LATEX_PATTERN: Regex = Regex::new(r"^\$(.*?)\$").unwrap();
	static ref FORMATTED_MARKDOWN_LINK_PATTERN: Regex = Regex::new(r"^\[\[(.+?)\|(.+?)\]\]").unwrap();
	static ref MARKDOWN_LINK_PATTERN: Regex = Regex::new(r"^\[\[(.+?)\]\]").unwrap();
	static ref FORMATTED_WEB_LINK_PATTERN: Regex = Regex::new(r"^\[(.+?)\]\((.+?)\)").unwrap();
	static ref WEB_LINK_PATTERN: Regex = Regex::new(r"^\[(.+?)\]").unwrap();

}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct AST {
	pub blocks: Vec<Block>,
	pub line_sep: NewlineType
}

#[allow(dead_code)]
impl AST {

	pub fn new(contents: String) -> Self {
		let contents = contents;
		let line_sep: NewlineType = if contents.contains("\r\n") {
			NewlineType::Windows
		} else {
			NewlineType::Unix
		};
		let lines: Vec<String> = contents.split('\n').map(std::string::ToString::to_string).collect();
		let blocks = parse_into_blocks(lines);
		Self {
			blocks,
			line_sep,
		}
	}

	pub fn get_yaml(&self, tag: &str) -> Option<serde_yaml::Value> {
		for block in &self.blocks {
			match block {
				Block::YAML(content, _) => {
					if content[tag].is_null() {
						continue;
					}
					return Some(content[tag].clone());
				},
				_ => continue,
			}
		}
		None
	}

	pub fn get_aliases(&self) -> Vec<String> {
		if self.get_yaml("aliases").is_none() {
			return vec![];
		}
		self.get_yaml("aliases").unwrap().as_sequence().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect()
	}

	pub fn get_tags(&self) -> Vec<String> {
		if self.get_yaml("tags").is_none() {
			return vec![];
		}
		self.get_yaml("tags").unwrap().as_sequence().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect()
	}

	pub fn get_lines(&self) -> Vec<&Line> {
		let mut lines: Vec<&Line> = Vec::new();
		for block in &self.blocks {
			for line in block.get_lines() {
				lines.push(line);
			}
		}
		lines
	}

	pub fn get_lines_mut(&mut self) -> Vec<&mut Line> {
		let mut lines: Vec<&mut Line> = Vec::new();
		for block in &mut self.blocks {
			for line in block.get_lines_mut() {
				lines.push(line);
			}
		}
		lines
	}
}

impl Display for AST {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut blocks_str_vec: Vec<String> = Vec::new();
		for block in &self.blocks {
			blocks_str_vec.push(format!("{block}"));
		}
		let mut file_contents: String = blocks_str_vec.join("\n");
		file_contents = file_contents.replace("\r\n", "\n");
		match self.line_sep {
			NewlineType::Unix => write!(f, "{file_contents}"),
			NewlineType::Windows => write!(f, "{}", file_contents.replace('\n', "\r\n")),
		}

	}

}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum NewlineType {
	#[default]
	Unix,
	Windows,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Block {
	YAML(serde_yaml::Value, Vec<String>),
	CodeBlock(Vec<String>),
	LatexBlock(Vec<String>),
	BlockQuote(Vec<Block>),
	TextBlock(Vec<Line>)
}


impl Block {
	pub fn get_lines(&self) -> Vec<&Line> {
		match self {
			Self::BlockQuote(blocks) => {
				let mut lines: Vec<&Line> = Vec::new();
				for block in blocks {
					for line in block.get_lines() {
						lines.push(line);
					}
				}
				lines
			},
			Self::TextBlock(text_lines) => {
				let mut lines: Vec<&Line> = Vec::new();
				for line in text_lines {
					lines.push(line);
				}
				lines
			},
			_ => Vec::new(),
		}
	}


	pub fn get_lines_mut(&mut self) -> Vec<&mut Line> {

		match self {
			Self::BlockQuote(blocks) => {
				let mut lines_mut: Vec<&mut Line> = Vec::new();
				for block in blocks {
					lines_mut.append(&mut block.get_lines_mut());
				}
				lines_mut
			},
			Self::TextBlock(lines) => {
				let mut lines_mut: Vec<&mut Line> = Vec::new();
				for line in lines {
					lines_mut.push(line);
				}
				lines_mut
			},
			_ => Vec::new(),
		}
	}
}



impl Display for Block {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::YAML(_, lines_vec) => {
				let lines_string: String = lines_vec.join("\n");
				write!(f, "{lines_string}")
			},
			Self::CodeBlock(lines_vec) => {
				let lines_string: String = lines_vec.join("\n");
				write!(f, "{lines_string}")
			},
			Self::LatexBlock(lines_vec) => {
				let lines_string: String = lines_vec.join("\n");
				write!(f, "{lines_string}")
			},
			Self::BlockQuote(blocks_vec) => {
				let blocks_str_vec: Vec<String> = blocks_vec.iter().map(|x| format!("{x}")).collect();
				let blocks_string: String = blocks_str_vec.join("\n");
				let prepended_blocks_string_vec: Vec<String> = blocks_string.lines().map(|x| format!(">{x}")).collect();
				let prepended_blocks_string: String = prepended_blocks_string_vec.join("\n");
				write!(f, "{prepended_blocks_string}")
			},
			Self::TextBlock(lines_vec) => {
				let lines_str_vec: Vec<String> = lines_vec.iter().map(|x| format!("{x}")).collect();
				let lines_string: String = lines_str_vec.join("\n");
				write!(f, "{lines_string}")
			},
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Line {
	Heading(Vec<Node>, u8),
	BulletPoint(Vec<Node>, String), // string is indentation
	ListItem(Vec<Node>, u32, String), // string is indentation
	String(Vec<Node>),
	Linebar,
}

impl Line {
	pub fn iterate_strings(&self) -> Vec<&String> {
		match self {
			Self::Heading(nodes, _) => {
				let mut strings: Vec<&String> = Vec::new();
				for node in nodes {
					strings.append(&mut node.iterate_strings());
				}
				strings
			},
			Self::BulletPoint(nodes, _) => {
				let mut strings: Vec<&String> = Vec::new();
				for node in nodes {
					strings.append(&mut node.iterate_strings());
				}
				strings
			},
			Self::ListItem(nodes, _, _) => {
				let mut strings: Vec<&String> = Vec::new();
				for node in nodes {
					strings.append(&mut node.iterate_strings());
				}
				strings
			},
			Self::String(nodes) => {
				let mut strings: Vec<&String> = Vec::new();
				for node in nodes {
					strings.append(&mut node.iterate_strings());
				}
				strings
			},
			Self::Linebar => Vec::new(),
		}
	}

	#[allow(dead_code)]
	pub fn iterate_nodes(&self) -> Vec<&Node> {
		match self {
			Self::Heading(nodes, _) => {
				let mut nodes_vec: Vec<&Node> = Vec::new();
				for node in nodes {
					nodes_vec.push(node);
				}
				nodes_vec
			},
			Self::BulletPoint(nodes, _) => {
				let mut nodes_vec: Vec<&Node> = Vec::new();
				for node in nodes {
					nodes_vec.push(node);
				}
				nodes_vec
			},
			Self::ListItem(nodes, _, _) => {
				let mut nodes_vec: Vec<&Node> = Vec::new();
				for node in nodes {
					nodes_vec.push(node);
				}
				nodes_vec
			},
			Self::String(nodes) => {
				let mut nodes_vec: Vec<&Node> = Vec::new();
				for node in nodes {
					nodes_vec.push(node);
				}
				nodes_vec
			},
			Self::Linebar => Vec::new(),
		}
	}

	pub fn get_nodes_mut(&mut self) -> Option<&mut Vec<Node>> {
		match self {
			Self::Heading(nodes, _) => {
				Some(nodes)
			},
			Self::BulletPoint(nodes, _) => {
				Some(nodes)
			},
			Self::ListItem(nodes, _, _) => {
				Some(nodes)
			},
			Self::String(nodes) => {
				Some(nodes)
			},
			Self::Linebar => None,
		}
	}

}

impl Display for Line {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Heading(nodes, level) => {
				// write!(f, "\n")?;
				// write the heading level
				for _ in 0..*level {
					write!(f, "#")?;
				}

				write!(f, " ")?;
				for node in nodes {
					write!(f, "{node}")?;
				}
				write!(f, "")
			},
			Self::BulletPoint(nodes, indentation) => {
				// write!(f, "\n")?;
				write!(f, "{indentation}- ")?;
				for node in nodes {
					write!(f, "{node}")?;
				}
				write!(f, "")
			},
			Self::ListItem(nodes, number, indentation) => {
				// write!(f, "\n")?;
				write!(f, "{indentation}{number}. ")?;
				for node in nodes {
					write!(f, "{node}")?;
				}
				write!(f, "")
			},
			Self::String(nodes) => {

				// write!(f, "\n")?;
				for node in nodes {
					write!(f, "{node}")?;
				}
				write!(f, "")
			},
			Self::Linebar => write!(f, "---"),
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Node {

	InlineCode(String),
	InlineBlockLatex(String),
	InlineLatex(String),

	FormattedMarkdownLink(String, String),
	MarkdownLink(String),

	FormattedWebLink(String, String),
	WebLink(String),

	BoldItalic(Vec<Node>),
	Bold(Vec<Node>),
	Italic(Vec<Node>),

	String(String),



}

impl Node {
	pub fn iterate_strings(&self) -> Vec<&String> {
		match self {
			Self::InlineCode(_) => Vec::new(),
			Self::InlineBlockLatex(_) => Vec::new(),
			Self::InlineLatex(_) => Vec::new(),
			Self::FormattedMarkdownLink(_, _) => Vec::new(),
			Self::MarkdownLink(_) => Vec::new(),
			Self::FormattedWebLink(_, _) => Vec::new(),
			Self::WebLink(_) => Vec::new(),
			Self::BoldItalic(nodes) => {
				let mut strings: Vec<&String> = Vec::new();
				for node in nodes {
					let mut other_strings = node.iterate_strings();
					strings.append(&mut other_strings);
				}
				strings
			},
			Self::Bold(nodes) => {
				let mut strings: Vec<&String> = Vec::new();
				for node in nodes {
					let mut other_strings = node.iterate_strings();
					strings.append(&mut other_strings);
				}
				strings
			},
			Self::Italic(nodes) => {
				let mut strings: Vec<&String> = Vec::new();
				for node in nodes {
					let mut other_strings = node.iterate_strings();
					strings.append(&mut other_strings);
				}
				strings
			},

			Self::String(string) => {
				vec![string]
			},
		}
	}
}

impl Display for Node {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InlineCode(content) => write!(f, "`{content}`"),
			Self::InlineBlockLatex(content) => write!(f, "$${content}$$"),
			Self::InlineLatex(content) => write!(f, "${content}$"),
			Self::MarkdownLink(content) => write!(f, "[[{content}]]"),
			Self::FormattedMarkdownLink(link, text) => write!(f, "[[{link}|{text}]]"),
			Self::WebLink(content) => write!(f, "[{content}]"),
			Self::FormattedWebLink(link, text) => write!(f, "[{text}]({link})"),
			Self::BoldItalic(content) => {
				write!(f, "***")?;
				for node in content {
					write!(f, "{node}")?;
				}
				write!(f, "***")
			},

			Self::Bold(content) => {
				write!(f, "**")?;
				for node in content {
					write!(f, "{node}")?;
				}
				write!(f, "**")
			},
			Self::Italic(content) => {
				write!(f, "*")?;
				for node in content {
					write!(f, "{node}")?;
				}
				write!(f, "*")
			}
			Self::String(content) => write!(f, "{content}"),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum  BlockParseState {
	#[default]
	Start,
	YAML,
	CodeBlock,
	LatexBlock,
	BlockQuote,
	TextBlock,
	Unknown,
}

pub fn parse_into_blocks(lines: Vec<String>) -> Vec<Block> {
	let mut lines = lines;
	if lines.is_empty() {
		return vec![];
	}
	let mut state = BlockParseState::Start;
	let mut current_lines: Vec<&str> = Vec::new();
	let mut start_index: usize = 0;
	let mut index: usize = 0;
	let mut block_markers: Vec<(BlockParseState, usize, usize)> = Vec::new();
	loop {
		let line: &String = &lines[index];
		match state {
			BlockParseState::Start => {
				if line.starts_with("---") {
					current_lines.push(line);
					state = BlockParseState::YAML;
				}
				else if line.starts_with("```") && !INLINE_CODE_REGEX.is_match(line) {
					current_lines.push(line);
					state = BlockParseState::CodeBlock;
				}
				else if line.starts_with("$$") && !INLINE_LATEX_REGEX.is_match(line) {
					current_lines.push(line);
					state = BlockParseState::LatexBlock;
				}
				else if line.starts_with('>') {
					current_lines.push(line.strip_prefix(">").unwrap());
					state = BlockParseState::BlockQuote;
				}
				else {
					current_lines.push(line);
					state = BlockParseState::TextBlock;
				}
			},
			BlockParseState::YAML => {
				if line.starts_with("---") {
					current_lines.push(line);
					block_markers.push((BlockParseState::YAML, start_index, index));
					start_index = index + 1;
					current_lines = Vec::new();
					state = BlockParseState::Unknown;
				}
				else {
					current_lines.push(line);
				}
			},
			BlockParseState::CodeBlock => {
				if line.starts_with("```") {
					current_lines.push(line);
					// let current_lines_string: Vec<String> = current_lines.iter().map(|x| x.to_string()).collect();
					// blocks.push(Block::CodeBlock(current_lines_string));
					block_markers.push((BlockParseState::CodeBlock, start_index, index));
					start_index = index + 1;
					current_lines = Vec::new();
					state = BlockParseState::Unknown;
				}
				else {
					current_lines.push(line);
				}
			},
			BlockParseState::LatexBlock => {
				if line.starts_with("$$") {
					current_lines.push(line);
					// let current_lines_string: Vec<String> = current_lines.iter().map(|x| x.to_string()).collect();
					// blocks.push(Block::LatexBlock(current_lines_string));
					block_markers.push((BlockParseState::LatexBlock, start_index, index));
					start_index = index + 1;
					current_lines = Vec::new();
					state = BlockParseState::Unknown;
				}
				else {
					current_lines.push(line);
				}
			},
			BlockParseState::BlockQuote => {
				// the current line may or may not be a block quote,
				// but the previous line was a block quote
				if line.starts_with('>') {
					current_lines.push(line.strip_prefix(">").unwrap());
				}
				else {
					// blocks.push(Block::BlockQuote(parse_into_blocks(current_lines)));
					block_markers.push((BlockParseState::BlockQuote, start_index, index - 1));
					start_index = index;
					index -= 1;
					current_lines = Vec::new();
					state = BlockParseState::Unknown;
				}
			},
			BlockParseState::TextBlock => {
				if line.starts_with("$$") && !INLINE_LATEX_REGEX.is_match(line) {
					// blocks.push(Block::TextBlock(parse_into_lines(current_lines)));
					block_markers.push((BlockParseState::TextBlock, start_index, index - 1));
					start_index = index;
					current_lines = Vec::new();
					current_lines.push(line);
					state = BlockParseState::LatexBlock;
				}
				else if line.starts_with("```") && !INLINE_CODE_REGEX.is_match(line) {
					// blocks.push(Block::TextBlock(parse_into_lines(current_lines)));
					block_markers.push((BlockParseState::TextBlock, start_index, index - 1));
					start_index = index;
					current_lines = Vec::new();
					current_lines.push(line);
					state = BlockParseState::CodeBlock;
				}
				else if line.starts_with('>') {

					// blocks.push(Block::TextBlock(parse_into_lines(current_lines)));
					block_markers.push((BlockParseState::TextBlock, start_index, index - 1));
					start_index = index;
					// text = line.to_string();
					current_lines = Vec::new();
					current_lines.push(line.strip_prefix(">").unwrap());
					state = BlockParseState::BlockQuote;
				}
				else {
					current_lines.push(line);

				}
			},
			BlockParseState::Unknown => {
				if line.starts_with("---") {
					current_lines.push(line);
					state = BlockParseState::YAML;
				}
				else if line.starts_with("```") && !INLINE_CODE_REGEX.is_match(line) {
					current_lines.push(line);
					state = BlockParseState::CodeBlock;
				}
				else if line.starts_with("$$") && !INLINE_LATEX_REGEX.is_match(line) {
					current_lines.push(line);
					state = BlockParseState::LatexBlock;
				}
				else if line.starts_with('>') {
					current_lines.push(line.strip_prefix(">").unwrap());
					state = BlockParseState::BlockQuote;
				}
				else {
					current_lines.push(line);
					state = BlockParseState::TextBlock;
				}
			}
		}
		if index == lines.len() - 1  {
			match state {
				BlockParseState::Start => {},
				BlockParseState::YAML => panic!("Block Parsing Failed! YAML block not closed"),
				BlockParseState::CodeBlock => panic!("Block Parsing Failed! Code block not closed"),
				BlockParseState::LatexBlock => panic!("Block Parsing Failed! Latex block not closed"),
				BlockParseState::BlockQuote => {
					block_markers.push((BlockParseState::BlockQuote, start_index, index));
				},
				BlockParseState::TextBlock => {
					block_markers.push((BlockParseState::TextBlock, start_index, index));
				},
				BlockParseState::Unknown => {}

			}
			break;
		}
		index += 1;

	}

	match state {
		BlockParseState::Start => {},
		BlockParseState::YAML => panic!("Block Parsing Failed! YAML block not closed"),
		BlockParseState::CodeBlock => panic!("Block Parsing Failed! Code block not closed"),
		BlockParseState::LatexBlock => panic!("Block Parsing Failed! Latex block not closed"),
		BlockParseState::BlockQuote => {},
		BlockParseState::TextBlock => {},
		BlockParseState::Unknown => {},
	}


	let mut block_lines: Vec<(BlockParseState, Vec<String>)> = Vec::new();
	for (state, start, end) in block_markers {
		let mut block_lines_inner: Vec<String> = Vec::new();
		for _index in start..=end {
			block_lines_inner.push(lines.remove(0));
		}
		block_lines.push((state, block_lines_inner));
	}

	parse_block_lines_into_blocks(block_lines)

}

pub fn parse_block_lines_into_blocks(block_lines: Vec<(BlockParseState, Vec<String>)>) -> Vec<Block> {
	let mut block_lines: Vec<(BlockParseState, Vec<String>)> = block_lines;
	let mut blocks: Vec<Block> = Vec::new();

	loop {
		if block_lines.is_empty() {
			break;
		}
		let (state, lines) = block_lines.remove(0);
		match state {
			BlockParseState::YAML => {
				let text: String = lines.join("\n");
				let caps = YAML_PATTERN.captures(&text).unwrap();
				let inner_yaml = caps.get(1).unwrap().as_str();
				blocks.push(Block::YAML(serde_yaml::from_str(inner_yaml).unwrap(), lines));
			},
			BlockParseState::CodeBlock => {
				blocks.push(Block::CodeBlock(lines));
			},
			BlockParseState::LatexBlock => {
				blocks.push(Block::LatexBlock(lines));
			},
			BlockParseState::BlockQuote => {
				// remove leading > character
				let lines: Vec<String> = lines.iter().map(|x| x.strip_prefix(">").unwrap().to_string()).collect();
				blocks.push(Block::BlockQuote(parse_into_blocks(lines)));
			},
			BlockParseState::TextBlock => {
				blocks.push(Block::TextBlock(parse_into_lines(lines)));
			},
			BlockParseState::Unknown => {},
			BlockParseState::Start => {},
		}
	}

	blocks
}


pub fn parse_into_lines(lines: Vec<String>) -> Vec<Line> {
	let lines = lines;
	let mut line_vec: Vec<Line> = Vec::new();
	for line in lines {

		// if heading_pattern.is_match(&line) {
		if line.starts_with('#') {
			let chars: Vec<char> = line.chars().collect();
			let level: u8 = chars.iter().take_while(|x| x == &&'#').count() as u8;
			let text: String = chars.iter().skip_while(|x| x == &&'#' || x == &&' ').collect();
			line_vec.push(Line::Heading(parse_into_nodes(text), level));
		}
		else if BULLET_POINT_PATTERN.is_match(&line) {
		// else if line.starts_with("-") {
			let caps = BULLET_POINT_PATTERN.captures(&line).unwrap();
			let indentation: &str = &caps["indent"];
			let text: &str = &caps["text"];
			line_vec.push(Line::BulletPoint(parse_into_nodes(text.to_string()), indentation.to_string()));
		}
		else if LIST_ITEM_PATTERN.is_match(&line) {
			let caps = LIST_ITEM_PATTERN.captures(&line).unwrap();
			let indentation = &caps["indent"];
			let number = caps["number"].parse::<u32>().unwrap();
			let text = &caps["text"];
			line_vec.push(Line::ListItem(parse_into_nodes(text.to_string()), number, indentation.to_string()));
		}
		else if LINEBAR_PATTERN.is_match(&line) {
			line_vec.push(Line::Linebar);
		}
		else {
			line_vec.push(Line::String(parse_into_nodes(line)));
		}
	}

	line_vec
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum NodeParseState {
	// Start,

	InlineCode,
	InlineBlockLatex,
	InlineLatex,
	FormattedMarkdownLink,
	MarkdownLink,
	FormattedWebLink,
	WebLink,
	BoldItalic,
	Bold,
	Italic,
	#[default]
	String,
}


pub fn parse_into_nodes(content: String) -> Vec<Node> {

	let _nodes: Vec<Node> = Vec::new();
	let content = content.strip_suffix("\n").unwrap_or(&content).to_string();
	let content = content.strip_suffix("\r").unwrap_or(&content).to_string();
	let mut start_index: usize = 0;
	let mut index: usize = 0;
	let mut node_parse_vector: Vec<(NodeParseState, usize, usize)> = Vec::new();
	let characters: Vec<char> = content.chars().collect();
	loop {
		if index >= characters.len()  {
			node_parse_vector.push((NodeParseState::String, start_index, index));
			break;
		}

		let loop_string = &String::from_iter(&characters[index..]);

		if INLINE_CODE_REGEX.is_match(loop_string) {
			if index != start_index {
				node_parse_vector.push((NodeParseState::String, start_index, index));
				// start_index = index;
			}
			let caps = INLINE_CODE_PATTERN.captures(loop_string).unwrap();
			let length = caps.get(0).unwrap().as_str().chars().count();
			node_parse_vector.push((NodeParseState::InlineCode, index, index + length));
			index += length;
			start_index = index;
		} else if INLINE_BLOCK_LATEX_PATTERN.is_match(loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
				// start_index = index;
			}
			let caps = INLINE_BLOCK_LATEX_PATTERN.captures(loop_string).unwrap();
			let cap_0 = caps.get(0).unwrap().as_str();
			// println!("Cap: {}", cap_0);
			let length =
				cap_0.chars().count();
			// println!("Block Latex Length: {}", length);
			// println!("characters: {:?}", cap_0.to_string());
			// nodes.push(Node::InlineBlockLatex(caps.get(1).unwrap().as_str().to_string()));
			node_parse_vector.push((NodeParseState::InlineBlockLatex, index, index + length));
			index +=  length;
			start_index = index;

		}
		else if INLINE_LATEX_PATTERN.is_match(loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
			}
			let caps = INLINE_LATEX_PATTERN.captures(loop_string).unwrap();
			// nodes.push(Node::InlineLatex(caps.get(1).unwrap().as_str().to_string()));
			let length = caps.get(0).unwrap().as_str().chars().count();
			node_parse_vector.push((NodeParseState::InlineLatex, index, index + length));
			index += length;
			start_index = index;
		} else if FORMATTED_MARKDOWN_LINK_PATTERN.is_match(loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
			}
			let caps = FORMATTED_MARKDOWN_LINK_PATTERN.captures(loop_string).unwrap();
			// nodes.push(Node::FormattedMarkdownLink(caps.get(1).unwrap().as_str().to_string(), caps.get(2).unwrap().as_str().to_string()));
			let length = caps.get(0).unwrap().as_str().chars().count();
			node_parse_vector.push((NodeParseState::FormattedMarkdownLink, index, index + length));
			index += length;
			start_index = index;
		} else if MARKDOWN_LINK_PATTERN.is_match(loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
			}
			let caps = MARKDOWN_LINK_PATTERN.captures(loop_string).unwrap();
			// nodes.push(Node::MarkdownLink(caps.get(1).unwrap().as_str().to_string()));
			let length = caps.get(0).unwrap().as_str().chars().count();
			node_parse_vector.push((NodeParseState::MarkdownLink, index, index + length));
			index += length;
			start_index = index;
		} else if FORMATTED_WEB_LINK_PATTERN.is_match(loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
			}
			let caps = FORMATTED_WEB_LINK_PATTERN.captures(loop_string).unwrap();
			// nodes.push(Node::FormattedWebLink(caps.get(2).unwrap().as_str().to_string(), caps.get(1).unwrap().as_str().to_string()));
			let length = caps.get(0).unwrap().as_str().chars().count();
			node_parse_vector.push((NodeParseState::FormattedWebLink, index, index + length));
			index += length;
			start_index = index;

		} else if WEB_LINK_PATTERN.is_match(loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
			}
			let caps = WEB_LINK_PATTERN.captures(loop_string).unwrap();
			// nodes.push(Node::WebLink(caps.get(1).unwrap().as_str().to_string()));
			let length = caps.get(0).unwrap().as_str().chars().count();
			node_parse_vector.push((NodeParseState::WebLink, index, index + length));
			index += length;
			start_index = index;
			// text = String::new();
		} else if BOLD_ITALIC_PATTERN.is_match(loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
			}
			let caps = BOLD_ITALIC_PATTERN.captures(loop_string).unwrap();
			// nodes.push(Node::BoldItalic(parse_into_nodes(caps.get(1).unwrap().as_str().to_string())));
			let length = caps.get(0).unwrap().as_str().chars().count();
			node_parse_vector.push((NodeParseState::BoldItalic, index, index + length));
			index += length;
			start_index = index;
			// text = String::new();
		} else if BOLD_PATTERN.is_match(loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
			}
			let caps = BOLD_PATTERN.captures(loop_string).unwrap();
			// nodes.push(Node::Bold(parse_into_nodes(caps.get(1).unwrap().as_str().to_string())));
			let length = caps.get(0).unwrap().as_str().chars().count();
			node_parse_vector.push((NodeParseState::Bold, index, index + length));
			index += length;
			start_index = index;
			// text = String::new();
		} else if ITALIC_PATTERN.is_match(loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
			}
			let caps = ITALIC_PATTERN.captures(loop_string).unwrap();
			// nodes.push(Node::Italic(parse_into_nodes(caps.get(1).unwrap().as_str().to_string())));
			let length = caps.get(0).unwrap().as_str().chars().count();
			node_parse_vector.push((NodeParseState::Italic, index, index + length));
			index += length;
			start_index = index;
			// text = String::new();
		} else {
			index += 1;
		}
	}
	let mut node_parts: Vec<(NodeParseState, Vec<char>)> = Vec::new();
	// end index is not inclusive
	for (state, start, end) in node_parse_vector {
		// println!("State: {:?}, Start: {}, End: {}", state, start, end);
		let mut chars: Vec<char> = Vec::new();
		for index in start..end {
			chars.push(characters[index]);
			// chars.push(characters.remove(0));
		}
		node_parts.push((state, chars));
	}
	parse_node_lines_into_nodes(node_parts)

}

pub fn parse_node_lines_into_nodes(node_parts: Vec<(NodeParseState, Vec<char>)>) -> Vec<Node> {
	let mut node_parts: Vec<(NodeParseState, Vec<char>)> = node_parts;
	let mut nodes: Vec<Node> = Vec::new();
	loop {
		if node_parts.is_empty() {
			break;
		}
		let (state, parts) = node_parts.remove(0);
		match state {
			NodeParseState::InlineCode => {
				let parts: String = parts[1..parts.len() - 1].iter().collect();
				nodes.push(Node::InlineCode(parts));
			},
			NodeParseState::InlineBlockLatex => {
				let parts: String = parts[2..parts.len() - 2].iter().collect();
				nodes.push(Node::InlineBlockLatex(parts));
			},
			NodeParseState::InlineLatex => {
				let parts: String = parts[1..parts.len() - 1].iter().collect();
				nodes.push(Node::InlineLatex(parts));
			},
			NodeParseState::FormattedMarkdownLink => {
				let parts: String = parts[2..parts.len() - 2].iter().collect();
				let parts: String = parts.chars().collect();
				let mut parts: Vec<String> = parts.split('|').map(std::string::ToString::to_string).collect();
				let part_0: String = parts.remove(0);
				let part_1: String = parts.remove(0);
				nodes.push(Node::FormattedMarkdownLink(part_0, part_1));
			},
			NodeParseState::MarkdownLink => {
				let parts: String = parts[2..parts.len() - 2].iter().collect();
				nodes.push(Node::MarkdownLink(parts));
			},
			NodeParseState::FormattedWebLink => {
				let parts: String = parts[1..parts.len() - 1].iter().collect();
				let parts: Vec<&str> = parts.split("](").collect();
				nodes.push(Node::FormattedWebLink(parts[1].to_string(), parts[0].to_string()));
			},
			NodeParseState::WebLink => {
				let parts: String = parts[1..parts.len() - 1].iter().collect();
				nodes.push(Node::WebLink(parts));
			},
			NodeParseState::BoldItalic => {
				let parts: String = parts[3..parts.len() - 3].iter().collect();
				nodes.push(Node::BoldItalic(parse_into_nodes(parts)));
			},
			NodeParseState::Bold => {
				let parts: String = parts[2..parts.len() - 2].iter().collect();
				nodes.push(Node::Bold(parse_into_nodes(parts)));
			},
			NodeParseState::Italic => {
				let parts: String = parts[1..parts.len() - 1].iter().collect();
				nodes.push(Node::Italic(parse_into_nodes(parts)));
			},
			NodeParseState::String => {
				let parts: String = parts.iter().collect();
				nodes.push(Node::String(parts));
			},
		}
	}
	nodes
}