pub struct Config<'a> {
    query: &'a str,
    path: &'a str,
}

impl<'a> Config<'a> {
    /// Build a new Config from arg slice
    pub fn build(args: &'a [String]) -> Result<Self, ArgError> {
        // Raise error on insufficient arguments
        if args.len() < 3 {
            return Err(ArgError {
                msg: "Usage: minigrep <query> <path>",
            });
        }

        // Return config on success
        Ok(Self {
            query: &args[1],
            path: &args[2],
        })
    }

    /// Return the query attribute
    pub fn query(&self) -> &str {
        self.query
    }

    /// Return the path attribute
    pub fn path(&self) -> &str {
        self.path
    }
}

pub struct ArgError {
    msg: &'static str,
}

impl ArgError {
    /// Return the error message
    pub fn msg(&self) -> &'static str {
        self.msg
    }
}
