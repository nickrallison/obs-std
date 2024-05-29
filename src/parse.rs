use std::cmp::PartialEq;
use std::fmt::Display;
use lazy_static::lazy_static;
use regex::Regex;
use crate::traits::Verbose;

lazy_static! {
	static ref inline_latex_regex: Regex = Regex::new(r"(?m)^\$\$.*?\$\$").unwrap();
	static ref inline_code_regex: Regex = Regex::new(r"(?m)^```.*?```").unwrap();
	static ref heading_pattern: Regex = Regex::new(r"(?m)^(?P<level>#+) (?P<text>.*?)$").unwrap();
	static ref bullet_point_pattern: Regex = Regex::new(r"(?m)^(?P<indent>[ \r\t]*)?- (?P<text>.*?)$").unwrap();
	static ref list_item_pattern: Regex = Regex::new(r"(?m)^(?P<indent>[ \r\t]*)?(?P<number>\d+)\. (?P<text>.*?)$").unwrap();
	static ref linebar_pattern: Regex = Regex::new(r"(?m)^---\s*?$").unwrap();
	static ref yaml_pattern: Regex = Regex::new(r"(?m)^---([\s\S]*?)---").unwrap();
	static ref bold_italic_pattern: Regex = Regex::new(r"^\*\*\*(.*?)\*\*\*").unwrap();
	static ref bold_pattern: Regex = Regex::new(r"^\*\*(.*?)\*\*").unwrap();
	static ref italic_pattern: Regex = Regex::new(r"^\*(.*?)\*").unwrap();
	static ref inline_code_pattern: Regex = Regex::new(r"^`(.*?)`").unwrap();
	static ref inline_block_latex_pattern: Regex = Regex::new(r"^\$\$(.*?)\$\$").unwrap();
	static ref inline_latex_pattern: Regex = Regex::new(r"^\$(.*?)\$").unwrap();
	static ref formatted_markdown_link_pattern: Regex = Regex::new(r"^\[\[(.+?)\|(.+?)\]\]").unwrap();
	static ref markdown_link_pattern: Regex = Regex::new(r"^\[\[(.+?)\]\]").unwrap();
	static ref formatted_web_link_pattern: Regex = Regex::new(r"^\[(.+?)\]\((.+?)\)").unwrap();
	static ref web_link_pattern: Regex = Regex::new(r"^\[(.+?)\]").unwrap();

}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AST {
	blocks: Vec<Block>,
	line_sep: LineSep
}

#[allow(dead_code)]
impl AST {

	pub(crate) fn new(contents: String) -> AST {
		let mut contents = contents;
		let line_sep: LineSep = if (&contents).contains("\r\n") {
			LineSep::Windows
		} else {
			LineSep::Unix
		};
		let lines: Vec<String> = contents.split('\n').map(|x| x.to_string()).collect();
		let blocks = parse_into_blocks(lines);
		AST {
			blocks,
			line_sep,
		}
	}

	fn get_yaml(&self, tag: &str) -> Option<serde_yaml::Value> {
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
		return None;
	}

	pub(crate) fn get_aliases(&self) -> Vec<String> {
		if self.get_yaml("aliases").is_none() {
			return vec![];
		}
		self.get_yaml("aliases").unwrap().as_sequence().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect()
	}

	pub(crate) fn lines(&self) -> Vec<String> {
		let mut lines: Vec<String> = Vec::new();
		for block in &self.blocks {
			let block_lines = block.lines();
			for line in block_lines {
				lines.push(format!("{}", line));
			}
		}
		lines
	}

}

impl Display for AST {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut blocks_str_vec: Vec<String> = Vec::new();
		for block in &self.blocks {
			blocks_str_vec.push(format!("{}", block));
		}
		let mut file_contents: String = blocks_str_vec.join("\n");
		file_contents = file_contents.replace("\r\n", "\n");
		match self.line_sep {
			LineSep::Unix => write!(f, "{}", file_contents),
			LineSep::Windows => write!(f, "{}", file_contents.replace("\n", "\r\n")),
		}

	}

}

// impl Verbose for AST {
// 	fn verbose(&self) -> String {
// 		let mut verbose: String = String::new();
// 		for block in &self.blocks {
//
// 			verbose = verbose + &block.verbose();
// 		}
// 		verbose
// 	}
//
// }

#[derive(Debug, Clone, PartialEq)]
enum LineSep {
	Unix,
	Windows,
}

#[derive(Debug, Clone, PartialEq)]
enum Block {
	YAML(serde_yaml::Value, Vec<String>),
	CodeBlock(Vec<String>),
	LatexBlock(Vec<String>),
	BlockQuote(Vec<Block>),
	TextBlock(Vec<Line>)
}


impl Block {
	pub(crate) fn lines(&self) -> Vec<String> {
		match self {
			Block::YAML(_, _) => vec![],
			Block::CodeBlock(_) => vec![],
			Block::LatexBlock(_) => vec![],
			Block::BlockQuote(blocks) => {
				let mut lines: Vec<String> = Vec::new();
				for block in blocks {
					let block_lines = block.lines();
					for line in block_lines {
						lines.push(format!("{}", line));
					}
				}
				lines
			},
			Block::TextBlock(lines) => {
				let mut lines_str: Vec<String> = Vec::new();
				for line in lines {
					lines_str.push(format!("{}", line.clean_line()));
				}
				lines_str
			},
		}
	}
}



impl Display for Block {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Block::YAML(_, lines_vec) => {
				let lines_string: String = lines_vec.join("\n");
				write!(f, "{}", lines_string)
			},
			Block::CodeBlock(lines_vec) => {
				let lines_string: String = lines_vec.join("\n");
				write!(f, "{}", lines_string)
			},
			Block::LatexBlock(lines_vec) => {
				let lines_string: String = lines_vec.join("\n");
				write!(f, "{}", lines_string)
			},
			Block::BlockQuote(blocks_vec) => {
				let blocks_str_vec: Vec<String> = blocks_vec.iter().map(|x| format!("{}", x)).collect();
				let blocks_string: String = blocks_str_vec.join("\n");
				let prepended_blocks_string_vec: Vec<String> = blocks_string.lines().map(|x| format!(">{}", x)).collect();
				let prepended_blocks_string: String = prepended_blocks_string_vec.join("\n");
				write!(f, "{}", prepended_blocks_string)
			},
			Block::TextBlock(lines_vec) => {
				let lines_str_vec: Vec<String> = lines_vec.iter().map(|x| format!("{}", x)).collect();
				let lines_string: String = lines_str_vec.join("\n");
				write!(f, "{}", lines_string)
			},
		}
	}
}

// impl Verbose for Block {
// 	fn verbose(&self) -> String {
// 		match self {
// 			Block::YAML(_, string) => {
// 				format!("YAML:\n{}", add_indentation("\t", string))
// 			}
// 			Block::CodeBlock(code_block) => {
// 				format!("Code Block:\n{}", add_indentation("\t", code_block))
// 			}
// 			Block::LatexBlock(latex_block) => {
// 				format!("Latex Block:\n{}", add_indentation("\t", latex_block))
// 			}
// 			Block::BlockQuote(block_quote) => {
// 				let mut verbose: String = String::new();
// 				verbose = verbose + "Block Quote:\n";
// 				for block in block_quote {
// 					verbose = verbose + &add_indentation("\t", &(block.verbose()));
// 				}
// 				verbose
// 			}
// 			Block::TextBlock(text_block) => {
// 				let mut verbose: String = String::new();
// 				verbose = verbose + "Text Block:\n";
// 				for line in text_block {
// 					verbose = verbose + &add_indentation("\t", &line.verbose());
// 				}
// 				verbose
// 			}
//
// 		}
//
// 	}
// }

#[derive(Debug, Clone, PartialEq)]
enum Line {
	Heading(Vec<Node>, u8),
	BulletPoint(Vec<Node>, String),
	ListItem(Vec<Node>, u32, String),
	String(Vec<Node>),
	Linebar,
}

impl Line {
	pub(crate) fn clean_line(&self) -> String {
		match self {
			Line::Heading(nodes, _) => {
				let mut clean_line: String = String::new();
				for node in nodes {
					match node {
						Node::String(content) => clean_line = clean_line + content,
						_ => continue,
					}
				}
				clean_line
			},
			Line::BulletPoint(nodes, _) => {
				let mut clean_line: String = String::new();
				for node in nodes {
					match node {
						Node::String(content) => clean_line = clean_line + content,
						_ => continue,
					}
				}
				clean_line
			},
			Line::ListItem(nodes, _, _) => {
				let mut clean_line: String = String::new();
				for node in nodes {
					match node {
						Node::String(content) => clean_line = clean_line + content,
						_ => continue,
					}
				}
				clean_line
			},
			Line::String(nodes) => {
				let mut clean_line: String = String::new();
				for node in nodes {
					match node {
						Node::String(content) => clean_line = clean_line + content,
						_ => continue,
					}
				}
				clean_line
			},
			Line::Linebar => "---".to_string(),
		}
	}
}

impl Display for Line {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Line::Heading(nodes, level) => {
				// write!(f, "\n")?;
				// write the heading level
				for _ in 0..*level {
					write!(f, "#")?;
				}

				write!(f, " ")?;
				for node in nodes {
					write!(f, "{}", node)?;
				}
				write!(f, "")
			},
			Line::BulletPoint(nodes, indentation) => {
				// write!(f, "\n")?;
				write!(f, "{}- ", indentation)?;
				for node in nodes {
					write!(f, "{}", node)?;
				}
				write!(f, "")
			},
			Line::ListItem(nodes, number, indentation) => {
				// write!(f, "\n")?;
				write!(f, "{}{}. ", indentation, number)?;
				for node in nodes {
					write!(f, "{}", node)?;
				}
				write!(f, "")
			},
			Line::String(nodes) => {

				// write!(f, "\n")?;
				for node in nodes {
					write!(f, "{}", node)?;
				}
				write!(f, "")
			},
			Line::Linebar => write!(f, "---"),
		}
	}
}

impl Verbose for Line {
	fn verbose(&self) -> String {
		match self {
			Line::Heading(nodes, level) => {
				let mut verbose: String = String::new();
				verbose = verbose + "Heading:\n";
				verbose = verbose + &add_indentation("\t", &format!("Level: {}", level));
				verbose = verbose + "Nodes:\n";
				for node in nodes {
					verbose = verbose + &add_indentation("\t", &node.verbose());
				}
				verbose
			},
			Line::BulletPoint(nodes, indentation) => {
				let mut verbose: String = String::new();
				verbose = verbose + "Bullet Point:\n";
				verbose = verbose + &add_indentation("\t", &format!("Indentation: {}", indentation));
				verbose = verbose + "Nodes:\n";
				for node in nodes {
					verbose = verbose + &add_indentation("\t", &node.verbose());
				}
				verbose
			},
			Line::ListItem(nodes, number, indentation) => {
				let mut verbose: String = String::new();
				verbose = verbose + "List Item:\n";
				verbose = verbose + &add_indentation("\t", &format!("Number: {}", number));
				verbose = verbose + &add_indentation("\t", &format!("Indentation: {}", indentation));
				verbose = verbose + "Nodes:\n";
				for node in nodes {
					verbose = verbose + &add_indentation("\t", &node.verbose());
				}
				verbose
			},
			Line::String(nodes) => {
				let mut verbose: String = String::new();
				verbose = verbose + "String:\n";
				verbose = verbose + "Nodes:\n";
				for node in nodes {
					verbose = verbose + &add_indentation("\t", &node.verbose());
				}
				verbose
			},
			Line::Linebar => "Linebar".to_string(),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
enum Node {

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

impl Display for Node {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Node::InlineCode(content) => write!(f, "`{}`", content),
			Node::InlineBlockLatex(content) => write!(f, "$${}$$", content),
			Node::InlineLatex(content) => write!(f, "${}$", content),
			Node::MarkdownLink(content) => write!(f, "[[{}]]", content),
			Node::FormattedMarkdownLink(link, text) => write!(f, "[[{}|{}]]", link, text),
			Node::WebLink(content) => write!(f, "[{}]", content),
			Node::FormattedWebLink(link, text) => write!(f, "[{}]({})", text, link),
			Node::BoldItalic(content) => {
				write!(f, "***")?;
				for node in content {
					write!(f, "{}", node)?;
				}
				write!(f, "***")
			},

			Node::Bold(content) => {
				write!(f, "**")?;
				for node in content {
					write!(f, "{}", node)?;
				}
				write!(f, "**")
			},
			Node::Italic(content) => {
				write!(f, "*")?;
				for node in content {
					write!(f, "{}", node)?;
				}
				write!(f, "*")
			}
			Node::String(content) => write!(f, "{}", content),
		}
	}
}

impl Verbose for Node {
	fn verbose(&self) -> String {
		match self {
			Node::InlineCode(content) => format!("Inline Code: {}", content),
			Node::InlineBlockLatex(content) => format!("Inline Latex Block: {}", content),
			Node::InlineLatex(content) => format!("Inline Latex: {}", content),
			Node::MarkdownLink(content) => format!("Markdown Link: {}", content),
			Node::FormattedMarkdownLink(link, text) => format!("Formatted Markdown Link: {} -> {}", text, link),
			Node::WebLink(content) => format!("Web Link: {}", content),
			Node::FormattedWebLink(link, text) => format!("Formatted Web Link: Link: {} -> {}", text, link),
			Node::BoldItalic(content) => {
				let mut verbose: String = String::new();
				verbose = verbose + "Bold Italic:\n";
				verbose = verbose + &add_indentation("\t", "Content:\n");
				for node in content {
					verbose = verbose + &add_indentation("\t", &node.verbose());
				}
				verbose
			},
			Node::Bold(content) => {
				let mut verbose: String = String::new();
				verbose = verbose + "Bold:\n";
				verbose = verbose + &add_indentation("\t", "Content:\n");
				for node in content {
					verbose = verbose + &add_indentation("\t", &node.verbose());
				}
				verbose
			},
			Node::Italic(content) => {
				let mut verbose: String = String::new();
				verbose = verbose + "Italic:\n";
				verbose = verbose + &add_indentation("\t", "Content:\n");
				for node in content {
					verbose = verbose + &add_indentation("\t", &node.verbose());
				}
				verbose
			}
			Node::String(content) => format!("String: {}", content),
		}
	}
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum  BlockParseState {
	Start,
	YAML,
	CodeBlock,
	LatexBlock,
	BlockQuote,
	TextBlock,
	Unknown,
}

fn parse_into_blocks(lines: Vec<String>) -> Vec<Block> {
	let mut lines = lines;
	if lines.len() == 0 {
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
				else if line.starts_with("```") && !inline_code_regex.is_match(line) {
					current_lines.push(line);
					state = BlockParseState::CodeBlock;
				}
				else if line.starts_with("$$") && !inline_latex_regex.is_match(line) {
					current_lines.push(line);
					state = BlockParseState::LatexBlock;
				}
				else if line.starts_with(">") {
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
				if line.starts_with(">") {
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
				if line.starts_with("$$") && !inline_latex_regex.is_match(line) {
					// blocks.push(Block::TextBlock(parse_into_lines(current_lines)));
					block_markers.push((BlockParseState::TextBlock, start_index, index - 1));
					start_index = index;
					start_index = index;
					current_lines = Vec::new();
					current_lines.push(line);
					state = BlockParseState::LatexBlock;
				}
				else if line.starts_with("```") && !inline_code_regex.is_match(line) {
					// blocks.push(Block::TextBlock(parse_into_lines(current_lines)));
					block_markers.push((BlockParseState::TextBlock, start_index, index - 1));
					start_index = index;
					current_lines = Vec::new();
					current_lines.push(line);
					state = BlockParseState::CodeBlock;
				}
				else if line.starts_with(">") {

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
				else if line.starts_with("```") && !inline_code_regex.is_match(line) {
					current_lines.push(line);
					state = BlockParseState::CodeBlock;
				}
				else if line.starts_with("$$") && !inline_latex_regex.is_match(line) {
					current_lines.push(line);
					state = BlockParseState::LatexBlock;
				}
				else if line.starts_with(">") {
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
		} else {
			index += 1;
		}
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
		for index in start..end + 1 {
			block_lines_inner.push(lines.remove(0));
		}
		block_lines.push((state, block_lines_inner));
	}

	parse_block_lines_into_blocks(block_lines)

}

fn parse_block_lines_into_blocks(block_lines: Vec<(BlockParseState, Vec<String>)>) -> Vec<Block> {
	let mut block_lines: Vec<(BlockParseState, Vec<String>)> = block_lines;
	let mut blocks: Vec<Block> = Vec::new();

	loop {
		if block_lines.len() == 0 {
			break;
		}
		let (state, lines) = block_lines.remove(0);
		match state {
			BlockParseState::YAML => {
				let text: String = lines.join("\n");
				let caps = yaml_pattern.captures(&text).unwrap();
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


fn parse_into_lines(lines: Vec<String>) -> Vec<Line> {
	let lines = lines;
	let mut line_vec: Vec<Line> = Vec::new();
	for line in lines {

		// if heading_pattern.is_match(&line) {
		if line.starts_with("#") {
			let mut chars: Vec<char> = line.chars().collect();
			let level: u8 = chars.iter().take_while(|x| x == &&'#').count() as u8;
			let text: String = chars.iter().skip_while(|x| x == &&'#' || x == &&' ').collect();
			line_vec.push(Line::Heading(parse_into_nodes(text), level));
		}
		else if bullet_point_pattern.is_match(&line) {
		// else if line.starts_with("-") {
			let caps = bullet_point_pattern.captures(&line).unwrap();
			let indentation: &str = &caps["indent"];
			let text: &str = &caps["text"];
			line_vec.push(Line::BulletPoint(parse_into_nodes(text.to_string()), indentation.to_string()));
		}
		else if list_item_pattern.is_match(&line) {
			let caps = list_item_pattern.captures(&line).unwrap();
			let indentation = &caps["indent"];
			let number = (&caps["number"]).parse::<u32>().unwrap();
			let text = &caps["text"];
			line_vec.push(Line::ListItem(parse_into_nodes(text.to_string()), number, indentation.to_string()));
		}
		else if linebar_pattern.is_match(&line) {
			line_vec.push(Line::Linebar);
		}
		else {
			line_vec.push(Line::String(parse_into_nodes(line)));
		}
	}

	return line_vec;
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum NodeParseState {
	Start,
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
	String,
}


fn parse_into_nodes(content: String) -> Vec<Node> {

	let mut nodes: Vec<Node> = Vec::new();
	// let mut text: String = String::new();
	let mut start_index: usize = 0;
	let mut index: usize = 0;
	let mut node_parse_vector: Vec<(NodeParseState, usize, usize)> = Vec::new();
	let mut characters: Vec<char> = content.chars().collect();
	loop {
		if index >= characters.len()  {
			node_parse_vector.push((NodeParseState::String, start_index, index));
			break;
		}

		let loop_string = &String::from_iter(&characters[index..]);

		// if inline_code_pattern.is_match(&loop_string) {
		if inline_code_regex.is_match(loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
				start_index = index;
			}
			let caps = inline_code_pattern.captures(&loop_string).unwrap();
			// nodes.push(Node::InlineCode(caps.get(1).unwrap().as_str().to_string()));
			let length = caps.get(0).unwrap().as_str().chars().count();
			node_parse_vector.push((NodeParseState::InlineCode, index, index + length));
			index += length;
			start_index = index;
		} else if inline_block_latex_pattern.is_match(&loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
				start_index = index;
			}
			let caps = inline_block_latex_pattern.captures(&loop_string).unwrap();
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
		else if inline_latex_pattern.is_match(&loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
			}
			let caps = inline_latex_pattern.captures(&loop_string).unwrap();
			// nodes.push(Node::InlineLatex(caps.get(1).unwrap().as_str().to_string()));
			let length = caps.get(0).unwrap().as_str().chars().count();
			node_parse_vector.push((NodeParseState::InlineLatex, index, index + length));
			index += length;
			start_index = index;
		} else if formatted_markdown_link_pattern.is_match(&loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
			}
			let caps = formatted_markdown_link_pattern.captures(&loop_string).unwrap();
			// nodes.push(Node::FormattedMarkdownLink(caps.get(1).unwrap().as_str().to_string(), caps.get(2).unwrap().as_str().to_string()));
			let length = caps.get(0).unwrap().as_str().chars().count();
			node_parse_vector.push((NodeParseState::FormattedMarkdownLink, index, index + length));
			index += length;
			start_index = index;
		} else if markdown_link_pattern.is_match(&loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
			}
			let caps = markdown_link_pattern.captures(&loop_string).unwrap();
			// nodes.push(Node::MarkdownLink(caps.get(1).unwrap().as_str().to_string()));
			let length = caps.get(0).unwrap().as_str().chars().count();
			node_parse_vector.push((NodeParseState::MarkdownLink, index, index + length));
			index += length;
			start_index = index;
		} else if formatted_web_link_pattern.is_match(&loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
			}
			let caps = formatted_web_link_pattern.captures(&loop_string).unwrap();
			// nodes.push(Node::FormattedWebLink(caps.get(2).unwrap().as_str().to_string(), caps.get(1).unwrap().as_str().to_string()));
			let length = caps.get(0).unwrap().as_str().chars().count();
			node_parse_vector.push((NodeParseState::FormattedWebLink, index, index + length));
			index += length;
			start_index = index;

		} else if web_link_pattern.is_match(&loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
			}
			let caps = web_link_pattern.captures(&loop_string).unwrap();
			// nodes.push(Node::WebLink(caps.get(1).unwrap().as_str().to_string()));
			let length = caps.get(0).unwrap().as_str().chars().count();
			node_parse_vector.push((NodeParseState::WebLink, index, index + length));
			index += length;
			start_index = index;
			// text = String::new();
		} else if bold_italic_pattern.is_match(&loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
			}
			let caps = bold_italic_pattern.captures(&loop_string).unwrap();
			// nodes.push(Node::BoldItalic(parse_into_nodes(caps.get(1).unwrap().as_str().to_string())));
			let length = caps.get(0).unwrap().as_str().chars().count();
			node_parse_vector.push((NodeParseState::BoldItalic, index, index + length));
			index += length;
			start_index = index;
			// text = String::new();
		} else if bold_pattern.is_match(&loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
			}
			let caps = bold_pattern.captures(&loop_string).unwrap();
			// nodes.push(Node::Bold(parse_into_nodes(caps.get(1).unwrap().as_str().to_string())));
			let length = caps.get(0).unwrap().as_str().chars().count();
			node_parse_vector.push((NodeParseState::Bold, index, index + length));
			index += length;
			start_index = index;
			// text = String::new();
		} else if italic_pattern.is_match(&loop_string) {
			if index != start_index {
				// nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
				node_parse_vector.push((NodeParseState::String, start_index, index));
			}
			let caps = italic_pattern.captures(&loop_string).unwrap();
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

fn parse_node_lines_into_nodes(node_parts: Vec<(NodeParseState, Vec<char>)>) -> Vec<Node> {
	// for (state, parts) in &node_parts {
	// 	println!("{:?} -> {}", state, parts.iter().collect::<String>());
	// }
	let mut node_parts: Vec<(NodeParseState, Vec<char>)> = node_parts;
	let mut nodes: Vec<Node> = Vec::new();
	loop {
		if node_parts.len() == 0 {
			break;
		}
		let (state, parts) = node_parts.remove(0);
		match state {
			NodeParseState::InlineCode => {
				// prune the backticks
				// let parts = parts.strip_prefix("`").unwrap().strip_suffix("`").unwrap().to_string();
				let parts: String = parts[1..parts.len() - 1].iter().collect();
				nodes.push(Node::InlineCode(parts));
			},
			NodeParseState::InlineBlockLatex => {
				// prune the dollar signs
				// let parts = parts.strip_prefix("$$").unwrap().strip_suffix("$$").unwrap().to_string();
				// println!("Parts Before: {:?}", parts.iter().collect::<String>());
				let parts: String = parts[2..parts.len() - 2].iter().collect();
				// println!("Parts After: {:?}", parts);
				nodes.push(Node::InlineBlockLatex(parts));
			},
			NodeParseState::InlineLatex => {
				// prune the dollar signs
				// let parts = parts.strip_prefix("$").unwrap().strip_suffix("$").unwrap().to_string();
				let parts: String = parts[1..parts.len() - 1].iter().collect();
				nodes.push(Node::InlineLatex(parts));
			},
			NodeParseState::FormattedMarkdownLink => {
				// prune the brackets
				// let parts = parts.strip_prefix("[[").unwrap().strip_suffix("]]").unwrap().to_string();
				let parts: String = parts[2..parts.len() - 2].iter().collect();
				let parts: String = parts.chars().collect();
				let mut parts: Vec<String> = parts.split("|").map(|x| x.to_string()).collect();
				let part_0: String = parts.remove(0);
				let part_1: String = parts.remove(0);
				nodes.push(Node::FormattedMarkdownLink(part_0, part_1));
			},
			NodeParseState::MarkdownLink => {
				// prune the brackets
				// let parts = parts.strip_prefix("[[").unwrap().strip_suffix("]]").unwrap().to_string();
				let parts: String = parts[2..parts.len() - 2].iter().collect();
				nodes.push(Node::MarkdownLink(parts));
			},
			NodeParseState::FormattedWebLink => {
				// prune the brackets
				// let parts = parts.strip_prefix("[").unwrap().strip_suffix(")").unwrap().to_string();
				let parts: String = parts[1..parts.len() - 1].iter().collect();
				let parts: Vec<&str> = parts.split("](").collect();
				nodes.push(Node::FormattedWebLink(parts[1].to_string(), parts[0].to_string()));
			},
			NodeParseState::WebLink => {
				// prune the brackets
				// let parts = parts.strip_prefix("[").unwrap().strip_suffix("]").unwrap().to_string();
				let parts: String = parts[1..parts.len() - 1].iter().collect();
				nodes.push(Node::WebLink(parts));
			},
			NodeParseState::BoldItalic => {
				// prune the asterisks
				// let parts = parts.strip_prefix("***").unwrap().strip_suffix("***").unwrap().to_string();
				let parts: String = parts[3..parts.len() - 3].iter().collect();
				nodes.push(Node::BoldItalic(parse_into_nodes(parts)));
			},
			NodeParseState::Bold => {
				// prune the asterisks
				// let parts = parts.strip_prefix("**").unwrap().strip_suffix("**").unwrap().to_string();
				let parts: String = parts[2..parts.len() - 2].iter().collect();
				nodes.push(Node::Bold(parse_into_nodes(parts)));
			},
			NodeParseState::Italic => {
				// prune the asterisks
				// let parts = parts.strip_prefix("*").unwrap().strip_suffix("*").unwrap().to_string();
				let parts: String = parts[1..parts.len() - 1].iter().collect();
				nodes.push(Node::Italic(parse_into_nodes(parts)));
			},
			NodeParseState::String => {
				let parts: String = parts.iter().collect();
				nodes.push(Node::String(parts));
			},
			NodeParseState::Start => {},
		}
	}
	nodes
}

pub(crate) fn add_indentation(indentation: &str, text: &str) -> String {
	let mut indented_text: String = String::new();
	for line in text.lines() {
		indented_text = indented_text + indentation + line + "\n";
	}
	return indented_text;
}