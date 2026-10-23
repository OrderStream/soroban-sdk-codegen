use clap::{Parser, Subcommand};
use soroban_sdk_codegen::{ContractSpec, PythonGenerator, TypeScriptGenerator};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "soroban-sdk-codegen",
    author = "OrderStream <healthdecoded77@gmail.com>",
    version = "0.1.0",
    about = "Multi-language client SDK generator CLI for Stellar Soroban contracts"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generates TypeScript and/or Python client SDKs from Soroban WASM or JSON spec.
    Generate {
        #[arg(short, long)]
        wasm: Option<PathBuf>,

        #[arg(short, long)]
        spec: Option<PathBuf>,

        #[arg(short, long, default_value = "all")]
        lang: String, // "ts", "python", "all"

        #[arg(short, long, default_value = "./generated_sdk")]
        out_dir: PathBuf,
    },

    /// Inspects and dumps the contract spec as formatted JSON.
    InspectSpec {
        #[arg(short, long)]
        wasm: Option<PathBuf>,

        #[arg(short, long)]
        spec: Option<PathBuf>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            wasm,
            spec,
            lang,
            out_dir,
        } => {
            let contract_spec = load_spec(wasm, spec)?;
            fs::create_dir_all(&out_dir)?;

            let lang_lower = lang.to_lowercase();
            if lang_lower == "ts" || lang_lower == "all" {
                let ts_code = TypeScriptGenerator::generate(&contract_spec);
                let ts_path = out_dir.join(format!("{}.client.ts", contract_spec.contract_name));
                fs::write(&ts_path, ts_code)?;
                println!("📘 Generated TypeScript client SDK at: {}", ts_path.display());
            }

            if lang_lower == "python" || lang_lower == "all" {
                let py_code = PythonGenerator::generate(&contract_spec);
                let py_path = out_dir.join(format!("{}_client.py", contract_spec.contract_name.to_lowercase()));
                fs::write(&py_path, py_code)?;
                println!("🐍 Generated Python client SDK at: {}", py_path.display());
            }

            println!("✨ Successfully generated client SDKs for `{}`", contract_spec.contract_name);
        }

        Commands::InspectSpec { wasm, spec } => {
            let contract_spec = load_spec(wasm, spec)?;
            println!("{}", contract_spec.to_json_pretty()?);
        }
    }

    Ok(())
}

fn load_spec(wasm: Option<PathBuf>, spec: Option<PathBuf>) -> Result<ContractSpec, Box<dyn std::error::Error>> {
    if let Some(spec_path) = spec {
        let content = fs::read_to_string(&spec_path)?;
        let s = ContractSpec::from_json(&content)?;
        return Ok(s);
    }

    if let Some(wasm_path) = wasm {
        let bytes = fs::read(&wasm_path)?;
        let s = ContractSpec::extract_from_wasm(&bytes)?;
        return Ok(s);
    }

    // Default synthetic spec if neither specified
    let mut default_spec = ContractSpec::new("VaultContract");
    default_spec.functions.push(soroban_sdk_codegen::ContractFunction {
        name: "deposit".into(),
        doc: Some("Deposits tokens".into()),
        inputs: vec![
            soroban_sdk_codegen::FunctionArg {
                name: "from".into(),
                arg_type: soroban_sdk_codegen::SpecType::Address,
                doc: None,
            },
            soroban_sdk_codegen::FunctionArg {
                name: "amount".into(),
                arg_type: soroban_sdk_codegen::SpecType::I128,
                doc: None,
            },
        ],
        output: soroban_sdk_codegen::SpecType::Bool,
    });
    Ok(default_spec)
}
