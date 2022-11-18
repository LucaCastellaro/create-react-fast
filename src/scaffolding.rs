use crate::{yarn_utils, ts_utils};

use std::{
    fs, 
    // fs::File,
    env, 
    path::Path
};

pub fn scaffold() -> Result<bool, String> {
    let result = create_git_ignore();
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(value) => if !value {return Ok(value)}
    }

    let result = ts_utils::create_tsconfig_json();
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(value) => if !value {return Ok(value)}
    }

    // start yarn
    
    let result = yarn_utils::init();
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(value) => if !value {return Ok(value)}
    }

    let result = yarn_utils::add_scripts();
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(value) => if !value {return Ok(value)}
    }

    let result = yarn_utils::add_browsers();
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(value) => if !value {return Ok(value)}
    }

    let result = yarn_utils::add_packages();
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(value) => if !value {return Ok(value)}
    }

    // end yarn

    // start src

    let result = create_folder(&"src".to_string());
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(value) => if !value {return Ok(value)}
    }

    let result = create_app_tsx();
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(value) => if !value {return Ok(value)}
    }

    let result = create_app_css();
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(value) => if !value {return Ok(value)}
    }

    let result = create_index_tsx();
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(value) => if !value {return Ok(value)}
    }

    let result = create_index_css();
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(value) => if !value {return Ok(value)}
    }

    let result = create_react_app_env_d_ts();
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(value) => if !value {return Ok(value)}
    }

    // end src

    // start public

    let folder = "../public".to_string();

    let result = create_folder(&folder);
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(value) => if !value {return Ok(value)}
    }

    let result = create_public_index_html();
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(value) => if !value {return Ok(value)}
    }

    // end public

    return Ok(true);
}

pub fn create_folder(title: &String) -> Result<bool, String> {
    let result = fs::create_dir(title);
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(_) => return change_dir(Path::new(title))
    }
}

fn create_git_ignore() -> Result<bool, String> {
    let mut lines: Vec<&str> = Vec::new();
    lines.push("# See https://help.github.com/articles/ignoring-files/ for more about ignoring files.\n");
    lines.push("# dependencies");
    lines.push("/node_modules");
    lines.push("/.pnp");
    lines.push(".pnp.js\n");

    lines.push("# testing");
    lines.push("/coverage\n");
    
    lines.push("# production");
    lines.push("/build\n");

    lines.push("# misc");
    lines.push(".DS_Store");
    lines.push(".env.local");
    lines.push(".env.development.local");
    lines.push(".env.test.local");
    lines.push(".env.production.local");
    lines.push("npm-debug.log*");
    lines.push("yarn-debug.log*");
    lines.push("yarn-error.log*\n");

    let lines = lines.join("\n").replace("  ", "\t");

    let result = fs::write(".gitignore", lines);
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Ok(true)
    }
}

fn create_app_tsx() -> Result<bool, String> {
    let mut lines: Vec<&str> = Vec::new();
    lines.push("import React from 'react';\nimport './App.css';\n");
    lines.push("export default function App(): JSX.Element {");
    lines.push("\treturn (");

    lines.push("\t\t<div className=\"App\">");
    lines.push("\t\t\t<header className=\"App-header\">");
    lines.push("\t\t\t\t<p>");
    lines.push("\t\t\t\t\tEdit <code>src/App.tsx</code> and save to reload.");
    lines.push("\t\t\t\t</p>");
    lines.push("\t\t\t\t<a className=\"App-link\" href=\"https://reactjs.org\" target=\"_blank\" rel=\"noopener noreferrer\">Learn React</a>");
    lines.push("\t\t\t</header>");
    lines.push("\t\t</div>");
    
    lines.push("\t);\n}");

    let lines = lines.join("\n");

    let result = fs::write("App.tsx", lines);
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Ok(true)
    }
}

fn create_app_css() -> Result<bool, String> {
    let mut lines: Vec<&str> = Vec::new();
    lines.push(".App {\n\ttext-align: center;\n}\n");
    lines.push(".App-header {\n\tbackground-color: #282c34;\n\tmin-height: 100vh;\n\tdisplay: flex;\n\tflex-direction: column;\n\talign-items: center;\n\tjustify-content: center;\n\tfont-size: calc(10px + 2vmin);\n\tcolor: white;\n}\n");
    lines.push(".App-link {\n\tcolor: #61dafb;\n}\n");

    let lines = lines.join("\n");

    let result = fs::write("App.css", lines);
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Ok(true)
    }
}

fn change_dir(path: &Path) -> Result<bool, String> {
    let result = env::set_current_dir(path);
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Ok(true)
    }
}

fn create_index_tsx() -> Result<bool, String> {
    let mut lines: Vec<&str> = Vec::new();
    lines.push("import React from 'react';\nimport ReactDOM from 'react-dom/client';\nimport './index.css';\nimport App from './App';\n");
    lines.push("const root = ReactDOM.createRoot(\n\tdocument.getElementById('root') as HTMLElement\n);\n");
    lines.push("root.render(\n\t<React.StrictMode>\n\t\t<App />\n\t</React.StrictMode>\n);");

    let lines = lines.join("\n");

    let result = fs::write("index.tsx", lines);
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Ok(true)
    }
}

fn create_index_css() -> Result<bool, String> {
    let mut lines: Vec<&str> = Vec::new();
    lines.push("body {\n\tmargin: 0;\n\tfont-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;\n\t-webkit-font-smoothing: antialiased;\n\t-moz-osx-font-smoothing: grayscale;\n}\n");
    lines.push("code {\n\tfont-family: source-code-pro, Menlo, Monaco, Consolas, 'Courier New', monospace;\n}\n");

    let lines = lines.join("\n");

    let result = fs::write("index.css", lines);
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Ok(true)
    }
}

fn create_react_app_env_d_ts() -> Result<bool, String> {
    let result = fs::write("react-app-env.d.ts", "/// <reference types=\"react-scripts\" />\n".to_string());
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Ok(true)
    }
}

fn create_public_index_html() -> Result<bool, String> {
    let mut lines: Vec<&str> = Vec::new();
    lines.push("<!DOCTYPE html>\n<html lang=\"en\">");
    lines.push("\t<head>");
    lines.push("\t\t<meta charset=\"utf-8\" />");
    lines.push("\t\t<link rel=\"icon\" href=\"%PUBLIC_URL%/favicon.ico\" />");
    lines.push("\t\t<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />");
    lines.push("\t\t<meta name=\"theme-color\" content=\"#000000\" />");
    lines.push("\t\t<meta name=\"description\" ontent=\"Web site created using create-react-app\" >");
    lines.push("\t\t<link rel=\"apple-touch-icon\" href=\"%PUBLIC_URL%/logo192.png\" />");
    lines.push("\t\t<link rel=\"manifest\" href=\"%PUBLIC_URL%/manifest.json\" />");
    lines.push("\t\t<title>React App</title>");
    lines.push("\t</head>");
    lines.push("\t<body>");
    lines.push("\t\t<noscript>You need to enable JavaScript to run this app.</noscript>");
    lines.push("\t\t<div id=\"root\"></div>");
    lines.push("\t</body>");
    lines.push("</html>");

    let lines = lines.join("\n");

    let result = fs::write("index.html", lines);
    match result {
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Ok(true)
    }
}
