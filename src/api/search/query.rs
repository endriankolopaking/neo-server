const COLUMNS: [&str; 62] = [
  "Research Scope Aim of Research",
  "Period of Study",
  "Period of Study_Year",
  "Location of Work",
  "Relevant Water Body_Detailed",
  "Relevant Water Body_General",
  "Coastal or Offshore",
  "Plastic Examined_Size or Type",
  "Adopted GESAMP Size",
  "Microplastic Sizes",
  "Contaminants Examined",
  "Fishing Gear Examined",
  "Legal/Regulatory Study",
  "Social/Cultural Study",
  "Economic/Management Study",
  "Policy Study",
  "Research Methodology Methodologies Used",
  "Field Sampling_Conducted",
  "Field Sampling_Compartment",
  "Field Sampling_Frequency",
  "Survey/Interview_Conducted",
  "Other Sampling_Type",
  "Biota_Species",
  "Biota (Phyllum)",
  "Biota_Applied",
  "Common names",
  "Literature Review_Conducted",
  "Literature Review_Volume",
  "Desktop / Deductive analysis",
  "Modelling_Conducted",
  "Modelling_Type",
  "Plankton Net_Mesh Size",
  "Water Sampling_Depth",
  "Shoreline Sediment Sampling_ Depth",
  "Seabed Sediment Sampling_Depth",
  "Mangrove/Mudflat Sediment Sampling_Depth",
  "Controls_Blanks",
  "Research Findings Key Findings",
  "Source of Plastics",
  "Source of Plastics_General",
  "Research Topics",
  "Plastic Characterisation_Conducted",
  "Plastic Characterisation_Colour",
  "Plastic Characterisation_Colours Found",
  "Plastic Characterisation_Shape",
  "Plastic Characterisation_Shapes Found",
  "Plastic Characterisation_Polymer",
  "Plastic Characterisation_Polymers Found",
  "Macro_Uses",
  "Macro_Mean Abundance_Count",
  "Macro_Mean Abundance_Weight",
  "Water_Mean Abundance_Count",
  "Water_Mean Abundance_Weight",
  "Shoreline Sediment_Mean Abundance_Count",
  "Shoreline Sediment_Mean Abundance_Weight",
  "Seabed Sediment_Mean Abundance_Count",
  "Seabed Sediment_Mean Abundance_Weight",
  "Mangrove_Mean Abundance_Count",
  "Mangrove_Mean Abundance_Weight",
  "Biota_Mean Abundance_Count",
  "Biota_Mean Abundance_Weight",
  "Degradation Indicated",
];

pub fn generate(obj: serde_json::Map<String, serde_json::Value>) -> String {
  let mut query = "SELECT * FROM data WHERE regexp('.', [ID])".to_string();

  for (_key, value) in obj {
    let object = value
      .as_object()
      .expect("Failed to convert to object");
    let pattern = serde_json::to_string(&object["pattern"])
      .expect("Failed to convert pattern");
    let pattern = pattern.trim_matches('"');

    let column = serde_json::to_string(&object["column"])
      .expect("Failed to convert column");
    let column = column.trim_matches('"');

    query = match column {
      "W:CF" => query + " " + &range_query(pattern.to_string()),
      _ => query + " " + &single_query(column, pattern),
    };
  }

  query
}

pub fn range_query(pattern: String) -> String {
  let mut result: String = "".to_string();

  for (index, value) in COLUMNS
    .iter()
    .enumerate()
  {
    if index == 0 {
      result += format!(
        "AND ( regexp('{}', [{}])",
        pattern, value
      )
      .as_str();
    } else {
      result += format!(
        " OR regexp('{}', [{}])",
        pattern, value
      )
      .as_str();
    }
  }
  result += " )";
  result
}

fn single_query(
  col: &str,
  pattern: &str,
) -> String {
  let result = format!("AND regexp('{pattern}', [{col}])");
  result
}
