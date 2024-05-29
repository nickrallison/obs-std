use std::cmp::PartialEq;
use std::fmt::Display;
use regex::Regex;
use crate::traits::Verbose;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AST {
	blocks: Vec<Block>,
	line_sep: LineSep
}

#[allow(dead_code)]
impl AST {

	pub(crate) fn new(contents: String) -> AST {

		let line_sep: LineSep = if (&contents).contains("\r\n") {
			LineSep::Windows
		} else {
			LineSep::Unix
		};
		let lines: Vec<&str> = contents.split("\n").collect();
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

fn parse_into_blocks(lines: Vec<&str>) -> Vec<Block> {
	// println!("Lines: {:?}", lines);
	// println!("Lines Len: {:?}", lines.len());
	if lines.len() == 0 {
		return vec![];
	}
	let inline_latex_regex = Regex::new(r"(?m)\$\$.*?\$\$").unwrap();
	let inline_code_regex = Regex::new(r"(?m)```.*?```").unwrap();
	let mut state = BlockParseState::Start;
	let mut current_lines: Vec<&str> = Vec::new();
	let mut blocks: Vec<Block> = Vec::new();
	let mut index = 0;
	loop {
		let line = lines[index];
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
					let text: String = current_lines.join("\n");
					let yaml_pattern = Regex::new(r"(?m)^---([\s\S]*?)---").unwrap();
					let caps = yaml_pattern.captures(&text).unwrap();
					let inner_yaml = caps.get(1).unwrap().as_str();
					let current_lines_string: Vec<String> = current_lines.iter().map(|x| x.to_string()).collect();
					blocks.push(Block::YAML(serde_yaml::from_str(inner_yaml).unwrap(), current_lines_string));
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
					let current_lines_string: Vec<String> = current_lines.iter().map(|x| x.to_string()).collect();
					blocks.push(Block::CodeBlock(current_lines_string));
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
					let current_lines_string: Vec<String> = current_lines.iter().map(|x| x.to_string()).collect();
					blocks.push(Block::LatexBlock(current_lines_string));
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
					blocks.push(Block::BlockQuote(parse_into_blocks(current_lines)));
					index -= 1;
					current_lines = Vec::new();
					state = BlockParseState::Unknown;
				}
			},
			BlockParseState::TextBlock => {
				if line.starts_with("$$") && !inline_latex_regex.is_match(line) {
					blocks.push(Block::TextBlock(parse_into_lines(current_lines)));
					current_lines = Vec::new();
					current_lines.push(line);
					state = BlockParseState::LatexBlock;
				}
				else if line.starts_with("```") && !inline_code_regex.is_match(line) {
					blocks.push(Block::TextBlock(parse_into_lines(current_lines)));
					current_lines = Vec::new();
					current_lines.push(line);
					state = BlockParseState::CodeBlock;
				}
				else if line.starts_with(">") {

					blocks.push(Block::TextBlock(parse_into_lines(current_lines)));

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
				BlockParseState::Start => {
					// blocks.push(Block::TextBlock(parse_into_lines(text)));
				},
				BlockParseState::YAML => panic!("Block Parsing Failed! YAML block not closed"),
				BlockParseState::CodeBlock => panic!("Block Parsing Failed! Code block not closed"),
				BlockParseState::LatexBlock => panic!("Block Parsing Failed! Latex block not closed"),
				BlockParseState::BlockQuote => {

					blocks.push(Block::BlockQuote(parse_into_blocks(current_lines)));

				},
				BlockParseState::TextBlock => {
					blocks.push(Block::TextBlock(parse_into_lines(current_lines)));
				},
				BlockParseState::Unknown => {}

			}
			break;
		} else {
			index += 1;
		}
	}
	return match state {
		BlockParseState::Start => blocks,
		BlockParseState::YAML => panic!("Block Parsing Failed! YAML block not closed"),
		BlockParseState::CodeBlock => panic!("Block Parsing Failed! Code block not closed"),
		BlockParseState::LatexBlock => panic!("Block Parsing Failed! Latex block not closed"),
		BlockParseState::BlockQuote => blocks,
		BlockParseState::TextBlock => blocks,
		BlockParseState::Unknown => blocks,
	}
}

fn parse_into_lines(lines: Vec<&str>) -> Vec<Line> {
	// strip the first character if it is a newline
	// let content: String = if content.starts_with("\n") {
	// 	(&content[1..]).to_string()
	// } else {
	// 	content
	// };
	let mut line_vec: Vec<Line> = Vec::new();
	for line in lines {
		let heading_pattern = Regex::new(r"(?m)^(?P<level>#+) (?P<text>.*?)$").unwrap();
		let bullet_point_pattern = Regex::new(r"(?m)^(?P<indent>[ \r\t]*)?- (?P<text>.*?)$").unwrap();
		let list_item_pattern = Regex::new(r"(?m)^(?P<indent>[ \r\t]*)?(?P<number>\d+)\. (?P<text>.*?)$").unwrap();
		let linebar_pattern = Regex::new(r"(?m)^---\s*?$").unwrap();
		if heading_pattern.is_match(line) {
			let caps = heading_pattern.captures(line).unwrap();
			let level: u8 = (&caps["level"]).chars().count() as u8;
			let text: &str = &caps["text"];
			line_vec.push(Line::Heading(parse_into_nodes(text), level));
		}
		else if bullet_point_pattern.is_match(line) {
			let caps = bullet_point_pattern.captures(line).unwrap();
			let indentation: &str = &caps["indent"];
			let text: &str = &caps["text"];
			line_vec.push(Line::BulletPoint(parse_into_nodes(text), indentation.to_string()));
		}
		else if list_item_pattern.is_match(line) {
			let caps = list_item_pattern.captures(line).unwrap();
			let indentation = &caps["indent"];
			let number = (&caps["number"]).parse::<u32>().unwrap();
			let text = &caps["text"];
			line_vec.push(Line::ListItem(parse_into_nodes(text), number, indentation.to_string()));
		}
		else if linebar_pattern.is_match(line) {
			line_vec.push(Line::Linebar);
		}
		else {
			line_vec.push(Line::String(parse_into_nodes(line)));
		}
	}
	// if the final line is a newline, then append an empty string to the end of the lines
	// if content.ends_with("\n") {
	// 	lines.push(Line::String(vec![Node::String("".to_string())]));
	// }

	return line_vec;
}


fn parse_into_nodes(content: &str) -> Vec<Node> {

	// let content = content.trim();
	let bold_italic_pattern = Regex::new(r"^\*\*\*(.*?)\*\*\*").unwrap();
	let bold_pattern = Regex::new(r"^\*\*(.*?)\*\*").unwrap();
	let italic_pattern = Regex::new(r"^\*(.*?)\*").unwrap();
	let inline_code_pattern = Regex::new(r"^`(.*?)`").unwrap();
	let inline_block_latex_pattern = Regex::new(r"^\$\$(.*?)\$\$").unwrap();
	let inline_latex_pattern = Regex::new(r"^\$(.*?)\$").unwrap();
	let formatted_markdown_link_pattern = Regex::new(r"^\[\[(.+?)\|(.+?)\]\]").unwrap();
	let markdown_link_pattern = Regex::new(r"^\[\[(.+?)\]\]").unwrap();
	let formatted_web_link_pattern = Regex::new(r"^\[(.+?)\]\((.+?)\)").unwrap();
	let web_link_pattern = Regex::new(r"^\[(.+?)\]").unwrap();

	let mut nodes: Vec<Node> = Vec::new();
	// let mut text: String = String::new();
	let mut start_index: usize = 0;
	let mut index: usize = 0;

	// if content[index:] matches any of the patterns add the match to nodes, and move index to the end of the match
	// else append the next character to text

	let characters: Vec<char> = content.chars().collect();


	loop {
		// if index >= characters.len() && start_index != index && start_index != 0 {
		// 	nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
		// 	return nodes;
		// } else if index >= characters.len()  && start_index == 0 {
		// 	nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
		// 	return nodes;
		// }
		// else if index >= characters.len() {
		// 	return nodes;
		// }
		if index >= characters.len()  {
			nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
			return nodes;
		}

		let loop_string: String = characters[index..].into_iter().collect();

		if inline_code_pattern.is_match(&loop_string) {
			if index != start_index {
				nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
			}
			let caps = inline_code_pattern.captures(&loop_string).unwrap();
			nodes.push(Node::InlineCode(caps.get(1).unwrap().as_str().to_string()));
			index += caps.get(0).unwrap().as_str().chars().count();
			start_index = index;
			// text = String::new();
		} else if inline_block_latex_pattern.is_match(&loop_string) {
			if index != start_index {
				nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
			}
			let caps = inline_block_latex_pattern.captures(&loop_string).unwrap();
			nodes.push(Node::InlineBlockLatex(caps.get(1).unwrap().as_str().to_string()));
			index += caps.get(0).unwrap().as_str().chars().count();
			start_index = index;
			// text = String::new();

		}
		else if inline_latex_pattern.is_match(&loop_string) {
			if index != start_index {
				nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
			}
			let caps = inline_latex_pattern.captures(&loop_string).unwrap();
			nodes.push(Node::InlineLatex(caps.get(1).unwrap().as_str().to_string()));
			index += caps.get(0).unwrap().as_str().chars().count();
			start_index = index;
			// text = String::new();
		} else if formatted_markdown_link_pattern.is_match(&loop_string) {
			if index != start_index {
				nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
			}
			let caps = formatted_markdown_link_pattern.captures(&loop_string).unwrap();
			nodes.push(Node::FormattedMarkdownLink(caps.get(1).unwrap().as_str().to_string(), caps.get(2).unwrap().as_str().to_string()));
			index += caps.get(0).unwrap().as_str().chars().count();
			start_index = index;
			// text = String::new();
		} else if markdown_link_pattern.is_match(&loop_string) {
			if index != start_index {
				nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
			}
			let caps = markdown_link_pattern.captures(&loop_string).unwrap();
			nodes.push(Node::MarkdownLink(caps.get(1).unwrap().as_str().to_string()));
			index += caps.get(0).unwrap().as_str().chars().count();
			start_index = index;
			// text = String::new();
		} else if formatted_web_link_pattern.is_match(&loop_string) {
			if index != start_index {
				nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
			}
			let caps = formatted_web_link_pattern.captures(&loop_string).unwrap();
			nodes.push(Node::FormattedWebLink(caps.get(2).unwrap().as_str().to_string(), caps.get(1).unwrap().as_str().to_string()));
			index += caps.get(0).unwrap().as_str().chars().count();
			start_index = index;

		} else if web_link_pattern.is_match(&loop_string) {
			if index != start_index {
				nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
			}
			let caps = web_link_pattern.captures(&loop_string).unwrap();
			nodes.push(Node::WebLink(caps.get(1).unwrap().as_str().to_string()));
			index += caps.get(0).unwrap().as_str().chars().count();
			start_index = index;
			// text = String::new();
		} else if bold_italic_pattern.is_match(&loop_string) {
			if index != start_index {
				nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
			}
			let caps = bold_italic_pattern.captures(&loop_string).unwrap();
			nodes.push(Node::BoldItalic(parse_into_nodes(&caps.get(1).unwrap().as_str().to_string())));
			index += caps.get(0).unwrap().as_str().chars().count();
			start_index = index;
			// text = String::new();
		} else if bold_pattern.is_match(&loop_string) {
			if index != start_index {
				nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
			}
			let caps = bold_pattern.captures(&loop_string).unwrap();
			nodes.push(Node::Bold(parse_into_nodes(&caps.get(1).unwrap().as_str().to_string())));
			index += caps.get(0).unwrap().as_str().chars().count();
			start_index = index;
			// text = String::new();
		} else if italic_pattern.is_match(&loop_string) {
			if index != start_index {
				nodes.push(Node::String(characters[start_index..index].into_iter().collect()));
			}
			let caps = italic_pattern.captures(&loop_string).unwrap();
			nodes.push(Node::Italic(parse_into_nodes(&caps.get(1).unwrap().as_str().to_string())));
			index += caps.get(0).unwrap().as_str().chars().count();
			start_index = index;
			// text = String::new();
		} else {
			index += 1;
		}
	}
}

pub(crate) fn add_indentation(indentation: &str, text: &str) -> String {
	let mut indented_text: String = String::new();
	for line in text.lines() {
		indented_text = indented_text + indentation + line + "\n";
	}
	return indented_text;
}