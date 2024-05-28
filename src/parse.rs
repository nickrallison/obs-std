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

		// let yaml_pattern = Regex::new(r"(?m)^---[\s\S]*?---").unwrap();
		// let yaml_str = yaml_pattern.find(&contents).map(|m| m.as_str().to_string());
		let blocks = parse_into_blocks(contents);
		AST {
			blocks,
			// yaml_str,
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
		let mut file_contents: String = String::new();
		for block in &self.blocks {

			file_contents = file_contents + &format!("{}", block);

		}
		file_contents = file_contents.replace("\r\n", "\n");
		// trim the last newline, and only the last newline
		file_contents = file_contents.trim_end_matches("\n").to_string();
		match self.line_sep {
			LineSep::Unix => write!(f, "{}", file_contents),
			LineSep::Windows => write!(f, "{}", file_contents.replace("\n", "\r\n")),
		}

	}

}

impl Verbose for AST {
	fn verbose(&self) -> String {
		let mut verbose: String = String::new();
		for block in &self.blocks {

			verbose = verbose + &block.verbose();
		}
		verbose
	}

}

#[derive(Debug, Clone, PartialEq)]
enum LineSep {
	Unix,
	Windows,
}

#[derive(Debug, Clone, PartialEq)]
enum Block {
	YAML(serde_yaml::Value, String),
	CodeBlock(String),
	LatexBlock(String),
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
			Block::YAML(_, string) => write!(f, "{}\n", string),
			Block::CodeBlock(content) => write!(f, "\n```\n{}```\n", content),
			Block::LatexBlock(content) => write!(f, "$${}\n$$\n", content),
			Block::BlockQuote(blocks) => {
				let mut blocks_str: String = String::new();
				for block in blocks {
					blocks_str = blocks_str + &format!("{}", block);
				}
				// trim the last newline, and only the last newline
				blocks_str = blocks_str.trim_end_matches("\n").to_string();


				// split the string into lines
				let mut blocks_str_vec: Vec<&str> = blocks_str.split("\n").collect();

				// prepend > to each line
				let mut prepended_lines: Vec<String> = Vec::new();
				for line in &mut blocks_str_vec {
					// let line_tmp = line.to_string();
					// line = ("> ".to_string() + &line_tmp);
					prepended_lines.push(">".to_string() + line);
				}

				// join the lines back together
				let blocks_str = prepended_lines.join("\n");

				write!(f, "{}\n", blocks_str)
			},
			Block::TextBlock(lines) => {
				let mut lines_str: String = String::new();
				for line in lines {
					lines_str = lines_str + &format!("{}\n", line);
				}
				write!(f, "{}", lines_str)
			},
		}
	}
}

impl Verbose for Block {
	fn verbose(&self) -> String {
		match self {
			Block::YAML(_, string) => {
				format!("YAML:\n{}", add_indentation("\t", string))
			}
			Block::CodeBlock(code_block) => {
				format!("Code Block:\n{}", add_indentation("\t", code_block))
			}
			Block::LatexBlock(latex_block) => {
				format!("Latex Block:\n{}", add_indentation("\t", latex_block))
			}
			Block::BlockQuote(block_quote) => {
				let mut verbose: String = String::new();
				verbose = verbose + "Block Quote:\n";
				for block in block_quote {
					verbose = verbose + &add_indentation("\t", &(block.verbose()));
				}
				verbose
			}
			Block::TextBlock(text_block) => {
				let mut verbose: String = String::new();
				verbose = verbose + "Text Block:\n";
				for line in text_block {
					verbose = verbose + &add_indentation("\t", &line.verbose());
				}
				verbose
			}

		}

	}
}

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
}

fn parse_into_blocks(content: String) -> Vec<Block> {
	let mut state = BlockParseState::Start;
	let mut last_state = BlockParseState::Start;
	let mut text: String = String::new();
	let lines: Vec<&str> = content.lines().collect();
	let mut blocks: Vec<Block> = Vec::new();
	let mut index = 0;
	loop {
		let line = lines[index];
		let last_state_temp = state;
		match state {
			BlockParseState::Start => {
				if line.starts_with("---") {
					state = BlockParseState::YAML;
				}
				else if line.starts_with("```") {
					state = BlockParseState::CodeBlock;
				}
				else if line.starts_with("$$") {
					state = BlockParseState::LatexBlock;
				}
				else if line.starts_with(">") {
					state = BlockParseState::BlockQuote;
				}
				else {
					text = line.to_string();
					state = BlockParseState::TextBlock;
				}
			},
			BlockParseState::YAML => {
				if line.starts_with("---") {
					let string = "---".to_string() + &text + "\n---";
					blocks.push(Block::YAML(serde_yaml::from_str(&text).unwrap(), string));
					text = String::new();
					state = BlockParseState::TextBlock;
				}
				else {
					text = text + "\n" + line;
				}
			},
			BlockParseState::CodeBlock => {
				if line.starts_with("```") {
					blocks.push(Block::CodeBlock(text));
					text = String::new();
					state = BlockParseState::TextBlock;
				}
				else {
					text = text + "\n" + line;
				}
			},
			BlockParseState::LatexBlock => {
				if line.starts_with("$$") {
					blocks.push(Block::LatexBlock(text));
					text = String::new();
					state = BlockParseState::TextBlock;
				}
				else {
					text = text + "\n" + line;
				}
			},
			BlockParseState::BlockQuote => {
				// if line starts with > then it is still part of the block quote
				if index + 1 < lines.len() && line.starts_with(">") && lines[index + 1].starts_with(">") {
					text = text + "\n" + line.strip_prefix(">").unwrap();
				}

				// the next line does not start with > so this is the last line of the block quote
				else {
					text = text + "\n" + line.strip_prefix(">").unwrap();
					blocks.push(Block::BlockQuote(parse_into_blocks(text)));
					text = String::new();
				}
			},
			BlockParseState::TextBlock => {
				if line.starts_with("$$") {
					blocks.push(Block::TextBlock(parse_into_lines(text)));
					text = String::new();
					state = BlockParseState::LatexBlock;
				}
				else if line.starts_with("```") {
					blocks.push(Block::TextBlock(parse_into_lines(text)));
					text = String::new();
					state = BlockParseState::CodeBlock;
				}
				else if line.starts_with(">") {
					blocks.push(Block::TextBlock(parse_into_lines(text)));
					// text = line.to_string();
					text = String::new();
					text += line.strip_prefix(">").unwrap();
					state = BlockParseState::BlockQuote;
				}
				else {
					if state != last_state && last_state != BlockParseState::Start {
						text = line.to_string();
					} else {
						text = text + "\n" + line;
					}
				}
			},
		}
		last_state = last_state_temp;
		if index == lines.len() - 1  {
			match state {
				BlockParseState::Start => {
					// blocks.push(Block::TextBlock(parse_into_lines(text)));
				},
				BlockParseState::YAML => panic!("Block Parsing Failed! YAML block not closed"),
				BlockParseState::CodeBlock => panic!("Block Parsing Failed! Code block not closed"),
				BlockParseState::LatexBlock => panic!("Block Parsing Failed! Latex block not closed"),
				BlockParseState::BlockQuote => {},
				BlockParseState::TextBlock => {
					blocks.push(Block::TextBlock(parse_into_lines(text)));
				}
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
		BlockParseState::TextBlock => blocks
	}
}

fn parse_into_lines(content: String) -> Vec<Line> {
	// strip the first character if it is a newline
	// let content: String = if content.starts_with("\n") {
	// 	(&content[1..]).to_string()
	// } else {
	// 	content
	// };

	let lines_str: Vec<&str> = content.lines().collect();
	let mut lines: Vec<Line> = Vec::new();
	for line in lines_str {
		let heading_pattern = Regex::new(r"(?m)^(?P<level>#+) (?P<text>.*?)$").unwrap();
		let bullet_point_pattern = Regex::new(r"(?m)^(?P<indent>[ \r\t]*)?- (?P<text>.*?)$").unwrap();
		let list_item_pattern = Regex::new(r"(?m)^(?P<indent>[ \r\t]*)?(?P<number>\d+)\. (?P<text>.*?)$").unwrap();
		let linebar_pattern = Regex::new(r"(?m)^---\s*?$").unwrap();
		if heading_pattern.is_match(line) {
			let caps = heading_pattern.captures(line).unwrap();
			let level: u8 = (&caps["level"]).chars().count() as u8;
			let text: &str = &caps["text"];
			lines.push(Line::Heading(parse_into_nodes(text), level));
		}
		else if bullet_point_pattern.is_match(line) {
			let caps = bullet_point_pattern.captures(line).unwrap();
			let indentation: &str = &caps["indent"];
			let text: &str = &caps["text"];
			lines.push(Line::BulletPoint(parse_into_nodes(text), indentation.to_string()));
		}
		else if list_item_pattern.is_match(line) {
			let caps = list_item_pattern.captures(line).unwrap();
			let indentation = &caps["indent"];
			let number = (&caps["number"]).parse::<u32>().unwrap();
			let text = &caps["text"];
			lines.push(Line::ListItem(parse_into_nodes(text), number, indentation.to_string()));
		}
		else if linebar_pattern.is_match(line) {
			lines.push(Line::Linebar);
		}
		else {
			lines.push(Line::String(parse_into_nodes(line)));
		}
	}
	// if the final line is a newline, then append an empty string to the end of the lines
	if content.ends_with("\n") {
		lines.push(Line::String(vec![Node::String("".to_string())]));
	}

	return lines;
}


fn parse_into_nodes(content: &str) -> Vec<Node> {

	// let content = content.trim();
	let bold_italic_pattern = Regex::new(r"^\*\*\*(.*?)\*\*\*").unwrap();
	let bold_pattern = Regex::new(r"^\*\*(.*?)\*\*").unwrap();
	let italic_pattern = Regex::new(r"^\*(.*?)\*").unwrap();
	let inline_code_pattern = Regex::new(r"^`(.*?)`").unwrap();
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
		} else if inline_latex_pattern.is_match(&loop_string) {
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