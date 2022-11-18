use std::{
    fs
};

pub fn create_tsconfig_json() -> Result<bool, String> {
    let mut lines: Vec<&str> = Vec::new();

    lines.push("{\n\t\"compilerOptions\": {\n\t\t\"target\": \"es5\",\n\t\t\"lib\": [\n\t\t\t\"dom\",\n\t\t\t\"dom.iterable\",\n\t\t\t\"esnext\"\n\t\t],");
    lines.push("\t\t\"allowJs\": true,\n\t\t\"skipLibCheck\": true,\n\t\t\"esModuleInterop\": true,\n\t\t\"allowSyntheticDefaultImports\": true,\n\t\t\"strict\": true,");
    lines.push("\t\t\"forceConsistentCasingInFileNames\": true,\n\t\t\"noFallthroughCasesInSwitch\": true,\n\t\t\"module\": \"esnext\",\n\t\t\"moduleResolution\": \"node\",");
    lines.push("\t\t\"resolveJsonModule\": true,\n\t\t\"isolatedModules\": true,\n\t\t\"noEmit\": true,\n\t\t\"jsx\": \"react-jsx\"\n\t},\n\t\"include\": [\n\t\t\"src\"\n\t]\n}");

    let lines = lines.join("\n");

    let result = fs::write("tsconfig.json", lines);
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Ok(true)
    }
}