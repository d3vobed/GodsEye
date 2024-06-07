use std::fs;

// Base Prompt trait
trait Prompt {
    fn get(&self) -> String;
    fn add_priming(&mut self, content: &str);
    fn add_problem(&mut self, content: &str);
    fn add_solution(&mut self, content: &str);
    fn save(&self, location: &str) -> Result<(), std::io::Error>;
}

// TextPrompt implementation (similar to TextPrompt in Python)
struct TextPrompt {
    text: String,
}

impl Prompt for TextPrompt {
    fn get(&self) -> String {
        self.text.clone()
    }

    fn add_priming(&mut self, content: &str) {
        self.text.push_str(format!("{}\n", content).as_str());
    }

    fn add_problem(&mut self, content: &str) {
        self.text.push_str(format!("{}\n", content).as_str());
    }

    fn add_solution(&mut self, content: &str) {
        self.text.push_str(format!("{}\n", content).as_str());
    }

    fn save(&self, location: &str) -> Result<(), std::io::Error> {
        fs::write(location, self.text.as_bytes())
    }
}

// OpenAIPrompt implementation (similar to OpenAIPrompt in Python)
struct OpenAIPrompt {
    prompts: Vec<PromptPiece>,
}

struct PromptPiece {
    role: String,
    content: String,
}

impl Prompt for OpenAIPrompt {
    fn get(&self) -> String {
        let mut prompt_text = String::new();
        for piece in &self.prompts {
            prompt_text.push_str(format!("{{ \"role\": \"{}\", \"content\": \"{}\" }},\n", piece.role, piece.content).as_str());
        }
        prompt_text.pop(); // Remove trailing comma
        prompt_text
    }

    fn add_priming(&mut self, content: &str) {
        self.prompts.push(PromptPiece { role: "system".to_string(), content: content.to_string() });
    }

    fn add_problem(&mut self, content: &str) {
        self.prompts.push(PromptPiece { role: "user".to_string(), content: content.to_string() });
    }

    fn add_solution(&mut self, content: &str) {
        self.prompts.push(PromptPiece { role: "assistant".to_string(), content: content.to_string() });
    }

    fn save(&self, location: &str) -> Result<(), std::io::Error> {
        let json_data = serde_json::to_string(&self.prompts)?;
        fs::write(location, json_data.as_bytes())
    }
}

// Usage example
fn main() {
    let mut prompt = TextPrompt::new();
    prompt.add_priming("Once upon a time...");
    prompt.add_problem("The hero faced a great challenge.");
    prompt.add_solution("With courage and wit, the hero prevailed.");
    prompt.save("text_prompt.txt").unwrap();

    let mut openai_prompt = OpenAIPrompt::new();
    openai_prompt.add_priming("It's a dark and stormy night...");
    openai_prompt.add_problem("The detective needs to solve a mystery.");
    openai_prompt.add_solution("By following the clues, the detective finds the culprit.");
    openai_prompt.save("openai_prompt.json").unwrap();
}
