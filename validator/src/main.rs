// Std
use std::env;
use std::error::Error;
use std::fs::File;
use std::process;

// External
use chrono::NaiveDate;

// Column metadata
struct DataCol {
    name: &'static str,
    required: bool,
    enum_type: bool,
    enum_values: Option<&'static [&'static str]>,
    date_type: bool,
}

fn get_schema(schema_name: &str) -> Result<&[DataCol], Box<dyn Error>> {
    let cols: &[DataCol];

    match schema_name {
        "hospital_price_transparency" => {
            // Columns
            cols = &[
                DataCol {
                    name: "reporting_entity_name_legal",
                    required: false,
                    enum_type: false,
                    enum_values: None,
                    date_type: false,
                },
                DataCol {
                    name: "reporting_entity_name_common",
                    required: true,
                    enum_type: false,
                    enum_values: None,
                    date_type: false,
                },
                DataCol {
                    name: "reporting_entity_type",
                    required: false,
                    enum_type: true,
                    enum_values: Some(&["hospital", "other"]),
                    date_type: false,
                },
                DataCol {
                    name: "machine_readable_url",
                    required: true,
                    enum_type: false,
                    enum_values: None,
                    date_type: false,
                },
                DataCol {
                    name: "machine_readable_url_status",
                    required: true,
                    enum_type: true,
                    enum_values: Some(&["up", "down", "corrupt"]),
                    date_type: false,
                },
                DataCol {
                    name: "machine_readable_page",
                    required: false,
                    enum_type: false,
                    enum_values: None,
                    date_type: false,
                },
                DataCol {
                    name: "supplemental_url",
                    required: false,
                    enum_type: false,
                    enum_values: None,
                    date_type: false,
                },
                DataCol {
                    name: "file_name",
                    required: false,
                    enum_type: false,
                    enum_values: None,
                    date_type: false,
                },
                DataCol {
                    name: "file_format",
                    required: true,
                    enum_type: true,
                    enum_values: Some(&[
                        "csv", "json", "xml", "xlsx", "zip/csv", "zip/json", "zip/xml", "zip/xlsx",
                        "other",
                    ]),
                    date_type: false,
                },
                DataCol {
                    name: "meets_standard",
                    required: false,
                    enum_type: true,
                    enum_values: Some(&["true", "false"]),
                    date_type: false,
                },
                DataCol {
                    name: "standard_issue",
                    required: false,
                    enum_type: false,
                    enum_values: None,
                    date_type: false,
                },
                DataCol {
                    name: "state_or_region",
                    required: false,
                    enum_type: true,
                    enum_values: Some(&[
                        "AL", "AK", "AS", "AZ", "AR", "CA", "CO", "CT", "DE", "DC", "FL", "GA",
                        "GU", "HI", "ID", "IL", "IN", "IA", "KS", "KY", "LA", "ME", "MD", "MA",
                        "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH", "NJ", "NM", "NY", "NC",
                        "ND", "MP", "OH", "OK", "OR", "PA", "PR", "RI", "SC", "SD", "TN", "TX",
                        "UT", "VT", "VI", "VA", "WA", "WV", "WI", "WY",
                    ]),
                    date_type: false,
                },
                DataCol {
                    name: "last_updated_date",
                    required: false,
                    enum_type: false,
                    enum_values: None,
                    date_type: true,
                },
                DataCol {
                    name: "entry_date",
                    required: true,
                    enum_type: false,
                    enum_values: None,
                    date_type: true,
                },
                DataCol {
                    name: "notes",
                    required: false,
                    enum_type: false,
                    enum_values: None,
                    date_type: false,
                },
            ];
        }
        "insurer_price_transparency" => {
            // Columns
            cols = &[
                DataCol {
                    name: "reporting_entity_name_legal",
                    required: false,
                    enum_type: false,
                    enum_values: None,
                    date_type: false,
                },
                DataCol {
                    name: "reporting_entity_name_common",
                    required: true,
                    enum_type: false,
                    enum_values: None,
                    date_type: false,
                },
                DataCol {
                    name: "reporting_entity_type",
                    required: false,
                    enum_type: true,
                    enum_values: Some(&["insurer", "other"]),
                    date_type: false,
                },
                DataCol {
                    name: "machine_readable_url",
                    required: true,
                    enum_type: false,
                    enum_values: None,
                    date_type: false,
                },
                DataCol {
                    name: "machine_readable_url_status",
                    required: true,
                    enum_type: true,
                    enum_values: Some(&["up", "down", "corrupt"]),
                    date_type: false,
                },
                DataCol {
                    name: "machine_readable_page",
                    required: false,
                    enum_type: false,
                    enum_values: None,
                    date_type: false,
                },
                DataCol {
                    name: "supplemental_url",
                    required: false,
                    enum_type: false,
                    enum_values: None,
                    date_type: false,
                },
                DataCol {
                    name: "file_name",
                    required: false,
                    enum_type: false,
                    enum_values: None,
                    date_type: false,
                },
                DataCol {
                    name: "file_format",
                    required: true,
                    enum_type: true,
                    enum_values: Some(&[
                        "csv", "json", "xml", "xlsx", "zip/csv", "zip/json", "zip/xml", "zip/xlsx",
                        "other",
                    ]),
                    date_type: false,
                },
                DataCol {
                    name: "meets_standard",
                    required: false,
                    enum_type: true,
                    enum_values: Some(&["true", "false"]),
                    date_type: false,
                },
                DataCol {
                    name: "standard_issue",
                    required: false,
                    enum_type: false,
                    enum_values: None,
                    date_type: false,
                },
                DataCol {
                    name: "state_or_region",
                    required: false,
                    enum_type: true,
                    enum_values: Some(&[
                        "AL", "AK", "AS", "AZ", "AR", "CA", "CO", "CT", "DE", "DC", "FL", "GA",
                        "GU", "HI", "ID", "IL", "IN", "IA", "KS", "KY", "LA", "ME", "MD", "MA",
                        "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH", "NJ", "NM", "NY", "NC",
                        "ND", "MP", "OH", "OK", "OR", "PA", "PR", "RI", "SC", "SD", "TN", "TX",
                        "UT", "VT", "VI", "VA", "WA", "WV", "WI", "WY",
                    ]),
                    date_type: false,
                },
                DataCol {
                    name: "last_updated_date",
                    required: true,
                    enum_type: false,
                    enum_values: None,
                    date_type: true,
                },
            ];
        }
        _ => return Err(format!("\"{}\" is not a valid schema name", schema_name))?,
    }

    return Ok(cols);
}

fn main() {
    // Parse command line args
    let args: Vec<String> = env::args().collect();
    let (file_path, schema_name): (&str, &str);
    match parse_args(&args) {
        Err(err) => {
            println!("ERROR:\n\n{}", err);
            process::exit(1);
        }
        Ok(res) => {
            (file_path, schema_name) = res;
        }
    }

    // Get relevant schema
    let schema_result = get_schema(schema_name);
    let cols: &[DataCol];
    match schema_result {
        Err(err) => {
            println!("ERROR:\n\n{}", err);
            process::exit(1);
        }
        Ok(schema) => {
            cols = schema;
        }
    }

    // Validate specified csv based on schema
    if let Err(err) = validate_csv(cols, file_path) {
        println!("ERROR:\n\n{}", err);
        process::exit(1);
    }
}

fn parse_args(args: &[String]) -> Result<(&str, &str), Box<dyn Error>> {
    if args.len() != 3 {
        return Err(
            "This utility script accepts exactly two arguments. A csv path, and a schema name.",
        )?;
    }

    let file_path = &args[1];
    let schema_name = &args[2];

    Ok((file_path, schema_name))
}

fn validate_csv(cols: &[DataCol], file_path: &str) -> Result<(), Box<dyn Error>> {
    // file handle and reader
    let file = File::open(file_path)?;
    let mut reader = csv::Reader::from_reader(file);
    let mut line: u32 = 1;

    // Collect intended headers
    let mut intended_headers: Vec<String> = vec![];
    for col in cols {
        intended_headers.push(col.name.to_string())
    }
    // Validate headers
    if let Err(err) = validate_headers(&mut reader, intended_headers) {
        return Err(From::from(err));
    }

    // Check each result, return read errors
    for result in reader.records() {
        line += 1;
        match result {
            Err(err) => return Err(From::from(err)),
            Ok(record) => {
                if let Err(err) = validate_record(record, line, cols) {
                    return Err(From::from(err));
                };
            }
        }
    }
    return Ok(());
}

fn validate_headers(
    reader: &mut csv::Reader<std::fs::File>,
    intended_headers: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let headers = reader.headers()?;
    let intended_headers = csv::StringRecord::from(intended_headers);
    let new_len = headers.len();
    let old_len = intended_headers.len();
    if new_len != old_len {
        return Err(format!("Expected {} header cols, but found {}.\nPlease don't propose alterations to the headers in the same PR as a data submission.\n", old_len, new_len))?;
    }
    for (idx, header) in headers.iter().enumerate() {
        let intended_header = &intended_headers[idx];
        if header != intended_header {
            return Err(format!("Expected header \"{}\" in column {} but found \"{}\".\nPlease don't alter the headers in a data submission.\n", intended_header, idx, header))?;
        }
    }
    return Ok(());
}

fn validate_record(
    record: csv::StringRecord,
    line: u32,
    cols: &[DataCol],
) -> Result<(), Box<dyn Error>> {
    for (idx, col) in cols.iter().enumerate() {
        let field = &record[idx];

        // Disallow beginning and trailng whitespace
        let trimmed_field = str::trim(field);
        if field != trimmed_field {
            return Err(format!(
                "Remove beginning or trailing whitespace from entry in column \"{}\" on line {}\n",
                col.name, line
            ))?;
        }

        if col.required {
            // Disallow empty
            if !col.enum_type {
                let disallowed = "";
                if trimmed_field == disallowed {
                    return Err(format!("Invalid entry: \"{}\" specified in required column \"{}\". Line {} does not abide.\n", field, col.name, line))?;
                }
            }
            if col.enum_type {
                let enum_values = col.enum_values.unwrap();
                if !enum_values.contains(&field) {
                    return Err(format!(
                        "Invalid entry: \"{}\" specified in required column \"{}\" on line {}.\n",
                        field, col.name, line
                    ))?;
                }
            }
        } else {
            if col.enum_type {
                let enum_values = col.enum_values.unwrap();
                if !enum_values.contains(&field) && &field != &"" {
                    return Err(format!(
                        "Invalid entry: \"{}\" specified in column \"{}\" on line {}.\n",
                        field, col.name, line
                    ))?;
                }
            }
        }

        if col.date_type {
            if field != "" {
                if NaiveDate::parse_from_str(field, "%Y-%m-%d").is_err() {
                    return Err(format!("Invalid date format entry: \"{}\" specified in column \"{}\" on line {}.\n Please use YYYY-MM-DD format.\n", field, col.name, line))?;
                }
            }
        }
    }
    return Ok(());
}
