use crate::commands::WholeStreamCommand;
use crate::prelude::*;
use nu_errors::ShellError;
use nu_protocol::{Signature, SyntaxShape};
use nu_source::Tagged;
use std::path::PathBuf;

pub struct Ls;

#[derive(Deserialize)]
pub struct LsArgs {
    pub path: Option<Tagged<PathBuf>>,
    pub all: bool,
    pub full: bool,
    #[serde(rename = "short-names")]
    pub short_names: bool,
    #[serde(rename = "with-symlink-targets")]
    pub with_symlink_targets: bool,
    #[serde(rename = "du")]
    pub du: bool,
}

impl WholeStreamCommand for Ls {
    fn name(&self) -> &str {
        "ls"
    }

    fn signature(&self) -> Signature {
        Signature::build("ls")
            .optional(
                "path",
                SyntaxShape::Pattern,
                "a path to get the directory contents from",
            )
            .switch("all", "also show hidden files", Some('a'))
            .switch(
                "full",
                "list all available columns for each entry",
                Some('f'),
            )
            .switch(
                "short-names",
                "only print the file names and not the path",
                Some('s'),
            )
            .switch(
                "with-symlink-targets",
                "display the paths to the target files that symlinks point to",
                Some('w'),
            )
            .switch(
                "du",
                "display the apparent directory size in place of the directory metadata size",
                Some('d'),
            )
    }

    fn usage(&self) -> &str {
        "View the contents of the current or given path."
    }

    fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        ls(args, registry)
    }

    fn examples(&self) -> &[Example] {
        &[
            Example {
                description: "List all files in the current directory",
                example: "ls",
            },
            Example {
                description: "List all files in a subdirectory",
                example: "ls subdir",
            },
            Example {
                description: "List all rust files",
                example: "ls *.rs",
            },
        ]
    }
}

fn ls(args: CommandArgs, registry: &CommandRegistry) -> Result<OutputStream, ShellError> {
    let registry = registry.clone();
    let stream = async_stream! {
        let name = args.call_info.name_tag.clone();
        let ctrl_c = args.ctrl_c.clone();
        let shell_manager = args.shell_manager.clone();
        let (args, _) = args.process(&registry).await?;
        let mut result = shell_manager.ls(args, name, ctrl_c)?;

        while let Some(item) = result.next().await {
            yield item;
        }
    };

    Ok(stream.to_output_stream())
}
