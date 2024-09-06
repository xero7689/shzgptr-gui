#[derive(Debug)]
pub enum BlockType {
    Text,
    Code(String), // Language of the code block, e.g., "rust"
    Heading(i32), // Level of the heading, e.g., 1 for h1, 2 for h2, etc.
}

#[derive(Debug)]
pub struct Block {
    pub block_type: BlockType,
    pub content: String,
}

pub fn parse_markdown(input: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut is_code_block = false;
    let mut code_lang = String::new();
    let mut current_block = String::new();

    for line in input.lines() {
        if line.trim().starts_with("```") {
            // Toggle code block
            if is_code_block {
                // End of code block
                blocks.push(Block {
                    block_type: BlockType::Code(code_lang.clone()),
                    content: current_block.trim().to_string(),
                });
                is_code_block = false;
                current_block = String::new();
                code_lang = String::new();
            } else {
                // Start of code block
                is_code_block = true;
                code_lang = line.trim().trim_start_matches("```").to_string();
            }
        } else if line.starts_with('#') {
            // Heading
            let level = line.chars().take_while(|c| *c == '#').count() as i32;
            blocks.push(Block {
                block_type: BlockType::Heading(level),
                content: line.trim().trim_start_matches('#').trim().to_string(),
            });
        } else if is_code_block {
            current_block.push_str(line);
            current_block.push('\n');
        } else {
            if line.is_empty() && !current_block.is_empty() {
                // Empty line signals the end of a text block
                blocks.push(Block {
                    block_type: BlockType::Text,
                    content: current_block.trim().to_string(),
                });
                current_block = String::new();
            } else {
                current_block.push_str(line);
                current_block.push('\n');
            }
        }
    }

    // Push any remaining text block
    if !current_block.is_empty() {
        blocks.push(Block {
            block_type: if is_code_block {
                BlockType::Code(code_lang)
            } else {
                BlockType::Text
            },
            content: current_block.trim().to_string(),
        });
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown() {
        let input = r#"
        Here is a python hello world program:

        ```python
        print("Hello, World!")
        ```

        Happy coding!
        "#;

        let blocks = parse_markdown(input);

        // There should be 3 blocks
        assert_eq!(blocks.len(), 3);

        // The 2nd block should be a code block with language "python"
        assert!(matches!(&blocks[1].block_type, BlockType::Code(lang) if lang == "python"));
    }
}
