use super::glm5::{ChatMsg, GLM5Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerInput {
    pub aa_sequences: Vec<AASequence>,
    pub architecture: String,
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AASequence {
    pub name: String,
    pub sequence: String,
    pub chain_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerOutput {
    pub constructs: Vec<CompiledConstruct>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledConstruct {
    pub name: String,
    pub dna_sequence: String,
    pub parts: Vec<CompiledPart>,
    pub signal_peptide: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledPart {
    pub name: String,
    pub part_type: String,
    pub sequence: String,
    pub description: String,
}

pub async fn run_compiler(client: &GLM5Client, input: &CompilerInput) -> Result<CompilerOutput, String> {
    let aa_desc: Vec<String> = input
        .aa_sequences
        .iter()
        .map(|aa| format!("- {} ({}): {}", aa.name, aa.chain_type, aa.sequence))
        .collect();

    let system_msg = ChatMsg {
        role: "system".to_string(),
        content: format!(
            "You are Plasmageddon Compiler, an expert molecular biology AI. Given amino acid sequences, architecture type, and host organism, you must:\n\
            1. Codon-optimize the DNA for the specified host\n\
            2. Suggest an appropriate signal peptide\n\
            3. Assemble a complete expression construct with promoter, signal peptide, CDS, and terminator\n\
            4. For multi-chain constructs, create separate constructs or use IRES/2A peptide linking\n\n\
            Respond ONLY with valid JSON matching this schema:\n\
            {{\n  \"constructs\": [{{\n    \"name\": \"string\",\n    \"dna_sequence\": \"full assembled DNA string\",\n    \"parts\": [{{\n      \"name\": \"string\",\n      \"part_type\": \"promoter|cds|terminator|signal_peptide|regulatory|tag|linker\",\n      \"sequence\": \"DNA string\",\n      \"description\": \"string\"\n    }}],\n    \"signal_peptide\": \"peptide name\"\n  }}],\n  \"notes\": \"string\"\n}}"
        ),
    };

    let user_msg = ChatMsg {
        role: "user".to_string(),
        content: format!(
            "Architecture: {}\nHost organism: {}\n\nAmino acid sequences:\n{}",
            input.architecture,
            input.host,
            aa_desc.join("\n")
        ),
    };

    let response = client.chat(vec![system_msg, user_msg], 0.3).await?;

    let cleaned = response
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    serde_json::from_str::<CompilerOutput>(cleaned)
        .map_err(|e| format!("Failed to parse compiler response: {e}\nRaw: {cleaned}"))
}
