use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::process::Command;
use std::{env, io};
use regex::Regex;

trait LLM {
    fn name(&self) -> &str;
    fn context_window(&self) -> usize;
    fn estimate_token_num(&self, text: &str) -> usize;
    fn generate_code(
        &self,
        prompt: &str,
        response_dir: &str,
        num_samples: usize,
        temperature: f64,
    ) -> Result<(), io::Error>;
}

struct GPT {
    name: &'static str,
    max_tokens: usize,
    num_samples: usize,
    temperature: f64,
}

impl LLM for GPT {
    fn name(&self) -> &str {
        self.name
    }

    fn context_window(&self) -> usize {
        2000
    }

    fn estimate_token_num(&self, text: &str) -> usize {
        unimplemented!("token estimation not implemented for GPT");
    }

    fn generate_code(
        &self,
        prompt: &str,
        response_dir: &str,
        num_samples: usize,
        temperature: f64,
    ) -> Result<(), io::Error> {
        println!("Generating code with GPT model (simulated)");
        for i in 0..num_samples {
            let filename = format!("{}/{:02}.rawoutput", response_dir, i + 1);
            let mut file = File::create(filename)?;
            write!(file, "Sample output from GPT {}", i + 1)?;
        }
        Ok(())
    }
}

struct GPT4;

impl LLM for GPT4 {
    fn name(&self) -> &str {
        "gpt-4"
    }

    fn context_window(&self) -> usize {
        2000
    }

}

struct GoogleModel {
    name: &'static str,
    max_tokens: usize,
    num_samples: usize,
    temperature: f64,
    ai_binary: Option<String>,
}

impl LLM for GoogleModel {
    fn name(&self) -> &str {
        self.name
    }

    fn context_window(&self) -> usize {
        // Placeholder value, adjust based on the specific model
        1500
    }

    fn estimate_token_num(&self, text: &str) -> usize {
        // Roughly 1.5 tokens per word
        (text.split(|c| !c.is_alphanumeric()).count() * 3 / 2) as usize
    }

    fn generate_code(
        &self,
        prompt: &str,
        response_dir: &str,
        num_samples: usize,
        temperature: f64,
    ) -> Result<(), io::Error> {
        if let Some(binary) = &self.ai_binary {
            let mut command = Command::new(binary);
            command
                .arg(format!("-model={}", self.name))
                .arg(format!("-prompt={}", prompt))
                .arg(format!("-response={}", response_dir))
                .arg(format!("-max-tokens={}", self.max_tokens))
                .arg(format!("-expected-samples={}", num_samples))
                .arg(format!("-temperature={}", temperature))
                .spawn()
                .expect("failed to spawn process");
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "AI binary not specified"))
        }
    }
}

enum VertexAIModel {
    CodeBison,
    CodeBison32k,
    GeminiPro,
}

impl LLM for VertexAIModel {
    fn name(&self) -> &str {
        match self {
            VertexAIModel::CodeBison => "vertex_ai_code-bison",
            VertexAIModel::CodeBison32k => "vertex_
