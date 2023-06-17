use std::path::PathBuf;


/// Holds the data that dictate how the program will run
#[derive(PartialEq, Debug)]
pub struct Config {
    pub directory_to_copy: PathBuf,
    pub target_directory: PathBuf,
}

impl Config {
    /// Builds an instance of `Config` via enviroment arguments
    /// 
    /// # Failure
    /// Will fail if there are no more then 3 arguments passed in
    /// 
    /// (2 enviroment arguments)
    pub fn build_from_args(args: &Vec<String>) -> Option<Config> {
        let directory_to_copy = PathBuf::from(args.get(1)?.clone());
        let target_directory = PathBuf::from(args.get(2)?.clone());
        Some(Config {
            directory_to_copy,
            target_directory,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build_from_args_empty() {
        assert_eq!(Config::build_from_args(&vec![]), None);
    }

    #[test]
    fn config_build_from_args_working() {
        let args = vec!["C:\\Path", "C:\\Directory", "D:\\"].iter()
            .map(|s| s.to_string()).collect::<Vec<String>>();

        let expected_directory_to_copy = PathBuf::from("C:\\Directory");
        let expected_target_directory = PathBuf::from("D:\\");

        let config = Config::build_from_args(&args).unwrap();

        assert_eq!(config.directory_to_copy, expected_directory_to_copy);
        assert_eq!(config.target_directory, expected_target_directory);
    }
}
