use crate::commands::WholeStreamCommand;
use crate::prelude::*;
use nu_errors::ShellError;
use nu_protocol::Signature;

pub struct Pwd;

impl WholeStreamCommand for Pwd {
    fn name(&self) -> &str {
        "pwd"
    }

    fn signature(&self) -> Signature {
        Signature::build("pwd")
    }

    fn usage(&self) -> &str {
        "Output the current working directory."
    }

    fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        pwd(args, registry)
    }

    fn examples(&self) -> &[Example] {
        &[Example {
            description: "Print the current working directory",
            example: "pwd",
        }]
    }
}

pub fn pwd(args: CommandArgs, registry: &CommandRegistry) -> Result<OutputStream, ShellError> {
    let registry = registry.clone();

    let stream = async_stream! {
        let shell_manager = args.shell_manager.clone();
        let args = args.evaluate_once(&registry).await?;
        let mut out = shell_manager.pwd(args)?;

        while let Some(l) = out.next().await {
            yield l;
        }
    };

    Ok(stream.to_output_stream())
}
