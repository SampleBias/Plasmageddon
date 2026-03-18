use tauri::State;

use crate::biosecurity;
use crate::db::AppDatabase;
use crate::parsers::fasta;
use crate::parsers::genbank;
use crate::parsers::snapgene;

#[derive(serde::Serialize)]
pub struct ImportResult {
    pub construct_id: String,
    pub name: String,
    pub sequence_length: usize,
    pub parts_found: usize,
}

#[tauri::command]
pub fn import_file(db: State<'_, AppDatabase>, path: &str, repo_id: &str) -> Result<ImportResult, String> {
    let content = std::fs::read(path).map_err(|e| format!("Failed to read file: {e}"))?;
    let path_lower = path.to_lowercase();

    if path_lower.ends_with(".gb") || path_lower.ends_with(".gbk") || path_lower.ends_with(".genbank") {
        let text = String::from_utf8(content).map_err(|e| format!("Invalid UTF-8: {e}"))?;
        import_genbank(db, &text, repo_id)
    } else if path_lower.ends_with(".fa") || path_lower.ends_with(".fasta") || path_lower.ends_with(".fna") {
        let text = String::from_utf8(content).map_err(|e| format!("Invalid UTF-8: {e}"))?;
        import_fasta(db, &text, repo_id)
    } else if path_lower.ends_with(".dna") {
        import_snapgene(db, &content, repo_id)
    } else {
        Err("Unsupported file format. Use .gb, .fasta, or .dna".into())
    }
}

fn import_genbank(db: State<'_, AppDatabase>, content: &str, repo_id: &str) -> Result<ImportResult, String> {
    let record = genbank::parse_genbank(content)?;
    let construct = db.create_construct(repo_id, &record.name, &record.description, &record.topology)?;
    let tags: Vec<String> = vec![];
    db.update_construct(&construct.id, &record.name, &record.description, &record.topology, &tags, &record.sequence)?;

    let mut parts_count = 0;
    for feat in &record.features {
        if feat.feature_type == "source" {
            continue;
        }
        let part_type = map_feature_type(&feat.feature_type);
        let label = if feat.label.is_empty() {
            feat.feature_type.clone()
        } else {
            feat.label.clone()
        };
        let seq_slice = if feat.end <= record.sequence.len() && feat.start < feat.end {
            &record.sequence[feat.start..feat.end]
        } else {
            ""
        };

        let part = db.create_part(&label, &part_type, seq_slice, &feat.feature_type)?;
        db.add_construct_part(&construct.id, &part.id, feat.start as i64, feat.strand, parts_count as i64)?;
        parts_count += 1;
    }

    Ok(ImportResult {
        construct_id: construct.id,
        name: record.name,
        sequence_length: record.sequence.len(),
        parts_found: parts_count,
    })
}

fn import_fasta(db: State<'_, AppDatabase>, content: &str, repo_id: &str) -> Result<ImportResult, String> {
    let records = fasta::parse_fasta(content)?;
    let rec = records.first().ok_or("No records found")?;
    let construct = db.create_construct(repo_id, &rec.name, &rec.description, "linear")?;
    let tags: Vec<String> = vec![];
    db.update_construct(&construct.id, &rec.name, &rec.description, "linear", &tags, &rec.sequence)?;

    Ok(ImportResult {
        construct_id: construct.id,
        name: rec.name.clone(),
        sequence_length: rec.sequence.len(),
        parts_found: 0,
    })
}

fn import_snapgene(db: State<'_, AppDatabase>, data: &[u8], repo_id: &str) -> Result<ImportResult, String> {
    let record = snapgene::parse_snapgene(data)?;
    let name = if record.name.is_empty() { "Imported .dna".to_string() } else { record.name.clone() };
    let construct = db.create_construct(repo_id, &name, "", &record.topology)?;
    let tags: Vec<String> = vec![];
    db.update_construct(&construct.id, &name, "", &record.topology, &tags, &record.sequence)?;

    let mut parts_count = 0;
    for feat in &record.features {
        let part_type = map_feature_type(&feat.feature_type);
        let label = if feat.label.is_empty() { feat.feature_type.clone() } else { feat.label.clone() };
        let seq_slice = if feat.end <= record.sequence.len() && feat.start < feat.end {
            &record.sequence[feat.start..feat.end]
        } else {
            ""
        };

        let part = db.create_part(&label, &part_type, seq_slice, &feat.feature_type)?;
        db.add_construct_part(&construct.id, &part.id, feat.start as i64, feat.strand, parts_count as i64)?;
        parts_count += 1;
    }

    Ok(ImportResult {
        construct_id: construct.id,
        name,
        sequence_length: record.sequence.len(),
        parts_found: parts_count,
    })
}

fn map_feature_type(gb_type: &str) -> String {
    match gb_type.to_lowercase().as_str() {
        "promoter" => "promoter",
        "cds" | "gene" => "cds",
        "terminator" => "terminator",
        "rep_origin" | "origin" => "ori",
        "resistance" | "selectable_marker" => "marker",
        _ => "other",
    }
    .to_string()
}

#[tauri::command]
pub fn export_genbank(db: State<'_, AppDatabase>, construct_id: &str, path: &str) -> Result<(), String> {
    let construct = db.get_construct(construct_id)?;
    let parts = db.get_construct_parts(construct_id)?;

    let mut features = Vec::new();
    for cp in &parts {
        if let Ok(part) = db.get_part(&cp.part_id) {
            features.push(genbank::GenBankFeature {
                feature_type: map_part_type_to_gb(&part.part_type),
                start: cp.position as usize,
                end: cp.position as usize + part.sequence.len(),
                strand: cp.strand,
                qualifiers: vec![
                    ("label".to_string(), part.name.clone()),
                    ("note".to_string(), part.description.clone()),
                ],
                label: part.name,
            });
        }
    }

    let record = genbank::GenBankRecord {
        name: construct.name,
        description: construct.description,
        topology: construct.topology,
        length: construct.sequence.len(),
        sequence: construct.sequence,
        features,
    };

    let output = genbank::export_genbank(&record);
    std::fs::write(path, output).map_err(|e| format!("Write error: {e}"))
}

#[tauri::command]
pub fn export_fasta(db: State<'_, AppDatabase>, construct_id: &str, path: &str) -> Result<(), String> {
    let construct = db.get_construct(construct_id)?;
    let output = fasta::export_fasta(&construct.name, &construct.description, &construct.sequence);
    std::fs::write(path, output).map_err(|e| format!("Write error: {e}"))
}

#[tauri::command]
pub fn export_csv(db: State<'_, AppDatabase>, construct_id: &str, path: &str) -> Result<(), String> {
    let parts = db.get_construct_parts(construct_id)?;
    let mut csv = String::from("name,type,position,strand,sequence_length\n");

    for cp in &parts {
        if let Ok(part) = db.get_part(&cp.part_id) {
            csv.push_str(&format!(
                "{},{},{},{},{}\n",
                part.name, part.part_type, cp.position, cp.strand, part.sequence.len()
            ));
        }
    }

    std::fs::write(path, csv).map_err(|e| format!("Write error: {e}"))
}

fn map_part_type_to_gb(part_type: &str) -> String {
    match part_type {
        "promoter" => "promoter",
        "cds" => "CDS",
        "terminator" => "terminator",
        "ori" => "rep_origin",
        "marker" => "CDS",
        _ => "misc_feature",
    }
    .to_string()
}

#[tauri::command]
pub async fn screen_sequence(sequence: String) -> Result<biosecurity::BiosecurityResult, String> {
    biosecurity::screen_sequence(&sequence).await
}
