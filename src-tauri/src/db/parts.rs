use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::AppDatabase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {
    pub id: String,
    pub name: String,
    pub part_type: String,
    pub sequence: String,
    pub description: String,
    pub metadata: serde_json::Value,
    pub created_at: String,
}

impl AppDatabase {
    pub fn create_part(
        &self,
        name: &str,
        part_type: &str,
        sequence: &str,
        description: &str,
    ) -> Result<Part, String> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO parts (id, name, part_type, sequence, description) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, name, part_type, sequence, description],
        )
        .map_err(|e| e.to_string())?;
        drop(conn);
        self.get_part(&id)
    }

    pub fn get_part(&self, id: &str) -> Result<Part, String> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT id, name, part_type, sequence, description, metadata, created_at FROM parts WHERE id = ?1",
            params![id],
            |row| {
                let meta_str: String = row.get(5)?;
                Ok(Part {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    part_type: row.get(2)?,
                    sequence: row.get(3)?,
                    description: row.get(4)?,
                    metadata: serde_json::from_str(&meta_str).unwrap_or(serde_json::Value::Object(Default::default())),
                    created_at: row.get(6)?,
                })
            },
        )
        .map_err(|e| e.to_string())
    }

    pub fn list_parts(&self, part_type: Option<&str>) -> Result<Vec<Part>, String> {
        let conn = self.conn.lock().unwrap();
        let (sql, bind): (String, Vec<Box<dyn rusqlite::types::ToSql>>) = if let Some(pt) = part_type {
            (
                "SELECT id, name, part_type, sequence, description, metadata, created_at FROM parts WHERE part_type = ?1 ORDER BY name".into(),
                vec![Box::new(pt.to_string())],
            )
        } else {
            (
                "SELECT id, name, part_type, sequence, description, metadata, created_at FROM parts ORDER BY part_type, name".into(),
                vec![],
            )
        };

        let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
        let params_slice: Vec<&dyn rusqlite::types::ToSql> = bind.iter().map(|b| b.as_ref()).collect();
        let items = stmt
            .query_map(params_slice.as_slice(), |row| {
                let meta_str: String = row.get(5)?;
                Ok(Part {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    part_type: row.get(2)?,
                    sequence: row.get(3)?,
                    description: row.get(4)?,
                    metadata: serde_json::from_str(&meta_str)
                        .unwrap_or(serde_json::Value::Object(Default::default())),
                    created_at: row.get(6)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(items)
    }

    pub fn search_parts(&self, query: &str) -> Result<Vec<Part>, String> {
        let conn = self.conn.lock().unwrap();
        let pattern = format!("%{query}%");
        let mut stmt = conn
            .prepare(
                "SELECT id, name, part_type, sequence, description, metadata, created_at FROM parts WHERE name LIKE ?1 OR description LIKE ?1 OR part_type LIKE ?1 ORDER BY name LIMIT 50",
            )
            .map_err(|e| e.to_string())?;

        let items = stmt
            .query_map(params![pattern], |row| {
                let meta_str: String = row.get(5)?;
                Ok(Part {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    part_type: row.get(2)?,
                    sequence: row.get(3)?,
                    description: row.get(4)?,
                    metadata: serde_json::from_str(&meta_str)
                        .unwrap_or(serde_json::Value::Object(Default::default())),
                    created_at: row.get(6)?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        Ok(items)
    }

    pub fn delete_part(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM parts WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn seed_default_parts(&self) -> Result<(), String> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM parts", [], |r| r.get(0))
            .map_err(|e| e.to_string())?;
        if count > 0 {
            return Ok(());
        }
        drop(conn);

        let defaults: Vec<(&str, &str, &str, &str)> = vec![
            // Promoters
            ("CMV Promoter", "promoter", "GACATTGATTATTGACTAGTTATTAATAGTAATCAATTACGGGGTCATTAGTTCATAGCCCATATATGGAGTTCCGCGTTACATAACTTACGGTAAATGGCCCGCCTGGCTGACCGCCCAACGACCCCCGCCCATTGACGTCAATAATGACGTATGTTCCCATAGTAACGCCAATAGGGACTTTCCATTGACGTCAATGGGTGGAGTATTTACGGTAAACTGCCCACTTGGCAGTACATCAAGTGTATCATATGCCAAGTACGCCCCCTATTGACGTCAATGACGGTAAATGGCCCGCCTGGCATTATGCCCAGTACATGACCTTATGGGACTTTCCTACTTGGCAGTACATCTACGTATTAGTCATCGCTATTACCATGGTGATGCGGTTTTGGCAGTACATCAATGGGCGTGGATAGCGGTTTGACTCACGGGGATTTCCAAGTCTCCACCCCATTGACGTCAATGGGAGTTTGTTTTGGCACCAAAATCAACGGGACTTTCCAAAATGTCGTAACAACTCCGCCCCATTGACGCAAATGGGCGGTAGGCGTGTACGGTGGGAGGTCTATATAAGCAGAGCT", "Human cytomegalovirus immediate-early promoter"),
            ("T7 Promoter", "promoter", "TAATACGACTCACTATAG", "Bacteriophage T7 RNA polymerase promoter"),
            ("lac Promoter", "promoter", "TTTACACTTTATGCTTCCGGCTCGTATGTTGTGTGGAATTGTGAGCGGATAACAATTTCACACAGG", "E. coli lac operon promoter"),
            ("EF1a Promoter", "promoter", "CGTGAGGCTCCGGTGCCCGTCAGTGGGCAGAGCGCACATCGCCCACAGTCCCCGAGAAGTTGGGGGGAGGGGTCGGCAATTGAACCGGTGCCTAGAGAAGGTGGCGCGGGGTAAACTGGGAAAGTGATGTCGTGTACTGGCTCCGCCTTTTTCCCGAGGGTGGGGGAGAACCGTATATAAGTGCAGTAGTCGCCGTGAACGTTCTTTTTCGCAACGGGTTTGCCGCCAGAACACAG", "Human elongation factor 1-alpha promoter"),
            ("SV40 Promoter", "promoter", "ATGCTTTGCATACTTCTGCCTGCTGGGGAGCCTGGGGACTTTCCACACCTGGTTGCTGACTAATTGAGATGCATGCTTTGCATACTTCTGCCTGCTGGGGAGCCTGGGGACTTTCCACAC", "Simian virus 40 early promoter"),
            ("trc Promoter", "promoter", "TTGACAATTAATCATCCGGCTCGTATAATGTGTGG", "Hybrid trp-lac promoter for E. coli"),
            ("CAG Promoter", "promoter", "GACATTGATTATTGACTAGTTATTAATAGTAATCAATTACGGGGTCATTAGTTCATAGCCCATATATGGAGTTCCGCGTTACATAACTTACGGTAAATGGCCCGCCTGGCTGACCGCCCAACGACCCCCGCCCATTGACGTCAATAATGACGTATGTTCCCATAGTAACGCCAATAGGGACTTTCCATTGACGTCAATGGGTGGAGTATTTACGGTAAACTGCCCACTTGGCAGTACATCAAGTGTATCATATGCCAAGTACGCCCCCTATTGACGTCAATGACGGTAAATGGCCCGCCTGGCATT", "CMV early enhancer/chicken beta-actin promoter"),
            // Terminators
            ("bGH polyA", "terminator", "CTGTGCCTTCTAGTTGCCAGCCATCTGTTGTTTGCCCCTCCCCCGTGCCTTCCTTGACCCTGGAAGGTGCCACTCCCACTGTCCTTTCCTAATAAAATGAGGAAATTGCATCGCATTGTCTGAGTAGGTGTCATTCTATTCTGGGGGGTGGGGTGGGGCAGGACAGCAAGGGGGAGGATTGGGAAGACAATAGCAGGCATGCTGGGGATGCGGTGGGCTCTATGG", "Bovine growth hormone polyadenylation signal"),
            ("SV40 polyA", "terminator", "AACTTGTTTATTGCAGCTTATAATGGTTACAAATAAAGCAATAGCATCACAAATTTCACAAATAAAGCATTTTTTTCACTGCATTCTAGTTGTGGTTTGTCCAAACTCATCAATGTATCTTA", "SV40 late polyadenylation signal"),
            ("T7 Terminator", "terminator", "CTAGCATAACCCCTTGGGGCCTCTAAACGGGTCTTGAGGGGTTTTTTG", "T7 transcription terminator"),
            ("rrnB T1", "terminator", "CAAATAAAACGAAAGGCTCAGTCGAAAGACTGGGCCTTTCGTTTTATCTGTTGTTTGTCGGTGAACGCTCTCCTGAGTAGGACAAATCCGCCGGGAGCGGATTTGAACGTTGCGAAGCAACGGCCCGGAGGGTGGCGGGCAGGACGCCCGCCATAAACTGCCAGGCATCAAATTAAGCAGAAGGCCATCCTGACGGATGGCCTTTTTGCGTTTCTAC", "E. coli rrnB T1 terminator"),
            // Origins of replication
            ("ColE1 ori", "ori", "TGGTATCTTTATAGTCCTGTCGGGTTTCGCCACCTCTGACTTGAGCGTCGATTTTTGTGATGCTCGTCAGGGGGGCGGAGCCTATGGAAAAACGCCAGCAACGCGGCCTTTTTACGGTTCCTGGCCTTTTGCTGGCCTTTTGCTCACATGTTCTTTCCTGCGTTATCCCCTGATTCTGTGGATAACCGTATTACCGCCTTTGAGTGAGCTGATACCGCTCGCCGCAGCCGAACGACCGAGCGCAGCGAGTCAGTGAGCGAGGAAGCGGAAG", "ColE1/pMB1/pBR322 high-copy origin"),
            ("p15A ori", "ori", "GGTAACTATCGTCTTGAGTCCAACCCGGTAAGACACGACTTATCGCCACTGGCAGCAGCCACTGGTAACAGGATTAGCAGAGCGAGGTATGTAGGCGGTGCTACAGAGTTCTTGAAGTGGTGGCCTAACTACGGCTACACTAGAAGGACAGTATTTGGTATCTGCGCTCTGCTGAAGCCAGTTACCTTCGGAAAAAGAGTTGGTAGCTCTTGATCCGGCAAACAAACCACCGCTGGTAGCGGTGGTTTTTTTGTTTGCAAGCAGCAGATTACGCGCAG", "p15A medium-copy origin"),
            ("pUC ori", "ori", "TGGTATCTTTATAGTCCTGTCGGGTTTCGCCACCTCTGACTTGAGCGTCGATTTTTGTGATGCTCGTCAGGGGGGCGGAGCCTATGGAAAAACGCCAGCAACGCGGCCTTTTTACGGTTCCTGGCCTTTTGCTGGCCTTTTGCTCACATG", "pUC high-copy origin (mutant ColE1)"),
            // Selectable markers
            ("AmpR", "marker", "ATGAGTATTCAACATTTCCGTGTCGCCCTTATTCCCTTTTTTGCGGCATTTTGCCTTCCTGTTTTTGCTCACCCAGAAACGCTGGTGAAAGTAAAAGATGCTGAAGATCAGTTGGGTGCACGAGTGGGTTACATCGAACTGGATCTCAACAGCGGTAAGATCCTTGAGAGTTTTCGCCCCGAAGAACGTTTTCCAATGATGAGCACTTTTAAAGTTCTGCTATGTGGCGCGGTATTATCCCGTATTGACGCCGGGCAAGAGCAACTCGGTCGCCGCATACACTATTCTCAGAATGACTTGGTTGAGTACTCACCAGTCACAGAAAAGCATCTTACGGATGGCATGACAGTAAGAGAATTATGCAGTGCTGCCATAACCATGAGTGATAACACTGCGGCCAACTTACTTCTGACAACGATCGGAGGACCGAAGGAGCTAACCGCTTTTTTGCACAACATGGGGGATCATGTAACTCGCCTTGATCGTTGGGAACCGGAGCTGAATGAAGCCATACCAAACGACGAGCGTGACACCACGATGCCTGTAGCAATGGCAACAACGTTGCGCAAACTATTAACTGGCGAACTACTTACTCTAGCTTCCCGGCAACAATTAATAGACTGGATGGAGGCGGATAAAGTTGCAGGACCACTTCTGCGCTCGGCCCTTCCGGCTGGCTGGTTTATTGCTGATAAATCTGGAGCCGGTGAGCGTGGGTCTCGCGGTATCATTGCAGCACTGGGGCCAGATGGTAAGCCCTCCCGTATCGTAGTTATCTACACGACGGGGAGTCAGGCAACTATGGATGAACGAAATAGACAGATCGCTGAGATAGGTGCCTCACTGATTAAGCATTGGTAA", "Ampicillin resistance (beta-lactamase)"),
            ("KanR", "marker", "ATGATTGAACAAGATGGATTGCACGCAGGTTCTCCGGCCGCTTGGGTGGAGAGGCTATTCGGCTATGACTGGGCACAACAGACAATCGGCTGCTCTGATGCCGCCGTGTTCCGGCTGTCAGCGCAGGGGCGCCCGGTTCTTTTTGTCAAGACCGACCTGTCCGGTGCCCTGAATGAACTGCAGGACGAGGCAGCGCGGCTATCGTGGCTGGCCACGACGGGCGTTCCTTGCGCAGCTGTGCTCGACGTTGTCACTGAAGCGGGAAGGGACTGGCTGCTATTGGGCGAAGTGCCGGGGCAGGATCTCCTGTCATCTCACCTTGCTCCTGCCGAGAAAGTATCCATCATGGCTGATGCAATGCGGCGGCTGCATACGCTTGATCCGGCTACCTGCCCATTCGACCACCAAGCGAAACATCGCATCGAGCGAGCACGTACTCGGATGGAAGCCGGTCTTGTCGATCAGGATGATCTGGACGAAGAGCATCAGGGGCTCGCGCCAGCCGAACTGTTCGCCAGGCTCAAGGCGCGCATGCCCGACGGCGATGATCTCGTCGTGACCCATGGCGATGCCTGCTTGCCGAATATCATGGTGGAAAATGGCCGCTTTTCTGGATTCATCGACTGTGGCCGGCTGGGTGTGGCGGACCGCTATCAGGACATAGCGTTGGCTACCCGTGATATTGCTGAAGAGCTTGGCGGCGAATGGGCTGACCGCTTCCTCGTGCTTTACGGTATCGCCGCTCCCGATTCGCAGCGCATCGCCTTCTATCGCCTTCTTGACGAGTTCTTCTGA", "Kanamycin resistance (neomycin phosphotransferase II)"),
            ("CmR", "marker", "ATGGAGAAAAAAATCACTGGATATACCACCGTTGATATATCCCAATGGCATCGTAAAGAACATTTTGAGGCATTTCAGTCAGTTGCTCAATGTACCTATAACCAGACCGTTCAGCTGGATATTACGGCCTTTTTAAAGACCGTAAAGAAAAATAAGCACAAGTTTTATCCGGCCTTTATTCACATTCTTGCCCGCCTGATGAATGCTCATCCGGAATTTCGTATGGCAATGAAAGACGGTGAGCTGGTGATATGGGATAGTGTTCACCCTTGTTACACCGTTTTCCATGAGCAAACTGAAACGTTTTCATCGCTCTGGAGTGAATACCACGACGATTTCCGGCAGTTTCTACACATATATTCGCAAGATGTGGCGTGTTACGGTGAAAACCTGGCCTATTTCCCTAAAGGGTTTATTGAGAATATGTTTTTCGTCTCAGCCAATCCCTGGGTGAGTTTCACCAGTTTTGATTTAAACGTGGCCAATATGGACAACTTCTTCGCCCCCGTTTTCACCATGGGCAAATATTATACGCAAGGCGACAAGGTGCTGATGCCGCTGGCGATTCAGGTTCATCATGCCGTTTGTGATGGCTTCCATGTCGGCAGAATGCTTAATGAATTACAACAGTACTGCGATGAGTGGCAGGGCGGGGCGTAA", "Chloramphenicol acetyltransferase"),
            // Tags
            ("6xHis Tag", "tag", "CATCACCATCACCATCAC", "Hexahistidine purification tag"),
            ("FLAG Tag", "tag", "GACTACAAAGACGATGACGACAAG", "FLAG epitope tag (DYKDDDDK)"),
            ("Myc Tag", "tag", "GAACAAAAACTCATCTCAGAAGAGGATCTG", "c-Myc epitope tag"),
            ("HA Tag", "tag", "TACCCATACGATGTTCCAGATTACGCT", "Hemagglutinin epitope tag"),
            ("V5 Tag", "tag", "GGTAAGCCTATCCCTAACCCTCTCCTCGGTCTCGATTCTACG", "V5 epitope tag"),
            // Linkers
            ("GS Linker (G4S)x3", "linker", "GGTGGCGGTGGCTCGGGCGGTGGTGGGTCGGGTGGCGGCGGATCG", "(GGGGS)x3 flexible linker"),
            ("Rigid Linker (EAAAK)x3", "linker", "GAAGCTGCTGCTAAAGAAGCTGCTGCTAAAGAAGCTGCTGCTAAA", "(EAAAK)x3 rigid alpha-helical linker"),
            // Signal peptides
            ("IgK Leader", "signal_peptide", "ATGGAGACAGACACACTCCTGCTATGGGTACTGCTGCTCTGGGTTCCAGGTTCCACTGGTGAC", "Mouse Ig kappa signal peptide"),
            ("IL2 Signal", "signal_peptide", "ATGTACAGGATGCAACTCCTGTCTTGCATTGCACTAAGTCTTGCACTTGTCACAAACAGT", "Human IL-2 signal peptide"),
            ("CD33 Signal", "signal_peptide", "ATGCCGCTGCTGCTACTGCTGCCCCTGCTGTGGGCAGGGGCCCTGGCT", "Human CD33 signal peptide"),
            // Regulatory
            ("Kozak-ATG", "regulatory", "GCCACCATG", "Kozak consensus with start codon"),
            ("IRES", "regulatory", "GCCCCTCTCCCTCCCCCCCCCCTAACGTTACTGGCCGAAGCCGCTTGGAATAAGGCCGGTGTGCGTTTGTCTATATGTTATTTTCCACCATATTGCCGTCTTTTGGCAATGTGAGGGCCCGGAAACCTGGCCCTGTCTTCTTGACGAGCATTCCTAGGGGTCTTTCCCCTCTCGCCAAAGGAATGCAAGGTCTGTTGAATGTCGTGAAGGAAGCAGTTCCTCTGGAAGCTTCTTGAAGACAAACAACGTCTGTAGCGACCCTTTGCAGGCAGCGGAACCCCCCACCTGGCGACAGGTGCCTCTGCGGCCAAAAGCCACGTGTATAAGATACACCTGCAAAGGCGGCACAACCCCAGTGCCACGTTGTGAGTTGGATAGTTGTGGAAAGAGTCAAATGGCTCTCCTCAAGCGTATTCAACAAGGGGCTGAAGGATGCCCAGAAGGTACCCCATTGTATGGGATCTGATCTGGGGCCTCGGTGCACATGCTTTACATGTGTTTAGTCGAGGTTAAAAAACGTCTAGGCCCCCCGAACCACGGGGACGTGGTTTTCCTTTGAAAAACACGAT", "Internal ribosome entry site (ECMV)"),
            ("Woodchuck WPRE", "regulatory", "AATCAACCTCTGGATTACAAAATTTGTGAAAGATTGACTGGTATTCTTAACTATGTTGCTCCTTTTACGCTATGTGGATACGCTGCTTTAATGCCTTTGTATCATGCTATTGCTTCCCGTATGGCTTTCATTTTCTCCTCCTTGTATAAATCCTGGTTGCTGTCTCTTTATGAGGAGTTGTGGCCCGTTGTCAGGCAACGTGGCGTGGTGTGCACTGTGTTTGCTGACGCAACCCCCACTGGTTGGGGCATTGCCACCACCTGTCAGCTCCTTTCCGGGACTTTCGCTTTCCCCCTCCCTATTGCCACGGCGGAACTCATCGCCGCCTGCCTTGCCCGCTGCTGGACAGGGGCTCGGCTGTTGGGCACTGACAATTCCGTGGTGTTGTCGGGGAAATCATCGTCCTTTCCTTGGCTGCTCGCCTGTGTTGCCACCTGGATTCTGCGCGGGACGTCCTTCTGCTACGTCCCTTCGGCCCTCAATCCAGCGGACCTTCCTTCCCGCGGCCTGCTGCCGGCTCTGCGGCCTCTTCCGCGTCTTCGCCTTCGCCCTCAGACGAGTCGGATCTCCCTTTGGGCCGCCTCCCCGC", "Woodchuck hepatitis post-transcriptional regulatory element"),
        ];

        for (name, ptype, seq, desc) in defaults {
            let id = Uuid::new_v4().to_string();
            let conn = self.conn.lock().unwrap();
            conn.execute(
                "INSERT INTO parts (id, name, part_type, sequence, description) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![id, name, ptype, seq, desc],
            )
            .map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}
